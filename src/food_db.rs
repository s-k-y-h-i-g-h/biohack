//! Food database integration for nutrient lookup
//!
//! This module provides clients for querying both USDA FoodData Central API
//! and OpenFoodFacts API to retrieve nutrient information for foods logged
//! via `biohack log food`.
//!
//! Priority: OpenFoodFacts (primary for UK branded foods) → USDA (fallback for generic)

use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::FoodDataSource;

/// USDA FoodData Central API base URL
const FDC_API_BASE: &str = "https://api.nal.usda.gov/fdc/v1";

/// OpenFoodFacts API base URL
const OFF_API_BASE: &str = "https://world.openfoodfacts.org";

/// Maximum search results to return
const MAX_SEARCH_RESULTS: usize = 10;

/// Client for food database APIs
#[derive(Debug, Clone)]
pub struct FoodDbClient {
    http: Client,
    usda_api_key: Option<String>,
    cache: Arc<RwLock<HashMap<String, CachedFood>>>,
}

/// Cached food entry with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedFood {
    food: FoodResult,
    cached_at: chrono::DateTime<chrono::Utc>,
}

/// Unified food search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodResult {
    /// Internal ID (FDC ID for USDA, barcode for OFF)
    pub id: String,
    pub description: String,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub source: FoodDataSource,
    /// Nutrients per 100g (when available)
    pub nutrients_per_100g: Option<Vec<FoodNutrient>>,
}

/// Nutrient within a food
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodNutrient {
    pub nutrient_id: i64,
    pub name: String,
    pub unit: String,
    pub amount: f64,
}

/// Simplified nutrient info for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientInfo {
    pub name: String,
    pub amount: f64,
    pub unit: String,
}

/// Key nutrients we care about for biohacking display (matches USDA nutrient IDs)
pub const KEY_NUTRIENTS: &[(&str, &str, i64)] = &[
    // (display name, unit, nutrient_id)
    ("Energy", "kcal", 1008),
    ("Protein", "g", 1003),
    ("Total Fat", "g", 1004),
    ("Carbohydrate", "g", 1005),
    ("Fiber", "g", 1079),
    ("Sugars", "g", 2000),
    ("Calcium", "mg", 1087),
    ("Iron", "mg", 1089),
    ("Magnesium", "mg", 1090),
    ("Phosphorus", "mg", 1091),
    ("Potassium", "mg", 1092),
    ("Sodium", "mg", 1093),
    ("Zinc", "mg", 1095),
    ("Copper", "mg", 1098),
    ("Manganese", "mg", 1101),
    ("Selenium", "µg", 1103),
    ("Vitamin C", "mg", 1162),
    ("Thiamin (B1)", "mg", 1165),
    ("Riboflavin (B2)", "mg", 1166),
    ("Niacin (B3)", "mg", 1167),
    ("Vitamin B6", "mg", 1175),
    ("Folate", "µg", 1177),
    ("Vitamin B12", "µg", 1178),
    ("Vitamin A (RAE)", "µg", 1106),
    ("Vitamin E", "mg", 1109),
    ("Vitamin D", "µg", 1114),
    ("Vitamin K", "µg", 1185),
    ("Omega-3 (DHA)", "g", 1402),
    ("Omega-3 (EPA)", "g", 1401),
    ("Omega-3 (ALA)", "g", 1399),
    ("Omega-6 (Linoleic)", "g", 1404),
    ("Cholesterol", "mg", 1253),
    ("Saturated Fat", "g", 1258),
    ("Monounsaturated Fat", "g", 1292),
    ("Polyunsaturated Fat", "g", 1293),
];

impl FoodDbClient {
    /// Create a new FoodDB client with optional USDA API key
    pub fn new(usda_api_key: Option<String>) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("biohack/0.1.0 (https://github.com/s-k-y-h-i-g-h/biohack)")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http,
            usda_api_key,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Search for foods by query string - tries OpenFoodFacts first, then USDA
    pub async fn search_foods(&self, query: &str) -> Result<Vec<FoodResult>> {
        let mut all_results = Vec::new();

        // 1. Try OpenFoodFacts first (primary for UK branded foods)
        if let Ok(off_results) = self.search_openfoodfacts(query).await {
            all_results.extend(off_results);
        }

        // 2. Fall back to USDA if we have API key
        if self.usda_api_key.is_some() {
            if let Ok(usda_results) = self.search_usda(query).await {
                all_results.extend(usda_results);
            }
        }

        Ok(all_results)
    }

    /// Search OpenFoodFacts by product name
    async fn search_openfoodfacts(&self, query: &str) -> Result<Vec<FoodResult>> {
        let url = format!("{}/cgi/search.pl", OFF_API_BASE);
        let response = self
            .http
            .get(&url)
            .query(&[
                ("search_terms", query),
                ("search_simple", "1"),
                ("action", "process"),
                ("json", "1"),
                ("page_size", &MAX_SEARCH_RESULTS.to_string()),
            ])
            .send()
            .await
            .context("Failed to send search request to OpenFoodFacts API")?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("OpenFoodFacts API error: {}: {}", status, text));
        }

        #[derive(Deserialize)]
        struct OffSearchResponse {
            products: Vec<OffProduct>,
        }

        #[derive(Deserialize)]
        struct OffProduct {
            code: String,
            product_name: Option<String>,
            brands: Option<String>,
            categories: Option<String>,
            #[serde(rename = "nutriments")]
            nutrients: Option<serde_json::Value>,
        }

        let text = response.text().await.context("Failed to get response text")?;

        let search_response: OffSearchResponse = serde_json::from_str(&text)
            .context("Failed to parse OpenFoodFacts API response")?;

        let mut results = Vec::new();
        for product in search_response.products {
            if let Some(name) = product.product_name {
                let nutrients = self.parse_off_nutrients(&product.nutrients);
                results.push(FoodResult {
                    id: product.code,
                    description: name,
                    brand: product.brands,
                    category: product.categories,
                    source: FoodDataSource::OpenFoodFacts,
                    nutrients_per_100g: nutrients,
                });
            }
        }

        Ok(results)
    }

    /// Parse OpenFoodFacts nutrients (per 100g) into our format
    fn parse_off_nutrients(&self, nutrients: &Option<serde_json::Value>) -> Option<Vec<FoodNutrient>> {
        let nutrients = nutrients.as_ref()?;
        let mut result = Vec::new();

        // Map OpenFoodFacts nutrient fields to our nutrient IDs
        let nutrient_map: &[(&str, i64, &str)] = &[
            ("energy-kcal_100g", 1008, "kcal"),
            ("proteins_100g", 1003, "g"),
            ("fat_100g", 1004, "g"),
            ("carbohydrates_100g", 1005, "g"),
            ("fiber_100g", 1079, "g"),
            ("sugars_100g", 2000, "g"),
            ("calcium_100g", 1087, "mg"),
            ("iron_100g", 1089, "mg"),
            ("magnesium_100g", 1090, "mg"),
            ("phosphorus_100g", 1091, "mg"),
            ("potassium_100g", 1092, "mg"),
            ("sodium_100g", 1093, "mg"),
            ("zinc_100g", 1095, "mg"),
            ("copper_100g", 1098, "mg"),
            ("manganese_100g", 1101, "mg"),
            ("selenium_100g", 1103, "µg"),
            ("vitamin-c_100g", 1162, "mg"),
            ("vitamin-b1_100g", 1165, "mg"),
            ("vitamin-b2_100g", 1166, "mg"),
            ("vitamin-pp_100g", 1167, "mg"),
            ("vitamin-b6_100g", 1175, "mg"),
            ("vitamin-b9_100g", 1177, "µg"),
            ("vitamin-b12_100g", 1178, "µg"),
            ("vitamin-a_100g", 1106, "µg"),
            ("vitamin-e_100g", 1109, "mg"),
            ("vitamin-d_100g", 1114, "µg"),
            ("vitamin-k_100g", 1185, "µg"),
            ("cholesterol_100g", 1253, "mg"),
            ("saturated-fat_100g", 1258, "g"),
            ("monounsaturated-fat_100g", 1292, "g"),
            ("polyunsaturated-fat_100g", 1293, "g"),
            ("omega-3-fat_100g", 1402, "g"), // DHA
        ];

        for (field, nutrient_id, unit) in nutrient_map {
            if let Some(value) = nutrients.get(*field) {
                if let Some(amount) = value.as_f64() {
                    if amount > 0.0 {
                        result.push(FoodNutrient {
                            nutrient_id: *nutrient_id,
                            name: self.nutrient_id_to_name(*nutrient_id),
                            unit: unit.to_string(),
                            amount,
                        });
                    }
                }
            }
        }

        if result.is_empty() { None } else { Some(result) }
    }

    /// Map USDA nutrient ID to display name
    fn nutrient_id_to_name(&self, nutrient_id: i64) -> String {
        for (name, _unit, id) in KEY_NUTRIENTS {
            if *id == nutrient_id {
                return name.to_string();
            }
        }
        format!("Nutrient {}", nutrient_id)
    }

    /// Search USDA FoodData Central
    async fn search_usda(&self, query: &str) -> Result<Vec<FoodResult>> {
        let api_key = self.usda_api_key.as_ref().unwrap();
        let url = format!("{}/foods/search", FDC_API_BASE);
        let response = self
            .http
            .post(&url)
            .query(&[("api_key", api_key)])
            .json(&serde_json::json!({
                "query": query,
                "pageSize": MAX_SEARCH_RESULTS,
                "dataType": ["Foundation", "Survey (FNDDS)"],
                "sortBy": "dataType.keyword",
                "sortOrder": "asc"
            }))
            .send()
            .await
            .context("Failed to send search request to USDA API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("USDA API error ({}): {}", status, text));
        }

        #[derive(Deserialize)]
        struct UsdaSearchResponse {
            foods: Vec<UsdaFood>,
        }

        #[derive(Deserialize)]
        struct UsdaFood {
            #[serde(rename = "fdcId")]
            fdc_id: i64,
            description: String,
            #[serde(rename = "dataType")]
            data_type: String,
            #[serde(rename = "foodCategory")]
            food_category: Option<String>,
            #[serde(default, rename = "foodNutrients")]
            food_nutrients: Option<Vec<UsdaFoodNutrient>>,
        }

        #[derive(Deserialize)]
        struct UsdaFoodNutrient {
            #[serde(rename = "nutrientId")]
            nutrient_id: i64,
            #[serde(rename = "nutrientName")]
            nutrient_name: String,
            #[serde(rename = "unitName")]
            unit_name: String,
            #[serde(alias = "value", alias = "amount", default)]
            amount: f64,
        }

        let search_response: UsdaSearchResponse = response
            .json()
            .await
            .context("Failed to parse USDA API response")?;

        let mut results = Vec::new();
        for food in search_response.foods {
            let nutrients = food.food_nutrients.map(|n| {
                n.into_iter()
                    .map(|fnut| FoodNutrient {
                        nutrient_id: fnut.nutrient_id,
                        name: fnut.nutrient_name,
                        unit: fnut.unit_name,
                        amount: fnut.amount,
                    })
                    .collect()
            });

            results.push(FoodResult {
                id: food.fdc_id.to_string(),
                description: food.description,
                brand: None,
                category: food.food_category,
                source: FoodDataSource::USDA,
                nutrients_per_100g: nutrients,
            });
        }

        Ok(results)
    }

    /// Get detailed nutrient info for a specific food by ID (barcode for OFF, FDC ID for USDA)
    pub async fn get_food_details(&self, id: &str) -> Result<FoodResult> {
        // Try to detect if it's a barcode (numeric, 8-14 digits) or FDC ID
        let is_barcode = id.chars().all(|c| c.is_ascii_digit()) && id.len() >= 8 && id.len() <= 14;

        if is_barcode {
            // Try OpenFoodFacts first
            if let Ok(result) = self.get_off_product(id).await {
                return Ok(result);
            }
        }

        // Fall back to USDA if we have API key
        if let Some(api_key) = &self.usda_api_key {
            if let Ok(fdc_id) = id.parse::<i64>() {
                if let Ok(result) = self.get_usda_food(fdc_id, api_key).await {
                    return Ok(result);
                }
            }
        }

        Err(anyhow!("Food not found in any database"))
    }

    /// Get OpenFoodFacts product by barcode
    async fn get_off_product(&self, barcode: &str) -> Result<FoodResult> {
        let url = format!("{}/api/v2/product/{}.json", OFF_API_BASE, barcode);
        let response = self
            .http
            .get(&url)
            .send()
            .await
            .context("Failed to send product request to OpenFoodFacts API")?;

        if !response.status().is_success() {
            return Err(anyhow!("OpenFoodFacts API error: {}", response.status()));
        }

        #[derive(Deserialize)]
        struct OffProductResponse {
            product: Option<OffProduct>,
            status: i32,
        }

        #[derive(Deserialize)]
        struct OffProduct {
            code: String,
            product_name: Option<String>,
            brands: Option<String>,
            categories: Option<String>,
            #[serde(rename = "nutriments")]
            nutrients: Option<serde_json::Value>,
        }

        let product_response: OffProductResponse = response
            .json()
            .await
            .context("Failed to parse OpenFoodFacts API response")?;

        if product_response.status != 1 || product_response.product.is_none() {
            return Err(anyhow!("Product not found"));
        }

        let product = product_response.product.unwrap();
        let nutrients = self.parse_off_nutrients(&product.nutrients);

        Ok(FoodResult {
            id: product.code,
            description: product.product_name.unwrap_or_default(),
            brand: product.brands,
            category: product.categories,
            source: FoodDataSource::OpenFoodFacts,
            nutrients_per_100g: nutrients,
        })
    }

    /// Get USDA food by FDC ID
    async fn get_usda_food(&self, fdc_id: i64, api_key: &str) -> Result<FoodResult> {
        let url = format!("{}/food/{}", FDC_API_BASE, fdc_id);
        let response = self
            .http
            .get(&url)
            .query(&[("api_key", api_key)])
            .send()
            .await
            .context("Failed to send food details request to USDA API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("USDA API error ({}): {}", status, text));
        }

        #[derive(Deserialize)]
        struct UsdaFood {
            #[serde(rename = "fdcId")]
            fdc_id: i64,
            description: String,
            #[serde(rename = "dataType")]
            data_type: String,
            #[serde(rename = "foodCategory")]
            food_category: Option<String>,
            #[serde(default, rename = "foodNutrients")]
            food_nutrients: Option<Vec<UsdaFoodNutrient>>,
        }

        #[derive(Deserialize)]
        struct UsdaFoodNutrient {
            #[serde(rename = "nutrientId")]
            nutrient_id: i64,
            #[serde(rename = "nutrientName")]
            nutrient_name: String,
            #[serde(rename = "unitName")]
            unit_name: String,
            #[serde(alias = "value", alias = "amount", default)]
            amount: f64,
        }

        let food: UsdaFood = response
            .json()
            .await
            .context("Failed to parse USDA API response")?;

        let nutrients = food.food_nutrients.map(|n| {
            n.into_iter()
                .map(|fnut| FoodNutrient {
                    nutrient_id: fnut.nutrient_id,
                    name: fnut.nutrient_name,
                    unit: fnut.unit_name,
                    amount: fnut.amount,
                })
                .collect()
        });

        Ok(FoodResult {
            id: food.fdc_id.to_string(),
            description: food.description,
            brand: None,
            category: food.food_category,
            source: FoodDataSource::USDA,
            nutrients_per_100g: nutrients,
        })
    }

    /// Get nutrient amounts for a food, scaled to the given amount and unit
    pub async fn get_nutrients_for_amount(
        &self,
        id: &str,
        amount: f32,
        unit: &str,
    ) -> Result<Vec<NutrientInfo>> {
        let food = self.get_food_details(id).await?;

        let scale = self.calculate_scale(amount, unit)?;

        let mut nutrients = Vec::new();
        if let Some(nutrients_list) = food.nutrients_per_100g {
            for (display_name, unit_name, nutrient_id) in KEY_NUTRIENTS {
                if let Some(nutrient) = nutrients_list.iter().find(|n| n.nutrient_id == *nutrient_id) {
                    let scaled_amount = nutrient.amount * scale as f64;
                    if scaled_amount > 0.001 {
                        nutrients.push(NutrientInfo {
                            name: display_name.to_string(),
                            amount: scaled_amount,
                            unit: unit_name.to_string(),
                        });
                    }
                }
            }
        }

        Ok(nutrients)
    }

    /// Calculate scale factor from 100g basis to user's amount/unit
    fn calculate_scale(&self, amount: f32, unit: &str) -> Result<f32> {
        match unit.to_lowercase().as_str() {
            "g" | "gram" | "grams" => Ok(amount / 100.0),
            "kg" | "kilogram" | "kilograms" => Ok(amount * 1000.0 / 100.0),
            "mg" | "milligram" | "milligrams" => Ok(amount / 1000.0 / 100.0),
            "oz" | "ounce" | "ounces" => Ok(amount * 28.3495 / 100.0),
            "lb" | "pound" | "pounds" => Ok(amount * 453.592 / 100.0),
            "cup" | "cups" => Ok(amount * 240.0 / 100.0), // Rough approximation
            "slice" | "slices" => Ok(amount * 30.0 / 100.0),
            _ => Ok(amount / 100.0), // Default to grams
        }
    }

    /// Build a FoodDbClient from environment variables
    pub fn build_client() -> Result<FoodDbClient> {
        let usda_api_key = std::env::var("USDA_API_KEY")
            .or_else(|_| std::env::var("FOODDATA_API_KEY"))
            .ok();

        if usda_api_key.is_none() {
            eprintln!(
                "Warning: USDA_API_KEY not set. USDA fallback will be unavailable. \
                Get a free key at https://fdc.nal.usda.gov/api-key-signup"
            );
        }

        Ok(FoodDbClient::new(usda_api_key))
    }
}
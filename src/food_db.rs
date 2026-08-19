//! USDA FoodData Central integration for nutrient lookup
//!
//! This module provides a client for querying the USDA FoodData Central API
//! to retrieve nutrient information for foods logged via `biohack log food`.

use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// USDA FoodData Central API base URL
const FDC_API_BASE: &str = "https://api.nal.usda.gov/fdc/v1";

/// Maximum search results to return
const MAX_SEARCH_RESULTS: usize = 10;

/// Client for USDA FoodData Central API
#[derive(Debug, Clone)]
pub struct FoodDbClient {
    http: Client,
    api_key: String,
    cache: Arc<RwLock<HashMap<String, CachedFood>>>,
}

/// Cached food entry with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedFood {
    food: FoodResult,
    cached_at: chrono::DateTime<chrono::Utc>,
}

/// Food search result from FDC API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodSearchResponse {
    pub foods: Vec<FoodResult>,
    #[serde(rename = "totalHits")]
    pub total_hits: usize,
    #[serde(rename = "currentPage")]
    pub current_page: usize,
    #[serde(rename = "totalPages")]
    pub total_pages: usize,
}

/// Individual food result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodResult {
    #[serde(rename = "fdcId")]
    pub fdc_id: i64,
    pub description: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "foodCategory")]
    pub food_category: Option<String>,
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<String>,
    #[serde(default, rename = "foodNutrients")]
    pub food_nutrients: Option<Vec<FoodNutrient>>,
}

/// Nutrient within a food
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodNutrient {
    #[serde(rename = "nutrientId")]
    pub nutrient_id: i64,
    #[serde(rename = "nutrientName")]
    pub nutrient_name: String,
    #[serde(rename = "nutrientNumber")]
    pub nutrient_number: Option<String>,
    #[serde(rename = "unitName")]
    pub unit_name: String,
    #[serde(rename = "derivationCode")]
    pub derivation_code: Option<String>,
    #[serde(rename = "derivationDescription")]
    pub derivation_description: Option<String>,
    // USDA API uses "value" in search results, "amount" in detail results
    // Some nutrients might not have amount/value in certain responses
    #[serde(alias = "value", alias = "amount", default)]
    pub amount: f64,
    #[serde(default, rename = "foodNutrientDerivation")]
    pub food_nutrient_derivation: Option<FoodNutrientDerivation>,
}

/// Nutrient derivation info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodNutrientDerivation {
    pub code: String,
    pub description: String,
    #[serde(rename = "foodNutrientSourceId")]
    pub food_nutrient_source_id: i64,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub median: Option<f64>,
}

/// Simplified nutrient info for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientInfo {
    pub name: String,
    pub amount: f64,
    pub unit: String,
}

/// Key nutrients we care about for biohacking display
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
    /// Create a new FoodDB client with the given API key
    pub fn new(api_key: String) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http,
            api_key,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Search for foods by query string
    pub async fn search_foods(&self, query: &str) -> Result<Vec<FoodResult>> {
        let cache_key = format!("search:{}", query.to_lowercase());

        // Check cache first
        if let Some(cached) = self.get_from_cache(&cache_key) {
            return Ok(cached);
        }

        let url = format!("{}/foods/search", FDC_API_BASE);
        let response = self
            .http
            .post(&url)
            .query(&[("api_key", &self.api_key)])
            .json(&serde_json::json!({
                "query": query,
                "pageSize": MAX_SEARCH_RESULTS,
                "dataType": ["Foundation"],
                "sortBy": "dataType.keyword",
                "sortOrder": "asc"
            }))
            .send()
            .await
            .context("Failed to send search request to USDA API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "USDA API error ({}): {}",
                status,
                text
            ));
        }

        let search_response: FoodSearchResponse = response
            .json()
            .await
            .context("Failed to parse USDA API response")?;

        let foods = search_response.foods;

        // Cache the results
        self.put_in_cache(cache_key, foods.clone()).await;

        Ok(foods)
    }

    /// Get detailed nutrient info for a specific food by FDC ID
    pub async fn get_food_details(&self, fdc_id: i64) -> Result<FoodResult> {
        let cache_key = format!("detail:{}", fdc_id);

        if let Some(cached) = self.get_from_cache(&cache_key) {
            return Ok(cached[0].clone());
        }

        let url = format!("{}/food/{}", FDC_API_BASE, fdc_id);
        let response = self
            .http
            .get(&url)
            .query(&[("api_key", &self.api_key)])
            .send()
            .await
            .context("Failed to send food details request to USDA API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "USDA API error ({}): {}",
                status,
                text
            ));
        }

        let food: FoodResult = response
            .json()
            .await
            .context("Failed to parse USDA API response")?;

        // Cache the result
        self.put_in_cache(cache_key, vec![food.clone()]).await;

        Ok(food)
    }

    /// Get nutrient amounts for a food, scaled to the given amount and unit
    pub async fn get_nutrients_for_amount(
        &self,
        fdc_id: i64,
        amount: f32,
        unit: &str,
    ) -> Result<Vec<NutrientInfo>> {
        let food = self.get_food_details(fdc_id).await?;

        // Calculate scaling factor (USDA data is per 100g)
        let scale = self.calculate_scale(&food, amount, unit)?;

        let mut nutrients = Vec::new();
        if let Some(nutrients_list) = food.food_nutrients {
            for (display_name, unit_name, nutrient_id) in KEY_NUTRIENTS {
                if let Some(nutrient) = nutrients_list.iter().find(|n| n.nutrient_id == *nutrient_id) {
                    let scaled_amount = nutrient.amount * scale as f64;
                    if scaled_amount > 0.001 {
                        // Only show non-trivial amounts
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

    /// Calculate scale factor from USDA's 100g basis to user's amount/unit
    fn calculate_scale(&self, _food: &FoodResult, amount: f32, unit: &str) -> Result<f32> {
        // USDA Foundation foods report per 100g
        // For now, assume user provides grams or convertible units
        match unit.to_lowercase().as_str() {
            "g" | "gram" | "grams" => Ok(amount / 100.0),
            "kg" | "kilogram" | "kilograms" => Ok(amount * 1000.0 / 100.0),
            "mg" | "milligram" | "milligrams" => Ok(amount / 1000.0 / 100.0),
            "oz" | "ounce" | "ounces" => Ok(amount * 28.3495 / 100.0),
            "lb" | "pound" | "pounds" => Ok(amount * 453.592 / 100.0),
            "cup" | "cups" => {
                // Rough approximation: 1 cup ≈ 240g for many foods
                // In a full implementation, we'd use food-specific density
                Ok(amount * 240.0 / 100.0)
            }
            "slice" | "slices" => {
                // Very rough: 1 slice ≈ 30g
                Ok(amount * 30.0 / 100.0)
            }
            _ => {
                // Default to assuming grams
                Ok(amount / 100.0)
            }
        }
    }

    /// Get cached entry if not expired (24 hour TTL)
    fn get_from_cache(&self, _key: &str) -> Option<Vec<FoodResult>> {
        // For simplicity, we'll skip the async lock here and just return None
        // In a full implementation, this would check the RwLock cache
        None
    }

    /// Put entry in cache
    async fn put_in_cache(&self, key: String, foods: Vec<FoodResult>) {
        let mut cache = self.cache.write().await;
        for food in foods {
            cache.insert(
                key.clone(),
                CachedFood {
                    food,
                    cached_at: chrono::Utc::now(),
                },
            );
        }
    }
}

/// Build a FoodDbClient from environment variable or config
pub fn build_client() -> Result<FoodDbClient> {
    let api_key = std::env::var("USDA_API_KEY")
        .or_else(|_| std::env::var("FOODDATA_API_KEY"))
        .context("USDA_API_KEY or FOODDATA_API_KEY environment variable not set. Get a free key at https://fdc.nal.usda.gov/api-key-signup")?;

    Ok(FoodDbClient::new(api_key))
}
//! Nutrient reference values (RDI/DRI) for deficiency/excess detection
//!
//! Based on FDA Daily Values (DV) and NIH Dietary Reference Intakes (DRI)
//! for adults 19-50 years. Values are per day.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Nutrient reference value with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientReference {
    /// Nutrient name (matches USDA nutrient names)
    pub name: &'static str,
    /// USDA nutrient ID for cross-referencing
    pub usda_id: i64,
    /// Recommended Dietary Allowance / Adequate Intake (per day)
    pub rdi: f64,
    /// Unit of measurement
    pub unit: &'static str,
    /// Tolerable Upper Intake Level (UL) - maximum safe intake
    pub ul: Option<f64>,
    /// Whether this is an essential nutrient (deficiency possible)
    pub essential: bool,
    /// Category for grouping
    pub category: NutrientCategory,
}

/// Nutrient categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NutrientCategory {
    Macro,
    Vitamin,
    Mineral,
    Other,
}

/// Nutrient status relative to RDI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientStatus {
    pub name: String,
    pub intake: f64,
    pub rdi: f64,
    pub unit: String,
    pub percent_rdi: f64,
    pub ul: Option<f64>,
    pub percent_ul: Option<f64>,
    pub status: NutrientStatusLevel,
    pub category: NutrientCategory,
}

/// Status level based on intake vs RDI/UL
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NutrientStatusLevel {
    /// Below 50% of RDI
    Deficient,
    /// 50-99% of RDI
    Low,
    /// 100-150% of RDI
    Adequate,
    /// 150-200% of RDI
    High,
    /// Above 200% of RDI but below UL
    VeryHigh,
    /// Above UL (potential toxicity risk)
    Excessive,
    /// No RDI established (AI only or not essential)
    NoRDI,
}

/// Daily nutrient status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNutrientStatus {
    pub date: chrono::NaiveDate,
    pub nutrients: Vec<NutrientStatus>,
}

impl DailyNutrientStatus {
    pub fn deficient_count(&self) -> usize {
        self.nutrients
            .iter()
            .filter(|n| n.status == NutrientStatusLevel::Deficient)
            .count()
    }

    pub fn low_count(&self) -> usize {
        self.nutrients
            .iter()
            .filter(|n| n.status == NutrientStatusLevel::Low)
            .count()
    }

    pub fn excessive_count(&self) -> usize {
        self.nutrients
            .iter()
            .filter(|n| n.status == NutrientStatusLevel::Excessive)
            .count()
    }

    pub fn very_high_count(&self) -> usize {
        self.nutrients
            .iter()
            .filter(|n| n.status == NutrientStatusLevel::VeryHigh)
            .count()
    }
}

/// Get all nutrient reference values
pub fn get_nutrient_references() -> Vec<NutrientReference> {
    vec![
        // MACRONUTRIENTS
        NutrientReference {
            name: "Energy",
            usda_id: 1008,
            rdi: 2000.0,
            unit: "kcal",
            ul: None,
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Protein",
            usda_id: 1003,
            rdi: 50.0,
            unit: "g",
            ul: None,
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Total Fat",
            usda_id: 1004,
            rdi: 78.0,
            unit: "g",
            ul: None,
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Carbohydrate",
            usda_id: 1005,
            rdi: 275.0,
            unit: "g",
            ul: None,
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Fiber",
            usda_id: 1079,
            rdi: 28.0,
            unit: "g",
            ul: None,
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Sugars",
            usda_id: 2000,
            rdi: 50.0,
            unit: "g",
            ul: None,
            essential: false,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Saturated Fat",
            usda_id: 1258,
            rdi: 20.0,
            unit: "g",
            ul: None,
            essential: false,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Monounsaturated Fat",
            usda_id: 1292,
            rdi: 0.0,
            unit: "g",
            ul: None,
            essential: false,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Polyunsaturated Fat",
            usda_id: 1293,
            rdi: 0.0,
            unit: "g",
            ul: None,
            essential: false,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Omega-3 (DHA)",
            usda_id: 1402,
            rdi: 0.25,
            unit: "g",
            ul: Some(3.0),
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Omega-3 (EPA)",
            usda_id: 1401,
            rdi: 0.25,
            unit: "g",
            ul: Some(3.0),
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Omega-3 (ALA)",
            usda_id: 1399,
            rdi: 1.6,
            unit: "g",
            ul: None,
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Omega-6 (Linoleic)",
            usda_id: 1404,
            rdi: 17.0,
            unit: "g",
            ul: None,
            essential: true,
            category: NutrientCategory::Macro,
        },
        NutrientReference {
            name: "Cholesterol",
            usda_id: 1253,
            rdi: 300.0,
            unit: "mg",
            ul: None,
            essential: false,
            category: NutrientCategory::Macro,
        },
        // VITAMINS
        NutrientReference {
            name: "Vitamin A (RAE)",
            usda_id: 1106,
            rdi: 900.0,
            unit: "µg",
            ul: Some(3000.0),
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Vitamin C",
            usda_id: 1162,
            rdi: 90.0,
            unit: "mg",
            ul: Some(2000.0),
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Vitamin D",
            usda_id: 1114,
            rdi: 20.0,
            unit: "µg",
            ul: Some(100.0),
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Vitamin E",
            usda_id: 1109,
            rdi: 15.0,
            unit: "mg",
            ul: Some(1000.0),
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Vitamin K",
            usda_id: 1185,
            rdi: 120.0,
            unit: "µg",
            ul: None,
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Thiamin (B1)",
            usda_id: 1165,
            rdi: 1.2,
            unit: "mg",
            ul: None,
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Riboflavin (B2)",
            usda_id: 1166,
            rdi: 1.3,
            unit: "mg",
            ul: None,
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Niacin (B3)",
            usda_id: 1167,
            rdi: 16.0,
            unit: "mg",
            ul: Some(35.0),
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Vitamin B6",
            usda_id: 1175,
            rdi: 1.7,
            unit: "mg",
            ul: Some(100.0),
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Folate",
            usda_id: 1177,
            rdi: 400.0,
            unit: "µg",
            ul: Some(1000.0),
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        NutrientReference {
            name: "Vitamin B12",
            usda_id: 1178,
            rdi: 2.4,
            unit: "µg",
            ul: None,
            essential: true,
            category: NutrientCategory::Vitamin,
        },
        // MINERALS
        NutrientReference {
            name: "Calcium",
            usda_id: 1087,
            rdi: 1300.0,
            unit: "mg",
            ul: Some(2500.0),
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Iron",
            usda_id: 1089,
            rdi: 18.0,
            unit: "mg",
            ul: Some(45.0),
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Magnesium",
            usda_id: 1090,
            rdi: 420.0,
            unit: "mg",
            ul: Some(350.0), // UL from supplements only
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Phosphorus",
            usda_id: 1091,
            rdi: 1250.0,
            unit: "mg",
            ul: Some(4000.0),
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Potassium",
            usda_id: 1092,
            rdi: 4700.0,
            unit: "mg",
            ul: None,
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Sodium",
            usda_id: 1093,
            rdi: 1500.0,
            unit: "mg",
            ul: Some(2300.0),
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Zinc",
            usda_id: 1095,
            rdi: 11.0,
            unit: "mg",
            ul: Some(40.0),
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Copper",
            usda_id: 1098,
            rdi: 0.9,
            unit: "mg",
            ul: Some(10.0),
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Manganese",
            usda_id: 1101,
            rdi: 2.3,
            unit: "mg",
            ul: Some(11.0),
            essential: true,
            category: NutrientCategory::Mineral,
        },
        NutrientReference {
            name: "Selenium",
            usda_id: 1103,
            rdi: 55.0,
            unit: "µg",
            ul: Some(400.0),
            essential: true,
            category: NutrientCategory::Mineral,
        },
    ]
}

/// Build lookup map from USDA nutrient ID to reference
pub fn build_reference_map() -> HashMap<i64, NutrientReference> {
    get_nutrient_references()
        .into_iter()
        .map(|r| (r.usda_id, r))
        .collect()
}

/// Calculate nutrient status from intake vs reference
pub fn calculate_nutrient_status(
    intake_amount: f64,
    reference: &NutrientReference,
) -> NutrientStatus {
    let percent_rdi = if reference.rdi > 0.0 {
        (intake_amount / reference.rdi) * 100.0
    } else {
        0.0
    };

    let percent_ul = reference.ul.map(|ul| (intake_amount / ul) * 100.0);

    let status = if let Some(ul) = reference.ul {
        if intake_amount > ul {
            NutrientStatusLevel::Excessive
        } else if reference.rdi > 0.0 {
            let ratio = intake_amount / reference.rdi;
            if ratio >= 2.0 {
                NutrientStatusLevel::VeryHigh
            } else if ratio >= 1.5 {
                NutrientStatusLevel::High
            } else if ratio >= 1.0 {
                NutrientStatusLevel::Adequate
            } else if ratio >= 0.5 {
                NutrientStatusLevel::Low
            } else {
                NutrientStatusLevel::Deficient
            }
        } else {
            NutrientStatusLevel::NoRDI
        }
    } else if reference.rdi > 0.0 {
        let ratio = intake_amount / reference.rdi;
        if ratio >= 2.0 {
            NutrientStatusLevel::VeryHigh
        } else if ratio >= 1.5 {
            NutrientStatusLevel::High
        } else if ratio >= 1.0 {
            NutrientStatusLevel::Adequate
        } else if ratio >= 0.5 {
            NutrientStatusLevel::Low
        } else {
            NutrientStatusLevel::Deficient
        }
    } else {
        NutrientStatusLevel::NoRDI
    };

    NutrientStatus {
        name: reference.name.to_string(),
        intake: intake_amount,
        rdi: reference.rdi,
        unit: reference.unit.to_string(),
        percent_rdi,
        ul: reference.ul,
        percent_ul,
        status,
        category: reference.category,
    }
}

/// Get display label for status level
pub fn status_label(status: NutrientStatusLevel) -> &'static str {
    match status {
        NutrientStatusLevel::Deficient => "[DEFICIENT]",
        NutrientStatusLevel::Low => "[LOW]",
        NutrientStatusLevel::Adequate => "[OK]",
        NutrientStatusLevel::High => "[HIGH]",
        NutrientStatusLevel::VeryHigh => "[VERY HIGH]",
        NutrientStatusLevel::Excessive => "[EXCESSIVE]",
        NutrientStatusLevel::NoRDI => "[--]",
    }
}

/// Get color hint for status (for terminal output)
pub fn status_color(status: NutrientStatusLevel) -> &'static str {
    match status {
        NutrientStatusLevel::Deficient => "red",
        NutrientStatusLevel::Low => "yellow",
        NutrientStatusLevel::Adequate => "green",
        NutrientStatusLevel::High => "yellow",
        NutrientStatusLevel::VeryHigh => "magenta",
        NutrientStatusLevel::Excessive => "red",
        NutrientStatusLevel::NoRDI => "dim",
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubstanceCategory {
    Supplement,
    Medication,
    Drug,
    Nootropic,
    Hormone,
    Peptide,
    Electrolyte,
    Vitamin,
    Mineral,
    Herb,
    Stimulant,
    Other(String),
}

impl std::fmt::Display for SubstanceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Supplement => write!(f, "supplement"),
            Self::Medication => write!(f, "medication"),
            Self::Drug => write!(f, "drug"),
            Self::Nootropic => write!(f, "nootropic"),
            Self::Hormone => write!(f, "hormone"),
            Self::Peptide => write!(f, "peptide"),
            Self::Electrolyte => write!(f, "electrolyte"),
            Self::Vitamin => write!(f, "vitamin"),
            Self::Mineral => write!(f, "mineral"),
            Self::Herb => write!(f, "herb"),
            Self::Stimulant => write!(f, "stimulant"),
            Self::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::str::FromStr for SubstanceCategory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "supplement" => Self::Supplement,
            "medication" => Self::Medication,
            "drug" => Self::Drug,
            "nootropic" => Self::Nootropic,
            "hormone" => Self::Hormone,
            "peptide" => Self::Peptide,
            "electrolyte" => Self::Electrolyte,
            "vitamin" => Self::Vitamin,
            "mineral" => Self::Mineral,
            "herb" => Self::Herb,
            "stimulant" => Self::Stimulant,
            other => Self::Other(other.to_string()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Substance {
    pub id: Uuid,
    pub name: String,
    pub aliases: Vec<String>,
    pub category: SubstanceCategory,
    pub min_dose_mg: Option<f64>,
    pub max_dose_mg: Option<f64>,
    pub typical_dose_mg: Option<f64>,
    pub half_life_hours: Option<f64>,
    pub contraindications: Vec<String>,
    pub interactions: Vec<String>,
    pub notes: Option<String>,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceLog {
    pub id: Uuid,
    #[serde(default = "uuid::Uuid::nil")]
    pub substance_id: Uuid,
    pub substance_name: String,
    pub dose_mg: f64,
    pub route: String,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
    /// Category of the substance (populated from database lookup)
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalsLog {
    pub id: Uuid,
    pub heart_rate: Option<u32>,
    pub sbp: Option<u32>,
    pub dbp: Option<u32>,
    pub temperature_c: Option<f32>,
    pub spo2: Option<u32>,
    pub hrv_rmssd: Option<u32>,
    pub weight_kg: Option<f32>,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Schedule {
    #[serde(rename = "morning")]
    Morning,
    #[serde(rename = "evening")]
    Evening,
    #[serde(rename = "prn")]
    Prn,
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Morning => write!(f, "morning"),
            Self::Evening => write!(f, "evening"),
            Self::Prn => write!(f, "prn"),
        }
    }
}

impl std::str::FromStr for Schedule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "morning" => Self::Morning,
            "evening" => Self::Evening,
            "prn" => Self::Prn,
            other => anyhow::bail!(
                "Invalid schedule: {}. Use 'morning', 'evening', or 'prn'",
                other
            ),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackItem {
    pub substance_name: String,
    pub dose: String,
    pub route: Option<String>,
    pub schedule: Option<Schedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    pub name: String,
    pub description: Option<String>,
    pub items: Vec<StackItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum ProtocolTriggerType {
    AllOf,
    AnyOf,
    Not,
    Atomic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProtocolCondition {
    pub trigger_type: ProtocolTriggerType,
    pub conditions: Vec<ProtocolCondition>,
    pub field: Option<String>,
    pub operator: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProtocolAction {
    pub action_type: String,
    pub priority: u32,
    pub message: String,
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Protocol {
    pub id: String,
    pub name: String,
    pub description: String,
    pub trigger: ProtocolCondition,
    pub actions: Vec<ProtocolAction>,
    pub evidence: Vec<String>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodLog {
    pub id: Uuid,
    pub food_name: String,
    pub amount: f32,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

// TODO: Add nutrient tracking fields in v1.1 (macronutrients, micronutrients)

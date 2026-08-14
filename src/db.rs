use anyhow::Result;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use sled::{Config, Db, Tree};
use std::path::PathBuf;
use uuid::Uuid;

use crate::models::{FoodLog, Stack, Substance, SubstanceLog, VitalsLog};

const SUBSTANCES_TREE: &str = "substances";
const SUBSTANCE_LOGS_TREE: &str = "substance_logs";
const VITALS_LOGS_TREE: &str = "vitals_logs";
const STACKS_TREE: &str = "stacks";
const PROTOCOLS_TREE: &str = "protocols";

#[derive(Debug, Clone)]
pub enum TimelineEntry {
    Substance(SubstanceLog),
    Vitals(VitalsLog),
    Food(FoodLog),
}

impl TimelineEntry {
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            TimelineEntry::Substance(log) => log.timestamp,
            TimelineEntry::Vitals(log) => log.timestamp,
            TimelineEntry::Food(log) => log.timestamp,
        }
    }
}

pub struct Database {
    db: Db,
    substances: Tree,
    substance_logs: Tree,
    vitals_logs: Tree,
    stacks: Tree,
    protocols: Tree,
}

impl Database {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let db_path = path.unwrap_or_else(Self::default_path);
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let config = Config::new().path(&db_path);
        let db = config.open()?;

        let substances = db.open_tree(SUBSTANCES_TREE)?;
        let substance_logs = db.open_tree(SUBSTANCE_LOGS_TREE)?;
        let vitals_logs = db.open_tree(VITALS_LOGS_TREE)?;
        let stacks = db.open_tree(STACKS_TREE)?;
        let protocols = db.open_tree(PROTOCOLS_TREE)?;

        Ok(Self {
            db,
            substances,
            substance_logs,
            vitals_logs,
            stacks,
            protocols,
        })
    }

    fn default_path() -> PathBuf {
        ProjectDirs::from("com", "skyhigh", "biohack")
            .map(|dirs| dirs.data_local_dir().join("biohack.db"))
            .unwrap_or_else(|| PathBuf::from("biohack.db"))
    }

    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }

    // ===== Helper: serialize/deserialize =====

    fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(value)?)
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(&self, bytes: &[u8]) -> Result<T> {
        Ok(serde_json::from_slice(bytes)?)
    }

    // ===== Substances =====

    pub fn insert_substance(&self, substance: &Substance) -> Result<()> {
        let key = substance.id.to_string();
        let value = self.serialize(substance)?;
        self.substances.insert(key, value)?;
        self.flush()?;
        Ok(())
    }

    pub fn get_substance_by_name(&self, name: &str) -> Result<Option<Substance>> {
        for item in self.substances.iter() {
            let (_, value) = item?;
            let substance: Substance = self.deserialize(&value)?;
            if substance.name.eq_ignore_ascii_case(name)
                || substance.aliases.iter().any(|a| a.eq_ignore_ascii_case(name))
            {
                return Ok(Some(substance));
            }
        }
        Ok(None)
    }

    pub fn search_substances(&self, query: &str) -> Result<Vec<Substance>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for item in self.substances.iter() {
            let (_, value) = item?;
            let substance: Substance = self.deserialize(&value)?;
            if substance.name.to_lowercase().contains(&query_lower)
                || substance.aliases.iter().any(|a| a.to_lowercase().contains(&query_lower))
            {
                results.push(substance);
            }
        }

        Ok(results)
    }

    pub fn list_substances(&self, category: Option<&str>) -> Result<Vec<Substance>> {
        let mut results = Vec::new();

        for item in self.substances.iter() {
            let (_, value) = item?;
            let substance: Substance = self.deserialize(&value)?;
            if let Some(cat) = category {
                if substance.category.to_string().eq_ignore_ascii_case(cat) {
                    results.push(substance);
                }
            } else {
                results.push(substance);
            }
        }

        Ok(results)
    }

    // ===== Substance Logs =====

    pub fn insert_substance_log(&self, log: &SubstanceLog) -> Result<()> {
        let key = log.timestamp.to_rfc3339() + "_" + &log.id.to_string();
        let value = self.serialize(log)?;
        self.substance_logs.insert(key, value)?;
        self.flush()?;
        Ok(())
    }

    pub fn get_recent_substance_logs(&self, days: u32, name_filter: Option<&str>) -> Result<Vec<SubstanceLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            let log: SubstanceLog = self.deserialize(&value)?;
            if log.timestamp < since {
                break; // Logs are stored in chronological order
            }
            if let Some(name) = name_filter {
                if log.substance_name.to_lowercase().contains(&name.to_lowercase()) {
                    results.push(log);
                }
            } else {
                results.push(log);
            }
        }

        Ok(results)
    }

    // ===== Vitals Logs =====

    pub fn insert_vitals_log(&self, log: &VitalsLog) -> Result<()> {
        let key = log.timestamp.to_rfc3339() + "_" + &log.id.to_string();
        let value = self.serialize(log)?;
        self.vitals_logs.insert(key, value)?;
        self.flush()?;
        Ok(())
    }

    pub fn get_recent_vitals_logs(&self, days: u32) -> Result<Vec<VitalsLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.vitals_logs.iter().rev() {
            let (_, value) = item?;
            let log: VitalsLog = self.deserialize(&value)?;
            if log.timestamp < since {
                break;
            }
            results.push(log);
        }

        Ok(results)
    }

    // ===== Stacks =====

    pub fn insert_stack(&self, stack: &Stack) -> Result<()> {
        let key = stack.name.clone();
        let value = self.serialize(stack)?;
        self.stacks.insert(key, value)?;
        self.flush()?;
        Ok(())
    }

    pub fn get_stack(&self, name: &str) -> Result<Option<Stack>> {
        if let Some(value) = self.stacks.get(name)? {
            let stack: Stack = self.deserialize(&value)?;
            Ok(Some(stack))
        } else {
            Ok(None)
        }
    }

    pub fn list_stacks(&self) -> Result<Vec<Stack>> {
        let mut results = Vec::new();

        for item in self.stacks.iter() {
            let (_, value) = item?;
            let stack: Stack = self.deserialize(&value)?;
            results.push(stack);
        }

        Ok(results)
    }

    // ===== Food Logs =====

    pub fn insert_food_log(&self, log: &FoodLog) -> Result<()> {
        let key = log.timestamp.to_rfc3339() + "_" + &log.id.to_string();
        let value = self.serialize(log)?;
        self.substance_logs.insert(key, value)?; // Note: reusing substance_logs tree for simplicity; could split to food_logs tree
        self.flush()?;
        Ok(())
    }

    pub fn get_recent_food_logs(&self, days: u32, name_filter: Option<&str>) -> Result<Vec<FoodLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            let log: FoodLog = self.deserialize(&value)?;
            if log.timestamp < since {
                break; // Logs are stored in chronological order
            }
            if let Some(name) = name_filter {
                if log.food_name.to_lowercase().contains(&name.to_lowercase()) {
                    results.push(log);
                }
            } else {
                results.push(log);
            }
        }

        Ok(results)
    }

    pub fn get_recent_timeline(&self, days: u32) -> Result<Vec<TimelineEntry>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        // Get substance logs
        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            // Try to deserialize as SubstanceLog first
            if let Ok(log) = self.deserialize::<SubstanceLog>(&value) {
                if log.timestamp >= since {
                    results.push(TimelineEntry::Substance(log));
                }
            }
            // Try to deserialize as FoodLog
            else if let Ok(log) = self.deserialize::<FoodLog>(&value) {
                if log.timestamp >= since {
                    results.push(TimelineEntry::Food(log));
                }
            }
        }

        // Get vitals logs
        for item in self.vitals_logs.iter().rev() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<VitalsLog>(&value) {
                if log.timestamp >= since {
                    results.push(TimelineEntry::Vitals(log));
                }
            }
        }

        // Sort by timestamp descending (most recent first)
        results.sort_by(|a, b| b.timestamp().cmp(&a.timestamp()));

        Ok(results)
    }

    /// Get report summary stats for a time range
    pub fn get_report_summary(&self, days: u32) -> Result<ReportSummary> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut substance_count = 0;
        let mut vitals_count = 0;
        let mut food_count = 0;
        let mut unique_substances = std::collections::HashSet::new();

        // Count substance logs
        for item in self.substance_logs.iter() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<SubstanceLog>(&value) {
                if log.timestamp >= since {
                    substance_count += 1;
                    unique_substances.insert(log.substance_name.clone());
                }
            }
        }

        // Count vitals logs
        for item in self.vitals_logs.iter() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<VitalsLog>(&value) {
                if log.timestamp >= since {
                    vitals_count += 1;
                }
            }
        }

        // Count food logs
        for item in self.substance_logs.iter() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<FoodLog>(&value) {
                if log.timestamp >= since {
                    food_count += 1;
                }
            }
        }

        Ok(ReportSummary {
            days,
            substance_logs: substance_count,
            vitals_logs: vitals_count,
            food_logs: food_count,
            unique_substances: unique_substances.len(),
        })
    }

    /// Get substance logs with full details for report
    pub fn get_recent_substance_logs_detailed(&self, days: u32) -> Result<Vec<SubstanceLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<SubstanceLog>(&value) {
                if log.timestamp >= since {
                    results.push(log);
                }
            }
        }

        // Sort by timestamp ascending (oldest first for report)
        results.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(results)
    }

    /// Get vitals logs with full details for report
    pub fn get_recent_vitals_logs_detailed(&self, days: u32) -> Result<Vec<VitalsLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.vitals_logs.iter().rev() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<VitalsLog>(&value) {
                if log.timestamp >= since {
                    results.push(log);
                }
            }
        }

        // Sort by timestamp ascending (oldest first for report)
        results.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(results)
    }

    /// Get food logs with full details for report
    pub fn get_recent_food_logs_detailed(&self, days: u32) -> Result<Vec<FoodLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<FoodLog>(&value) {
                if log.timestamp >= since {
                    results.push(log);
                }
            }
        }

        // Sort by timestamp ascending (oldest first for report)
        results.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(results)
    }
}

/// Summary statistics for report header
#[derive(Debug, Clone)]
pub struct ReportSummary {
    pub days: u32,
    pub substance_logs: usize,
    pub vitals_logs: usize,
    pub food_logs: usize,
    pub unique_substances: usize,
}
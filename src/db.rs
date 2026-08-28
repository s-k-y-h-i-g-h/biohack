use anyhow::Result;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use sled::{Config, Db, Tree};
use std::path::PathBuf;
use uuid::Uuid;

use crate::models::{FoodLog, NutrientInfo, Protocol, Stack, Substance, SubstanceLog, VitalsLog};
use crate::nutrient_ref::{DailyNutrientStatus, NutrientStatus, NutrientStatusLevel};

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
    #[allow(dead_code)]
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
                || substance
                    .aliases
                    .iter()
                    .any(|a| a.eq_ignore_ascii_case(name))
            {
                return Ok(Some(substance));
            }
        }
        Ok(None)
    }

    #[allow(dead_code)]
    pub fn search_substances(&self, query: &str) -> Result<Vec<Substance>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for item in self.substances.iter() {
            let (_, value) = item?;
            let substance: Substance = self.deserialize(&value)?;
            if substance.name.to_lowercase().contains(&query_lower)
                || substance
                    .aliases
                    .iter()
                    .any(|a| a.to_lowercase().contains(&query_lower))
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

    pub fn get_recent_substance_logs(
        &self,
        days: u32,
        name_filter: Option<&str>,
    ) -> Result<Vec<SubstanceLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            let log: SubstanceLog = self.deserialize(&value)?;
            if log.timestamp < since {
                break; // Logs are stored in chronological order
            }
            if let Some(name) = name_filter {
                if log
                    .substance_name
                    .to_lowercase()
                    .contains(&name.to_lowercase())
                {
                    results.push(log);
                }
            } else {
                results.push(log);
            }
        }

        Ok(results)
    }

    /// Get the most recent substance log for a given substance name (any time).
    pub fn get_most_recent_substance_log(&self, name: &str) -> Result<Option<SubstanceLog>> {
        // Iterate from newest to oldest (by reversing the iterator)
        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            let log: SubstanceLog = self.deserialize(&value)?;
            if log.substance_name.eq_ignore_ascii_case(name) {
                return Ok(Some(log));
            }
        }
        Ok(None)
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

    #[allow(dead_code)]
    pub fn get_recent_food_logs(
        &self,
        days: u32,
        name_filter: Option<&str>,
    ) -> Result<Vec<FoodLog>> {
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
                #[allow(clippy::collapsible_if)]
                if log.timestamp >= since {
                    results.push(TimelineEntry::Food(log));
                }
            }
        }

        // Get vitals logs
        for item in self.vitals_logs.iter().rev() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<VitalsLog>(&value) {
                #[allow(clippy::collapsible_if)]
                if log.timestamp >= since {
                    results.push(TimelineEntry::Vitals(log));
                }
            }
        }

        // Sort by timestamp descending (most recent first)
        results.sort_by_key(|b| std::cmp::Reverse(b.timestamp()));

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
                #[allow(clippy::collapsible_if)]
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
                #[allow(clippy::collapsible_if)]
                if log.timestamp >= since {
                    vitals_count += 1;
                }
            }
        }

        // Count food logs
        for item in self.substance_logs.iter() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<FoodLog>(&value) {
                #[allow(clippy::collapsible_if)]
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
                #[allow(clippy::collapsible_if)]
                if log.timestamp >= since {
                    results.push(log);
                }
            }
        }

        // Sort by timestamp ascending (oldest first for report)
        results.sort_by_key(|a| a.timestamp);
        Ok(results)
    }

    /// Get vitals logs with full details for report
    pub fn get_recent_vitals_logs_detailed(&self, days: u32) -> Result<Vec<VitalsLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.vitals_logs.iter().rev() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<VitalsLog>(&value) {
                #[allow(clippy::collapsible_if)]
                if log.timestamp >= since {
                    results.push(log);
                }
            }
        }

        // Sort by timestamp ascending (oldest first for report)
        results.sort_by_key(|a| a.timestamp);
        Ok(results)
    }

    /// Get food logs with full details for report
    pub fn get_recent_food_logs_detailed(&self, days: u32) -> Result<Vec<FoodLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<FoodLog>(&value) {
                #[allow(clippy::collapsible_if)]
                if log.timestamp >= since {
                    results.push(log);
                }
            }
        }

        // Sort by timestamp ascending (oldest first for report)
        results.sort_by_key(|a| a.timestamp);
        Ok(results)
    }

    // ===== Delete Operations =====

    /// Delete a substance log by ID
    pub fn delete_substance_log(&self, id: Uuid) -> Result<bool> {
        let mut found = false;
        let mut keys_to_delete = Vec::new();

        for item in self.substance_logs.iter() {
            let (key, value) = item?;
            if let Ok(log) = self.deserialize::<SubstanceLog>(&value) {
                if log.id == id {
                    keys_to_delete.push(key);
                    found = true;
                }
            }
        }

        for key in keys_to_delete {
            self.substance_logs.remove(key)?;
        }

        if found {
            self.flush()?;
        }

        Ok(found)
    }

    /// Delete a vitals log by ID
    pub fn delete_vitals_log(&self, id: Uuid) -> Result<bool> {
        let mut found = false;
        let mut keys_to_delete = Vec::new();

        for item in self.vitals_logs.iter() {
            let (key, value) = item?;
            if let Ok(log) = self.deserialize::<VitalsLog>(&value) {
                if log.id == id {
                    keys_to_delete.push(key);
                    found = true;
                }
            }
        }

        for key in keys_to_delete {
            self.vitals_logs.remove(key)?;
        }

        if found {
            self.flush()?;
        }

        Ok(found)
    }

    /// Delete a food log by ID
    pub fn delete_food_log(&self, id: Uuid) -> Result<bool> {
        let mut found = false;
        let mut keys_to_delete = Vec::new();

        for item in self.substance_logs.iter() {
            let (key, value) = item?;
            if let Ok(log) = self.deserialize::<FoodLog>(&value) {
                if log.id == id {
                    keys_to_delete.push(key);
                    found = true;
                }
            }
        }

        for key in keys_to_delete {
            self.substance_logs.remove(key)?;
        }

        if found {
            self.flush()?;
        }

        Ok(found)
    }
}

/// Summary statistics for report header
#[derive(Debug, Clone)]
pub struct ReportSummary {
    #[allow(dead_code)]
    pub days: u32,
    pub substance_logs: usize,
    pub vitals_logs: usize,
    pub food_logs: usize,
    pub unique_substances: usize,
}

/// Daily nutrient aggregate for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNutrientAggregate {
    pub date: chrono::NaiveDate,
    pub nutrients: Vec<NutrientInfo>,
}

/// Nutrient totals across a time range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientTotals {
    pub days: u32,
    pub totals: Vec<NutrientInfo>,
}

impl Database {
    /// Get all food logs with nutrient data for a time range (for report)
    pub fn get_recent_food_logs_with_nutrients(&self, days: u32) -> Result<Vec<FoodLog>> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let mut results = Vec::new();

        for item in self.substance_logs.iter().rev() {
            let (_, value) = item?;
            if let Ok(log) = self.deserialize::<FoodLog>(&value) {
                #[allow(clippy::collapsible_if)]
                if log.timestamp >= since {
                    results.push(log);
                }
            }
        }

        // Sort by timestamp ascending (oldest first for report)
        results.sort_by_key(|a| a.timestamp);
        Ok(results)
    }

    /// Get nutrient totals across a time range
    pub fn get_nutrient_totals(&self, days: u32) -> Result<NutrientTotals> {
        let food_logs = self.get_recent_food_logs_with_nutrients(days)?;

        // Aggregate nutrients by name
        let mut nutrient_map: std::collections::HashMap<String, (f64, String)> =
            std::collections::HashMap::new();

        for log in food_logs {
            if let Some(nutrients) = log.nutrients {
                for nutrient in nutrients {
                    let entry = nutrient_map
                        .entry(nutrient.name.clone())
                        .or_insert((0.0, nutrient.unit.clone()));
                    entry.0 += nutrient.amount;
                }
            }
        }

        let mut totals: Vec<NutrientInfo> = nutrient_map
            .into_iter()
            .map(|(name, (amount, unit))| NutrientInfo { name, amount, unit })
            .collect();

        // Sort by name for consistent output
        totals.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(NutrientTotals { days, totals })
    }

    /// Get daily nutrient aggregates for a time range
    pub fn get_daily_nutrient_aggregates(&self, days: u32) -> Result<Vec<DailyNutrientAggregate>> {
        let food_logs = self.get_recent_food_logs_with_nutrients(days)?;

        // Group by date
        let mut daily_map: std::collections::HashMap<chrono::NaiveDate, Vec<NutrientInfo>> =
            std::collections::HashMap::new();

        for log in food_logs {
            let date = log.timestamp.date_naive();
            if let Some(nutrients) = log.nutrients {
                let entry = daily_map.entry(date).or_insert_with(Vec::new);
                entry.extend(nutrients);
            }
        }

        // Aggregate nutrients within each day
        let mut aggregates: Vec<DailyNutrientAggregate> = daily_map
            .into_iter()
            .map(|(date, nutrients)| {
                let mut nutrient_map: std::collections::HashMap<String, (f64, String)> =
                    std::collections::HashMap::new();
                for nutrient in nutrients {
                    let entry = nutrient_map
                        .entry(nutrient.name)
                        .or_insert((0.0, nutrient.unit));
                    entry.0 += nutrient.amount;
                }
                let totals: Vec<NutrientInfo> = nutrient_map
                    .into_iter()
                    .map(|(name, (amount, unit))| NutrientInfo { name, amount, unit })
                    .collect();
                DailyNutrientAggregate {
                    date,
                    nutrients: totals,
                }
            })
            .collect();

        // Sort by date
        aggregates.sort_by_key(|a| a.date);

        Ok(aggregates)
    }

    /// Get nutrient status (intake vs RDI) for a time range
    pub fn get_nutrient_status(&self, days: u32) -> Result<Vec<NutrientStatus>> {
        use crate::nutrient_ref::{
            NutrientStatusLevel, calculate_nutrient_status, get_nutrient_references,
        };

        let totals = self.get_nutrient_totals(days)?;
        let refs = get_nutrient_references();

        let mut statuses = Vec::new();
        for nutrient in totals.totals {
            if let Some(reference) = refs.iter().find(|r| r.name == nutrient.name) {
                let status = calculate_nutrient_status(nutrient.amount, reference);
                statuses.push(status);
            }
        }

        // Sort by status priority: Deficient, Low, Excessive, VeryHigh, High, Adequate, NoRDI
        statuses.sort_by_key(|s| match s.status {
            NutrientStatusLevel::Deficient => 0,
            NutrientStatusLevel::Low => 1,
            NutrientStatusLevel::Excessive => 2,
            NutrientStatusLevel::VeryHigh => 3,
            NutrientStatusLevel::High => 4,
            NutrientStatusLevel::Adequate => 5,
            NutrientStatusLevel::NoRDI => 6,
        });

        Ok(statuses)
    }

    /// Get daily nutrient status for each day in range
    pub fn get_daily_nutrient_status(&self, days: u32) -> Result<Vec<DailyNutrientStatus>> {
        use crate::nutrient_ref::{
            NutrientStatusLevel, calculate_nutrient_status, get_nutrient_references,
        };

        let daily_aggregates = self.get_daily_nutrient_aggregates(days)?;
        let refs = get_nutrient_references();

        let mut results = Vec::new();
        for daily in daily_aggregates {
            let mut nutrient_statuses = Vec::new();
            for nutrient in daily.nutrients {
                if let Some(reference) = refs.iter().find(|r| r.name == nutrient.name) {
                    let status = calculate_nutrient_status(nutrient.amount, reference);
                    nutrient_statuses.push(status);
                }
            }

            // Sort by status priority
            nutrient_statuses.sort_by_key(|s| match s.status {
                NutrientStatusLevel::Deficient => 0,
                NutrientStatusLevel::Low => 1,
                NutrientStatusLevel::Excessive => 2,
                NutrientStatusLevel::VeryHigh => 3,
                NutrientStatusLevel::High => 4,
                NutrientStatusLevel::Adequate => 5,
                NutrientStatusLevel::NoRDI => 6,
            });

            results.push(DailyNutrientStatus {
                date: daily.date,
                nutrients: nutrient_statuses,
            });
        }

        Ok(results)
    }

    // ===== Protocol Versioning & Migration =====

    /// Insert or update a protocol in the database
    pub fn upsert_protocol(&self, protocol: &Protocol) -> Result<()> {
        let key = protocol.id.clone();
        let value = self.serialize(protocol)?;
        self.protocols.insert(key, value)?;
        self.flush()?;
        Ok(())
    }

    /// Get a protocol by ID from the database
    pub fn get_protocol(&self, id: &str) -> Result<Option<Protocol>> {
        if let Some(value) = self.protocols.get(id)? {
            let protocol: Protocol = self.deserialize(&value)?;
            Ok(Some(protocol))
        } else {
            Ok(None)
        }
    }

    /// List all protocols in the database
    pub fn list_protocols(&self) -> Result<Vec<Protocol>> {
        let mut results = Vec::new();
        for item in self.protocols.iter() {
            let (_, value) = item?;
            let protocol: Protocol = self.deserialize(&value)?;
            results.push(protocol);
        }
        results.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(results)
    }

    /// Delete a protocol from the database
    pub fn delete_protocol(&self, id: &str) -> Result<bool> {
        let removed = self.protocols.remove(id)?.is_some();
        if removed {
            self.flush()?;
        }
        Ok(removed)
    }

    /// Migrate a protocol from an older version to the current schema
    /// This applies version-specific transformations
    pub fn migrate_protocol(&self, protocol: &mut Protocol) -> Result<()> {
        // Parse version
        let version = protocol.version.clone();
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Ok(()); // Can't parse, skip migration
        }

        let major: u32 = parts[0].parse().unwrap_or(0);
        let minor: u32 = parts[1].parse().unwrap_or(0);
        let patch: u32 = parts[2].parse().unwrap_or(0);

        let mut current_version = (major, minor, patch);

        // Migration: 1.0.x -> 1.1.0 (example: added default evidence field if missing)
        if current_version < (1, 1, 0) {
            // Ensure evidence field is present (added in schema v1.1)
            if protocol.evidence.is_empty() {
                protocol.evidence = vec!["(migrated from v1.0 - evidence field added)".to_string()];
            }
            current_version = (1, 1, 0);
        }

        // Migration: 1.1.x -> 1.2.0 (example: added rationale to actions if missing)
        if current_version < (1, 2, 0) {
            for action in &mut protocol.actions {
                if action.rationale.is_none() {
                    action.rationale =
                        Some("(migrated from v1.1 - rationale field added)".to_string());
                }
            }
            current_version = (1, 2, 0);
        }

        // Migration: 1.x.x -> 2.0.0 (major version bump - breaking changes)
        // Example: renamed field "field" to "target_field" in atomic conditions
        if current_version < (2, 0, 0) {
            // For now, we just bump the version - actual field migrations would be more complex
            // and depend on specific breaking changes
            current_version = (2, 0, 0);
        }

        // Update version if migrated
        if current_version != (major, minor, patch) {
            protocol.version = format!(
                "{}.{}.{}",
                current_version.0, current_version.1, current_version.2
            );
        }

        Ok(())
    }

    /// Load all protocols from database, migrating as needed
    pub fn load_all_protocols(&self) -> Result<Vec<Protocol>> {
        let mut protocols = self.list_protocols()?;
        for protocol in &mut protocols {
            self.migrate_protocol(protocol)?;
            // Save migrated version back
            self.upsert_protocol(protocol)?;
        }
        Ok(protocols)
    }

    /// Save built-in protocols to database (for first-time setup or reset)
    pub fn seed_builtin_protocols(&self, protocols: Vec<Protocol>) -> Result<()> {
        for protocol in protocols {
            self.upsert_protocol(&protocol)?;
        }
        Ok(())
    }
}

//! Tests for show substances command

use biohack::db::Database;
use chrono::{Duration, Utc};
use tempfile::tempdir;
use uuid::Uuid;

#[cfg(test)]
mod show_substances_tests {
    use super::*;

    #[test]
    fn test_show_substances_empty_db() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();

        let args = biohack::cli::ShowCommands::Substances(biohack::cli::ShowSubstancesArgs {
            days: 3,
            name: None,
        });

        // Should not error, just show empty
        let result = biohack::commands::handle_show_substances(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_substances_with_logs() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path.clone())).unwrap();

        // Insert test substance
        let caffeine = biohack::models::Substance {
            id: Uuid::new_v4(),
            name: "Caffeine".to_string(),
            aliases: vec![],
            category: biohack::models::SubstanceCategory::Stimulant,
            min_dose_mg: Some(25.0),
            max_dose_mg: Some(400.0),
            typical_dose_mg: Some(100.0),
            half_life_hours: Some(5.0),
            contraindications: vec!["anxiety".to_string()],
            interactions: vec![],
            notes: None,
            sources: vec![],
        };

        db.insert_substance(&caffeine).unwrap();

        // Insert a substance log
        let log = biohack::models::SubstanceLog {
            id: Uuid::new_v4(),
            substance_id: Uuid::new_v4(),
            substance_name: "Caffeine".to_string(),
            dose_mg: 100.0,
            route: "oral".to_string(),
            timestamp: Utc::now() - Duration::hours(2),
            notes: Some("Morning coffee".to_string()),
            category: Some("stimulant".to_string()),
        };

        db.insert_substance_log(&log).unwrap();

        // Now test show substances
        let args = biohack::cli::ShowCommands::Substances(biohack::cli::ShowSubstancesArgs {
            days: 3,
            name: None,
        });

        let result = biohack::commands::handle_show_substances(&db, &args, false);
        assert!(result.is_ok());
    }
}

use biohack::cli::{LogCommands, SubstanceArgs};
use biohack::commands::handle_log_substance;
use biohack::db::Database;
use biohack::models::Substance;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[cfg(test)]
mod log_substance_integration_tests {
    use super::*;

    fn setup_db_with_substance(name: &str) -> (Database, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();

        // Insert a test substance
        let substance = Substance {
            id: Uuid::new_v4(),
            name: name.to_string(),
            aliases: vec![],
            category: biohack::models::SubstanceCategory::Supplement,
            min_dose_mg: Some(100.0),
            max_dose_mg: Some(500.0),
            typical_dose_mg: Some(250.0),
            half_life_hours: Some(8.0),
            contraindications: vec![],
            interactions: vec![],
            notes: None,
            sources: vec![],
        };
        db.insert_substance(&substance).unwrap();

        (db, temp_dir)
    }

    #[test]
    fn test_log_substance_success() {
        let (db, _temp_dir) = setup_db_with_substance("Vitamin C");

        let args = SubstanceArgs {
            name: "Vitamin C".to_string(),
            dose: "500mg".to_string(),
            route: "oral".to_string(),
            time: None,
            notes: Some("Test log".to_string()),
        };
        let cmd = LogCommands::Substance(args);

        let result = handle_log_substance(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the log was inserted correctly
        let since = Utc::now() - Duration::hours(1);
        let logs = db.get_recent_substance_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.substance_name, "Vitamin C");
        assert_eq!(log.dose_mg, 500.0);
        assert_eq!(log.route, "oral");
        assert_eq!(log.notes.as_deref(), Some("Test log"));
        assert!(log.timestamp >= since);
    }

    #[test]
    fn test_log_substance_with_different_routes() {
        let (db, _temp_dir) = setup_db_with_substance("Magnesium");

        let routes = vec!["oral", "sublingual", "transdermal"];
        for route in routes {
            let args = SubstanceArgs {
                name: "Magnesium".to_string(),
                dose: "200mg".to_string(),
                route: route.to_string(),
                time: None,
                notes: None,
            };
            let cmd = LogCommands::Substance(args);

            let result = handle_log_substance(&db, &cmd, false);
            assert!(result.is_ok(), "Failed for route: {}", route);
        }
    }

    #[test]
    fn test_log_substance_with_custom_timestamp() {
        let (db, _temp_dir) = setup_db_with_substance("Zinc");
        let one_hour_ago = Utc::now() - Duration::hours(1);
        let custom_time = one_hour_ago.to_rfc3339();

        let args = SubstanceArgs {
            name: "Zinc".to_string(),
            dose: "25mg".to_string(),
            route: "oral".to_string(),
            time: Some(custom_time.to_string()),
            notes: None,
        };
        let cmd = LogCommands::Substance(args);

        let result = handle_log_substance(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the timestamp was parsed correctly
        let logs = db.get_recent_substance_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        let expected = DateTime::parse_from_rfc3339(&custom_time)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(log.timestamp, expected);
    }

    #[test]
    fn test_log_substance_substance_not_found() {
        let (db, _temp_dir) = setup_db_with_substance("Placeholder"); // We won't use this substance

        let args = SubstanceArgs {
            name: "NonExistentSubstance".to_string(),
            dose: "100mg".to_string(),
            route: "oral".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Substance(args);

        let result = handle_log_substance(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Substance not found in database"));
    }

    #[test]
    fn test_log_substance_invalid_dose() {
        let (db, _temp_dir) = setup_db_with_substance("TestSubstance");

        let args = SubstanceArgs {
            name: "TestSubstance".to_string(),
            dose: "invalid".to_string(),
            route: "oral".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Substance(args);

        let result = handle_log_substance(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid dose format"));
    }

    #[test]
    fn test_log_substance_invalid_timestamp() {
        let (db, _temp_dir) = setup_db_with_substance("TestSubstance");

        let args = SubstanceArgs {
            name: "TestSubstance".to_string(),
            dose: "100mg".to_string(),
            route: "oral".to_string(),
            time: Some("not-a-timestamp".to_string()),
            notes: None,
        };
        let cmd = LogCommands::Substance(args);

        let result = handle_log_substance(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid timestamp format"));
    }
}

use biohack::cli::{FoodArgs, LogCommands, SubstanceArgs, VitalsArgs};
use biohack::commands::{
    handle_check, handle_log_food, handle_log_substance, handle_log_vitals, handle_substance_seed,
};
use biohack::db::Database;

#[cfg(test)]
mod error_handling_integration_tests {
    use super::*;

    fn setup_empty_db() -> (Database, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();
        (db, temp_dir)
    }

    #[test]
    fn test_log_substance_missing_required_args() {
        let (db, _temp_dir) = setup_empty_db();

        // Test missing name
        let args = SubstanceArgs {
            name: "".to_string(), // Empty name
            dose: "100mg".to_string(),
            route: "oral".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Substance(args);
        let result = handle_log_substance(&db, &cmd, false);
        // Empty name should still work (will fail on substance not found, which is expected)
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Substance not found in database"));

        // Test missing dose
        let args = SubstanceArgs {
            name: "Test".to_string(),
            dose: "".to_string(), // Empty dose
            route: "oral".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Substance(args);
        let result = handle_log_substance(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Dose cannot be empty"));
    }

    #[test]
    fn test_log_vitals_minimal_args() {
        let (db, _temp_dir) = setup_empty_db();

        // Test with no vitals at all (all None)
        let args = VitalsArgs {
            hr: None,
            sbp: None,
            dbp: None,
            temp: None,
            spo2: None,
            hrv: None,
            weight: None,
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Vitals(args);
        let result = handle_log_vitals(&db, &cmd, false);
        // Should succeed but create a log with all None values
        assert!(result.is_ok());

        let logs = db.get_recent_vitals_logs(1).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.heart_rate, None);
        assert_eq!(log.sbp, None);
        assert_eq!(log.dbp, None);
        assert_eq!(log.temperature_c, None);
        assert_eq!(log.spo2, None);
        assert_eq!(log.hrv_rmssd, None);
        assert_eq!(log.weight_kg, None);
    }

    #[test]
    fn test_log_food_missing_required_args() {
        let (db, _temp_dir) = setup_empty_db();

        // Test missing name
        let args = FoodArgs {
            name: "".to_string(), // Empty name
            amount: 1.0,
            unit: "pieces".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Food(args);
        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok()); // Empty name is allowed

        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.food_name, "");
    }

    #[test]
    fn test_substance_seed_file_not_found() {
        let (db, _temp_dir) = setup_empty_db();

        let args = biohack::cli::SubstanceSeedArgs {
            path: std::path::PathBuf::from("non_existent.yaml"),
        };
        let cmd = biohack::cli::SubstanceCommands::Seed(args);

        let result = handle_substance_seed(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("No such file or directory"));
    }

    #[test]
    fn test_check_command_always_succeeds() {
        let (db, _temp_dir) = setup_empty_db();

        // Check command should always succeed, even with empty DB
        let result = handle_check(&db, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_substance_name_in_log() {
        let (db, _temp_dir) = setup_empty_db();

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
    fn test_database_creation_works() {
        // Test that we can create a database successfully
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let result = Database::new(Some(db_path));
        // This should succeed
        assert!(result.is_ok());

        // And we should be able to use it
        let db = result.unwrap();
        let substances = db.list_substances(None).unwrap();
        assert_eq!(substances.len(), 0); // Empty DB
    }
}
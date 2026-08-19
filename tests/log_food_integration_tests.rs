//! Integration tests for food logging (OpenFoodFacts + USDA)

use biohack::cli::{FoodArgs, LogCommands};
use biohack::commands::handle_log_food;
use biohack::db::Database;
use biohack::models::FoodDataSource;
use chrono::{DateTime, Duration, Utc};
#[allow(unused_imports)]
use tempfile::tempdir;

#[cfg(test)]
mod log_food_integration_tests {
    use super::*;

    fn setup_empty_db() -> (Database, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();
        (db, temp_dir)
    }

    #[test]
    fn test_log_food_success() {
        let (db, _temp_dir) = setup_empty_db();

        let args = FoodArgs {
            name: "Apple".to_string(),
            amount: 2.0,
            unit: "pieces".to_string(),
            time: None,
            notes: Some("Snack".to_string()),
        };
        let cmd = LogCommands::Food(args);

        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the log was inserted correctly
        let since = Utc::now() - Duration::hours(1);
        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.food_name, "Apple");
        assert_eq!(log.amount, 2.0);
        assert_eq!(log.unit, "pieces");
        assert_eq!(log.notes.as_deref(), Some("Snack"));
        assert!(log.timestamp >= since);
    }

    #[test]
    fn test_log_food_different_units() {
        let (db, _temp_dir) = setup_empty_db();

        let test_cases = vec![
            ("Banana", 1.5, "pieces"),
            ("Milk", 250.0, "ml"),
            ("Bread", 2.0, "slices"),
            ("Rice", 1.0, "cup"),
            ("Chicken", 100.0, "g"),
        ];

        let expected_count = test_cases.len();
        for (name, amount, unit) in test_cases {
            let args = FoodArgs {
                name: name.to_string(),
                amount,
                unit: unit.to_string(),
                time: None,
                notes: None,
            };
            let cmd = LogCommands::Food(args);

            let result = handle_log_food(&db, &cmd, false);
            assert!(result.is_ok(), "Failed for {} {} {}", name, amount, unit);
        }

        // Verify all logs were inserted
        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), expected_count);
    }

    #[test]
    fn test_log_food_with_custom_timestamp() {
        let (db, _temp_dir) = setup_empty_db();
        let one_hour_ago = Utc::now() - Duration::hours(1);
        let custom_time = one_hour_ago.to_rfc3339();

        let args = FoodArgs {
            name: "Yogurt".to_string(),
            amount: 1.0,
            unit: "cup".to_string(),
            time: Some(custom_time.to_string()),
            notes: None,
        };
        let cmd = LogCommands::Food(args);

        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the timestamp was parsed correctly
        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        let expected = DateTime::parse_from_rfc3339(&custom_time)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(log.timestamp, expected);
    }

    #[test]
    fn test_log_food_zero_amount() {
        let (db, _temp_dir) = setup_empty_db();

        let args = FoodArgs {
            name: "Water".to_string(),
            amount: 0.0,
            unit: "ml".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Food(args);

        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok());

        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.amount, 0.0);
    }

    #[test]
    fn test_log_food_invalid_timestamp() {
        let (db, _temp_dir) = setup_empty_db();

        let args = FoodArgs {
            name: "Toast".to_string(),
            amount: 2.0,
            unit: "slices".to_string(),
            time: Some("not-a-timestamp".to_string()),
            notes: None,
        };
        let cmd = LogCommands::Food(args);

        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid timestamp format"));
    }

    #[test]
    fn test_log_food_very_large_amount() {
        let (db, _temp_dir) = setup_empty_db();

        let args = FoodArgs {
            name: "Salt".to_string(),
            amount: 1000000.0,
            unit: "mg".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Food(args);

        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok());

        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.amount, 1000000.0);
    }

    // --- OpenFoodFacts Integration Tests ---

    #[test]
    fn test_log_food_unknown_food_still_logs_without_db_id() {
        let (db, _temp_dir) = setup_empty_db();

        // Log a food that definitely won't be in any database
        let args = FoodArgs {
            name: "xyzzy_nonexistent_food_12345".to_string(),
            amount: 100.0,
            unit: "g".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Food(args);

        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the log was inserted but without food_db_id or source
        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];

        // Should NOT have food_db_id or source since no match was found
        assert!(log.food_db_id.is_none(), "food_db_id should be None when no match found");
        assert!(log.source.is_none(), "source should be None when no match found");
        assert!(log.nutrients.is_none(), "nutrients should be None when no match found");
    }

    #[test]
    fn test_log_food_preserves_user_provided_name() {
        let (db, _temp_dir) = setup_empty_db();

        // Log with user's custom name
        let args = FoodArgs {
            name: "My homemade smoothie".to_string(),
            amount: 500.0,
            unit: "ml".to_string(),
            time: None,
            notes: Some("breakfast".to_string()),
        };
        let cmd = LogCommands::Food(args);

        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the original user-provided name is preserved
        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.food_name, "My homemade smoothie");
    }

    // Note: The following tests depend on external API (OpenFoodFacts/USDA) availability.
    // They test the code paths but don't assert on external API results since those
    // can vary based on rate limiting, network, and API changes.
    
    #[test]
    fn test_log_food_code_path_for_food_database_lookup() {
        // This test verifies the code path for food database lookup executes without error
        // It uses a query that may or may not return results depending on API availability
        let (db, _temp_dir) = setup_empty_db();

        let args = FoodArgs {
            name: "banana".to_string(),
            amount: 100.0,
            unit: "g".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Food(args);

        // This should not error regardless of API response
        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the log was inserted
        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.food_name, "banana");
        
        // food_db_id, source, and nutrients are populated IF the API returns a match
        // We don't assert on them since external API availability varies
    }

    #[test]
    fn test_log_food_with_nutrient_display_path() {
        // Test the nutrient display code path executes
        let (db, _temp_dir) = setup_empty_db();

        let args = FoodArgs {
            name: "chicken breast".to_string(),
            amount: 150.0,
            unit: "g".to_string(),
            time: None,
            notes: None,
        };
        let cmd = LogCommands::Food(args);

        let result = handle_log_food(&db, &cmd, false);
        assert!(result.is_ok());

        let logs = db.get_recent_food_logs(1, None).unwrap();
        assert_eq!(logs.len(), 1);
    }
}
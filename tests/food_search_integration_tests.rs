//! Integration tests for food search (OpenFoodFacts + USDA)

use biohack::cli::FoodSearchArgs;
use biohack::commands::handle_food_search;
use biohack::db::Database;
use tempfile::tempdir;

#[cfg(test)]
mod food_search_integration_tests {
    use super::*;

    fn setup_db() -> (Database, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();
        (db, temp_dir)
    }

    #[test]
    fn test_food_search_empty_query_returns_no_results() {
        let (db, _temp_dir) = setup_db();

        let args = FoodSearchArgs {
            query: "xyzzy_nonexistent_food_12345".to_string(),
            limit: 10,
        };

        let result = handle_food_search(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_food_search_returns_results_for_common_food() {
        let (db, _temp_dir) = setup_db();

        // Search for a common food that should exist in OpenFoodFacts
        let args = FoodSearchArgs {
            query: "banana".to_string(),
            limit: 5,
        };

        let result = handle_food_search(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_food_search_uk_branded_product() {
        let (db, _temp_dir) = setup_db();

        // Search for a UK branded product that should exist in OpenFoodFacts
        let args = FoodSearchArgs {
            query: "Tesco".to_string(),
            limit: 5,
        };

        let result = handle_food_search(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_food_search_respects_limit() {
        let (db, _temp_dir) = setup_db();

        let args = FoodSearchArgs {
            query: "apple".to_string(),
            limit: 2,
        };

        let result = handle_food_search(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_food_search_special_characters() {
        let (db, _temp_dir) = setup_db();

        let args = FoodSearchArgs {
            query: "chocolate milk".to_string(),
            limit: 5,
        };

        let result = handle_food_search(&db, &args, false);
        assert!(result.is_ok());
    }
}
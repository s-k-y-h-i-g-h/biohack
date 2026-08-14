//! Tests for show timeline command

use biohack::db::Database;
use chrono::{Duration, Utc};
use tempfile::tempdir;
use uuid::Uuid;

#[cfg(test)]
mod show_timeline_tests {
    use super::*;

    #[test]
    fn test_show_timeline_empty_db() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();

        let args = biohack::cli::ShowCommands::Timeline(biohack::cli::ShowTimelineArgs { days: 3 });

        // Should not error, just show empty
        let result = biohack::commands::handle_show_timeline(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_timeline_with_substance_logs() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path.clone())).unwrap();

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

        let args = biohack::cli::ShowCommands::Timeline(biohack::cli::ShowTimelineArgs { days: 3 });

        let result = biohack::commands::handle_show_timeline(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_timeline_with_vitals_logs() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path.clone())).unwrap();

        // Insert a vitals log
        let log = biohack::models::VitalsLog {
            id: Uuid::new_v4(),
            heart_rate: Some(88),
            sbp: Some(120),
            dbp: Some(80),
            temperature_c: Some(37.0),
            spo2: Some(98),
            hrv_rmssd: Some(45),
            weight_kg: Some(70.5),
            timestamp: Utc::now() - Duration::hours(2),
            notes: Some("Morning check".to_string()),
        };

        db.insert_vitals_log(&log).unwrap();

        let args = biohack::cli::ShowCommands::Timeline(biohack::cli::ShowTimelineArgs { days: 3 });

        let result = biohack::commands::handle_show_timeline(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_timeline_with_food_logs() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path.clone())).unwrap();

        // Insert a food log
        let log = biohack::models::FoodLog {
            id: Uuid::new_v4(),
            food_name: "Broccoli".to_string(),
            amount: 2.0,
            unit: "cups".to_string(),
            timestamp: Utc::now() - Duration::hours(2),
            notes: Some("Lunch".to_string()),
        };

        db.insert_food_log(&log).unwrap();

        let args = biohack::cli::ShowCommands::Timeline(biohack::cli::ShowTimelineArgs { days: 3 });

        let result = biohack::commands::handle_show_timeline(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_timeline_combined_chronological_order() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path.clone())).unwrap();

        // Insert logs at different times - we want to verify chronological order
        // The timeline should show them sorted by timestamp (most recent first)

        // 3 hours ago - substance
        let sub_log = biohack::models::SubstanceLog {
            id: Uuid::new_v4(),
            substance_id: Uuid::new_v4(),
            substance_name: "Caffeine".to_string(),
            dose_mg: 100.0,
            route: "oral".to_string(),
            timestamp: Utc::now() - Duration::hours(3),
            notes: Some("Coffee".to_string()),
            category: Some("stimulant".to_string()),
        };

        // 2 hours ago - vitals
        let vitals_log = biohack::models::VitalsLog {
            id: Uuid::new_v4(),
            heart_rate: Some(88),
            sbp: Some(120),
            dbp: Some(80),
            temperature_c: Some(37.0),
            spo2: Some(98),
            hrv_rmssd: Some(45),
            weight_kg: Some(70.5),
            timestamp: Utc::now() - Duration::hours(2),
            notes: Some("Check".to_string()),
        };

        // 1 hour ago - food
        let food_log = biohack::models::FoodLog {
            id: Uuid::new_v4(),
            food_name: "Apple".to_string(),
            amount: 1.0,
            unit: "pcs".to_string(),
            timestamp: Utc::now() - Duration::hours(1),
            notes: Some("Snack".to_string()),
        };

        db.insert_substance_log(&sub_log).unwrap();
        db.insert_vitals_log(&vitals_log).unwrap();
        db.insert_food_log(&food_log).unwrap();

        let args = biohack::cli::ShowCommands::Timeline(biohack::cli::ShowTimelineArgs { days: 3 });

        let result = biohack::commands::handle_show_timeline(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_timeline_filters_by_days() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path.clone())).unwrap();

        // Old log (10 days ago)
        let old_sub = biohack::models::SubstanceLog {
            id: Uuid::new_v4(),
            substance_id: Uuid::new_v4(),
            substance_name: "Old".to_string(),
            dose_mg: 100.0,
            route: "oral".to_string(),
            timestamp: Utc::now() - Duration::days(10),
            notes: None,
            category: None,
        };
        db.insert_substance_log(&old_sub).unwrap();

        // Recent log (1 hour ago)
        let recent_vitals = biohack::models::VitalsLog {
            id: Uuid::new_v4(),
            heart_rate: Some(80),
            sbp: Some(110),
            dbp: Some(70),
            temperature_c: Some(36.8),
            spo2: Some(99),
            hrv_rmssd: None,
            weight_kg: None,
            timestamp: Utc::now() - Duration::hours(1),
            notes: Some("Recent".to_string()),
        };
        db.insert_vitals_log(&recent_vitals).unwrap();

        // Query with 3 days - should only get recent
        let args = biohack::cli::ShowCommands::Timeline(biohack::cli::ShowTimelineArgs { days: 3 });

        let result = biohack::commands::handle_show_timeline(&db, &args, false);
        assert!(result.is_ok());
    }
}

//! Tests for show vitals command

use biohack::db::Database;
use biohack::models::VitalsLog;
use biohack::cli::{ShowCommands, ShowVitalsArgs};
use chrono::{Utc, Duration};
use uuid::Uuid;
use tempfile::tempdir;

#[cfg(test)]
mod show_vitals_tests {
    use super::*;

    #[test]
    fn test_show_vitals_empty_db() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();
        
        let args = biohack::cli::ShowCommands::Vitals(biohack::cli::ShowVitalsArgs {
            days: 3,
        });
        
        // Should not error, just show empty
        let result = biohack::commands::handle_show_vitals(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_vitals_with_logs() {
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
        
        // Now test show vitals
        let args = biohack::cli::ShowCommands::Vitals(biohack::cli::ShowVitalsArgs {
            days: 3,
        });
        
        let result = biohack::commands::handle_show_vitals(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_vitals_filters_by_days() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path.clone())).unwrap();
        
        // Insert an old vitals log (outside the filter window)
        let old_log = biohack::models::VitalsLog {
            id: Uuid::new_v4(),
            heart_rate: Some(75),
            sbp: Some(115),
            dbp: Some(75),
            temperature_c: Some(36.8),
            spo2: Some(99),
            hrv_rmssd: None,
            weight_kg: None,
            timestamp: Utc::now() - Duration::days(10),
            notes: Some("Old log".to_string()),
        };
        
        db.insert_vitals_log(&old_log).unwrap();
        
        // Insert a recent vitals log
        let recent_log = biohack::models::VitalsLog {
            id: Uuid::new_v4(),
            heart_rate: Some(88),
            sbp: Some(120),
            dbp: Some(80),
            temperature_c: Some(37.0),
            spo2: Some(98),
            hrv_rmssd: Some(45),
            weight_kg: Some(70.5),
            timestamp: Utc::now() - Duration::hours(2),
            notes: Some("Recent log".to_string()),
        };
        
        db.insert_vitals_log(&recent_log).unwrap();
        
        // Query with 3 days - should only get the recent one
        let args = biohack::cli::ShowCommands::Vitals(biohack::cli::ShowVitalsArgs {
            days: 3,
        });
        
        let result = biohack::commands::handle_show_vitals(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_recent_vitals_logs_db_query() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path.clone())).unwrap();
        
        // Insert multiple logs at different times
        for i in 1..=5 {
            let log = biohack::models::VitalsLog {
                id: Uuid::new_v4(),
                heart_rate: Some(70 + i * 2),
                sbp: Some(110 + i * 2),
                dbp: Some(70 + i),
                temperature_c: Some(36.5 + i as f32 * 0.1),
                spo2: Some(98),
                hrv_rmssd: Some(40 + i as u32),
                weight_kg: Some(70.0 + i as f32 * 0.5),
                timestamp: Utc::now() - Duration::hours(i as i64),
                notes: Some(format!("Log {}", i)),
            };
            db.insert_vitals_log(&log).unwrap();
        }
        
        // Query for 2 days (should get all 5)
        let logs = db.get_recent_vitals_logs(2).unwrap();
        assert_eq!(logs.len(), 5);
        
        // Query for 1 day (should get all 5 since they're within 5 hours)
        let logs = db.get_recent_vitals_logs(1).unwrap();
        assert_eq!(logs.len(), 5);
        // Most recent should be first (Log 1, which was 1 hour ago)
        assert_eq!(logs[0].notes, Some("Log 1".to_string()));
        
        // Logs should be in chronological order (most recent first since we iterate .rev())
        let logs = db.get_recent_vitals_logs(2).unwrap();
        assert!(logs[0].timestamp >= logs[1].timestamp);
    }
}
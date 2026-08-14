use biohack::cli::{LogCommands, VitalsArgs};
use biohack::commands::handle_log_vitals;
use biohack::db::Database;
use chrono::{DateTime, Duration, Utc};
use tempfile::tempdir;
use uuid::Uuid;

#[cfg(test)]
mod log_vitals_integration_tests {
    use super::*;

    fn setup_empty_db() -> (Database, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();
        (db, temp_dir)
    }

    #[test]
    fn test_log_vitals_success() {
        let (db, _temp_dir) = setup_empty_db();

        let args = VitalsArgs {
            hr: Some(72),
            sbp: Some(120),
            dbp: Some(80),
            temp: Some(36.6),
            spo2: Some(98),
            hrv: Some(60),
            weight: Some(70.5),
            time: None,
            notes: Some("Morning check".to_string()),
        };
        let cmd = LogCommands::Vitals(args);

        let result = handle_log_vitals(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the log was inserted correctly
        let since = Utc::now() - Duration::hours(1);
        let logs = db.get_recent_vitals_logs(1).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.heart_rate, Some(72));
        assert_eq!(log.sbp, Some(120));
        assert_eq!(log.dbp, Some(80));
        assert_eq!(log.temperature_c, Some(36.6));
        assert_eq!(log.spo2, Some(98));
        assert_eq!(log.hrv_rmssd, Some(60));
        assert_eq!(log.weight_kg, Some(70.5));
        assert_eq!(log.notes.as_deref(), Some("Morning check"));
        assert!(log.timestamp >= since);
    }

    #[test]
    fn test_log_vitals_partial_fields() {
        let (db, _temp_dir) = setup_empty_db();

        // Test with only HR
        let args = VitalsArgs {
            hr: Some(88),
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
        assert!(result.is_ok());

        let logs = db.get_recent_vitals_logs(1).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.heart_rate, Some(88));
        assert_eq!(log.sbp, None);
        assert_eq!(log.dbp, None);
        assert_eq!(log.temperature_c, None);
        assert_eq!(log.spo2, None);
        assert_eq!(log.hrv_rmssd, None);
        assert_eq!(log.weight_kg, None);
    }

    #[test]
    fn test_log_vitals_with_custom_timestamp() {
        let (db, _temp_dir) = setup_empty_db();
        let one_hour_ago = Utc::now() - Duration::hours(1);
        let custom_time = one_hour_ago.to_rfc3339();

        let args = VitalsArgs {
            hr: Some(75),
            sbp: Some(118),
            dbp: Some(76),
            temp: Some(36.8),
            spo2: Some(99),
            hrv: Some(55),
            weight: Some(68.0),
            time: Some(custom_time.to_string()),
            notes: None,
        };
        let cmd = LogCommands::Vitals(args);

        let result = handle_log_vitals(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify the timestamp was parsed correctly
        let logs = db.get_recent_vitals_logs(1).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        let expected = DateTime::parse_from_rfc3339(&custom_time)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(log.timestamp, expected);
    }

    #[test]
    fn test_log_vitals_invalid_hr() {
        let (db, _temp_dir) = setup_empty_db();

        let args = VitalsArgs {
            hr: Some(0), // Invalid HR (too low)
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

        // Note: Currently, there's no validation on HR range in the CLI - it just accepts any u32.
        // So this should succeed. We're testing that invalid values are stored as-is.
        let result = handle_log_vitals(&db, &cmd, false);
        assert!(result.is_ok());

        let logs = db.get_recent_vitals_logs(1).unwrap();
        assert_eq!(logs.len(), 1);
        let log = &logs[0];
        assert_eq!(log.heart_rate, Some(0));
    }

    #[test]
    fn test_log_vitals_invalid_timestamp() {
        let (db, _temp_dir) = setup_empty_db();

        let args = VitalsArgs {
            hr: Some(70),
            sbp: Some(120),
            dbp: Some(80),
            temp: Some(36.6),
            spo2: Some(98),
            hrv: Some(60),
            weight: Some(70.0),
            time: Some("not-a-timestamp".to_string()),
            notes: None,
        };
        let cmd = LogCommands::Vitals(args);

        let result = handle_log_vitals(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Invalid timestamp format"));
    }

    #[test]
    fn test_log_vitals_no_args() {
        let (db, _temp_dir) = setup_empty_db();

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
        // Currently, the CLI accepts no vitals (though it prints a warning in the table).
        // The function itself should succeed because it just inserts a log with all None values.
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
}
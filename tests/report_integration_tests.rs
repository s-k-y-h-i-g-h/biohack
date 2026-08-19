use biohack::cli::{ProtocolCommands, ProtocolTestArgs, ReportArgs};
use biohack::commands::{handle_protocol_test, handle_report};
use biohack::db::Database;
use biohack::models::{Schedule, Stack, StackItem, Substance, SubstanceLog, VitalsLog};
use chrono::{Duration, Utc};
use std::fs;
use tempfile::tempdir;
use uuid::Uuid;

#[cfg(test)]
mod report_integration_tests {
    use super::*;

    fn setup_db_with_data() -> (Database, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();

        // Insert test substances
        let substances = vec![
            Substance {
                id: Uuid::new_v4(),
                name: "L-Theanine".to_string(),
                aliases: vec![],
                category: biohack::models::SubstanceCategory::Nootropic,
                min_dose_mg: Some(100.0),
                max_dose_mg: Some(500.0),
                typical_dose_mg: Some(200.0),
                half_life_hours: Some(1.2),
                contraindications: vec![],
                interactions: vec![],
                notes: None,
                sources: vec![],
            },
            Substance {
                id: Uuid::new_v4(),
                name: "Caffeine".to_string(),
                aliases: vec![],
                category: biohack::models::SubstanceCategory::Stimulant,
                min_dose_mg: Some(50.0),
                max_dose_mg: Some(400.0),
                typical_dose_mg: Some(100.0),
                half_life_hours: Some(5.0),
                contraindications: vec![],
                interactions: vec![],
                notes: None,
                sources: vec![],
            },
            Substance {
                id: Uuid::new_v4(),
                name: "Vitamin D3".to_string(),
                aliases: vec![],
                category: biohack::models::SubstanceCategory::Vitamin,
                min_dose_mg: Some(0.01),
                max_dose_mg: Some(0.1),
                typical_dose_mg: Some(0.05),
                half_life_hours: Some(15.0),
                contraindications: vec![],
                interactions: vec![],
                notes: None,
                sources: vec![],
            },
        ];

        for substance in &substances {
            db.insert_substance(substance).unwrap();
        }

        // Insert substance logs
        let logs = vec![
            SubstanceLog {
                id: Uuid::new_v4(),
                substance_id: substances[0].id,
                substance_name: "L-Theanine".to_string(),
                dose_mg: 200.0,
                route: "oral".to_string(),
                timestamp: Utc::now() - Duration::hours(2),
                notes: Some("Morning dose".to_string()),
                category: Some("nootropic".to_string()),
            },
            SubstanceLog {
                id: Uuid::new_v4(),
                substance_id: substances[1].id,
                substance_name: "Caffeine".to_string(),
                dose_mg: 100.0,
                route: "oral".to_string(),
                timestamp: Utc::now() - Duration::hours(1),
                notes: None,
                category: Some("stimulant".to_string()),
            },
        ];

        for log in &logs {
            db.insert_substance_log(log).unwrap();
        }

        // Insert vitals logs
        let vitals = vec![VitalsLog {
            id: Uuid::new_v4(),
            heart_rate: Some(72),
            sbp: Some(120),
            dbp: Some(80),
            temperature_c: Some(36.8),
            spo2: Some(98),
            hrv_rmssd: Some(45),
            weight_kg: Some(75.5),
            timestamp: Utc::now() - Duration::hours(3),
            notes: Some("Morning vitals".to_string()),
        }];

        for vital in &vitals {
            db.insert_vitals_log(vital).unwrap();
        }

        // Insert food logs (stored in substance_logs tree)
        // We'll just verify the report handles empty food logs

        // Insert a stack
        let stack = Stack {
            name: "Test Stack".to_string(),
            description: Some("Test stack".to_string()),
            items: vec![StackItem {
                substance_name: "L-Theanine".to_string(),
                dose: "200mg".to_string(),
                route: Some("oral".to_string()),
                schedule: Some(Schedule::Morning),
            }],
        };
        db.insert_stack(&stack).unwrap();

        (db, temp_dir)
    }

    #[test]
    fn test_report_markdown_default() {
        let (db, _temp_dir) = setup_db_with_data();

        let args = ReportArgs {
            days: 7,
            format: "markdown".to_string(),
            output: None,
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_report_markdown_to_file() {
        let (db, temp_dir) = setup_db_with_data();
        let output_path = temp_dir.path().join("report.md");

        let args = ReportArgs {
            days: 7,
            format: "markdown".to_string(),
            output: Some(output_path.clone()),
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_ok());

        // Verify file was created
        assert!(output_path.exists());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("# Biohack Health Report"));
        assert!(content.contains("Substance Logs"));
        assert!(content.contains("L-Theanine"));
        assert!(content.contains("Caffeine"));
    }

    #[test]
    fn test_report_csv_format() {
        let (db, temp_dir) = setup_db_with_data();
        let output_path = temp_dir.path().join("report.csv");

        let args = ReportArgs {
            days: 7,
            format: "csv".to_string(),
            output: Some(output_path.clone()),
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_ok());

        // Verify file was created
        assert!(output_path.exists());
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("timestamp,substance_name,dose_mg,route,category,notes"));
        assert!(content.contains("L-Theanine"));
        assert!(content.contains("Caffeine"));
    }

    #[test]
    fn test_report_csv_to_stdout() {
        let (db, _temp_dir) = setup_db_with_data();

        let args = ReportArgs {
            days: 7,
            format: "csv".to_string(),
            output: None,
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_report_invalid_format() {
        let (db, _temp_dir) = setup_db_with_data();

        let args = ReportArgs {
            days: 7,
            format: "invalid".to_string(),
            output: None,
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unknown format"));
    }

    #[test]
    fn test_report_custom_days() {
        let (db, _temp_dir) = setup_db_with_data();

        // Test with 1 day (should still include our test data)
        let args = ReportArgs {
            days: 1,
            format: "markdown".to_string(),
            output: None,
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_ok());

        // Test with 30 days
        let args = ReportArgs {
            days: 30,
            format: "markdown".to_string(),
            output: None,
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_report_empty_db() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("empty.db");
        let db = Database::new(Some(db_path)).unwrap();

        let args = ReportArgs {
            days: 7,
            format: "markdown".to_string(),
            output: None,
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_protocol_test_command() {
        let (db, _temp_dir) = setup_db_with_data();

        // Test with stimulant_tachycardia protocol
        let args = ProtocolTestArgs {
            protocol_id: "stimulant_tachycardia".to_string(),
        };
        let cmd = ProtocolCommands::Test(args);

        let result = handle_protocol_test(&db, &cmd, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_protocol_test_not_found() {
        let (db, _temp_dir) = setup_db_with_data();

        let args = ProtocolTestArgs {
            protocol_id: "nonexistent_protocol".to_string(),
        };
        let cmd = ProtocolCommands::Test(args);

        let result = handle_protocol_test(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                .contains("Protocol 'nonexistent_protocol' not found")
        );
    }

    #[test]
    fn test_report_md_format_alias() {
        let (db, temp_dir) = setup_db_with_data();
        let output_path = temp_dir.path().join("report.md");

        let args = ReportArgs {
            days: 7,
            format: "md".to_string(),
            output: Some(output_path.clone()),
        };

        let result = handle_report(&db, &args, false);
        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}

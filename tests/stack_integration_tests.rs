use biohack::cli::{LogCommands, StackArgs, StackCommands, StackCreateArgs, StackShowArgs};
use biohack::commands::{
    handle_log_stack, handle_stack_create, handle_stack_list, handle_stack_show,
};
use biohack::db::Database;
use biohack::models::Schedule;
use std::fs;
use tempfile::tempdir;
use uuid::Uuid;

#[cfg(test)]
mod stack_integration_tests {
    use super::*;

    fn setup_db_with_substances() -> (Database, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();

        // Insert test substances
        let substances = vec![
            biohack::models::Substance {
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
            biohack::models::Substance {
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
            biohack::models::Substance {
                id: Uuid::new_v4(),
                name: "Omega-3 Fish Oil".to_string(),
                aliases: vec![],
                category: biohack::models::SubstanceCategory::Supplement,
                min_dose_mg: Some(500.0),
                max_dose_mg: Some(5000.0),
                typical_dose_mg: Some(2000.0),
                half_life_hours: Some(48.0),
                contraindications: vec![],
                interactions: vec![],
                notes: None,
                sources: vec![],
            },
        ];

        for substance in &substances {
            db.insert_substance(substance).unwrap();
        }

        (db, temp_dir)
    }

    fn create_test_stack_yaml(temp_dir: &tempfile::TempDir) -> std::path::PathBuf {
        let yaml_path = temp_dir.path().join("test_stack.yaml");
        let content = r#"
name: "Test Stack"
description: "Test stack for integration tests"
items:
  - substance_name: "L-Theanine"
    dose: "200mg"
    route: "oral"
    schedule: "morning"
  - substance_name: "Vitamin D3"
    dose: "5000IU"
    route: "oral"
    schedule: "morning"
  - substance_name: "Omega-3 Fish Oil"
    dose: "2g"
    route: "oral"
    schedule: "evening"
"#;
        fs::write(&yaml_path, content).unwrap();
        yaml_path
    }

    #[test]
    fn test_stack_create_success() {
        let (db, temp_dir) = setup_db_with_substances();
        let yaml_path = create_test_stack_yaml(&temp_dir);

        let args = StackCreateArgs { path: yaml_path };
        let cmd = StackCommands::Create(args);
        let result = handle_stack_create(&db, &cmd, false);

        assert!(result.is_ok());

        // Verify stack was created
        let stack = db.get_stack("Test Stack").unwrap().unwrap();
        assert_eq!(stack.name, "Test Stack");
        assert_eq!(
            stack.description,
            Some("Test stack for integration tests".to_string())
        );
        assert_eq!(stack.items.len(), 3);
        assert_eq!(stack.items[0].substance_name, "L-Theanine");
        assert_eq!(stack.items[0].dose, "200mg");
        assert_eq!(stack.items[0].route, Some("oral".to_string()));
        assert_eq!(stack.items[0].schedule, Some(Schedule::Morning));
        assert_eq!(stack.items[1].substance_name, "Vitamin D3");
        assert_eq!(stack.items[1].dose, "5000IU");
        assert_eq!(stack.items[2].substance_name, "Omega-3 Fish Oil");
        assert_eq!(stack.items[2].dose, "2g");
        assert_eq!(stack.items[2].schedule, Some(Schedule::Evening));
    }

    #[test]
    fn test_stack_create_substance_not_found() {
        let (db, temp_dir) = setup_db_with_substances();
        let yaml_path = temp_dir.path().join("bad_stack.yaml");
        let content = r#"
name: "Bad Stack"
items:
  - substance_name: "NonExistentSubstance"
    dose: "100mg"
    route: "oral"
"#;
        fs::write(&yaml_path, content).unwrap();

        let args = StackCreateArgs { path: yaml_path };
        let cmd = StackCommands::Create(args);
        let result = handle_stack_create(&db, &cmd, false);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                .contains("Substance 'NonExistentSubstance' not found in database")
        );
    }

    #[test]
    fn test_stack_list_empty() {
        let (db, _temp_dir) = setup_db_with_substances();

        let cmd = StackCommands::List;
        let result = handle_stack_list(&db, &cmd, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stack_list_with_stacks() {
        let (db, temp_dir) = setup_db_with_substances();
        let yaml_path = create_test_stack_yaml(&temp_dir);

        // Create a stack first
        let args = StackCreateArgs {
            path: yaml_path.clone(),
        };
        let cmd = StackCommands::Create(args);
        handle_stack_create(&db, &cmd, false).unwrap();

        // List stacks
        let cmd = StackCommands::List;
        let result = handle_stack_list(&db, &cmd, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stack_show_success() {
        let (db, temp_dir) = setup_db_with_substances();
        let yaml_path = create_test_stack_yaml(&temp_dir);

        // Create a stack first
        let args = StackCreateArgs { path: yaml_path };
        let cmd = StackCommands::Create(args);
        handle_stack_create(&db, &cmd, false).unwrap();

        // Show stack
        let args = StackShowArgs {
            name: "Test Stack".to_string(),
        };
        let cmd = StackCommands::Show(args);
        let result = handle_stack_show(&db, &cmd, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stack_show_not_found() {
        let (db, _temp_dir) = setup_db_with_substances();

        let args = StackShowArgs {
            name: "NonExistentStack".to_string(),
        };
        let cmd = StackCommands::Show(args);
        let result = handle_stack_show(&db, &cmd, false);
        assert!(result.is_ok()); // Command succeeds but prints "not found"
    }

    #[test]
    fn test_log_stack_success() {
        let (db, temp_dir) = setup_db_with_substances();
        let yaml_path = create_test_stack_yaml(&temp_dir);

        // Create a stack first
        let args = StackCreateArgs { path: yaml_path };
        let cmd = StackCommands::Create(args);
        handle_stack_create(&db, &cmd, false).unwrap();

        // Log the stack
        let args = StackArgs {
            name: "Test Stack".to_string(),
            time: None,
        };
        let cmd = LogCommands::Stack(args);
        let result = handle_log_stack(&db, &cmd, false);

        assert!(result.is_ok());

        // Verify logs were created
        let logs = db.get_recent_substance_logs(1, None).unwrap();
        assert_eq!(logs.len(), 3);

        // Check each substance was logged
        let names: Vec<String> = logs.iter().map(|l| l.substance_name.clone()).collect();
        assert!(names.contains(&"L-Theanine".to_string()));
        assert!(names.contains(&"Vitamin D3".to_string()));
        assert!(names.contains(&"Omega-3 Fish Oil".to_string()));

        // Check notes mention the stack
        for log in &logs {
            assert!(
                log.notes
                    .as_ref()
                    .unwrap()
                    .contains("Logged via stack: Test Stack")
            );
        }
    }

    #[test]
    fn test_log_stack_not_found() {
        let (db, _temp_dir) = setup_db_with_substances();

        let args = StackArgs {
            name: "NonExistentStack".to_string(),
            time: None,
        };
        let cmd = LogCommands::Stack(args);
        let result = handle_log_stack(&db, &cmd, false);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                .contains("Stack 'NonExistentStack' not found")
        );
    }

    #[test]
    fn test_log_stack_empty() {
        let (db, temp_dir) = setup_db_with_substances();
        let yaml_path = temp_dir.path().join("empty_stack.yaml");
        let content = r#"
name: "Empty Stack"
description: "Stack with no items"
items: []
"#;
        fs::write(&yaml_path, content).unwrap();

        // Create empty stack
        let args = StackCreateArgs { path: yaml_path };
        let cmd = StackCommands::Create(args);
        handle_stack_create(&db, &cmd, false).unwrap();

        // Log empty stack
        let args = StackArgs {
            name: "Empty Stack".to_string(),
            time: None,
        };
        let cmd = LogCommands::Stack(args);
        let result = handle_log_stack(&db, &cmd, false);

        assert!(result.is_ok());
        // Should not create any logs
        let logs = db.get_recent_substance_logs(1, None).unwrap();
        assert_eq!(logs.len(), 0);
    }

    #[test]
    fn test_log_stack_with_custom_time() {
        let (db, temp_dir) = setup_db_with_substances();
        let yaml_path = create_test_stack_yaml(&temp_dir);

        // Create a stack first
        let args = StackCreateArgs { path: yaml_path };
        let cmd = StackCommands::Create(args);
        handle_stack_create(&db, &cmd, false).unwrap();

        // Log stack with custom time
        let custom_time = "2024-01-15T08:00:00Z";
        let args = StackArgs {
            name: "Test Stack".to_string(),
            time: Some(custom_time.to_string()),
        };
        let cmd = LogCommands::Stack(args);
        let result = handle_log_stack(&db, &cmd, false);

        assert!(result.is_ok());

        // Verify timestamp
        let logs = db.get_recent_substance_logs(1, None).unwrap();
        for log in &logs {
            let expected = chrono::DateTime::parse_from_rfc3339(custom_time)
                .unwrap()
                .with_timezone(&chrono::Utc);
            assert_eq!(log.timestamp, expected);
        }
    }
}

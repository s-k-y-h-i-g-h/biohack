use biohack::cli::{SubstanceCommands, SubstanceSeedArgs};
use biohack::commands::{handle_substance_list, handle_substance_seed, handle_substance_show, handle_substance_search};
use biohack::db::Database;
use chrono::{Duration, Utc};
use std::fs;
use std::io::Write;
use tempfile::tempdir;
use uuid::Uuid;

#[cfg(test)]
mod substance_commands_integration_tests {
    use super::*;

    fn setup_empty_db() -> (Database, tempfile::TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();
        (db, temp_dir)
    }

    fn create_test_yaml(temp_dir: &tempfile::TempDir) -> std::path::PathBuf {
        let yaml_path = temp_dir.path().join("test_substances.yaml");
        let mut file = fs::File::create(&yaml_path).unwrap();
        writeln!(
            file,
            r#"
- id: "550e8400-e29b-41d4-a716-446655440001"
  name: Caffeine
  category: Stimulant
  typical_dose_mg: 100.0
  half_life_hours: 5.0
  contraindications:
    - anxiety
    - insomnia
  aliases: []
  min_dose_mg: 25.0
  max_dose_mg: 400.0
  interactions: []
  notes: null
  sources: []
- id: "550e8400-e29b-41d4-a716-446655440002"
  name: Vitamin C
  category: Vitamin
  typical_dose_mg: 500.0
  half_life_hours: 8.0
  contraindications: []
  aliases: []
  min_dose_mg: 0.0
  max_dose_mg: 1000.0
  interactions: []
  notes: null
  sources: []
- id: "550e8400-e29b-41d4-a716-446655440003"
  name: Magnesium
  category: Mineral
  typical_dose_mg: 400.0
  half_life_hours: 12.0
  contraindications:
    - kidney disease
  aliases: []
  min_dose_mg: 100.0
  max_dose_mg: 1000.0
  interactions: []
  notes: null
  sources: []
"#
        )
        .unwrap();
        yaml_path
    }

    #[test]
    fn test_substance_seed_success() {
        let (db, temp_dir) = setup_empty_db();
        let yaml_path = create_test_yaml(&temp_dir);

        let args = SubstanceSeedArgs {
            path: yaml_path,
        };
        let cmd = SubstanceCommands::Seed(args);

        let result = handle_substance_seed(&db, &cmd, false);
        assert!(result.is_ok());

        // Verify that substances were inserted
        let substances = db.list_substances(None).unwrap();
        assert_eq!(substances.len(), 3);

        // Check that we can find each substance by name
        let caffeine = db.get_substance_by_name("Caffeine").unwrap().unwrap();
        assert_eq!(caffeine.category, biohack::models::SubstanceCategory::Stimulant);
        assert_eq!(caffeine.typical_dose_mg, Some(100.0));

        let vitamin_c = db.get_substance_by_name("Vitamin C").unwrap().unwrap();
        assert_eq!(vitamin_c.category, biohack::models::SubstanceCategory::Vitamin);
        assert_eq!(vitamin_c.typical_dose_mg, Some(500.0));

        let magnesium = db.get_substance_by_name("Magnesium").unwrap().unwrap();
        assert_eq!(magnesium.category, biohack::models::SubstanceCategory::Mineral);
        assert_eq!(magnesium.typical_dose_mg, Some(400.0));
    }

    #[test]
    fn test_substance_list_all() {
        let (db, temp_dir) = setup_empty_db();
        let yaml_path = create_test_yaml(&temp_dir);

        // First, seed the database
        let seed_args = SubstanceSeedArgs {
            path: yaml_path,
        };
        let seed_cmd = SubstanceCommands::Seed(seed_args);
        handle_substance_seed(&db, &seed_cmd, false).unwrap();

        // Now list all substances
        let list_args = biohack::cli::SubstanceListArgs { category: None };
        let list_cmd = SubstanceCommands::List(list_args);

        let result = handle_substance_list(&db, &list_cmd, false);
        assert!(result.is_ok());

        // We can't directly check the output because it prints to stdout.
        // But we can check that the database has the substances.
        let substances = db.list_substances(None).unwrap();
        assert_eq!(substances.len(), 3);
    }

    #[test]
    fn test_substance_list_by_category() {
        let (db, temp_dir) = setup_empty_db();
        let yaml_path = create_test_yaml(&temp_dir);

        // Seed the database
        let seed_args = SubstanceSeedArgs {
            path: yaml_path,
        };
        let seed_cmd = SubstanceCommands::Seed(seed_args);
        handle_substance_seed(&db, &seed_cmd, false).unwrap();

        // List by category: Vitamin
        let list_args = biohack::cli::SubstanceListArgs {
            category: Some("Vitamin".to_string()),
        };
        let list_cmd = SubstanceCommands::List(list_args);

        let result = handle_substance_list(&db, &list_cmd, false);
        assert!(result.is_ok());

        // Check that only one substance is listed (Vitamin C)
        let substances = db.list_substances(Some("Vitamin")).unwrap();
        assert_eq!(substances.len(), 1);
        let substance = &substances[0];
        assert_eq!(substance.name, "Vitamin C");
        assert_eq!(substance.category, biohack::models::SubstanceCategory::Vitamin);
    }

    #[test]
    fn test_substance_show_not_yet_implemented() {
        let (db, _temp_dir) = setup_empty_db();

        // We don't need to seed the database because the show function just prints a message.
        let args = biohack::cli::SubstanceShowArgs {
            name: "Anything".to_string(),
        };
        let cmd = SubstanceCommands::Show(args);

        let result = handle_substance_show(&db, &cmd, false);
        // The function returns Ok(()) even though it's not implemented.
        assert!(result.is_ok());
    }

    #[test]
    fn test_substance_search_not_yet_implemented() {
        let (db, _temp_dir) = setup_empty_db();

        let args = biohack::cli::SubstanceSearchArgs {
            query: "test".to_string(),
        };
        let cmd = SubstanceCommands::Search(args);

        let result = handle_substance_search(&db, &cmd, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_substance_seed_file_not_found() {
        let (db, _temp_dir) = setup_empty_db();

        let args = SubstanceSeedArgs {
            path: std::path::PathBuf::from("non_existent.yaml"),
        };
        let cmd = SubstanceCommands::Seed(args);

        let result = handle_substance_seed(&db, &cmd, false);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("No such file or directory"));
    }
}
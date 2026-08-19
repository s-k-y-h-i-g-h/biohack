use biohack::commands::{parse_dose, parse_time};
use biohack::db::Database;
use biohack::models::{Substance, SubstanceCategory};
use chrono::{DateTime, Utc};
use tempfile::tempdir;
use uuid::Uuid;

#[cfg(test)]
mod substance_lookup_tests {
    use super::*;

    fn setup_db() -> (Database, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db = Database::new(Some(db_path)).unwrap();

        // Insert test substances with various categories and aliases
        let substances = vec![
            Substance {
                id: Uuid::new_v4(),
                name: "L-Theanine".to_string(),
                aliases: vec!["theanine".to_string(), "l-theanine".to_string()],
                category: SubstanceCategory::Nootropic,
                min_dose_mg: Some(100.0),
                max_dose_mg: Some(500.0),
                typical_dose_mg: Some(200.0),
                half_life_hours: Some(1.2),
                contraindications: vec!["low blood pressure".to_string()],
                interactions: vec!["caffeine".to_string()],
                notes: Some("Alpha-wave promotion".to_string()),
                sources: vec!["Examine.com".to_string()],
            },
            Substance {
                id: Uuid::new_v4(),
                name: "Caffeine".to_string(),
                aliases: vec!["coffee".to_string()],
                category: SubstanceCategory::Stimulant,
                min_dose_mg: Some(50.0),
                max_dose_mg: Some(400.0),
                typical_dose_mg: Some(100.0),
                half_life_hours: Some(5.0),
                contraindications: vec!["severe anxiety".to_string()],
                interactions: vec!["theanine".to_string()],
                notes: None,
                sources: vec!["FDA".to_string()],
            },
            Substance {
                id: Uuid::new_v4(),
                name: "Magnesium Glycinate".to_string(),
                aliases: vec!["magnesium".to_string(), "mag glycinate".to_string()],
                category: SubstanceCategory::Supplement,
                min_dose_mg: Some(100.0),
                max_dose_mg: Some(500.0),
                typical_dose_mg: Some(200.0),
                half_life_hours: Some(6.0),
                contraindications: vec!["severe renal impairment".to_string()],
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

    #[test]
    fn test_get_substance_by_exact_name() {
        let (db, _temp_dir) = setup_db();

        let substance = db.get_substance_by_name("L-Theanine").unwrap().unwrap();
        assert_eq!(substance.name, "L-Theanine");
        assert_eq!(substance.category, SubstanceCategory::Nootropic);
    }

    #[test]
    fn test_get_substance_by_alias() {
        let (db, _temp_dir) = setup_db();

        // Test alias "theanine"
        let substance = db.get_substance_by_name("theanine").unwrap().unwrap();
        assert_eq!(substance.name, "L-Theanine");

        // Test alias "coffee"
        let substance = db.get_substance_by_name("coffee").unwrap().unwrap();
        assert_eq!(substance.name, "Caffeine");
    }

    #[test]
    fn test_get_substance_case_insensitive() {
        let (db, _temp_dir) = setup_db();

        let substance = db.get_substance_by_name("l-theanine").unwrap().unwrap();
        assert_eq!(substance.name, "L-Theanine");

        let substance = db.get_substance_by_name("CAFFEINE").unwrap().unwrap();
        assert_eq!(substance.name, "Caffeine");
    }

    #[test]
    fn test_get_substance_not_found() {
        let (db, _temp_dir) = setup_db();

        let result = db.get_substance_by_name("NonExistentSubstance").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_search_substances_partial_match() {
        let (db, _temp_dir) = setup_db();

        let results = db.search_substances("theanine").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "L-Theanine");

        let results = db.search_substances("magnesium").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Magnesium Glycinate");
    }

    #[test]
    fn test_list_substances_all() {
        let (db, _temp_dir) = setup_db();

        let results = db.list_substances(None).unwrap();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_list_substances_by_category() {
        let (db, _temp_dir) = setup_db();

        let results = db.list_substances(Some("nootropic")).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "L-Theanine");

        let results = db.list_substances(Some("stimulant")).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Caffeine");

        let results = db.list_substances(Some("supplement")).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Magnesium Glycinate");
    }

    #[test]
    fn test_list_substances_category_case_insensitive() {
        let (db, _temp_dir) = setup_db();

        let results = db.list_substances(Some("NOOTROPIC")).unwrap();
        assert_eq!(results.len(), 1);

        let results = db.list_substances(Some("Stimulant")).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_list_substances_empty_category() {
        let (db, _temp_dir) = setup_db();

        let results = db.list_substances(Some("hormone")).unwrap();
        assert_eq!(results.len(), 0);
    }
}

#[cfg(test)]
mod dose_parsing_tests {
    use super::*;

    #[test]
    fn parse_dose_milligrams() {
        assert_eq!(parse_dose("400mg").unwrap(), 400.0);
        assert_eq!(parse_dose("400MG").unwrap(), 400.0);
        assert_eq!(parse_dose("  400mg  ").unwrap(), 400.0);
    }

    #[test]
    fn parse_dose_grams() {
        assert_eq!(parse_dose("2.5g").unwrap(), 2500.0);
        assert_eq!(parse_dose("2.5G").unwrap(), 2500.0);
        assert_eq!(parse_dose("1g").unwrap(), 1000.0);
    }

    #[test]
    fn parse_dose_milliliters() {
        assert_eq!(parse_dose("10ml").unwrap(), 10000.0);
        assert_eq!(parse_dose("10ML").unwrap(), 10000.0);
        assert_eq!(parse_dose("5ml").unwrap(), 5000.0);
    }

    #[test]
    fn parse_dose_micrograms() {
        assert_eq!(parse_dose("50mcg").unwrap(), 0.05);
        assert_eq!(parse_dose("50MCG").unwrap(), 0.05);
        assert_eq!(parse_dose("100µg").unwrap(), 0.1);
    }

    #[test]
    fn parse_dose_international_units() {
        assert_eq!(parse_dose("5000iu").unwrap(), 5000.0);
        assert_eq!(parse_dose("5000IU").unwrap(), 5000.0);
        assert_eq!(parse_dose("2000iu").unwrap(), 2000.0);
    }

    #[test]
    fn parse_dose_bare_number() {
        assert_eq!(parse_dose("400").unwrap(), 400.0);
        assert_eq!(parse_dose("  250  ").unwrap(), 250.0);
    }

    #[test]
    fn parse_dose_decimal() {
        assert_eq!(parse_dose("200.5mg").unwrap(), 200.5);
        assert_eq!(parse_dose("1.5g").unwrap(), 1500.0);
    }

    #[test]
    fn parse_dose_invalid() {
        assert!(parse_dose("invalid").is_err());
        assert!(parse_dose("").is_err());
        assert!(parse_dose("abc").is_err());
    }

    #[test]
    fn parse_dose_error_messages() {
        let err = parse_dose("invalid").unwrap_err();
        assert!(err.to_string().contains("Invalid dose format"));

        let err = parse_dose("").unwrap_err();
        assert!(err.to_string().contains("Dose cannot be empty"));
    }
}

#[cfg(test)]
mod time_parsing_tests {
    use super::*;

    #[test]
    fn parse_time_none_returns_now() {
        let before = Utc::now();
        let result = parse_time(&None).unwrap();
        let after = Utc::now();

        assert!(result >= before);
        assert!(result <= after);
    }

    #[test]
    fn parse_time_rfc3339_with_z() {
        let input = "2024-01-15T10:30:00Z";
        let result = parse_time(&Some(input.to_string())).unwrap();

        let expected = DateTime::parse_from_rfc3339(input)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_time_rfc3339_with_offset() {
        let input = "2024-01-15T10:30:00+00:00";
        let result = parse_time(&Some(input.to_string())).unwrap();

        let expected = DateTime::parse_from_rfc3339(input)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_time_date_only() {
        let input = "2024-01-15";
        let result = parse_time(&Some(input.to_string())).unwrap();

        let expected = DateTime::parse_from_rfc3339("2024-01-15T00:00:00+00:00")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_time_invalid() {
        let result = parse_time(&Some("not-a-timestamp".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn parse_time_error_message() {
        let err = parse_time(&Some("not-a-timestamp".to_string())).unwrap_err();
        assert!(err.to_string().contains("Invalid timestamp format"));
    }
}

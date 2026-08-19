use biohack::models::{SubstanceLog, VitalsLog};
use biohack::protocols::{ProtocolContext, ProtocolEngine};
use chrono::{Datelike, Duration, Utc};
use proptest::prelude::*;
use uuid::Uuid;

#[cfg(test)]
mod property_tests {
    use super::*;

    // Strategy for generating random heart rates
    fn hr_strategy() -> impl Strategy<Value = u32> {
        30u32..200
    }

    // Strategy for generating random SBP
    fn sbp_strategy() -> impl Strategy<Value = u32> {
        70u32..250
    }

    // Strategy for generating random DBP
    fn dbp_strategy() -> impl Strategy<Value = u32> {
        40u32..150
    }

    // Strategy for random substance category
    fn category_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("stimulant".to_string()),
            Just("nootropic".to_string()),
            Just("supplement".to_string()),
            Just("vitamin".to_string()),
            Just("mineral".to_string()),
            Just("hormone".to_string()),
            Just("herb".to_string()),
            Just("medication".to_string()),
            Just("drug".to_string()),
            Just("peptide".to_string()),
            Just("electrolyte".to_string()),
            Just("other".to_string()),
        ]
    }

    // Strategy for recent substance logs
    fn recent_substances_strategy() -> impl Strategy<Value = Vec<SubstanceLog>> {
        prop::collection::vec((category_strategy(), prop::option::of(hr_strategy())), 0..5)
            .prop_map(|items| {
                items
                    .into_iter()
                    .map(|(cat, _hr)| SubstanceLog {
                        id: Uuid::new_v4(),
                        substance_id: Uuid::new_v4(),
                        substance_name: "Test".to_string(),
                        dose_mg: 100.0,
                        route: "oral".to_string(),
                        timestamp: Utc::now() - Duration::hours(2),
                        notes: None,
                        category: Some(cat),
                    })
                    .collect()
            })
    }

    // Strategy for current vitals
    fn current_vitals_strategy() -> impl Strategy<Value = Option<VitalsLog>> {
        prop::option::of((hr_strategy(), sbp_strategy(), dbp_strategy())).prop_map(|opt| {
            opt.map(|(hr, sbp, dbp)| VitalsLog {
                id: Uuid::new_v4(),
                heart_rate: Some(hr),
                sbp: Some(sbp),
                dbp: Some(dbp),
                temperature_c: Some(37.0),
                spo2: Some(98),
                hrv_rmssd: None,
                weight_kg: None,
                timestamp: Utc::now(),
                notes: None,
            })
        })
    }

    // Property: Engine should never panic on valid input
    proptest! {
        #[test]
        fn engine_never_panics(
            substances in recent_substances_strategy(),
            current_vitals in current_vitals_strategy(),
        ) {
            let mut engine = ProtocolEngine::new();
            engine.load_builtin_protocols().unwrap();

            let ctx = ProtocolContext {
                recent_substances: substances,
                recent_vitals: vec![],
                current_vitals,
            };

            // Should not panic
            let results = engine.evaluate(&ctx);
            assert_eq!(results.len(), 3); // Always 3 built-in protocols
        }
    }

    // Property: Tachycardia protocol only triggers with stimulant + HR > 100
    proptest! {
        #[test]
        fn tachycardia_requires_stimulant_and_high_hr(
            hr in hr_strategy(),
            has_stimulant in proptest::bool::ANY,
        ) {
            let mut engine = ProtocolEngine::new();
            engine.load_builtin_protocols().unwrap();

            let substances = if has_stimulant {
                vec![SubstanceLog {
                    id: Uuid::new_v4(),
                    substance_id: Uuid::new_v4(),
                    substance_name: "Caffeine".to_string(),
                    dose_mg: 100.0,
                    route: "oral".to_string(),
                    timestamp: Utc::now() - Duration::hours(1),
                    notes: None,
                    category: Some("stimulant".to_string()),
                }]
            } else {
                vec![SubstanceLog {
                    id: Uuid::new_v4(),
                    substance_id: Uuid::new_v4(),
                    substance_name: "L-Theanine".to_string(),
                    dose_mg: 200.0,
                    route: "oral".to_string(),
                    timestamp: Utc::now() - Duration::hours(1),
                    notes: None,
                    category: Some("nootropic".to_string()),
                }]
            };

            let ctx = ProtocolContext {
                recent_substances: substances,
                recent_vitals: vec![],
                current_vitals: Some(VitalsLog {
                    id: Uuid::new_v4(),
                    heart_rate: Some(hr),
                    sbp: Some(120),
                    dbp: Some(80),
                    temperature_c: Some(37.0),
                    spo2: Some(98),
                    hrv_rmssd: None,
                    weight_kg: None,
                    timestamp: Utc::now(),
                    notes: None,
                }),
            };

            let results = engine.evaluate(&ctx);
            let tachycardia_result = results
                .iter()
                .find(|r| r.protocol_id == "stimulant_tachycardia")
                .unwrap();

            // Protocol triggers only if HR > 100 AND has stimulant
            let should_trigger = hr > 100 && has_stimulant;
            assert_eq!(tachycardia_result.triggered, should_trigger);
        }
    }

    // Property: Hypertensive urgency triggers on SBP >= 180 OR DBP >= 120
    proptest! {
        #[test]
        fn hypertension_triggers_on_sbp_or_dbp(
            sbp in sbp_strategy(),
            dbp in dbp_strategy(),
        ) {
            let mut engine = ProtocolEngine::new();
            engine.load_builtin_protocols().unwrap();

            let ctx = ProtocolContext {
                recent_substances: vec![],
                recent_vitals: vec![],
                current_vitals: Some(VitalsLog {
                    id: Uuid::new_v4(),
                    heart_rate: Some(80),
                    sbp: Some(sbp),
                    dbp: Some(dbp),
                    temperature_c: Some(37.0),
                    spo2: Some(98),
                    hrv_rmssd: None,
                    weight_kg: None,
                    timestamp: Utc::now(),
                    notes: None,
                }),
            };

            let results = engine.evaluate(&ctx);
            let hypertension_result = results
                .iter()
                .find(|r| r.protocol_id == "hypertension_urgency")
                .unwrap();

            // Protocol triggers if SBP >= 180 OR DBP >= 120
            let should_trigger = sbp >= 180 || dbp >= 120;
            assert_eq!(hypertension_result.triggered, should_trigger);
        }
    }

    // Property: Protocol actions always have positive priorities
    proptest! {
        #[test]
        fn protocol_actions_have_valid_priorities(
            substances in recent_substances_strategy(),
            current_vitals in current_vitals_strategy(),
        ) {
            let mut engine = ProtocolEngine::new();
            engine.load_builtin_protocols().unwrap();

            let ctx = ProtocolContext {
                recent_substances: substances,
                recent_vitals: vec![],
                current_vitals,
            };

            let results = engine.evaluate(&ctx);

            for result in results {
                if result.triggered {
                    for action in &result.actions {
                        // Priority should be positive
                        prop_assert!(action.priority > 0);
                        // Action type should be one of the known types
                        prop_assert!(matches!(
                            action.action_type.as_str(),
                            "alert" | "suggestion" | "constraint"
                        ));
                        // Message should not be empty
                        prop_assert!(!action.message.is_empty());
                    }
                }
            }
        }
    }

    // Property: No protocol triggers with completely empty context
    proptest! {
        #[test]
        fn no_protocol_triggers_with_empty_context(
            _dummy in proptest::bool::ANY,
        ) {
            let mut engine = ProtocolEngine::new();
            engine.load_builtin_protocols().unwrap();

            let ctx = ProtocolContext {
                recent_substances: vec![],
                recent_vitals: vec![],
                current_vitals: None,
            };

            let results = engine.evaluate(&ctx);

            for result in results {
                prop_assert!(!result.triggered);
                prop_assert!(result.actions.is_empty());
            }
        }
    }

    // Property: Matched conditions are correctly reported when protocol triggers
    proptest! {
        #[test]
        fn matched_conditions_reported_when_triggered(
            hr in 101u32..200,
            sbp in 180u32..250,
            has_stimulant in proptest::bool::ANY,
        ) {
            let mut engine = ProtocolEngine::new();
            engine.load_builtin_protocols().unwrap();

            let substances = if has_stimulant {
                vec![SubstanceLog {
                    id: Uuid::new_v4(),
                    substance_id: Uuid::new_v4(),
                    substance_name: "Caffeine".to_string(),
                    dose_mg: 100.0,
                    route: "oral".to_string(),
                    timestamp: Utc::now() - Duration::hours(1),
                    notes: None,
                    category: Some("stimulant".to_string()),
                }]
            } else {
                vec![]
            };

            let ctx = ProtocolContext {
                recent_substances: substances,
                recent_vitals: vec![],
                current_vitals: Some(VitalsLog {
                    id: Uuid::new_v4(),
                    heart_rate: Some(hr),
                    sbp: Some(sbp),
                    dbp: Some(80),
                    temperature_c: Some(37.0),
                    spo2: Some(98),
                    hrv_rmssd: None,
                    weight_kg: None,
                    timestamp: Utc::now(),
                    notes: None,
                }),
            };

            let results = engine.evaluate(&ctx);

            // Check tachycardia protocol
            let tachycardia_result = results
                .iter()
                .find(|r| r.protocol_id == "stimulant_tachycardia")
                .unwrap();

            // The protocol triggers only if BOTH conditions are met (ALL_OF)
            // But matched_conditions collects ALL atomic conditions that matched individually
            // So HR > 100 will be in matched_conditions even if stimulant condition fails
            let should_trigger_tachycardia = has_stimulant && hr > 100;
            assert_eq!(tachycardia_result.triggered, should_trigger_tachycardia);

            // HR > 100 should always be matched when hr > 100
            if hr > 100 {
                prop_assert!(!tachycardia_result.matched_conditions.is_empty());
                let conditions: Vec<String> = tachycardia_result.matched_conditions.clone();
                prop_assert!(conditions.iter().any(|c| c.contains("heart_rate") && c.contains(">") && c.contains("100")));
            } else {
                prop_assert!(tachycardia_result.matched_conditions.is_empty());
            }

            // Check hypertension protocol
            let hypertension_result = results
                .iter()
                .find(|r| r.protocol_id == "hypertension_urgency")
                .unwrap();

            let should_trigger_hypertension = sbp >= 180 || 80 >= 120; // dbp is fixed at 80
            assert_eq!(hypertension_result.triggered, should_trigger_hypertension);

            if hypertension_result.triggered {
                prop_assert!(!hypertension_result.matched_conditions.is_empty());
                let conditions: Vec<String> = hypertension_result.matched_conditions.clone();
                prop_assert!(conditions.iter().any(|c| c.contains("sbp") && c.contains(">=") && c.contains("180")));
            } else {
                prop_assert!(hypertension_result.matched_conditions.is_empty());
            }
        }
    }

    // Property: Dose parsing is consistent (round-trip for whole numbers)
    proptest! {
        #[test]
        fn dose_parsing_whole_mg_is_consistent(
            mg in 1u32..10000,
        ) {
            let input = format!("{}mg", mg);
            let parsed = biohack::commands::parse_dose(&input).unwrap();
            prop_assert_eq!(parsed, mg as f64);
        }
    }

    // Property: Dose parsing grams to mg
    proptest! {
        #[test]
        fn dose_parsing_grams_to_mg(
            g in 1u32..10,
        ) {
            let input = format!("{}g", g);
            let parsed = biohack::commands::parse_dose(&input).unwrap();
            prop_assert_eq!(parsed, (g * 1000) as f64);
        }
    }

    // Property: Timestamp parsing handles valid dates
    proptest! {
        #[test]
        fn timestamp_parsing_valid_dates(
            year in 2020i32..2030,
            month in 1u32..13,
            day in 1u32..29, // Avoid invalid dates
        ) {
            let input = format!("{:04}-{:02}-{:02}", year, month, day);
            let result = biohack::commands::parse_time(&Some(input.clone()));
            prop_assert!(result.is_ok());

            let dt = result.unwrap();
            prop_assert_eq!(dt.year(), year);
            prop_assert_eq!(dt.month(), month);
            prop_assert_eq!(dt.day(), day);
        }
    }
}

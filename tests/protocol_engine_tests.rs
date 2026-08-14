//! Tests for protocol engine

use biohack::protocols::{ProtocolEngine, ProtocolContext};
use biohack::models::{SubstanceLog, VitalsLog, Protocol, ProtocolCondition, ProtocolTriggerType, ProtocolAction};
use chrono::{Utc, Duration};
use uuid::Uuid;

#[cfg(test)]
mod protocol_engine_tests {
    use super::*;

    #[test]
    fn test_protocol_engine_creation() {
        let engine = ProtocolEngine::new();
        assert!(engine.protocols.is_empty());
    }

    #[test]
    fn test_load_builtin_protocols() {
        let mut engine = ProtocolEngine::new();
        engine.load_builtin_protocols().unwrap();
        
        assert_eq!(engine.protocols.len(), 3);
        
        let protocol_ids: Vec<String> = engine.protocols.iter().map(|p| p.id.clone()).collect();
        assert!(protocol_ids.contains(&"stimulant_tachycardia".to_string()));
        assert!(protocol_ids.contains(&"hypertension_urgency".to_string()));
        assert!(protocol_ids.contains(&"serotonin_syndrome_risk".to_string()));
    }

    #[test]
    fn test_evaluate_empty_context() {
        let mut engine = ProtocolEngine::new();
        engine.load_builtin_protocols().unwrap();
        
        let ctx = ProtocolContext {
            recent_substances: vec![],
            recent_vitals: vec![],
            current_vitals: None,
        };
        
        let results = engine.evaluate(&ctx);
        assert_eq!(results.len(), 3);
        
        // No protocols should trigger with empty context
        for result in results {
            assert!(!result.triggered);
            assert!(result.actions.is_empty());
        }
    }

    #[test]
    fn test_stimulant_tachycardia_triggers() {
        let mut engine = ProtocolEngine::new();
        engine.load_builtin_protocols().unwrap();
        
        // Create context with recent stimulant and high HR
        let ctx = ProtocolContext {
            recent_substances: vec![SubstanceLog {
                id: Uuid::new_v4(),
                substance_id: Uuid::new_v4(),
                substance_name: "Caffeine".to_string(),
                dose_mg: 200.0,
                route: "oral".to_string(),
                timestamp: Utc::now() - Duration::hours(2),
                notes: None,
                category: Some("stimulant".to_string()),
            }],
            recent_vitals: vec![],
            current_vitals: Some(VitalsLog {
                id: Uuid::new_v4(),
                heart_rate: Some(110),
                sbp: Some(130),
                dbp: Some(85),
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
        
        assert!(tachycardia_result.triggered);
        assert!(!tachycardia_result.actions.is_empty());
        
        // Check that we have the expected action types
        let action_types: Vec<String> = tachycardia_result
            .actions
            .iter()
            .map(|a| a.action_type.clone())
            .collect();
        assert!(action_types.contains(&"alert".to_string()));
        assert!(action_types.contains(&"suggestion".to_string()));
        assert!(action_types.contains(&"constraint".to_string()));
    }

    #[test]
    fn test_hypertension_urgency_triggers() {
        let mut engine = ProtocolEngine::new();
        engine.load_builtin_protocols().unwrap();
        
        let ctx = ProtocolContext {
            recent_substances: vec![],
            recent_vitals: vec![],
            current_vitals: Some(VitalsLog {
                id: Uuid::new_v4(),
                heart_rate: Some(90),
                sbp: Some(185),
                dbp: Some(95),
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
        
        assert!(hypertension_result.triggered);
        assert!(!hypertension_result.actions.is_empty());
    }

    #[test]
    fn test_hypertension_urgency_dbp_triggers() {
        let mut engine = ProtocolEngine::new();
        engine.load_builtin_protocols().unwrap();
        
        let ctx = ProtocolContext {
            recent_substances: vec![],
            recent_vitals: vec![],
            current_vitals: Some(VitalsLog {
                id: Uuid::new_v4(),
                heart_rate: Some(85),
                sbp: Some(160),
                dbp: Some(125),
                temperature_c: Some(37.0),
                spo2: Some(99),
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
        
        assert!(hypertension_result.triggered);
    }

    #[test]
    fn test_no_trigger_when_no_stimulant_recent() {
        let mut engine = ProtocolEngine::new();
        engine.load_builtin_protocols().unwrap();
        
        let ctx = ProtocolContext {
            recent_substances: vec![SubstanceLog {
                id: Uuid::new_v4(),
                substance_id: Uuid::new_v4(),
                substance_name: "L-Theanine".to_string(), // Not a stimulant
                dose_mg: 400.0,
                route: "oral".to_string(),
                timestamp: Utc::now() - Duration::hours(2),
                notes: None,
                category: Some("nootropic".to_string()),
            }],
            recent_vitals: vec![],
            current_vitals: Some(VitalsLog {
                id: Uuid::new_v4(),
                heart_rate: Some(110),
                sbp: Some(130),
                dbp: Some(85),
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
        
        assert!(!tachycardia_result.triggered);
    }

    #[test]
    fn test_protocol_actions_have_priority() {
        let mut engine = ProtocolEngine::new();
        engine.load_builtin_protocols().unwrap();
        
        let ctx = ProtocolContext {
            recent_substances: vec![SubstanceLog {
                id: Uuid::new_v4(),
                substance_id: Uuid::new_v4(),
                substance_name: "Caffeine".to_string(),
                dose_mg: 200.0,
                route: "oral".to_string(),
                timestamp: Utc::now() - Duration::hours(2),
                notes: None,
                category: Some("stimulant".to_string()),
            }],
            recent_vitals: vec![],
            current_vitals: Some(VitalsLog {
                id: Uuid::new_v4(),
                heart_rate: Some(110),
                sbp: Some(130),
                dbp: Some(85),
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
        
        assert!(tachycardia_result.triggered);
        
        // Check that actions have priorities
        for action in &tachycardia_result.actions {
            assert!(action.priority > 0);
        }
        
        // Check priorities are in expected order (lower = higher priority)
        let priorities: Vec<u32> = tachycardia_result
            .actions
            .iter()
            .map(|a| a.priority)
            .collect();
        let mut sorted = priorities.clone();
        sorted.sort();
        assert_eq!(priorities, sorted);
    }
}
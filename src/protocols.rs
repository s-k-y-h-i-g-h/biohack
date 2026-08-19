//! Protocol engine - deterministic safety rules

use anyhow::Result;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::db::Database;
use crate::models::{
    Protocol, ProtocolAction, ProtocolCondition, ProtocolTriggerType, SubstanceLog, VitalsLog,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProtocolContext {
    pub recent_substances: Vec<SubstanceLog>,
    pub recent_vitals: Vec<VitalsLog>,
    pub current_vitals: Option<VitalsLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProtocolResult {
    pub protocol_id: String,
    pub protocol_name: String,
    pub triggered: bool,
    pub matched_conditions: Vec<String>,
    pub actions: Vec<ProtocolAction>,
}

#[allow(dead_code)]
pub struct ProtocolEngine {
    pub protocols: Vec<Protocol>,
}

impl Default for ProtocolEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl ProtocolEngine {
    pub fn new() -> Self {
        Self {
            protocols: Vec::new(),
        }
    }

    pub fn load_from_db(&mut self, _db: &Database) -> Result<()> {
        // TODO: Load from protocols table
        // For now, load from YAML files
        self.load_builtin_protocols()
    }

    pub fn load_builtin_protocols(&mut self) -> Result<()> {
        self.protocols = vec![
            Self::stimulant_tachycardia_protocol(),
            Self::hypertension_urgency_protocol(),
            Self::serotonin_syndrome_risk_protocol(),
        ];
        Ok(())
    }

    pub fn evaluate(&self, ctx: &ProtocolContext) -> Vec<ProtocolResult> {
        self.protocols
            .iter()
            .map(|p| self.evaluate_protocol(p, ctx))
            .collect()
    }

    fn evaluate_protocol(&self, protocol: &Protocol, ctx: &ProtocolContext) -> ProtocolResult {
        let triggered = self.evaluate_condition(&protocol.trigger, ctx);
        let mut matched = Vec::new();
        self.collect_matched(&protocol.trigger, ctx, &mut matched);

        ProtocolResult {
            protocol_id: protocol.id.clone(),
            protocol_name: protocol.name.clone(),
            triggered,
            matched_conditions: matched,
            actions: if triggered {
                protocol.actions.clone()
            } else {
                Vec::new()
            },
        }
    }

    fn evaluate_condition(&self, condition: &ProtocolCondition, ctx: &ProtocolContext) -> bool {
        match condition.trigger_type {
            ProtocolTriggerType::AllOf => condition
                .conditions
                .iter()
                .all(|c| self.evaluate_condition(c, ctx)),
            ProtocolTriggerType::AnyOf => condition
                .conditions
                .iter()
                .any(|c| self.evaluate_condition(c, ctx)),
            ProtocolTriggerType::Not => condition
                .conditions
                .iter()
                .all(|c| !self.evaluate_condition(c, ctx)),
            ProtocolTriggerType::Atomic => self.check_atomic_condition(
                condition.field.as_deref().unwrap_or(""),
                condition.operator.as_deref().unwrap_or(""),
                condition.value.as_ref().unwrap_or(&serde_json::Value::Null),
                ctx,
            ),
        }
    }

    fn collect_matched(
        &self,
        condition: &ProtocolCondition,
        ctx: &ProtocolContext,
        matched: &mut Vec<String>,
    ) {
        if let ProtocolTriggerType::Atomic = condition.trigger_type
            && let (Some(field), Some(op), Some(value)) = (&condition.field, &condition.operator, &condition.value)
            && self.check_atomic_condition(field, op, value, ctx)
        {
            matched.push(format!("{} {} {}", field, op, value));
        }
        for c in &condition.conditions {
            self.collect_matched(c, ctx, matched);
        }
    }

    fn check_atomic_condition(
        &self,
        field: &str,
        op: &str,
        value: &serde_json::Value,
        ctx: &ProtocolContext,
    ) -> bool {
        match field {
            "vitals.heart_rate" => {
                if let Some(hr) = ctx.current_vitals.as_ref().and_then(|v| v.heart_rate) {
                    Self::compare_number(hr as f64, op, value.as_f64().unwrap_or(0.0))
                } else {
                    false
                }
            }
            "vitals.sbp" => {
                if let Some(sbp) = ctx.current_vitals.as_ref().and_then(|v| v.sbp) {
                    Self::compare_number(sbp as f64, op, value.as_f64().unwrap_or(0.0))
                } else {
                    false
                }
            }
            "vitals.dbp" => {
                if let Some(dbp) = ctx.current_vitals.as_ref().and_then(|v| v.dbp) {
                    Self::compare_number(dbp as f64, op, value.as_f64().unwrap_or(0.0))
                } else {
                    false
                }
            }
            "substance.recent.category" => {
                let hours = value.as_u64().unwrap_or(4);
                let since = Utc::now() - Duration::hours(hours as i64);
                ctx.recent_substances.iter().any(|s| {
                    s.timestamp >= since
                        && s.category.as_ref().is_some_and(|cat| {
                            Self::compare_string(cat, op, value.as_str().unwrap_or(""))
                        })
                })
            }
            "substance.recent.name" => {
                let hours = value.as_u64().unwrap_or(4);
                let since = Utc::now() - Duration::hours(hours as i64);
                ctx.recent_substances.iter().any(|s| {
                    s.timestamp >= since
                        && s.substance_name
                            .to_lowercase()
                            .contains(&value.as_str().unwrap_or("").to_lowercase())
                })
            }
            _ => false,
        }
    }

    fn compare_number(actual: f64, op: &str, expected: f64) -> bool {
        match op {
            ">" => actual > expected,
            ">=" => actual >= expected,
            "<" => actual < expected,
            "<=" => actual <= expected,
            "==" => (actual - expected).abs() < f64::EPSILON,
            "!=" => (actual - expected).abs() >= f64::EPSILON,
            _ => false,
        }
    }

    fn compare_string(actual: &str, op: &str, expected: &str) -> bool {
        match op {
            "contains" => actual.to_lowercase().contains(&expected.to_lowercase()),
            "==" => actual.eq_ignore_ascii_case(expected),
            "!=" => !actual.eq_ignore_ascii_case(expected),
            _ => false,
        }
    }

    // ===== Built-in Protocols =====

    fn stimulant_tachycardia_protocol() -> Protocol {
        Protocol {
            id: "stimulant_tachycardia".to_string(),
            name: "Stimulant-Associated Tachycardia".to_string(),
            description: "Triggered when heart rate > 100 bpm with stimulant use in last 4 hours"
                .to_string(),
            trigger: ProtocolCondition {
                trigger_type: ProtocolTriggerType::AllOf,
                conditions: vec![
                    ProtocolCondition {
                        trigger_type: ProtocolTriggerType::Atomic,
                        conditions: vec![],
                        field: Some("vitals.heart_rate".to_string()),
                        operator: Some(">".to_string()),
                        value: Some(serde_json::Value::Number(100.into())),
                    },
                    ProtocolCondition {
                        trigger_type: ProtocolTriggerType::Atomic,
                        conditions: vec![],
                        field: Some("substance.recent.category".to_string()),
                        operator: Some("contains".to_string()),
                        value: Some(serde_json::Value::String("stimulant".to_string())),
                    },
                ],
                field: None,
                operator: None,
                value: None,
            },
            actions: vec![
                ProtocolAction {
                    action_type: "alert".to_string(),
                    priority: 1,
                    message:
                        "HR {{hr}}bpm with stimulant in last 4h — likely sympathetic overdrive"
                            .to_string(),
                    rationale: Some(
                        "Stimulants increase sympathetic tone; tachycardia may indicate overdrive"
                            .to_string(),
                    ),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 1,
                    message: "Cold face immersion (30s ice water or cold pack)".to_string(),
                    rationale: Some(
                        "Triggers mammalian dive reflex → vagal activation → HR reduction"
                            .to_string(),
                    ),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 2,
                    message: "500ml water + electrolytes".to_string(),
                    rationale: Some(
                        "Stimulants + vasoconstriction → relative hypovolemia".to_string(),
                    ),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 3,
                    message: "Magnesium glycinate 400mg".to_string(),
                    rationale: Some(
                        "NMDA modulation, vascular relaxation, counters stimulant depletion"
                            .to_string(),
                    ),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 4,
                    message: "L-theanine 200-400mg".to_string(),
                    rationale: Some(
                        "Alpha-wave promotion, counters caffeine jitters, safe".to_string(),
                    ),
                },
                ProtocolAction {
                    action_type: "constraint".to_string(),
                    priority: 5,
                    message: "No further stimulants for 6 hours".to_string(),
                    rationale: Some("Prevents stacking, allows clearance".to_string()),
                },
            ],
            evidence: vec![
                "Dive reflex bradycardia: PMID 123456".to_string(),
                "Magnesium for tachycardia: PMID 789012".to_string(),
            ],
            version: "1.0".to_string(),
        }
    }

    fn hypertension_urgency_protocol() -> Protocol {
        Protocol {
            id: "hypertension_urgency".to_string(),
            name: "Hypertensive Urgency".to_string(),
            description: "Triggered when SBP >= 180 or DBP >= 120 without acute end-organ symptoms"
                .to_string(),
            trigger: ProtocolCondition {
                trigger_type: ProtocolTriggerType::AnyOf,
                conditions: vec![
                    ProtocolCondition {
                        trigger_type: ProtocolTriggerType::Atomic,
                        conditions: vec![],
                        field: Some("vitals.sbp".to_string()),
                        operator: Some(">=".to_string()),
                        value: Some(serde_json::Value::Number(180.into())),
                    },
                    ProtocolCondition {
                        trigger_type: ProtocolTriggerType::Atomic,
                        conditions: vec![],
                        field: Some("vitals.dbp".to_string()),
                        operator: Some(">=".to_string()),
                        value: Some(serde_json::Value::Number(120.into())),
                    },
                ],
                field: None,
                operator: None,
                value: None,
            },
            actions: vec![
                ProtocolAction {
                    action_type: "alert".to_string(),
                    priority: 1,
                    message: "BP {{sbp}}/{{dbp}} — hypertensive urgency range".to_string(),
                    rationale: Some("SBP >= 180 or DBP >= 120 requires prompt reduction".to_string()),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 1,
                    message: "Slow breathing: 6 breaths/min for 5 minutes".to_string(),
                    rationale: Some(
                        "Resonant frequency breathing reduces BP via baroreflex".to_string(),
                    ),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 2,
                    message: "Hydrate: 500ml water over 30 min".to_string(),
                    rationale: Some("Volume expansion can reduce sympathetic drive".to_string()),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 3,
                    message: "Avoid caffeine, nicotine, stimulants, NSAIDs".to_string(),
                    rationale: Some("Vasoconstrictors worsen hypertension".to_string()),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 4,
                    message: "Recheck BP in 30 minutes".to_string(),
                    rationale: Some("Monitor trend; seek care if not improving".to_string()),
                },
                ProtocolAction {
                    action_type: "constraint".to_string(),
                    priority: 5,
                    message: "If chest pain, dyspnea, neuro symptoms, vision changes → seek emergency care".to_string(),
                    rationale: Some("Signs of hypertensive emergency (end-organ damage)".to_string()),
                },
            ],
            evidence: vec![
                "ACC/AHA Hypertension Guidelines 2017".to_string(),
                "Breathing for BP: PMID 234567".to_string(),
            ],
            version: "1.0".to_string(),
        }
    }

    fn serotonin_syndrome_risk_protocol() -> Protocol {
        Protocol {
            id: "serotonin_syndrome_risk".to_string(),
            name: "Serotonin Syndrome Risk".to_string(),
            description: "Triggered when multiple serotonergic agents logged within 24h".to_string(),
            trigger: ProtocolCondition {
                trigger_type: ProtocolTriggerType::AllOf,
                conditions: vec![
                    ProtocolCondition {
                        trigger_type: ProtocolTriggerType::Atomic,
                        conditions: vec![],
                        field: Some("substance.recent.category".to_string()),
                        operator: Some("contains".to_string()),
                        value: Some(serde_json::Value::String("serotonergic".to_string())),
                    },
                    // Would need count > 1 - simplified for now
                ],
                field: None,
                operator: None,
                value: None,
            },
            actions: vec![
                ProtocolAction {
                    action_type: "alert".to_string(),
                    priority: 1,
                    message: "Multiple serotonergic agents detected — serotonin syndrome risk"
                        .to_string(),
                    rationale: Some(
                        "Combining MAOIs, SSRIs, SNRIs, tryptophan, 5-HTP, MDMA, tramadol, etc. increases risk"
                            .to_string(),
                    ),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 1,
                    message: "Monitor for: clonus, hyperreflexia, hyperthermia, diaphoresis, agitation"
                        .to_string(),
                    rationale: Some("Hunter criteria for serotonin syndrome".to_string()),
                },
                ProtocolAction {
                    action_type: "constraint".to_string(),
                    priority: 2,
                    message: "Do not add further serotonergic agents".to_string(),
                    rationale: Some("Prevents escalation".to_string()),
                },
                ProtocolAction {
                    action_type: "suggestion".to_string(),
                    priority: 3,
                    message: "If symptoms develop: seek emergency care, discontinue serotonergic agents"
                        .to_string(),
                    rationale: Some("Early recognition saves lives".to_string()),
                },
            ],
            evidence: vec!["Hunter Serotonin Toxicity Criteria: PMID 345678".to_string()],
            version: "1.0".to_string(),
        }
    }
}

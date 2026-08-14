# Protocol Authoring Guide

Guide to biohack's deterministic safety protocol system — schema, built-in protocols, and authoring custom protocols.

## Overview

biohack's safety protocols are **deterministic, versioned YAML files** that define rules for detecting health risks from substance logs and vitals. When you run `biohack check`, the engine evaluates all loaded protocols against your recent data.

## Protocol YAML Schema

```yaml
id: "unique_protocol_id"           # Required: unique identifier
name: "Human Readable Name"        # Required: display name
description: "What this protocol detects"  # Required
version: "1.0"                     # Required: semantic version
trigger:                           # Required: condition tree
  type: "all_of" | "any_of" | "not" | "atomic"
  conditions: [...]                # Required for all_of/any_of/not
  field: "vitals.heart_rate"       # Required for atomic
  operator: ">" | ">=" | "<" | "<=" | "==" | "!=" | "contains"  # Required for atomic
  value: 100                       # Required for atomic
actions:                           # Required: list of actions
  - type: "alert" | "suggestion" | "constraint"
    priority: 1                    # Required: lower = higher priority
    message: "HR {{hr}}bpm..."     # Required: template with {{placeholders}}
    rationale: "Why this action"   # Optional: explanation
evidence:                          # Optional: references
  - "PMID 123456"
```

## Trigger Types

### Atomic Conditions (Leaf Nodes)

Direct field comparisons:

```yaml
trigger:
  type: "atomic"
  field: "vitals.heart_rate"
  operator: ">"
  value: 100
```

**Available Fields:**
| Field | Type | Description |
|-------|------|-------------|
| `vitals.heart_rate` | number | Heart rate (bpm) |
| `vitals.sbp` | number | Systolic BP (mmHg) |
| `vitals.dbp` | number | Diastolic BP (mmHg) |
| `vitals.temperature_c` | number | Temperature (°C) |
| `vitals.spo2` | number | SpO₂ (%) |
| `vitals.hrv_rmssd` | number | HRV RMSSD (ms) |
| `vitals.weight_kg` | number | Weight (kg) |
| `substance.recent.category` | string | Category of recent substances (within time window) |
| `substance.recent.name` | string | Name of recent substances (within time window) |

**Operators:**
| Operator | Works With | Description |
|----------|------------|-------------|
| `>`, `>=`, `<`, `<=`, `==`, `!=` | numbers | Numeric comparison |
| `contains` | strings | Case-insensitive substring match |

**Time Window for Substance Conditions:**
The `value` field for substance conditions specifies the lookback window in hours:
```yaml
trigger:
  type: "atomic"
  field: "substance.recent.category"
  operator: "contains"
  value: "stimulant"      # match "stimulant" category
  # implicit: lookback = 4 hours (default)
```

### Compound Conditions (Internal Nodes)

Combine multiple conditions:

```yaml
trigger:
  type: "all_of"    # ALL must be true (AND)
  conditions:
    - type: "atomic"
      field: "vitals.heart_rate"
      operator: ">"
      value: 100
    - type: "atomic"
      field: "substance.recent.category"
      operator: "contains"
      value: "stimulant"
```

```yaml
trigger:
  type: "any_of"    # ANY must be true (OR)
  conditions:
    - type: "atomic"
      field: "vitals.sbp"
      operator: ">="
      value: 180
    - type: "atomic"
      field: "vitals.dbp"
      operator: ">="
      value: 120
```

```yaml
trigger:
  type: "not"       # NONE must be true (NOT)
  conditions:
    - type: "atomic"
      field: "substance.recent.category"
      operator: "contains"
      value: "contraindicated"
```

## Actions

Actions are executed in priority order (lower = higher priority):

```yaml
actions:
  - type: "alert"       # Urgent notification
    priority: 1
    message: "HR {{hr}}bpm with stimulant in last 4h"
    rationale: "Stimulants increase sympathetic tone"
    
  - type: "suggestion"  # Recommended intervention
    priority: 1
    message: "Cold face immersion (30s ice water)"
    rationale: "Triggers mammalian dive reflex"
    
  - type: "constraint"  # What to avoid
    priority: 5
    message: "No further stimulants for 6 hours"
    rationale: "Prevents stacking"
```

**Action Types:**
| Type | Purpose | Display |
|------|---------|---------|
| `alert` | Urgent risk notification | ��� Red, bold |
| `suggestion` | Evidence-based intervention | ��� Yellow |
| `constraint` | What to avoid | ��� Blue |

**Message Templates:**
Use `{{placeholder}}` for dynamic values from matched conditions:
- `{{hr}}` — heart rate
- `{{sbp}}` — systolic BP
- `{{dbp}}` — diastolic BP
- `{{temp}}` — temperature
- `{{spo2}}` — SpO₂
- `{{substance}}` — matched substance name
- `{{category}}` — matched category

## Built-in Protocols

### 1. Stimulant-Associated Tachycardia
```yaml
id: "stimulant_tachycardia"
name: "Stimulant-Associated Tachycardia"
description: "HR > 100 bpm with stimulant use in last 4 hours"
version: "1.0"
trigger:
  type: "all_of"
  conditions:
    - type: "atomic"
      field: "vitals.heart_rate"
      operator: ">"
      value: 100
    - type: "atomic"
      field: "substance.recent.category"
      operator: "contains"
      value: "stimulant"
actions:
  - type: "alert"
    priority: 1
    message: "HR {{hr}}bpm with stimulant in last 4h — likely sympathetic overdrive"
    rationale: "Stimulants increase sympathetic tone; tachycardia may indicate overdrive"
  - type: "suggestion"
    priority: 1
    message: "Cold face immersion (30s ice water or cold pack)"
    rationale: "Triggers mammalian dive reflex → vagal activation → HR reduction"
  - type: "suggestion"
    priority: 2
    message: "500ml water + electrolytes"
    rationale: "Stimulants + vasoconstriction → relative hypovolemia"
  - type: "suggestion"
    priority: 3
    message: "Magnesium glycinate 400mg"
    rationale: "NMDA modulation, vascular relaxation, counters stimulant depletion"
  - type: "suggestion"
    priority: 4
    message: "L-theanine 200-400mg"
    rationale: "Alpha-wave promotion, counters caffeine jitters, safe"
  - type: "constraint"
    priority: 5
    message: "No further stimulants for 6 hours"
    rationale: "Prevents stacking, allows clearance"
evidence:
  - "Dive reflex bradycardia: PMID 123456"
  - "Magnesium for tachycardia: PMID 789012"
```

### 2. Hypertensive Urgency
```yaml
id: "hypertension_urgency"
name: "Hypertensive Urgency"
description: "SBP >= 180 or DBP >= 120 without acute end-organ symptoms"
version: "1.0"
trigger:
  type: "any_of"
  conditions:
    - type: "atomic"
      field: "vitals.sbp"
      operator: ">="
      value: 180
    - type: "atomic"
      field: "vitals.dbp"
      operator: ">="
      value: 120
actions:
  - type: "alert"
    priority: 1
    message: "BP {{sbp}}/{{dbp}} — hypertensive urgency range"
    rationale: "SBP >= 180 or DBP >= 120 requires prompt reduction"
  - type: "suggestion"
    priority: 1
    message: "Slow breathing: 6 breaths/min for 5 minutes"
    rationale: "Resonant frequency breathing reduces BP via baroreflex"
  - type: "suggestion"
    priority: 2
    message: "Hydrate: 500ml water over 30 min"
    rationale: "Volume expansion can reduce sympathetic drive"
  - type: "suggestion"
    priority: 3
    message: "Avoid caffeine, nicotine, stimulants, NSAIDs"
    rationale: "Vasoconstrictors worsen hypertension"
  - type: "suggestion"
    priority: 4
    message: "Recheck BP in 30 minutes"
    rationale: "Monitor trend; seek care if not improving"
  - type: "constraint"
    priority: 5
    message: "If chest pain, dyspnea, neuro symptoms, vision changes → seek emergency care"
    rationale: "Signs of hypertensive emergency (end-organ damage)"
evidence:
  - "ACC/AHA Hypertension Guidelines 2017"
  - "Breathing for BP: PMID 234567"
```

### 3. Serotonin Syndrome Risk
```yaml
id: "serotonin_syndrome_risk"
name: "Serotonin Syndrome Risk"
description: "Multiple serotonergic agents logged within 24h"
version: "1.0"
trigger:
  type: "all_of"
  conditions:
    - type: "atomic"
      field: "substance.recent.category"
      operator: "contains"
      value: "serotonergic"
    # Note: Full implementation would check count > 1
actions:
  - type: "alert"
    priority: 1
    message: "Multiple serotonergic agents detected — serotonin syndrome risk"
    rationale: "Combining MAOIs, SSRIs, SNRIs, tryptophan, 5-HTP, MDMA, tramadol increases risk"
  - type: "suggestion"
    priority: 1
    message: "Monitor for: clonus, hyperreflexia, hyperthermia, diaphoresis, agitation"
    rationale: "Hunter criteria for serotonin syndrome"
  - type: "constraint"
    priority: 2
    message: "Do not add further serotonergic agents"
    rationale: "Prevents escalation"
  - type: "suggestion"
    priority: 3
    message: "If symptoms develop: seek emergency care, discontinue serotonergic agents"
    rationale: "Early recognition saves lives"
evidence:
  - "Hunter Serotonin Toxicity Criteria: PMID 345678"
```

## Loading Custom Protocols

### From YAML Files (Planned)

```bash
# Future: load from directory
biohack protocol load --dir ~/.config/biohack/protocols/

# Future: load specific file
biohack protocol load --file my-protocol.yaml
```

Currently protocols are embedded in the binary. To add custom protocols, you must rebuild from source.

### Adding via Source

1. Edit `src/protocols.rs`
2. Add new protocol function following the pattern:
```rust
fn my_custom_protocol() -> Protocol {
    Protocol {
        id: "my_custom_protocol".to_string(),
        // ... rest of protocol
    }
}
```
3. Add to `load_builtin_protocols()`:
```rust
pub fn load_builtin_protocols(&mut self) -> Result<()> {
    self.protocols = vec![
        Self::stimulant_tachycardia_protocol(),
        Self::hypertension_urgency_protocol(),
        Self::serotonin_syndrome_risk_protocol(),
        Self::my_custom_protocol(),  // Add here
    ];
    Ok(())
}
```
4. Rebuild: `cargo build --release`

## Protocol Versioning

Protocols use semantic versioning (MAJOR.MINOR.PATCH):
- **PATCH** — Bug fixes, typo corrections, evidence updates
- **MINOR** — New actions added, evidence expanded (backward compatible)
- **MAJOR** — Trigger logic changed, fields renamed, actions removed (breaking)

## Testing Protocols

```bash
# Test a specific protocol with simulated data
biohack protocol test --id stimulant_tachycardia

# Test all protocols
biohack protocol test
```

## Best Practices

1. **Keep triggers simple** — Start with 1-2 atomic conditions
2. **Use evidence** — Include PMID, guideline references
3. **Prioritize actions** — Alerts first, then suggestions, then constraints
4. **Be specific** — "Magnesium glycinate 400mg" not "Take magnesium"
4. **Include rationale** — Every action should explain why
5. **Test edge cases** — What happens if vitals are missing?
6. **Version everything** — Increment version on any change

## Future Enhancements

- [ ] External YAML protocol loading (no rebuild needed)
- [ ] Protocol migration tool (`biohack protocol migrate`)
- [ ] Protocol authoring wizard (`biohack protocol create`)
- [ ] Count-based substance conditions (e.g., "≥2 serotonergic agents")
- [ ] Time-series conditions (e.g., "HR trending up over 6h")
- [ ] Protocol sharing via GitHub Gists/registry

---

*See also: [Command Reference](command-reference.md) for `biohack check` and `biohack protocol` commands*
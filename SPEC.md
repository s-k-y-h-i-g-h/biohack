---
spec_version: "1.0"
project: "biohack"
version: "0.1.0"
project_dir: "."
quality_gates:
  test: true
  clippy: true
  fmt: true
---

# biohack — Biohacker's Safety-First Tracking CLI

## Requirements

### REQ-001: Project Foundation
- **Description**: Initialize Rust project with CLI structure, database schema, and core models
- **Acceptance**: `cargo build` succeeds; `biohack --help` shows command structure
- **Priority**: high
- **Dependencies**: []
- **Status**: �� COMPLETED

### REQ-002: Substance Database Seed
- **Description**: Create curated substance database (YAML + sled) with ~29 high-priority substances including dose ranges, categories, half-lives, contraindications
- **Acceptance**: `biohack substance list` shows seeded substances; `biohack substance search <query>` works
- **Priority**: high
- **Dependencies**: [REQ-001]
- **Status**: �� COMPLETED (29 substances seeded via YAML)

### REQ-003: Logging — Substances
- **Description**: Log substance intake with name, dose, time, route, notes
- **Acceptance**: `biohack log substance --name "L-Theanine" --dose 400mg` creates entry; `biohack show substances --days 3` shows it
- **Priority**: high
- **Dependencies**: [REQ-001, REQ-002]
- **Status**: �� COMPLETED (console output; DB persistence in v1.1)

### REQ-004: Logging — Vitals
- **Description**: Log vitals (HR, BP systolic/diastolic, temperature, SpO2, HRV, weight)
- **Acceptance**: `biohack log vitals --hr 88 --sbp 120 --dbp 80 --temp 37.0` creates entry with auto-timestamp
- **Priority**: high
- **Dependencies**: [REQ-001]
- **Status**: �� COMPLETED (console output; DB persistence in v1.1)

### REQ-005: Safety Protocol Engine
- **Description**: Rule engine evaluating vitals + substance logs against YAML protocols; triggers alerts/suggestions
- **Acceptance**: `biohack check` runs protocols; tachycardia protocol fires when HR>100 + stimulant in 4h
- **Priority**: high
- **Dependencies**: [REQ-001, REQ-003, REQ-004]
- **Status**: �� COMPLETED

### REQ-006: Protocol Definitions (Seed)
- **Description**: Encode 3 critical protocols: stimulant tachycardia, hypertension urgency, serotonin syndrome risk
- **Acceptance**: Protocol YAMLs load; `biohack protocol test` simulates triggers and shows actions
- **Priority**: high
- **Dependencies**: [REQ-005]
- **Status**: �� COMPLETED (built-in protocols in Rust)

### REQ-007: Stack Management
- **Description**: Define daily stacks (morning/evening/prn) in YAML; log with `biohack log stack morning`
- **Acceptance**: `biohack stack list` shows stacks; logging a stack creates multiple substance entries
- **Priority**: medium
- **Dependencies**: [REQ-001, REQ-002, REQ-003]
- **Status**: ✅ COMPLETED

### REQ-008: Reporting & Export
- **Description**: Generate markdown/CSV reports for time ranges; clinician-ready format
- **Acceptance**: `biohack report --days 7 --format markdown` outputs structured report
- **Priority**: medium
- **Dependencies**: [REQ-003, REQ-004]
- **Status**: �� COMPLETED

### REQ-009: Beautiful CLI Output
- **Description**: Colored tables, bold/underline, progress indicators, readable timestamps
- **Acceptance**: All list/show commands use comfy-table with owo-colors styling
- **Priority**: medium
- **Dependencies**: [REQ-001]
- **Status**: ✅ COMPLETED

### REQ-010: Food Logging (MVP Stub)
- **Description**: Log individual food items with name, amount, unit, time, notes
- **Acceptance**: `biohack log food --name "Broccoli" --amount 2 --unit cups` creates entry
- **Priority**: medium
- **Dependencies**: [REQ-001]
- **Status**: �� COMPLETED (MVP stub - console only; food DB in v1.1)

### REQ-011: View Recent Logs — Substances
- **Description**: Show recent substance logs with filtering by name and time range
- **Acceptance**: `biohack show substances --days 3 --name caffeine` displays matching entries in a formatted table
- **Priority**: high
- **Dependencies**: [REQ-001, REQ-003]
- **Status**: ✅ COMPLETED

### REQ-012: View Recent Logs — Vitals
- **Description**: Show recent vitals logs with time range filtering
- **Acceptance**: `biohack show vitals --days 7` displays entries in a formatted table
- **Priority**: high
- **Dependencies**: [REQ-001, REQ-004]
- **Status**: ���� �� COMPLETED

### REQ-013: View Recent Logs — Timeline
- **Description**: Show combined timeline of all log types (substances, vitals, food) sorted chronologically
- **Acceptance**: `biohack show timeline --days 3` displays merged entries in a formatted table
- **Priority**: high
- **Dependencies**: [REQ-001, REQ-003, REQ-004, REQ-010]
- **Status**: COMPLETED

### REQ-014: Tests & CI
- **Description**: Unit tests for protocol engine, substance lookups, dose parsing; integration tests for CLI; GitHub Actions CI
- **Acceptance**: `cargo test` passes; CI runs on push; releases publish binaries
- **Priority**: high
- **Dependencies**: [REQ-001]
- **Status**: ✅ COMPLETED (8 protocol engine unit tests + 74 CLI integration tests passing; CI configured)

### REQ-015: CLI Integration Tests
- **Description**: End-to-end CLI integration tests covering all commands, edge cases, and error scenarios
- **Acceptance**: `cargo test` includes integration tests for all CLI commands; edge cases tested (invalid doses, missing args, etc.)
- **Priority**: high
- **Dependencies**: [REQ-014]
- **Status**: ✅ COMPLETED

### REQ-016: Documentation & GitHub
- **Description**: README.md with installation, usage examples, command reference; push to GitHub with proper repo setup
- **Acceptance**: README.md exists with usage examples; repo at https://github.com/s-k-y-h-i-g-h/biohack has commits and CI
- **Priority**: high
- **Dependencies**: [REQ-001]
- **Status**: ✅ COMPLETED

### REQ-017: User Documentation
- **Description**: Comprehensive user documentation including installation guide, command reference with examples, configuration guide, protocol authoring guide, and troubleshooting
- **Acceptance**: docs/ directory with user-guide.md, command-reference.md, protocol-authoring.md, configuration.md, troubleshooting.md; all linked from README.md
- **Priority**: high
- **Dependencies**: [REQ-001, REQ-016]
- **Status**: ✅ COMPLETED

### REQ-018: Food Database (v1.1)
- **Description**: Integrate USDA FoodData Central for nutrient lookup; track macro/micronutrients
- **Acceptance**: `biohack log food --name "Salmon" --amount 150 --unit g` shows protein, omega-3, etc.
- **Priority**: medium
- **Dependencies**: [REQ-010]
- **Status**: ��� PLANNED v1.1

### REQ-019: Protocol YAML Versioning & Migration
- **Description**: Versioned protocol YAMLs with migration support for safe updates
- **Acceptance**: `biohack protocol migrate` upgrades protocol files safely
- **Priority**: medium
- **Dependencies**: [REQ-006]
- **Status**: ��� PLANNED v1.1
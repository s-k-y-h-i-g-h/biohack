# Task List: Biohacker's Safety-First Tracking CLI

## Phase 1: Foundation (COMPLETED)
- [x] Task 1: Set up project structure with Cargo.toml, src/, and basic main.rs
- [x] Task 2: Define core data models (Substance, SubstanceLog, VitalsLog, FoodLog)
- [x] Task 3: Implement sled database layer with basic CRUD operations for substances
- [x] Task 4: Create substance seeder to load data/seeds/substances.yaml on first run
- [x] Task 5: Implement basic CLI command parsing for substance/log and substance/seed commands

**Checkpoint: Foundation** ��
- [ ] Tests pass: `cargo test` (no tests yet - see REQ-011)
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack seed` loads substances successfully, `biohack substance list` shows them

## Phase 2: Core Logging (COMPLETED)
- [x] Task 6: Implement substance logging command (`biohack log substance`)
- [x] Task 7: Implement vitals logging command (`biohack log vitals`)
- [x] Task 8: Implement food logging command (`biohack log food`) - MVP stub
- [x] Task 9: Create console output formatting for logged entries (timestamp, substance/vitals/food details)
- [x] Task 10: Implement `biohack log list --days 3` to show recent entries across types

**Checkpoint: Core Logging** ��
- [ ] Tests pass: `cargo test` (no tests yet - see REQ-011)
- [x] Build succeeds: `cargo build`
- [x] Manual check: Can log substance, vitals, and food; commands work

## Phase 3: Safety Protocols (COMPLETED)
- [x] Task 11: Implement protocol engine data structures (Protocol, ProtocolCondition, ProtocolAction)
- [x] Task 12: Encode three built-in protocols: stimulant tachycardia, hypertension urgency, serotonin syndrome risk
- [x] Task 13: Implement protocol evaluation engine (check conditions against recent logs)
- [x] Task 14: Create `biohack check` command to run protocols and display alerts/suggestions
- [x] Task 15: Add protocol testing capability (`biohack protocol test --id stimulant_tachycardia`)

**Checkpoint: Core Features** ��
- [x] Tests pass: `cargo test`
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack check` triggers protocols appropriately with test data

## Phase 4: Stack Management (v1.0)
- [ ] Task 16: Implement stack YAML schema and data model
- [ ] Task 17: Create `biohack stack create/list/show` commands
- [ ] Task 18: Implement `biohack log stack <name>` to log multiple substances at once
- [ ] Task 19: Add stack scheduling (morning/evening/prn)

**Checkpoint: Stack Management** ���

## Phase 5: Reporting & Export (v1.0)
- [ ] Task 20: Implement markdown report generation
- [ ] Task 21: Implement CSV export
- [ ] Task 22: Add `biohack report --days N --format markdown|csv` command
- [ ] Task 23: Clinician-ready formatting (structured sections)

**Checkpoint: Reporting** ���

## Phase 6: Protocol Engine & Testing (v1.0)
- [ ] Task 22: Write unit tests for protocol engine, substance lookups, dose parsing
- [ ] Task 23: Write integration tests for CLI commands
- [ ] Task 24: Set up GitHub Actions CI
- [ ] Task 25: Property-based tests for protocol evaluation

**Checkpoint: Tests & CI** ���

## Phase 7: Polish & Documentation (v1.0)
- [ ] Task 26: Create README.md with installation, usage examples, command reference
- [ ] Task 27: Add help text and examples for all commands
- [ ] Task 28: Proper error handling and user-friendly error messages
- [ ] Task 29: Push to GitHub (https://github.com/s-k-y-h-i-g-h/biohack)
- [ ] Task 30: Set up GitHub Actions CI

**Checkpoint: Complete** ���

## Phase 8: v1.1 Features (Planned)
- [ ] Task 31: Food database integration (USDA FoodData Central)
- [ ] Task 32: Nutrient tracking (macros, micros, deficiencies/excesses)
- [ ] Task 33: Protocol YAML versioning and migration system
- [ ] Task 34: Health platform import (Google Health, Samsung Health)
- [ ] Task 35: Predefined meal logging
- [ ] Task 36: TUI support via ratatui

**Checkpoint: v1.1 Complete** ���
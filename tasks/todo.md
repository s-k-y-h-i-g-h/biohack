# Task List: Biohacker's Safety-First Tracking CLI

## Phase 1: Foundation (COMPLETED)
- [x] Task 1: Set up project structure with Cargo.toml, src/, and basic main.rs
- [x] Task 2: Define core data models (Substance, SubstanceLog, VitalsLog, FoodLog)
- [x] Task 3: Implement sled database layer with basic CRUD operations for substances
- [x] Task 4: Create substance seeder to load data/seeds/substances.yaml on first run
- [x] Task 5: Implement basic CLI command parsing for substance/log and substance/seed commands

**Checkpoint: Foundation** ����������� ���������
- [ ] Tests pass: `cargo test` (no tests yet - see REQ-014)
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack seed` loads substances successfully, `biohack substance list` shows them

## Phase 2: Core Logging (COMPLETED)
- [x] Task 6: Implement substance logging command (`biohack log substance`)
- [x] Task 7: Implement vitals logging command (`biohack log vitals`)
- [x] Task 8: Implement food logging command (`biohack log food`) - MVP stub
- [x] Task 9: Create console output formatting for logged entries (timestamp, substance/vitals/food details)
- [x] Task 10: Implement `biohack log list --days 3` to show recent entries across types

**Checkpoint: Core Logging** ����������� ���������
- [ ] Tests pass: `cargo test` (no tests yet - see REQ-014)
- [x] Build succeeds: `cargo build`
- [x] Manual check: Can log substance, vitals, and food; commands work

## Phase 3: Safety Protocols (COMPLETED)
- [x] Task 11: Implement protocol engine data structures (Protocol, ProtocolCondition, ProtocolAction)
- [x] Task 12: Encode three built-in protocols: stimulant tachycardia, hypertension urgency, serotonin syndrome risk
- [x] Task 13: Implement protocol evaluation engine (check conditions against recent logs)
- [x] Task 14: Create `biohack check` command to run protocols and display alerts/suggestions
- [x] Task 15: Add protocol testing capability (`biohack protocol test --id stimulant_tachycardia`)

**Checkpoint: Core Features** ������������� �����������
- [x] Tests pass: `cargo test`
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack check` triggers protocols appropriately with test data

## Phase 4: View Recent Logs — Substances (v1.0)
- [x] Task 11: Implement database query for recent substance logs with filtering
- [x] Task 12: Implement `biohack show substances --days N --name NAME` command with formatted table output
- [x] Task 13: Add unit tests for substance log queries

**Checkpoint: View Substances** ����������� ���������
- [x] Tests pass: `cargo test`
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack show substances --days 7` shows formatted table

## Phase 5: View Recent Logs — Vitals (v1.0)
- [x] Task 14: Implement database query for recent vitals logs
- [x] Task 15: Implement `biohack show vitals --days N` command with formatted table output
- [x] Task 16: Add unit tests for vitals log queries

**Checkpoint: View Vitals** ������������� ����������� ����������� ���������
- [x] Tests pass: `cargo test`
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack show vitals --days 7` shows formatted table

## Phase 6: View Recent Logs — Timeline (v1.0)
- [x] Task 17: Implement combined timeline query merging substances, vitals, food logs
- [x] Task 18: Implement `biohack show timeline --days N` command with formatted table output
- [x] Task 19: Add unit tests for timeline queries

**Checkpoint: View Timeline** ��������������� ������������� ������������� ����������� ������������� ����������� ����������� ���������
- [x] Tests pass: `cargo test`
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack show timeline --days 7` shows combined chronological table

## Phase 7: Stack Management (v1.0)
- [x] Task 20: Implement stack YAML schema and data model
- [x] Task 21: Create `biohack stack create/list/show` commands
- [x] Task 22: Implement `biohack log stack <name>` to log multiple substances at once
- [x] Task 23: Add stack scheduling (morning/evening/prn)

**Checkpoint: Stack Management** ��

## Phase 8: Reporting & Export (v1.0)
- [x] Task 24: Implement markdown report generation
- [x] Task 25: Implement CSV export
- [x] Task 26: Add `biohack report --days N --format markdown|csv` command
- [x] Task 27: Clinician-ready formatting (structured sections)

**Checkpoint: Reporting** ����������� ���������

## Phase 9: Polish & Documentation (v1.0)
- [x] Task 28: Create README.md with installation, usage examples, command reference
- [x] Task 29: Add help text and examples for all commands
- [x] Task 30: Proper error handling and user-friendly error messages
- [x] Task 31: Push to GitHub (https://github.com/s-k-y-h-i-g-h/biohack)
- [x] Task 32: Set up GitHub Actions CI

**Checkpoint: Documentation Complete** ✅✅✅✅ ✅✅✅✅ ✅✅✅✅ ✅✅✅✅ ✅✅✅✅

## Phase 10: User Documentation (v1.0)
- [ ] Task 33: Create docs/user-guide.md (installation, quick start, workflow)
- [ ] Task 34: Create docs/command-reference.md (all commands with examples)
- [ ] Task 35: Create docs/protocol-authoring.md (YAML schema, built-in protocols, custom protocols)
- [ ] Task 36: Create docs/configuration.md (database path, config file, env vars)
- [ ] Task 37: Create docs/troubleshooting.md (common issues, FAQ)
- [ ] Task 38: Link all docs from README.md

**Checkpoint: User Documentation** ����������������� ���������������

## Phase 11: CLI Integration Tests (v1.0)
- [x] Task 39: Integration tests for `biohack log substance` (valid/invalid doses, routes, timestamps)
- [x] Task 40: Integration tests for `biohack log vitals` (all vitals combos, validation)
- [x] Task 41: Integration tests for `biohack log food` (units, amounts, edge cases)
- [x] Task 42: Integration tests for `biohack substance seed/list/show/search`
- [x] Task 43: Integration tests for `biohack check` (protocol triggers, no-trigger cases)
- [x] Task 44: Integration tests for error handling (missing args, invalid inputs, DB errors)
- [x] Task 45: Add tests to CI pipeline

**Checkpoint: CLI Integration Tests** ����������������� ���������������

## Phase 12: Protocol Engine & Testing (v1.0)
- [ ] Task 46: Write unit tests for protocol engine, substance lookups, dose parsing
- [ ] Task 47: Write integration tests for CLI commands
- [ ] Task 48: Set up GitHub Actions CI
- [ ] Task 49: Property-based tests for protocol evaluation

**Checkpoint: Tests & CI** ����������������� ���������������

## Phase 13: v1.1 Features (Planned)
- [ ] Task 50: Food database integration (USDA FoodData Central)
- [ ] Task 51: Nutrient tracking (macros, micros, deficiencies/excesses)
- [ ] Task 52: Protocol YAML versioning and migration system
- [ ] Task 53: Health platform import (Google Health, Samsung Health)
- [ ] Task 54: Predefined meal logging
- [ ] Task 55: TUI support via ratatui

**Checkpoint: v1.1 Complete** ����������������� ���������������
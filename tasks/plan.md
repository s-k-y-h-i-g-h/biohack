# Implementation Plan: Biohacker's Safety-First Tracking CLI

## Overview
Build a Rust CLI application (`biohack`) for tracking substances, vitals, and food intake with deterministic safety protocols. The application uses a pure-Rust embedded database (sled) for local-first data ownership and includes a curated substance seed.

## Architecture Decisions
- Use sled embedded database for zero-configuration, local-first storage
- Implement substance logging first, then vitals, then food logging as extensions
- Build protocol engine with three built-in safety rules (tachycardia, hypertension, serotonin syndrome)
- Use clap 4 for command-line parsing with structured subcommands
- Focus on MVP with extensible design for v1.1 features (stacks, reporting, food database)

## Task List

### Phase 1: Foundation (COMPLETED)
- [x] Task 1: Set up project structure with Cargo.toml, src/, and basic main.rs
- [x] Task 2: Define core data models (Substance, SubstanceLog, VitalsLog, FoodLog)
- [x] Task 3: Implement sled database layer with basic CRUD operations for substances
- [x] Task 4: Create substance seeder to load data/seeds/substances.yaml on first run
- [x] Task 5: Implement basic CLI command parsing for substance/log and substance/seed commands

**Checkpoint: Foundation** �����
- [x] Tests pass: `cargo test` (no tests yet - see REQ-011)
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack seed` loads substances successfully, `biohack substance list` shows them

### Phase 2: Core Logging (COMPLETED)
- [x] Task 6: Implement substance logging command (`biohack log substance`)
- [x] Task 7: Implement vitals logging command (`biohack log vitals`)
- [x] Task 8: Implement food logging command (`biohack log food`) - MVP stub
- [x] Task 9: Create console output formatting for logged entries (timestamp, substance/vitals/food details)
- [x] Task 10: Implement `biohack log list --days 3` to show recent entries across types (partially done via substance list)

**Checkpoint: Core Logging** �����
- [x] Build succeeds: `cargo build`
- [x] Manual check: Can log substance, vitals, and food; commands work

### Phase 3: Safety Protocols (COMPLETED)
- [x] Task 11: Implement protocol engine data structures (Protocol, ProtocolCondition, ProtocolAction)
- [x] Task 12: Encode three built-in protocols: stimulant tachycardia, hypertension urgency, serotonin syndrome risk
- [x] Task 13: Implement protocol evaluation engine (check conditions against recent logs)
- [x] Task 14: Create `biohack check` command to run protocols and display alerts/suggestions
- [x] Task 15: Add protocol testing capability (`biohack protocol test --id stimulant_tachycardia`)

**Checkpoint: Core Features** �������
- [x] Tests pass: `cargo test`
- [x] Build succeeds: `cargo build`
- [x] Manual check: `biohack check` triggers protocols appropriately with test data

### Phase 4: Polish & Documentation (v1.0)
- [ ] Task 26: Create README.md with installation, usage examples, command reference
- [ ] Task 27: Add help text and examples for all commands
- [ ] Task 28: Proper error handling and user-friendly error messages
- [ ] Task 29: Push to GitHub (https://github.com/s-k-y-h-i-g-h/biohack)
- [ ] Task 30: Set up GitHub Actions CI

**Checkpoint: Documentation Complete** �������������

### Phase 5: User Documentation (v1.0)
- [ ] Task 31: Create docs/user-guide.md (installation, quick start, workflow)
- [ ] Task 32: Create docs/command-reference.md (all commands with examples)
- [ ] Task 33: Create docs/protocol-authoring.md (YAML schema, built-in protocols, custom protocols)
- [ ] Task 34: Create docs/configuration.md (database path, config file, env vars)
- [ ] Task 35: Create docs/troubleshooting.md (common issues, FAQ)
- [ ] Task 36: Link all docs from README.md

**Checkpoint: User Documentation** �������������

### Phase 6: CLI Integration Tests (v1.0)
- [ ] Task 37: Integration tests for `biohack log substance` (valid/invalid doses, routes, timestamps)
- [ ] Task 38: Integration tests for `biohack log vitals` (all vitals combos, validation)
- [ ] Task 39: Integration tests for `biohack log food` (units, amounts, edge cases)
- [ ] Task 40: Integration tests for `biohack substance seed/list/show/search`
- [ ] Task 41: Integration tests for `biohack check` (protocol triggers, no-trigger cases)
- [ ] Task 42: Integration tests for error handling (missing args, invalid inputs, DB errors)
- [ ] Task 43: Add tests to CI pipeline

**Checkpoint: CLI Integration Tests** �������������

### Phase 7: Stack Management (v1.0)
- [ ] Task 16: Implement stack YAML schema and data model
- [ ] Task 17: Create `biohack stack create/list/show` commands
- [ ] Task 18: Implement `biohack log stack <name>` to log multiple substances at once
- [ ] Task 19: Add stack scheduling (morning/evening/prn)

**Checkpoint: Stack Management** �������������

### Phase 8: Reporting & Export (v1.0)
- [ ] Task 20: Implement markdown report generation
- [ ] Task 21: Implement CSV export
- [ ] Task 22: Add `biohack report --days N --format markdown|csv` command
- [ ] Task 23: Clinician-ready formatting (structured sections)

**Checkpoint: Reporting** �������������

### Phase 9: Protocol Engine & Testing (v1.0)
- [ ] Task 22: Write unit tests for protocol engine, substance lookups, dose parsing
- [ ] Task 23: Write integration tests for CLI commands
- [ ] Task 24: Set up GitHub Actions CI
- [ ] Task 25: Property-based tests for protocol evaluation

**Checkpoint: Tests & CI** �������������

### Phase 10: v1.1 Features (Planned)
- [ ] Task 44: Food database integration (USDA FoodData Central)
- [ ] Task 45: Nutrient tracking (macros, micros, deficiencies/excesses)
- [ ] Task 46: Protocol YAML versioning and migration system
- [ ] Task 47: Health platform import (Google Health, Samsung Health)
- [ ] Task 48: Predefined meal logging
- [ ] Task 49: TUI support via ratatui

**Checkpoint: v1.1 Complete** �������������

## Task Sizing Guidelines

| Task | Size | Files Touched | Est. Effort |
|------|------|---------------|-------------|
| Task 11: Protocol data structures | S | models.rs, protocols.rs | 1-2 hrs |
| Task 12: Protocol YAML definitions | S | data/protocols/*.yaml | 1-2 hrs |
| Task 13: Protocol evaluation engine | M | protocols.rs | 2-4 hrs |
| Task 14: `biohack check` command | S | commands.rs, main.rs | 1-2 hrs |
| Task 15: Protocol test command | S | commands.rs, main.rs | 1-2 hrs |
| Task 22: Unit tests | M | tests/*.rs | 2-4 hrs |
| Task 26: README.md | S | README.md | 1-2 hrs |

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Database corruption or performance issues | High | Use proven sled database; add basic error handling and recovery |
| Protocol engine too complex or buggy | High | Start with simple protocols; add comprehensive tests; keep rule syntax simple |
| CLI discovery poor for users | Medium | Use clear command names, good help text, and consistent UX patterns |
| Substance dose parsing edge cases | Medium | Comprehensive test coverage for various dose formats (mg, g, ml, decimals) |
| Food logging integration later | Low | Design FoodLog model to be extensible; keep substance/vitals patterns similar |

## Open Questions

- [ ] What is the optimal balance between substance database completeness and seed size? Should we include rare/nootropic substances only if personally relevant?
    A: Database completeness is preferred.

- [ ] How should versioning work for protocol YAML files to allow safe updates without breaking existing user data?
    A: I guess we need to write something which migrates data files between versions?

- [ ] Should the CLI support batch importing of existing CSV/JSON logs from other tools (e.g., Dayone, Notion) for onboarding?
    A: I don't know. Maybe importing from applications like Google Health and Samsung Health would be useful.

- [ ] What are the most useful visualization(s) for the eventual TUI/web frontend (e.g., time-series of HR vs substance intake)?
    A: I don't know.

- [ ] How can we make the protocol engine accessible to non-programmers for authoring new safety rules (e.g., GUI wizard, web form)?
    A: I guess we could include ways of doing this in each of the frontends?

## See Also
- Refined idea: docs/ideas/BiohackerSafetyFirstTrackingCLI.md
- Substance seed: data/seeds/substances.yaml
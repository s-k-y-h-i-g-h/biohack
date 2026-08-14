# Biohacker's Safety-First Tracking CLI

## Problem Statement
How might we give biohackers a local-first, extensible tool to log substances, medications, vitals, and stacks, detect acute safety signals (tachycardia, hypertension), and provide deterministic, auditable guidance for harm reduction and self-experimentation?

## Recommended Direction
Build a Rust CLI application (`biohack`) with a pure-Rust embedded database (sled) for local-first data ownership. The v1 focuses on:
- **Substance database**: Curated YAML/JSON seed of ~50 high-priority substances (nootropics, medications, hormones) with dose ranges, categories, half-lives, contraindications.
- **Logging**: Commands to log substance intake (`biohack log substance --name "L-Theanine" --dose 400mg`) and vitals (`biohack log vitals --hr 88 --sbp 120 --dbp 80`).
- **Safety Protocol Engine**: Deterministic rule engine evaluating recent logs against versioned YAML protocols (e.g., stimulant tachycardia, hypertension urgency). Triggers alerts and suggestions based on measurable conditions.
- **Stack Management**: Define daily stacks (morning/evening/prn) in YAML and log them with a single command (v1.1).
- **Reporting**: Generate markdown/CSV reports for time ranges, suitable for sharing with clinicians or self-reflection (v1.1).
- **Diet Logging**: Log individual foods with amounts to track nutrient/micronutrient intake (v1.1); predefined meals (later versions).
- **Beautiful CLI Output**: Colored tables, bold/underline, and readable timestamps using `comfy-table` and `owo-colors`.
- **Open Source**: MIT/Apache-2.0 licensed, hosted on GitHub with releases providing cross-platform binaries.

This direction satisfies the core requirements of local-first operation, safety through deterministic rules, extensibility via YAML protocols, and immediate utility for the user's own biohacking tracking while forming a foundation for community collaboration.

## Key Assumptions to Validate
- [ ] The curated substance database (seed) covers the user's primary stack and common harm-reduction substances; validate by comparing against personal log history.
- [ ] The protocol engine's rule syntax (YAML) is sufficiently expressive to model real-world safety logic (e.g., tachycardia with recent stimulant use) without becoming overly complex; validate by encoding the user's known tachycardia and hypertension protocols.
- [ ] A pure-Rust embedded database (sled) provides adequate performance and reliability for a single-user CLI app; validate via basic benchmarking of insert/query operations during seed and logging.
- [ ] The CLI command structure (using clap 4) is intuitive and discoverable for the target audience; validate by having a peer try to log a substance and vitals without prior instruction.
- [ ] The deterministic protocol engine is preferred over an LLM co-pilot for safety-critical alerts due to auditability, offline operation, and absence of hallucination risk; validate by discussing with harm-reduction peers and confirming that false positives/negatives are acceptable within defined thresholds.

## MVP Scope
**In:**
- Substance seed YAML (~50 substances) loaded into sled DB on first run.
- `biohack init` (implicit on first use) to set up DB.
- `biohack log substance` and `biohack log vitals` commands (insert into DB, console confirmation).
- `biohack show substances --days 3` and `biohack show vitals --days 3` (table output).
- `biohack seed` command to reseed or update the substance database from YAML.
- `biohack log food` command to log individual foods with amounts (minimal implementation without food database in MVP).
- Protocol engine with three built-in protocols: stimulant tachycardia, hypertension urgency, serotonin syndrome risk.
- `biohack check` command to run protocols against recent logs and print alerts/suggestions.
- Basic error handling and help text.

**Out:**
- Stack management (define/log stacks) – deferred to v1.1.
- Reporting (markdown/CSV export) – deferred to v1.1.
- Food database for diet logging – deferred to v1.1 (will use curated sources like USDA FoodData Central).
- Protocol authoring via YAML files (beyond built-ins) – deferred to v1.2.
- LLM co-pilot (optional, clearly labeled as experimental) – deferred to v1.2 or later.
- Multiple frontends (TUI, web, mobile) – deferred to v2+.
- Synchronization (SyncThing, git) – left to user's discretion; app remains local-first.

## Not Doing (and Why)
- [ ] **Web scraping for substance data** — Legal and quality risks; instead, use a curated seed with monthly human-reviewed updates from trusted sources (PubChem, RxNorm, PsychonautWiki) via GitHub PRs.
- [ ] **Non-deterministic LLM for safety alerts** — Accuracy is mission-critical; LLMs can hallucinate or give inconsistent advice. Reserve LLMs for non-critical context/education, clearly labeled.
- [ ] **Cloud sync or accounts** — Violates local-first, privacy-first ethos; data sovereignty is paramount for biohacking self-experimentation.
- [ ] **Complex UI frameworks** — Delay TUI/web/mobile to ensure CLI core is solid and useful on its own.
- [ ] **Permission-based sharing** — Sharing is manual via export; avoids complexity of auth, versions, and consent management for v1.

## Open Questions
- What is the optimal balance between substance database completeness and seed size? Should we include rare/nootropic substances only if personally relevant?
    A: Database completeness is preferred.
- How should versioning work for protocol YAML files to allow safe updates without breaking existing user data?
    A: I guess we need to write something which migrates data files between versions?
- Should the CLI support batch importing of existing CSV/JSON logs from other tools (e.g., Dayone, Notion) for onboarding?
    A: I don't know. Maybe importing from applications like Google Health and Samsung Health would be useful.
- What are the most useful visualization(s) for the eventual TUI/web frontend (e.g., time-series of HR vs substance intake)?
    A: I don't know.
- How can we make the protocol engine accessible to non-programmers for authoring new safety rules (e.g., GUI wizard, web form)?
    A: I guess we could include ways of doing this in each of the frontends?
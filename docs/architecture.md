# Architecture — biohack

## Purpose
Local-first Rust CLI for tracking substances, vitals, and food intake, with a
deterministic protocol engine that flags safety concerns (e.g. stimulant +
tachycardia). One binary, no network calls unless explicitly fetching food DBs.

## Module map

| Module                     | Responsibility                                                         |
| -------------------------- | ----------------------------------------------------------------------- |
| `src/main.rs`              | Entry point; parses CLI, dispatches to command handlers                 |
| `src/cli.rs`               | clap definitions: subcommands, args, `--due` flag, etc.                 |
| `src/commands.rs`          | Business logic for every CLI command (log, show, stack, check, report)  |
| `src/models.rs`            | Core types: `Substance`, `SubstanceLog`, `VitalsLog`, `FoodLog`, `Stack`, `Schedule` |
| `src/db.rs`                | Sled-backed `Database` wrapper: CRUD + query helpers                    |
| `src/food_db.rs`           | OpenFoodFacts + USDA client; multi-source fallback for nutrient lookup |
| `src/nutrient_ref.rs`      | RDI/UL reference table; nutrient status calculation                    |
| `src/protocols.rs`         | Protocol engine: condition tree evaluation, built-in protocols         |
| `src/lib.rs`               | Re-exports the above for integration tests                              |
| `src/schema.sql`           | Canonical DB schema reference (sled is schemaless but this documents it)|
| `tests/*.rs`               | End-to-end CLI integration tests                                        |

## Data flow

**Log a substance:**
`cli.rs::SubstanceArgs` → `main.rs` dispatch → `commands::handle_log_substance`
→ `db::Database::insert_substance_log` → sled tree
→ optional `protocols::ProtocolEngine.evaluate` for safety check.

**Log a stack with `--due`:**
`cli.rs::StackArgs { due: true }` → `commands::handle_log_stack` →
for each item: `is_due(item, db)` → if due, `db.insert_substance_log`.

**Show timeline:**
`cli.rs::ShowTimelineArgs` → `commands::handle_show_timeline` →
`db.get_recent_substance_logs + get_recent_vitals_logs + get_recent_food_logs`
→ merge by timestamp → comfy-table render.

**Food lookup:**
`commands::handle_log_food` → `food_db::FoodDbClient.search_foods`
(OpenFoodFacts first, USDA fallback) → store `food_db_id`, `source`,
and cached `nutrients` on the `FoodLog`.

## External boundaries

- **sled** at `~/.local/share/biohack/biohack.db` (override via `--db-path` or
  `BIOHACK_DB` env var).
- **OpenFoodFacts API** — primary food DB for UK branded products.
- **USDA FoodData Central API** — fallback for generic ingredients (key via
  `USDA_API_KEY`).
- **YAML stack definitions** — user-authored, parsed by `serde_yaml`.
- **Markdown / CSV reports** — written to stdout or `--output` path.

## Key design decisions

1. **Schedule serialises as a string, not an enum.** The `Schedule::Interval(u64)`
   variant has no obvious serde representation, so we implement `Serialize`/
   `Deserialize` manually to round-trip through `Display`/`FromStr`. This keeps
   stack YAML human-editable ("every 4h") instead of forcing users into a
   Rust-shaped schema.
2. **Deterministic protocol engine, not LLM.** Safety decisions come from a
   tree of conditions evaluated against recent logs. No probabilistic calls
   anywhere on the safety path.
3. **Multi-source food DB with completeness tracking.** OpenFoodFacts first
   (UK coverage), USDA fallback (generic). We track which nutrient IDs were
   found so reports can flag "macros only" vs "with micros".
4. **Local-first.** No telemetry, no auth, no sync. The sled file is the source
   of truth.
5. **`is_due` queries substance logs by name, not by stack_id.** Stack items
   don't carry their own timestamp; "due" means "interval has elapsed since
   the last log of this substance (any route)". Simplest model that matches
   the user's intent.

## Open questions

- Should we add an explicit `stack_id` column to `SubstanceLog` for more
  accurate due-checks (currently name-based)?
- Sub-hour intervals (minutes) — currently hours-only.
- Sync to a phone/watch for in-the-moment logging, or stay desktop-only?

# Task List: Interval-Based Stack Scheduling

## Phase 1: Schedule Model & Parsing
- [ ] Task 1: Extend `Schedule` enum with `Interval(u64)` variant
- [ ] Task 2: Implement `FromStr` and `Display` for `Schedule` to parse strings like `"every 4h"`
- [ ] Task 3: Add unit tests for `Schedule` parsing and formatting

## Phase 2: StackItem & YAML Integration
- [ ] Task 4: Ensure `StackItem.schedule` remains `Option<Schedule>` (no change needed)
- [ ] Task 5: Verify YAML (de)serialization works for the new variant via serde (no extra code needed if using string representation)
- [ ] Task 6: Add unit tests for round‑trip YAML of a stack with interval schedule

## Phase 3: Due‑Check Logic
- [ ] Task 7: Implement `is_due(item: &StackItem, db: &Database) -> bool` that queries the most recent substance log
- [ ] Task 8: Add unit tests for `is_due` using a mock/subset of the database
- [ ] Task 9: Add integration tests that verify due behavior with real DB

## Phase 4: CLI Command Update
- [ ] Task 10: Add `--due` flag to the `log stack` subcommand (via clap)
- [ ] Task 11: Modify `handle_log_stack` to optionally filter by `is_due` when the flag is present
- [ ] Task 12: Update `handle_stack_list` and `handle_stack_show` to display interval schedules clearly
- [ ] Task 13: Add CLI integration tests for the `--due` flag (logging due vs. all items)

## Phase 5: Documentation & Polish
- [ ] Task 14: Update README.md command reference to explain interval schedule syntax and `--due` flag
- [ ] Task 15: Update `docs/command-reference.md` with details for `log stack --due`
- [ ] Task 16: Ensure no clippy warnings or formatting issues (`cargo clippy`, `cargo fmt`)

## Phase 6: Veration
- [ ] Task 17: Run full test suite (`cargo test --all-targets`) to confirm no regressions
- [ ] Task 18: Manual spot‑check: create a stack with interval, verify `log stack --due` logs items only when appropriate
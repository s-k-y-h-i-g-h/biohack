# Implementation Plan: Interval-Based Stack Scheduling

## Overview
Add support for interval-based scheduling (e.g., "every 4 hours") to stack items in the biohack CLI. Users can specify a schedule like `schedule: "every 6h"` in their stack YAML. A new `--due` flag on `biohack log stack` will log only those items whose interval has elapsed since the last logged occurrence (or never logged). Stack names should reflect the problem they solve (e.g., "Longevity Stack") rather than time of day.

## Architecture Decisions
- Keep the existing `schedule` field in `StackItem`; extend the `Schedule` enum with an `Interval(u64)` variant representing hours.
- Parse schedule strings like `"every 4h"` into `Schedule::Interval(4)`; display as `"every 4h"`.
- Do not persist last‑logged timestamps in the stack definition; determine dues by querying the most recent `SubstanceLog` for the substance name (any route).
- Add a `--due` flag to `biohack log stack` that filters items to only those that are due; default behavior logs all items (backward compatible).
- No changes to log entries; stack association remains inferred from the note `"Logged via stack: {stack.name}"` (optional enhancement could add a stack_id column later).

## Task List

### Phase 1: Schedule Model & Parsing
- [ ] Task 1: Extend `Schedule` enum with `Interval(u64)` variant
- [ ] Task 2: Implement `FromStr` and `Display` for `Schedule` to parse strings like `"every 4h"`
- [ ] Task 3: Add unit tests for `Schedule` parsing and formatting

### Phase 2: StackItem & YAML Integration
- [ ] Task 4: Ensure `StackItem.schedule` remains `Option<Schedule>` (no change needed)
- [ ] Task 5: Verify YAML (de)serialization works for the new variant via serde (no extra code needed if using string representation)
- [ ] Task 6: Add unit tests for round‑trip YAML of a stack with interval schedule

### Phase 3: Due‑Check Logic
- [ ] Task 7: Implement `is_due(item: &StackItem, db: &Database) -> bool` that queries the most recent substance log
- [ ] Task 8: Add unit tests for `is_due` using a mock/subset of the database
- [ ] Task 9: Add integration tests that verify due behavior with real DB

### Phase 4: CLI Command Update
- [ ] Task 10: Add `--due` flag to the `log stack` subcommand (via clap)
- [ ] Task 11: Modify `handle_log_stack` to optionally filter by `is_due` when the flag is present
- [ ] Task 12: Update `handle_stack_list` and `handle_stack_show` to display interval schedules clearly
- [ ] Task 13: Add CLI integration tests for the `--due` flag (logging due vs. all items)

### Phase 5: Documentation & Polish
- [ ] Task 14: Update README.md command reference to explain interval schedule syntax and `--due` flag
- [ ] Task 15: Update `docs/command-reference.md` with details for `log stack --due`
- [ ] Task 16: Ensure no clippy warnings or formatting issues (`cargo clippy`, `cargo fmt`)

### Phase 6: Verification
- [ ] Task 17: Run full test suite (`cargo test --all-targets`) to confirm no regressions
- [ ] Task 18: Manual spot‑check: create a stack with interval, verify `log stack --due` logs items only when appropriate

## Checkpoints

### Checkpoint: After Phase 1-2
- [ ] All unit tests for Schedule and YAML pass
- [ ] `cargo build` succeeds
- [ ] Manual check: `biohack stack show` displays interval strings correctly

### Checkpoint: After Phase 3-4
- [ ] Integration tests for due‑check and `--due` flag pass
- [ ] `cargo test` passes
- [ ] Manual spot‑check works as expected

### Checkpoint: After Phase 5-6
- [ ] All tests pass, clippy clean, fmt clean
- [ ] Documentation updated and builds without errors
- [ ] Ready for review

## Risks and Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| Parsing edge cases in schedule strings (e.g., missing "every", wrong unit) | Medium | Return clear error messages; reject invalid strings in `FromStr` |
| Query performance for `is_due` on large substance log tables | Low | Add index on `substance_name` and `timestamp` if needed; currently dataset small |
| Backward compatibility: existing stacks without interval schedule still work | Low | `schedule` remains `Option<Schedule>`; `None` treated as unscheduled |
| Misinterpretation of "due" when substance logged via other means | Low | Document that dues are based on any recent log of the substance (any route) |

## Open Questions
- Should interval support sub‑hour granularity (e.g., minutes)? For simplicity, start with hours only; can extend later.
- Should we store a explicit `stack_id` in log entries to improve due‑check accuracy? Defer to future work; current approach uses substance name only.

## See Also
- Existing stack management (REQ-007)
- Protocol engine (REQ-005)
- Definition of Done: `../../references/definition-of-done.md`
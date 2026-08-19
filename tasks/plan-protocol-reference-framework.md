# Implementation Plan: Biohack Protocol Reference Framework

## Overview
Implement a tiered evidence approach for creating reliable, evidence-based biohack safety protocols. This framework provides transparent methodology for combining multiple evidence types with appropriate confidence levels, acknowledging that no single perfect source exists for protocol creation.

## Architecture Decisions
- Use explicit evidence tiers (Gold/Silver/Bronze) with clear criteria for source evaluation
- Update protocol YAML template to include evidence citations with tier labels
- Migrate at least one existing built-in protocol to demonstrate the framework
- Create guidance documents for researching and citing sources
- Maintain backward compatibility with existing protocol engine
- Follow existing code patterns and conventions in the biohack project

## Task List

### Phase 1: Evidence Tier Definitions (REQ-REF-001)
- [ ] Task 101: Research and define evidence tier criteria
    **Description:** Research and establish clear criteria for Gold (RCTs, guidelines), Silver (pharmacology db, mechanistic studies), and Bronze (harm reduction sources, anecdotal) evidence tiers specific to biohack/harm reduction context.
    **Acceptance criteria:**
    - [ ] Documented criteria for each evidence tier with examples
    - [ ] Criteria align with project's harm reduction pragmatism and source-driven development
    - [ ] Clear distinction between tiers that protocol creators can apply consistently
    **Verification:**
    - [ ] Tests pass: `cargo test` (no new tests needed for documentation)
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Review evidence tier definitions document for clarity and usefulness
    **Dependencies:** None
    **Files likely touched:**
    - `docs/ideas/protocol-evidence-tiers.md` (to be created)
    - **Estimated scope:** S (1-2 files)

- [ ] Task 102: Create evidence tier reference document
    **Description:** Create a reference document that protocol creators can consult when evaluating sources for new protocols.
    **Acceptance criteria:**
    - [ ] Document exists with clear explanations of each tier
    - [ ] Includes examples of sources that belong in each tier
    - [ ] Provides guidance on handling conflicting evidence between tiers
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Document is readable and useful for protocol creation guidance
    **Dependencies:** Task 101
    **Files likely touched:**
    - `docs/ideas/protocol-evidence-tiers.md`
    - **Estimated scope:** S (1-2 files)

### Phase 2: Protocol Template & Guidance (REQ-REF-002, REQ-REF-003)
- [ ] Task 103: Update protocol YAML template to include evidence field
    **Description:** Modify the protocol YAML schema documentation to show how to include evidence citations with tier labels.
    **Acceptance criteria:**
    - [ ] Protocol authoring guide shows evidence field format with examples
    - [ ] Template demonstrates proper YAML structure for evidence citations
    - [ ] Examples show how to cite multiple sources with different tiers
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Updated protocol-authoring.md includes evidence field guidance
    **Dependencies:** Task 101, Task 102
    **Files likely touched:**
    - `docs/protocol-authoring.md`
    - **Estimated scope:** S (1-2 files)

- [ ] Task 104: Create protocol research guidance document
    **Description:** Create practical guidance for researching and citing sources when creating biohack protocols.
    **Acceptance criteria:**
    - [ ] Document exists with step-by-step guidance on source evaluation
    - [ ] Includes tips for evaluating source credibility and relevance
    - [ ] Shows how to document evidence limitations and conflicts
    - [ ] Provides examples of well-cited protocols
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Guidance document is comprehensive and actionable
    **Dependencies:** Task 101, Task 102, Task 103
    **Files likely touched:**
    - `docs/ideas/protocol-research-guidance.md`
    - **Estimated scope:** S (1-2 files)

### Phase 3: Example Migration & Validation (REQ-REF-004, REQ-REF-005)
- [ ] Task 105: Migrate stimulant_tachycardia protocol to use evidence tiers
    **Description:** Update the stimulant_tachycardia protocol in protocols.rs to include evidence citations using the tiered framework.
    **Acceptance criteria:**
    - [ ] Protocol includes evidence field with properly tiered citations
    - [ ] Evidence citations follow the format established in updated protocol-authoring.md
    - [ ] Protocol still evaluates correctly and passes all existing tests
    - [ ] Evidence demonstrates the tiered approach (mix of Gold/Silver/Bronze where appropriate)
    **Verification:**
    - [ ] Tests pass: `cargo test` (specifically protocol engine tests)
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify migrated protocol still triggers correctly and shows evidence in protocol listings
    **Dependencies:** Task 101, Task 102, Task 103
    **Files likely touched:**
    - `src/protocols.rs`
    - **Estimated scope:** S (1-2 files)

- [ ] Task 106: Create protocol validation checklist
    **Description:** Create a simple checklist that helps protocol creators evaluate their evidence sources before submission.
    **Acceptance criteria:**
    - [ ] Checklist document exists with clear evaluation criteria
    - [ ] Helps creators assess whether they have sufficient evidence for each tier
    - [ ] Includes questions about evidence quality, conflicts, and gaps
    - [ ] References the evidence tier definitions and research guidance
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Checklist is practical and useful for new protocol creation
    **Dependencies:** Task 101, Task 102, Task 103, Task 104
    **Files likely touched:**
    - `docs/ideas/protocol-validation-checklist.md`
    - **Estimated scope:** S (1-2 files)

## Phase 1: Evidence Tier Definitions Checkpoint
- [ ] Evidence tier criteria documented and reviewed
- [ ] Reference document created and usable
- [ ] Ready to proceed to template updates

## Phase 2: Protocol Template & Guidance Checkpoint  
- [ ] Protocol template updated with evidence field guidance
- [ ] Research guidance document created
- [ ] Foundation for evidence-based protocol creation established

## Phase 3: Example Migration & Validation Checkpoint
- [ ] Example protocol successfully migrated to use evidence tiers
- [ ] Validation checklist created and practical
- [ ] All tests passing
- [ ] Framework demonstrated and ready for community use

## Risks and Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| Evidence tier criteria too subjective or inconsistent | Medium | Create clear examples and borderline cases; refine based on feedback |
| Protocol creators find evidence tiers burdensome or confusing | Medium | Start with simple implementation; provide plenty of examples; iterate based on usage |
| Evidence documentation becomes outdated quickly | Low | Design principles to be timeless; focus on evaluation methodology rather than specific sources |
| Migration breaks existing protocol functionality | High | Migrate one protocol at a time; run tests after each change; use git for easy rollback |
| Documentation doesn't get used by protocol creators | Medium | Make it practical and actionable; integrate into protocol creation workflow; solicit feedback |

## Open Questions
- What specific sources belong in each evidence tier for biohack/harm reduction protocols?
- How should we handle cases where high-quality evidence conflicts with valuable harm reduction community knowledge?
- What is the minimum viable evidence package for a protocol to be considered for inclusion in the built-in set?
- How might we evolve this framework over time based on community feedback and usage patterns?

## See Also
- Refined idea: docs/ideas/biohack-protocol-reference-framework.md
- Spec: specs/biohack-protocol-reference-framework.md

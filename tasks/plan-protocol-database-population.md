# Implementation Plan: Biohack Protocol Database Population

## Overview
Populate the biohack protocol database with evidence-based safety protocols using the tiered evidence framework. This plan focuses on creating new protocols and migrating existing ones to build a comprehensive safety system.

## Architecture Decisions
- Use evidence tiers (Gold/Silver/Bronze) to evaluate and document sources
- Start with migrating existing protocols, then create new ones based on user needs
- Focus on high-value protocols that address common biohacking risks
- Maintain backward compatibility with existing protocol engine
- Follow the principle: "Create protocols for real risks we face, not hypothetical ones"

## Task List

### Phase 1: Foundation & Framework (REQ-REF-001 through REQ-REF-003)
- [ ] Task 201: Research and define evidence tier criteria (Gold/Silver/Bronze)
    **Description:** Establish clear criteria for evidence tiers specific to biohack/harm reduction context.
    **Acceptance criteria:**
    - [ ] Documented criteria for each evidence tier with examples
    - [ ] Criteria align with harm reduction pragmatism and source-driven development
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Review evidence tier definitions for clarity
    **Dependencies:** None
    **Files likely touched:** `docs/ideas/protocol-evidence-tiers.md`
    **Estimated scope:** S (1-2 files)

- [ ] Task 202: Create evidence tier reference document
    **Description:** Create reference for evaluating sources when creating protocols.
    **Acceptance criteria:**
    - [ ] Document exists with explanations and examples for each tier
    - [ ] Includes guidance on handling conflicting evidence
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Document is useful for protocol creation
    **Dependencies:** Task 201
    **Files likely touched:** `docs/ideas/protocol-evidence-tiers.md`
    **Estimated scope:** S (1-2 files)

- [ ] Task 203: Update protocol YAML template to include evidence field
    **Description:** Modify protocol authoring guide to show evidence citation format.
    **Acceptance criteria:**
    - [ ] Protocol authoring guide shows evidence field format with examples
    - [ ] Template demonstrates proper YAML structure for evidence citations
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Updated protocol-authoring.md includes evidence field guidance
    **Dependencies:** Task 201, Task 202
    **Files likely touched:** `docs/protocol-authoring.md`
    **Estimated scope:** S (1-2 files)

### Phase 2: Migrate Existing Protocols to Evidence Framework
- [ ] Task 204: Migrate stimulant_tachycardia protocol to use evidence tiers
    **Description:** Update stimulant_tachycardia in protocols.rs with evidence citations.
    **Acceptance criteria:**
    - [ ] Protocol includes evidence field with properly tiered citations
    - [ ] Evidence follows format from protocol-authoring.md
    - [ ] Protocol still evaluates correctly and passes all existing tests
    - [ ] Evidence demonstrates tiered approach (mix of sources where appropriate)
    **Verification:**
    - [ ] Tests pass: `cargo test` (protocol engine tests)
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify migrated protocol still triggers correctly
    **Dependencies:** Task 201, Task 202, Task 203
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 205: Migrate hypertension_urgency protocol to use evidence tiers
    **Description:** Update hypertension_urgency in protocols.rs with evidence citations.
    **Acceptance criteria:**
    - [ ] Protocol includes evidence field with properly tiered citations
    - [ ] Evidence follows format from protocol-authoring.md
    - [ ] Protocol still evaluates correctly and passes all existing tests
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify migrated protocol still triggers correctly
    **Dependencies:** Task 201, Task 202, Task 203, Task 204
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 206: Migrate serotonin_syndrome_risk protocol to use evidence tiers
    **Description:** Update serotonin_syndrome_risk in protocols.rs with evidence citations.
    **Acceptance criteria:**
    - [ ] Protocol includes evidence field with properly tiered citations
    - [ ] Evidence follows format from protocol-authoring.md
    - [ ] Protocol still evaluates correctly and passes all existing tests
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify migrated protocol still triggers correctly
    **Dependencies:** Task 201, Task 202, Task 203, Task 204, Task 205
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

### Phase 3: CREATE NEW PROTOCOLS TO POPULATE THE DATABASE
- [ ] Task 207: Create protocol for cannabis-cardiovascular safety
    **Description:** Create new protocol addressing cardiovascular risks from cannabis use (tachycardia, hypotension, orthostatic issues).
    **Acceptance criteria:**
    - [ ] New protocol YAML added to protocols.rs with evidence tiers
    - [ ] Addresses cannabis-induced tachycardia, hypotension, or orthostatic intolerance
    - [ ] Includes evidence from pharmacological sources and harm reduction communities
    - [ ] Protocol evaluates correctly and integrates with engine
    - [ ] Can be tested with `biohack protocol test <protocol_id>`
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify new protocol creates, tests, and evaluates correctly
    **Dependencies:** Task 201, Task 202, Task 203, Task 204, Task 205, Task 206
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 208: Create protocol for stimulant-serotonin syndrome risk (enhanced)
    **Description:** Create enhanced protocol beyond basic serotonin syndrome, focusing on stimulant combinations.
    **Acceptance criteria:**
    - [ ] New protocol YAML added to protocols.rs addressing stimulant-serotonin risk
    - [ ] Goes beyond basic serotonin syndrome to cover MDMA, cocaine, etc. combinations
    - [ ] Includes evidence from clinical toxicology and harm reduction sources
    - [ ] Includes appropriate actions and constraints
    - [ ] Protocol evaluates correctly and integrates with engine
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify new protocol creates, tests, and evaluates correctly
    **Dependencies:** Task 201, Task 202, Task 203, Task 204, Task 205, Task 206, Task 207
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 209: Create protocol for nootropic-stack safety (racetam/choline)
    **Description:** Create protocol addressing risks from racetam-nootropic stacks (headache, jaw tension, insomnia).
    **Acceptance criteria:**
    - [ ] New protocol YAML added to protocols.rs for racetam/choline stack safety
    - [ ] Addresses common side effects: headache, muscle tension, sleep disruption
    - [ ] Includes evidence from pharmacological sources and user reports
    - [ ] Includes appropriate preventive actions and constraints
    - [ ] Protocol evaluates correctly and integrates with engine
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify new protocol creates, tests, and evaluates correctly
    **Dependencies:** Task 201, Task 202, Task 203, Task 204, Task 205, Task 206, Task 207, Task 208
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 210: Create protocol for dissociative-psychosis risk (ketamine/PCP/MXE)
    **Description:** Create protocol addressing psychosis risk from dissociative use, especially with cannabis/stimulants.
    **Acceptance criteria:**
    - [ ] New protocol YAML added to protocols.rs for dissociative psychosis risk
    - [ ] Addresses psychosis, mania, or severe anxiety from dissociatives
    - [ ] Includes evidence from clinical sources and harm reduction communities
    - [ ] Includes appropriate actions (benzodiazepines?, antipsychotics?) and constraints
    - [ ] Protocol evaluates correctly and integrates with engine
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify new protocol creates, tests, and evaluates correctly
    **Dependencies:** Task 201, Task 202, Task 203, Task 204, Task 205, Task 206, Task 207, Task 208, Task 209
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

### Phase 4: Validation & Guidance Tools
- [ ] Task 211: Create protocol validation checklist
    **Description:** Create simple checklist for evaluating new protocol submissions.
    **Acceptance criteria:**
    - [ ] Checklist document exists with clear evaluation criteria
    - [ ] Helps creators assess evidence quality, conflicts, and gaps
    - [ ] References evidence tier definitions and research guidance
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Checklist is practical and useful
    **Dependencies:** Task 201, Task 202, Task 203, Task 204, Task 205, Task 206, Task 207, Task 208, Task 209, Task 210
    **Files likely touched:** `docs/ideas/protocol-validation-checklist.md`
    **Estimated scope:** S (1-2 files)

## Phase 1: Foundation Checkpoint
- [ ] Evidence tier criteria documented and reviewed
- [ ] Reference document created
- [ ] Protocol template updated
- [ ] Ready to migrate existing protocols

## Phase 2: Migration Checkpoint
- [ ] All three existing protocols migrated to use evidence tiers
- [ ] Database contains 3 evidence-based protocols
- [ ] All tests passing
- [ ] Framework validated with existing content

## Phase 3: Population Checkpoint
- [ ] Database populated with 7 total protocols (3 migrated + 4 new)
- [ ] Addresses high-risk areas: cannabis, stimulant combinations, nootropic stacks, dissociatives
- [ ] All protocols use evidence tiers appropriately
- [ ] All tests passing

## Phase 4: Validation Checkpoint
- [ ] Validation checklist created and practical
- [ ] Complete framework for evidence-based protocol creation established
- [ ] System ready for community protocol contributions

## Risks and Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| Evidence tier criteria too subjective | Medium | Create clear examples; refine based on actual protocol creation |
| New protocols duplicate existing functionality | Low | Focus on distinct risk areas not well-covered by existing protocols |
| Evidence documentation becomes outdated | Low | Focus on evaluation principles rather than specific sources that change |
| New protocols conflict with existing ones in engine | High | Test thoroughly after each addition; use git for easy rollback |
| Protocol creation process too burdensome | Medium | Start simple; provide templates and examples; iterate based on feedback |

## Open Questions
- What specific protocols address the highest risks in the biohack community?
- How should we prioritize which new protocols to create first?
- What is the minimum viable evidence package for a protocol to be considered for inclusion?
- How might we evolve this protocol set over time based on emerging risks and community feedback?

## See Also
- Refined idea: docs/ideas/biohack-protocol-reference-framework.md
- Spec: specs/biohack-protocol-reference-framework.md

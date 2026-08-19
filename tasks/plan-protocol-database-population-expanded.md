# Implementation Plan: Biohack Protocol Database Population (EXPANDED)

## Overview
Populate the biohack protocol database with evidence-based safety protocols using the tiered evidence framework. This expanded plan focuses on creating protocols specifically relevant to the user's tracked substances (N-Methyl-Cyclazodone, Estradiol Valerate, Kratom, CBD) while also addressing broader biohacking risks.

## Architecture Decisions
- Use evidence tiers (Gold/Silver/Bronze) to evaluate and document sources
- Start with migrating existing protocols, then create new ones based on user's specific substance tracking
- Focus on high-value protocols that address risks from the user's actual substance use
- Maintain backward compatibility with existing protocol engine
- Follow the principle: "Create protocols for real risks we face, not hypothetical ones"

## Task List

### Phase 1: Foundation & Framework (REQ-REF-001 through REQ-REF-003)
- [ ] Task 301: Research and define evidence tier criteria (Gold/Silver/Bronze)
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

- [ ] Task 302: Create evidence tier reference document
    **Description:** Create reference for evaluating sources when creating protocols.
    **Acceptance criteria:**
    - [ ] Document exists with explanations and examples for each tier
    - [ ] Includes guidance on handling conflicting evidence
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Document is useful for protocol creation
    **Dependencies:** Task 301
    **Files likely touched:** `docs/ideas/protocol-evidence-tiers.md`
    **Estimated scope:** S (1-2 files)

- [ ] Task 303: Update protocol YAML template to include evidence field
    **Description:** Modify protocol authoring guide to show evidence citation format.
    **Acceptance criteria:**
    - [ ] Protocol authoring guide shows evidence field format with examples
    - [ ] Template demonstrates proper YAML structure for evidence citations
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Updated protocol-authoring.md includes evidence field guidance
    **Dependencies:** Task 301, Task 302
    **Files likely touched:** `docs/protocol-authoring.md`
    **Estimated scope:** S (1-2 files)

### Phase 2: Migrate Existing Protocols to Evidence Framework
- [ ] Task 304: Migrate stimulant_tachycardia protocol to use evidence tiers
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
    **Dependencies:** Task 301, Task 302, Task 303
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 305: Migrate hypertension_urgency protocol to use evidence tiers
    **Description:** Update hypertension_urgency in protocols.rs with evidence citations.
    **Acceptance criteria:**
    - [ ] Protocol includes evidence field with properly tiered citations
    - [ ] Evidence follows format from protocol-authoring.md
    - [ ] Protocol still evaluates correctly and passes all existing tests
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify migrated protocol still triggers correctly
    **Dependencies:** Task 301, Task 302, Task 303, Task 304
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 306: Migrate serotonin_syndrome_risk protocol to use evidence tiers
    **Description:** Update serotonin_syndrome_risk in protocols.rs with evidence citations.
    **Acceptance criteria:**
    - [ ] Protocol includes evidence field with properly tiered citations
    - [ ] Evidence follows format from protocol-authoring.md
    - [ ] Protocol still evaluates correctly and passes all existing tests
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify migrated protocol still triggers correctly
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

### Phase 3: CREATE NEW PROTOCOLS TO POPULATE THE DATABASE (EXPANDED)
- [ ] Task 307: Create protocol for cannabis-cardiovascular safety
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
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 308: Create protocol for estrogen-thrombosis risk (HRT safety)
    **Description:** Create new protocol addressing thrombotic risk from estrogen HRT use, especially with smoking, immobilization, or genetic factors.
    **Acceptance criteria:**
    - [ ] New protocol YAML added to protocols.rs with evidence tiers
    - [ ] Addresses increased clot risk from estrogen therapy
    - [ ] Includes evidence from clinical guidelines (ACOG, ENDO) and pharmacological sources
    - [ ] Includes appropriate actions (aspirin?, lifestyle changes?) and constraints (smoking cessation)
    - [ ] Protocol evaluates correctly and integrates with engine
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify new protocol creates, tests, and evaluates correctly
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306, Task 307
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 309: Create protocol for stimulant-estrogen interaction risk
    **Description:** Create new protocol addressing cardiovascular and thrombotic risks from combining stimulants with estrogen HRT.
    **Acceptance criteria:**
    - [ ] New protocol YAML added to protocols.rs with evidence tiers
    - [ ] Addresses risks from combining substances like N-Methyl-Cyclazodone with Estradiol Valerate
    - [ ] Includes evidence on synergistic cardiovascular strain and clotting risk
    - [ ] Includes appropriate actions (vitamin K?, monitoring?) and constraints (avoid combination)
    - [ ] Protocol evaluates correctly and integrates with engine
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify new protocol creates, tests, and evaluates correctly
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306, Task 307, Task 308
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 310: Create protocol for kratom-opioid interaction risk
    **Description:** Create new protocol addressing respiratory depression and sedation risks from combining kratom with other opioids, benzos, or alcohol.
    **Acceptance criteria:**
    - [ ] New protocol YAML added to protocols.rs with evidence tiers
    - [ ] Addresses increased risk of respiratory arrest from kratom + other CNS depressants
    - [ ] Includes evidence from pharmacological sources and harm reduction communities
    - [ ] Includes appropriate actions (naloxone availability?) and constraints (avoid combinations)
    - [ ] Protocol evaluates correctly and integrates with engine
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify new protocol creates, tests, and evaluates correctly
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306, Task 307, Task 308, Task 309
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 311: Create protocol for CBD-drug interaction risk (CYP450)
    **Description:** Create new protocol addressing cytochrome P450 mediated drug interactions with CBD.
    **Acceptance criteria:**
    - [ ] New protocol YAML added to protocols.rs with evidence tiers
    - [ ] Addresses CBD's inhibition of CYP3A4, CYP2D6, and other enzymes affecting drug metabolism
    - [ ] Includes evidence from pharmacological sources and clinical studies
    - [ ] Includes appropriate actions (dose adjustment?) and constraints (avoid with narrow therapeutic index drugs)
    - [ ] Protocol evaluates correctly and integrates with engine
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Verify new protocol creates, tests, and evaluates correctly
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306, Task 307, Task 308, Task 309, Task 310
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 312: Create protocol for stimulant-serotonin syndrome risk (enhanced)
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
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306, Task 307, Task 308, Task 309, Task 310, Task 311
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 313: Create protocol for nootropic-stack safety (racetam/choline)
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
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306, Task 307, Task 308, Task 309, Task 310, Task 311, Task 312
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

- [ ] Task 314: Create protocol for dissociative-psychosis risk (ketamine/PCP/MXE)
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
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306, Task 307, Task 308, Task 309, Task 310, Task 311, Task 312, Task 313
    **Files likely touched:** `src/protocols.rs`
    **Estimated scope:** S (1-2 files)

### Phase 4: Validation & Guidance Tools
- [ ] Task 315: Create protocol validation checklist
    **Description:** Create simple checklist for evaluating new protocol submissions.
    **Acceptance criteria:**
    - [ ] Checklist document exists with clear evaluation criteria
    - [ ] Helps creators assess evidence quality, conflicts, and gaps
    - [ ] References evidence tier definitions and research guidance
    **Verification:**
    - [ ] Tests pass: `cargo test`
    - [ ] Build succeeds: `cargo build`
    - [ ] Manual check: Checklist is practical and useful
    **Dependencies:** Task 301, Task 302, Task 303, Task 304, Task 305, Task 306, Task 307, Task 308, Task 309, Task 310, Task 311, Task 312, Task 313, Task 314
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
- [ ] Database populated with 12 total protocols (3 migrated + 9 new)
- [ ] Addresses high-risk areas including user's specific substance use
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

## Summary of Protocols to be Created:
**Migrated Existing (3):**
1. stimulant_tachycardia
2. hypertension_urgency  
3. serotonin_syndrome_risk

**Newly Created (9):**
4. cannabis-cardiovascular safety
5. estrogen-thrombosis risk (HRT safety)
6. stimulant-estrogen interaction risk
7. kratom-opioid interaction risk
8. CBD-drug interaction risk (CYP450)
9. stimulant-serotonin syndrome risk (enhanced)
10. nootropic-stack safety (racetam/choline)
11. dissociative-psychosis risk (ketamine/PCP/MXE)

**Total Protocols in Database: 12**

## See Also
- Refined idea: docs/ideas/biohack-protocol-reference-framework.md
- Spec: specs/biohack-protocol-reference-framework.md

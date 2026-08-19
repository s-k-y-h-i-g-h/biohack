# Implementation Plan: Biohack Protocol Reference Framework

## Phase 1: Evidence Tier Definitions (REQ-REF-001)
- [ ] Define Gold tier: RCTs, meta-analyses, clinical guidelines (NIH, FDA, Cochrane)
- [ ] Define Silver tier: Pharmacology databases (PubChem, DrugBank), mechanistic studies, case reports
- [ ] Define Bronze tier: Harm reduction sources (Erowid, PsychonautWiki, TripSit), anecdotal reports, traditional use
- [ ] Create documentation: docs/ideas/protocol-evidence-tiers.md

## Phase 2: Protocol Template & Guidance (REQ-REF-002, REQ-REF-003)
- [ ] Update protocol YAML template in docs/protocol-authoring.md to show evidence format
- [ ] Add evidence citation examples to existing protocols
- [ ] Create guidance document: docs/ideas/protocol-research-guidance.md
- [ ] Update ProtocolAuthoring skill if needed

## Phase 3: Example Migration & Validation (REQ-REF-004, REQ-REF-005)
- [ ] Migrate stimulant_tachycardia protocol to use evidence tiers in protocols.rs
- [ ] Create validation checklist: docs/ideas/protocol-validation-checklist.md
- [ ] Test that migrated protocol still functions correctly
- [ ] Update tests if needed

## Verification Steps
- [ ] Run cargo test to ensure nothing broken
- [ ] Verify new protocol can be created using the framework
- [ ] Check that evidence tiers appear in protocol listings
- [ ] Validate that migrated protocols still trigger correctly

## Estimated Effort
- Phase 1: 2-3 hours (research + documentation)
- Phase 2: 2-3 hours (template updates + guidance)
- Phase 3: 3-4 hours (migration + validation + testing)
- Total: 7-10 hours

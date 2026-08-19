---
spec_version: "1.0"
project: "biohack-protocol-reference-framework"
version: "0.1.0"
project_dir: "."
quality_gates:
  test: true
  clippy: true
  fmt: true
---

# Biohack Protocol Reference Framework

## Requirements

### REQ-REF-001: Evidence Tier Definitions
- **Description**: Define clear evidence tiers (Gold/Silver/Bronze) with criteria for evaluating sources in biohack protocol creation
- **Acceptance**: Documentation exists with explicit criteria for each tier; example protocol demonstrates proper tier usage
- **Priority**: high
- **Dependencies**: []

### REQ-REF-002: Protocol Template Update
- **Description**: Update protocol YAML template to include evidence citations with tier labels
- **Acceptance**: Protocol YAML schema includes evidence field with tier annotations; at least one example protocol shows proper usage
- **Priority**: high
- **Dependencies**: [REQ-REF-001]

### REQ-REF-003: Guidance Document Creation
- **Description**: Create guidance document for researching and citing sources for biohack protocols
- **Acceptance**: Docs/ideas/protocol-research-guidance.md exists with practical advice on source evaluation and citation
- **Priority**: medium
- **Dependencies**: [REQ-REF-001, REQ-REF-002]

### REQ-REF-004: Example Protocol Migration
- **Description**: Migrate one existing built-in protocol (e.g., stimulant_tachycardia) to use the tiered evidence framework
- **Acceptance**: At least one protocol in protocols.rs demonstrates evidence tier usage; protocol still passes all tests
- **Priority**: high
- **Dependencies**: [REQ-REF-001, REQ-REF-002]

### REQ-REF-005: Validation Checklist
- **Description**: Create simple validation checklist for new protocol submissions using the evidence framework
- **Acceptance**: Checklist document exists that helps protocol creators evaluate their evidence sources
- **Priority**: medium
- **Dependencies**: [REQ-REF-001, REQ-REF-002, REQ-REF-003]

## Implementation Notes

This spec implements the tiered evidence approach refined in the idea-refine session for creating reliable, evidence-based biohack safety protocols.

The framework acknowledges that no single perfect source exists and provides a transparent methodology for combining multiple evidence types with appropriate confidence levels.

Implementation should follow these phases:
1. Define evidence tiers and criteria (REQ-REF-001)
2. Update protocol template and create guidance (REQ-REF-002, REQ-REF-003) 
3. Migrate example protocol and create validation tools (REQ-REF-004, REQ-REF-005)

# Biohack Protocol Reference Framework: Tiered Evidence Approach

## Problem Statement
How might we identify reliable, evidence-based reference materials for creating biohack safety protocols that prevent adverse events and provide actionable guidance for self-experimenters and harm reduction practitioners?

## Recommended Direction
Adopt a tiered evidence approach that explicitly grades and combines multiple source types according to their reliability and relevance to the biohack/harm reduction context. This approach acknowledges that no single perfect source exists (as identified in project memory) while creating a transparent, defensible methodology for protocol creation. Rather than seeking one "true" source, we create a framework where protocols cite multiple lines of evidence with clear confidence levels, allowing users to understand the basis for each safety rule.

This direction was chosen because it:
1. Aligns with the project's harm-reduction pragmatism and source-driven development methodology
2. Provides a scalable framework that can grow with the protocol library
3. Creates transparency about evidence quality, enabling informed user decisions
4. Builds on existing patterns in substances.yaml (which already cites sources like Examine.com, PubChem, NIH ODS)
5. Allows starting simple (Tier 1 only) and evolving to more sophisticated evidence grading

## Key Assumptions to Validate
- [ ] That a 3-tier evidence system (Gold/Silver/Bronze) is sufficiently granular yet usable
  - *Test:* Implement in 2-3 prototype protocols and survey potential users on clarity
- [ ] That protocol creators can consistently apply evidence grading rubric
  - *Test:* Have 2-3 people grade the same sources and compare inter-rater reliability
- [ ] That users will trust and act on protocols with clearly labeled evidence limitations
  - *Test:* A/B test protocol presentations with/without evidence tiers in safety scenarios
- [ ] That the most valuable protocols combine mechanistic understanding with harm reduction pragmatism
  - *Test:* Analyze which existing community protocols are most referenced/used

## MVP Scope
**What's IN:**
- Evidence tier definitions with clear criteria (Gold/Silver/Bronze)
- Template for protocol YAML files showing evidence citation format
- Example protocol (e.g., stimulant tachycardia) re-written using the tiered evidence approach
- Guidance document: "How to Research and Cite Sources for Biohack Protocols"
- Simple validation checklist for new protocol submissions

**What's OUT (for MVP):**
- Automated evidence scoring or validation tools
- Community protocol submission/review system
- Integration with external databases for automatic evidence lookup
- Protocol expiration or automatic review system
- Multi-language support or formal peer review process

## Not Doing (and Why)
- [ ] **Creating a master protocol database from scratch** — Starting with extending the existing 3 built-in protocols is more practical and validates the framework first
- [ ] **Requiring double-blind RCT evidence for all protocols** — This would exclude valuable harm reduction knowledge and mechanistic insights critical tonovel compound safety
- [ ] **Building an AI-powered evidence aggregator** — Over-engineering; the human judgment in evidence selection is valuable for context understanding
- [ ] **Including sources with commercial conflicts of interest without disclosure** — Would undermine trust, but we note this as a consideration in the guidance
- [ ] **Waiting for perfect evidence before creating any protocol** — Would prevent starting; we begin with best available evidence and improve over time

## Open Questions
- What specific evidence tiers and criteria best balance rigor with usability for the biohack context?
- How should we handle conflicts between evidence tiers (e.g., Gold says X, Bronze says not-X)?
- What is the minimum viable evidence package for a protocol to be considered "actionable" versus "informational"?
- How do we effectively communicate evidence limitations without causing alarm or paralysis?

---
*This refinement was created via the idea-refine skill on 2026-08-19 to guide implementation of evidence-based protocol creation for the biohack project.*

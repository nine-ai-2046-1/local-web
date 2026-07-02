## Context

The project currently has a detailed README.md in English (166 lines). The user wants:
1. A simplified English version for quick reference
2. A Cantonese version for Hong Kong developers

**Current State**: Detailed English README exists at project root.

**Constraints**:
- Keep documentation concise and easy to understand
- Maintain consistency between versions
- No code changes - documentation only

**Stakeholders**: English and Cantonese speaking developers

## Goals / Non-Goals

**Goals:**
- Create a simplified English README with essential usage information
- Create a Cantonese README for Hong Kong developers
- Keep both versions consistent in structure and content
- Make documentation accessible to target audiences

**Non-Goals:**
- Not replacing the existing detailed README
- Not adding new technical documentation
- Not translating code comments
- Not creating multi-language support infrastructure

## Decisions

### Decision 1: File Naming Convention

**Choice**: 
- `README.md` - Simplified English (overwrite existing)
- `README-HK.md` - Cantonese version

**Alternatives Considered**:
- `README.en.md` and `README.hk.md` - More explicit but less standard
- `README-simplified.md` and `README-cantonese.md` - Clear but verbose

**Rationale**: `README.md` is standard for primary documentation. `README-HK.md` follows common convention for localized versions (like `README-ja.md` for Japanese).

### Decision 2: Content Structure

**Choice**: Both files follow same structure:
1. One-line description
3-5 key features
Installation
Usage examples
Command line options (table)
Supported file types (brief)

**Alternatives Considered**:
- Different structures for each language
- Minimal version vs detailed version

**Rationale**: Consistent structure makes it easier to maintain and compare versions. Simplified content focuses on what users need most.

### Decision 3: Cantonese Translation Style

**Choice**: Professional Cantonese with technical terms kept in English

**Format**:
- Technical terms: English (e.g., "HTTPS", "CLI", "port")
- Instructions: Cantonese
- Code examples: English (unchanged)

**Alternatives Considered**:
- Pure Cantonese with all terms translated
- Mixed Cantonese-English (more casual)

**Rationale**: Technical terms are universally understood in English. Keeping them in English maintains accuracy and avoids confusion.

## Risks / Trade-offs

### Risk 1: Content Drift
**Risk**: Two versions may diverge over time
**Mitigation**: 
- Same structure makes updates easier
- Can add maintenance notes in future

### Risk 2: Translation Accuracy
**Risk**: Cantonese translation may not be perfect
**Mitigation**: 
- Focus on clarity over perfect grammar
- Technical accuracy prioritized

### Trade-off: Simplification vs Completeness
**Choice**: Prioritize simplicity over completeness
- Detailed README remains available
- Simplified versions serve quick reference
- Users can refer to detailed version for advanced topics

## Migration Plan

This is a documentation-only change. No migration needed.

**Rollback Strategy**: N/A - can simply revert file changes.

## Open Questions

1. **Which README should be primary?** Should `README.md` be the simplified version or keep the detailed version?
2. **Cantonese dialect**: Should we use formal written Cantonese or more colloquial style?
3. **Maintenance**: Who will keep both versions in sync?

---

**Decision**: Proceed with current design. Open questions can be addressed during implementation.
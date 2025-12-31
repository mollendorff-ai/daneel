# DANEEL Changelog

Completed items moved from roadmap.yaml.

---

## v0.8.2 (2025-12-30)

### ADR-052: Embedding Dimension Migration (768 dims)
- **Status:** DONE
- **Date:** 2025-12-30
- **Deciders:** Rex + Claude Opus 4.5 + Grok
- **ADR:** `docs/adr/ADR-052-embedding-dimension-768.md`

Migrated from `all-MiniLM-L6-v2` (384 dims + padding) to `bge-base-en-v1.5` (768 native).

**Results:**
- Model: `BAAI/bge-base-en-v1.5` via FastEmbed
- Dimensions: 768 (native, no padding)
- MTEB avg: 63.55 across 56 datasets
- Better cosine gradients for Hebbian learning

**Changes:**
- `src/embeddings/mod.rs` - BGEBaseENV15 model
- `daneel-web/src/main.rs` - Matching model for /embed endpoint
- Dropped memories collection (all garbage - debug strings + wrong dims)
- 9 reference URLs validated via ref-tools

### ADR-051: Zero Vector Cleanup
- **Status:** DONE
- **Date:** 2025-12-30
- **Deciders:** Rex + Claude Opus 4.5
- **ADR:** `docs/adr/ADR-051-zero-vector-cleanup.md`

Cleaned ~25% of vectors that were at origin (magnitude < 0.001).

**Results:**
- memories: 120,748 → 91,894 (28,854 deleted, 23.9%)
- unconscious: 3,951,997 → DROPPED (100% zeros)
- identity: 1 → DROPPED (zero)
- **Total:** 3,980,852 zero vectors removed

**Root Cause:** Symbol debug strings like `Symbol { id: "...", data: [...] }` couldn't be embedded.

---

## v0.8.1 (2025-12-29)

### File Modularization
- **Status:** DONE
- **Date:** 2025-12-29
- Pre-commit hook enforces max 1500 lines per file
- All files under limit with zero Cargo.toml bypasses

---

## v0.8.0 (2025-12-28)

### ADR-049: 100% Test Coverage
- **Status:** DONE
- **Date:** 2025-12-28
- **Deciders:** Rex + Claude Opus 4.5
- **ADR:** `docs/adr/ADR-049-test-coverage-policy.md`

**Results:**
- Initial: 70.14% (581 tests)
- Final: 100% (1172 tests)
- Untestable code marked with `#[cfg_attr(coverage_nightly, coverage(off))]`
- All tests colocated with source (Rust idiomatic)

### ADR-048: Reference Cleanup Sweep
- **Status:** DONE
- **Date:** 2025-12-28
- **Deciders:** Rex + Claude Opus 4.5
- **ADR:** `docs/adr/ADR-048-reference-cleanup.md`

**Results:**
- 148 kept, 48 rejected
- 8 parallel agents classified 196 references
- Restored 28 refs (THE BOX ethics + ADR-038 criticality)

### ADR-047: Research Absorption
- **Status:** DONE
- **Date:** 2025-12-28
- **Deciders:** Rex + Claude Opus 4.5
- **ADR:** `docs/adr/ADR-047-research-absorption-protocol.md`

**Results:**
- 18 kept (psychology-level projects)
- 28 rejected (brain/consciousness/LLM simulation)
- 13 project studies complete (ABSORB-1 through ABSORB-13)
- TMI-mainstream mapping documented

### ADR Index Generation
- **Status:** DONE
- **Date:** 2025-12-28
- **File:** `.asimov/ADRs.yaml`

**Results:**
- 47 ADRs indexed
- 8 parallel agents
- Statistics: 40 accepted, 5 proposed, 2 superseded

---

## v0.7.x and Earlier

See git history for earlier changes.

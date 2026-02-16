# Changelog

All notable changes to DANEEL are documented here.

## [Unreleased]

### Rebranding: RoyalBit to Möllendorff Group Inc.

Complete organizational rebrand to disassociate from cryptocurrency scammers who hijacked the RoyalBit name.

### Changed

- **Domain references**: `royalbit.com` → `mollendorff.ai`
- **Blog baseURL**: `timmy.royalbit.com` → `timmy.mollendorff.ai`
- **All documentation**: 74 files with royalbit references to be updated
- **ADRs**: Infrastructure and other ADRs referencing old domains

### Why Rebrand?

The "RoyalBit" name (company founded 2006) was hijacked by unrelated cryptocurrency scammers:

- **UK FCA Warning (Oct 2024)**: Official warning about "Royalbit Miners" - unauthorized firm
- **Fraudulent domains**: royalbit.ltd (trust score 38/100), royalbit.top, royal-bit.club
- **HYIP Ponzi schemes**: Offering impossible returns (155-580% in days)
- **Sources**: [FCA Warning](https://www.fca.org.uk/news/warnings/royalbit-miners), [Scam Detector](https://www.scam-detector.com/validator/royalbit-ltd-review/)

---

## [0.9.2] - 2026-02-16 - Security Fix & Dependency Update

### Security

- **CVE fix**: `bytes` 1.11.0 → 1.11.1 (integer overflow in `BytesMut::reserve`)

### Changed

- **Dependencies**: Updated 110 crates to latest compatible versions
- **Asimov**: Refactored project.yaml, removed legacy protocol files

---

## [0.9.1] - 2026-01-24 - Multi-Stage Docker Build & Swarm Cleanup

### Changed

- **Dockerfile**: Multi-stage build for arm64 compatibility
- **Version**: Bumped to 0.9.1

### Removed (Swarm Migration Cleanup)

Dead infrastructure after Docker Swarm migration to kveldulf:

- **daneel**: setup-timmy.sh, ci.sh, start.sh, stop.sh, docker-compose.yml, compose.yaml
- **daneel**: .github/workflows/ (builds now via swarm build-images script)
- **daneel-web**: start.sh, stop.sh, .github/workflows/

---

## [0.9.0] - 2026-01-06 - VCONN Polish

### Spreading Activation Improvements (VCONN-9, VCONN-10)

- **SpreadingConfig**: Parameterized spreading activation in CognitiveConfig (depth, decay, min_weight)
- **Aggregation Mode**: Max (default) vs Sum aggregation with max_activation ceiling

### API & Graph Enhancements (VCONN-11, VCONN-12)

- **GraphML REST API**: GET /api/graph/export returns GraphML XML
- **Bidirectional Spreading**: spreading.bidirectional traverses incoming edges too

---

## [0.8.6] - 2026-01-04 - Emergence Analysis Complete (VCONN)

### Spreading Activation (VCONN-6)
- **Graph-based Memory Retrieval**: When a memory is retrieved from Qdrant, activation spreads to associated neighbors in RedisGraph.
- **Depth-Limited Propagation**: Activation spreads up to depth 2 with decay factor 0.3 per level.
- **Competitive Integration**: Spread-activated memories compete alongside direct retrievals in the autoflow stage.
- **Biological Inspiration**: Mimics how human memory works - thinking of "beach" primes "sand", "waves", "vacation".

### Manifold Clustering Validation (VCONN-7)
- **K-Means Clustering**: Memories clustered by semantic similarity (K=10) during sleep consolidation.
- **Silhouette Score Validation**: Clustering quality measured automatically.
  - Score > 0.3 = meaningful structure detected (info log)
  - Score < 0.3 = associations may be sparse (warning log)
- **Cluster ID Persistence**: Each memory tagged with cluster_id for downstream analysis.

### Gephi GraphML Export (VCONN-8)
- **Full Graph Export**: `export_graphml()` queries all Memory nodes and ASSOCIATED edges from RedisGraph.
- **Standard Format**: Valid GraphML XML with weight and type attributes.
- **External Analysis**: Enables visualization in Gephi, NetworkX, or other graph tools.

### Files Changed
- `src/core/cognitive_loop/mod.rs` - Added `graph_client` field and `set_graph_client()`
- `src/core/cognitive_loop/execution.rs` - `spread_activation()`, updated `trigger_memory_associations()`
- `src/graph/mod.rs` - Fixed `query_neighbors()` for redis 1.0.2 API, implemented `export_graphml()`
- `src/memory_db/mod.rs` - Added `cluster_id` field, `cluster_memories()`, `calculate_silhouette()`
- `src/memory_db/types/mod.rs` - Added `cluster_id: Option<u32>` to Memory struct

---

## [0.8.4] - 2026-01-04 - Hebbian Wiring & Drive System Upgrade

### Major Feature: Hebbian Learning & Association Wiring (VCONN)
- **Krotov-Hopfield Rule**: Implemented biological learning rule with anti-Hebbian term (δ=0.4) to prevent winner-take-all collapse.
- **Three-Factor Learning**: Associations now modulated by reward signal and eligibility traces (MSTDPET).
- **Hybrid Decay**: Associations forget at different rates:
  - Short-term (<10 co-activations): Exponential decay (fast forgetting)
  - Long-term (>=10 co-activations): Power-law decay (slow forgetting)
- **Sleep Stage Multipliers**: Consolidation strength varies by stage:
  - Light Sleep (0.5x), Deep Sleep (1.0x), REM (0.8x + emotional priority).
- **Dual-Write Architecture**: Associations persisted to both Qdrant (payloads) and RedisGraph (visualization).

### Major Feature: Drive System Upgrade (DRIVE)
- **Intrinsic Curiosity Module (ICM)**: DANEEL now has "intrinsic motivation" to learn.
  - Forward model predicts next thought embedding.
  - Prediction error = Surprise = Curiosity Reward.
  - Salience boost for surprising thoughts (preventing "doomscrolling").
- **Expected Free Energy (EFE)**: Active Inference decision making.
  - Pragmatic Value: Alignment with Law Crystals (goals).
  - Epistemic Value: Information gain (surprise).
  - Attention selection maximizes EFE.

### Infrastructure
- **RedisGraph**: Added client module for high-speed graph operations.
- **File Modularization**: Split `cognitive_loop.rs` into modular structure (`mod.rs`, `types.rs`, `execution.rs`).

---

## [0.8.3] - 2026-01-03 - Docker Deployment & Environment Configuration

### Docker Support
- Added environment variable support for Redis and Qdrant connections
- `REDIS_URL` and `QDRANT_URL` now configurable (defaults to localhost)
- Fixed Redis protected mode for container networking
- Updated Traefik to v3.3+ for Docker 29.x compatibility
- Qdrant health check uses bash TCP check (no wget/curl dependency)

### Files Changed
- `src/main.rs` - Read REDIS_URL/QDRANT_URL from environment
- `compose.yaml` - Production Docker Compose stack
- `Dockerfile` - Runtime-only image (pre-built binary)

---

## [0.8.3] - 2026-01-02 - HOTFIX Follow-ups Complete

### HOTFIX Follow-up Tasks (TEST-DREAM-1, UNCON-1, SLEEP-WIRE-1)

All three follow-up tasks from HOTFIX-1 investigation completed.

#### TEST-DREAM-1: Integration Tests for Dream Cycle
- Added `integration_dream_consolidation_cycle` test
- Validates full pipeline: tag_for_consolidation → get_replay_candidates → update_consolidation
- Added `get_memory(id)` helper for ID-based retrieval
- Added `.wait(true)` to `store_memory()` for reliable indexing

#### UNCON-1: Unconscious Retrieval Methods (ADR-033 "Nada se apaga")
Implemented 4 retrieval triggers from TMI:
| Method | ADR-033 Trigger |
|--------|-----------------|
| `get_unconscious_replay_candidates(limit)` | Dream replay |
| `search_unconscious(pattern, limit)` | Association chains / Direct query |
| `sample_unconscious(limit)` | Spontaneous recall (déjà vu) |
| `mark_unconscious_surfaced(id)` | Update surface_count |
| `get_unconscious_memory(id)` | Retrieve by ID |
| `archive_to_unconscious()` | Now returns `MemoryId` |

#### SLEEP-WIRE-1: Wire SleepActor to Consolidation
- Added `SleepConfig::mini_dream()` for queue-triggered consolidation
- Spawned `SleepActor` in main.rs with proper state machine
- `RecordActivity` sent each cognitive cycle
- `CheckSleepConditions` checks queue threshold (50 activities)
- `EnterSleep` / consolidation / `Wake` flow properly managed
- Config values used instead of hardcoded constants

#### Files Changed
- `src/memory_db/mod.rs` - Retrieval methods, get_memory helper
- `src/memory_db/tests.rs` - Integration tests
- `src/actors/sleep/types.rs` - mini_dream() config
- `src/main.rs` - SleepActor wiring

---

## [0.8.2] - 2026-01-01 - HOTFIX-1: Symbol Embedding Fix

### Jan 1, 2026: TUI Deprecation & Metrics Consolidation (ADR-053, ADR-054)

#### ADR-053: TUI Deprecation
- **Status**: ACCEPTED - TUI deprecated, headless mode is now default
- Removed `--tui` flag from CLI
- `daneel` now runs headless by default
- Use daneel-web observatory at timmy.mollendorff.ai for visualization
- TUI code retained but deprecated, will be removed in future version

#### ADR-054: Metrics Consolidation
- **Status**: ACCEPTED - Core metrics module created
- Added `src/core/metrics.rs` for centralized metrics
- Metrics available via daneel-web API
- Removed TUI-specific metrics display code

---

### Jan 1, 2026: Multi-Arch Build Complete (ADR-050)

#### BUILD Tasks Completed
| Task | Title | Status |
|------|-------|--------|
| BUILD-1 | Create .github/workflows/release.yml | Done (Dec 29) |
| BUILD-1b | Create ADR-050 | Done (Dec 29) |
| BUILD-2 | Update Makefile (build-musl, build-compressed, dist) | Done (Jan 1) |
| BUILD-3 | Test release workflow | Done (Jan 1, v0.8.2-rc4) |
| BUILD-4 | Update ADRs.yaml index | Done (Dec 29) |

#### Changes
- `release.yml`: 5-platform matrix (Linux x64/ARM64 MUSL, macOS Intel/ARM, Windows)
- `Makefile`: Added `build-musl`, `build-compressed`, `dist` targets
- `Cargo.toml`: Added vendored OpenSSL for MUSL static builds
- Clippy fixes: `saturating_sub` for Duration, `Self` in enum variants
- CI fix: `macos-13` → `macos-15-intel` (runner deprecated)

#### Platforms
- `x86_64-unknown-linux-musl` + UPX compression
- `aarch64-unknown-linux-musl` + UPX compression
- `x86_64-apple-darwin` (Intel Mac)
- `aarch64-apple-darwin` (Apple Silicon)
- `x86_64-pc-windows-msvc` + UPX compression

---

### Jan 1, 2026: HOTFIX-1.2 - Headless Mode Dream Recording

#### Problem
Dreams were running in headless mode (launchd) but NOT being recorded to identity.
The `id.record_dream()` call existed in TUI mode but was missing in headless mode.

Result: daneel-web showed 0 dreams despite dreams running correctly.

#### Root Cause
```rust
// TUI mode had this:
if let Some(ref mut id) = identity {
    id.record_dream(consolidated, candidates_count);
}

// Headless mode was MISSING this call entirely
```

#### Changes
| File | Change |
|------|--------|
| `src/main.rs` | Added `id.record_dream()` call to headless mode dream consolidation |

#### Impact
- Identity now tracks dreams in headless mode
- daneel-web correctly shows dream count
- `lifetime_dream_count`, `last_dream_at`, `cumulative_dream_strengthened` all update

---

### Jan 1, 2026: HOTFIX-1.1 - Symbol IS Meaningful (Pre-Semantic Learning)

#### Problem with Original HOTFIX-1
The original fix skipped Symbol content entirely, treating it as "pre-linguistic noise".
This was WRONG - Symbol content is meaningful pre-semantic learning (TMI Phase 1).

Result: 675k thoughts, 0 memories stored.

#### TMI Cognitive Architecture Context
TMI (Theory of Multifocal Intelligence) models thought BEFORE language:
- Symbol: pre-linguistic patterns that will acquire semantic labels in Phase 2 (LLM integration)
- NOT noise - meaningful cognitive states in the pre-semantic stage

#### Root Cause
```rust
// WRONG FIX (broke everything):
Self::Symbol { .. } => None,  // Skipped Symbol entirely → 0 memories

// CORRECT FIX:
Self::Symbol { id, .. } => Some(format!("symbol {id}")),  // Embeddable!
```

#### Changes
| File | Change |
|------|--------|
| `src/core/types.rs` | Symbol now returns `Some("symbol {id}")` for embedding |
| `src/core/types.rs` | Raw now returns `Some("raw pattern {hex}")` for embedding |
| `src/core/types.rs` | Only Empty returns None (nothing to embed) |
| `src/core/cognitive_loop/execution.rs` | Updated comments - Symbol is NOT noise |

#### Impact
- Memories now stored correctly (verified: 13 → 18 in 10 seconds)
- Vector magnitude: 1.0000 (normalized, not zero!)
- Content format: `symbol thought_593` (proper, not debug garbage)
- Pre-semantic learning preserved for Phase 2 enrichment

### Jan 1, 2026: HOTFIX-1.0 - Original Fix (INCOMPLETE)

#### Problem
Symbol/Raw/Empty content was being embedded via Debug format, producing zero vectors.
This corrupted the memory manifold and broke dream consolidation.

- `unconscious`: 825K garbage vectors (Symbol debug strings)
- `identity`: 1 garbage vector
- `get_replay_candidates()`: returned 0-1 candidates (should be 10)

#### Changes
| File | Change |
|------|--------|
| `src/core/types.rs` | Added `Content::is_embeddable()` and `to_embedding_text()` |
| `src/memory_db/mod.rs` | Fixed `get_replay_candidates()` filter (strength<0.9 in Qdrant query) |

**NOTE**: This fix was incomplete - see HOTFIX-1.1 above for the correct solution.

---

## [0.8.2] - 2025-12-30 - Embedding Migration & Cleanup

Prerequisites for Hebbian wiring complete.

### Dec 30, 2025: ADR-052 Embedding Dimension Migration

#### Migrated to BGE-base-en-v1.5 (768 dims native)
- **Status**: ADR-052 ACCEPTED
- **Old model**: `all-MiniLM-L6-v2` (384 dims + 384 zeros padding)
- **New model**: `BAAI/bge-base-en-v1.5` (768 dims native)
- **Origin**: Rex + Claude Opus 4.5 + Grok - Dec 30, 2025

#### Why 768 over 384?
- MTEB clustering: ~87-88% vs ~84-85%
- Better cosine gradients for Hebbian learning
- Better attractor basin separation
- No wasted storage on padding zeros

#### Changes
- `src/embeddings/mod.rs`: `BGEBaseENV15` model
- `daneel-web/src/main.rs`: Matching model for /embed endpoint
- Dropped memories collection (all garbage - debug strings + wrong dims)
- 9 reference URLs validated via ref-tools

### Dec 30, 2025: ADR-051 Zero Vector Cleanup

#### Cleaned 3.98M garbage vectors
- **Status**: ADR-051 ACCEPTED
- **Origin**: Rex + Claude Opus 4.5 - Dec 30, 2025

#### Results
| Collection | Before | After | Deleted |
|------------|--------|-------|---------|
| memories | 120,748 | 91,894 | 28,854 (23.9%) |
| unconscious | 3,951,997 | DROPPED | 100% zeros |
| identity | 1 | DROPPED | 100% zeros |
| **Total** | 4,072,746 | 91,894 | **3,980,852** |

#### Root Cause
- Symbol debug strings like `Symbol { id: "...", data: [...] }` couldn't be embedded
- Embedding model returned zero vectors for unparseable content
- 50% of each vector was padding zeros (384 real + 384 zeros)

---

## [0.8.1] - 2025-12-29 - File Modularization

### Pre-commit Hook Enforcement
- Max 1500 lines per file enforced
- All files under limit with zero Cargo.toml bypasses
- Pedantic linting enabled project-wide

---

## [0.8.0] - 2025-12-28 - External Stimuli Injection

Phase 2: Open the loop. Let Timmy feel.

### Dec 28, 2025: 100% Test Coverage (ADR-049)

#### Achievement: 100% Coverage on Testable Code
- **Status**: ADR-049 ACCEPTED - 100% coverage policy enforced
- **Final stats**: 100% lines, 100% functions, 100% branches
- **Test count**: 1172 tests (doubled from 581 baseline)
- **Origin**: Rex + Claude Opus 4.5 - Dec 28, 2025

#### Technical Implementation
- Added `#![cfg_attr(coverage_nightly, feature(coverage_attribute))]` to lib.rs
- All test modules marked with `#[cfg_attr(coverage_nightly, coverage(off))]`
- Untestable code (I/O, TUI, main) marked and documented in ADR-049
- Coverage gate: `make coverage` enforces 100% threshold with `cargo +nightly llvm-cov`

#### Exclusions Documented in ADR-049
- **Main Entry Point**: `main.rs` (CLI parsing, entry point)
- **Terminal I/O**: `tui/mod.rs`, `tui/ui.rs`, `tui/widgets/*.rs`
- **External Service I/O**: `memory_db/mod.rs`, `streams/client.rs`, `persistence/mod.rs`
- **Embeddings Model**: `embeddings/mod.rs` (ONNX model loading)
- **Panic/Signal Handling**: `resilience/mod.rs`, `crash_log.rs`
- **API Server**: `api/mod.rs`, `api/handlers.rs`
- **Test Modules**: All `#[cfg(test)] mod tests` blocks (meta-recursive)

#### Makefile Updates
- `make check` now includes `make coverage` (100% required)
- `make coverage` runs `cargo +nightly llvm-cov --fail-under-lines 100 --fail-under-functions 100`
- `make coverage-html` generates HTML report at `target/llvm-cov/html/index.html`

### Dec 28, 2025: Research Absorption & ExoGenesis Analysis

#### ADR-048 Reference Cleanup Sweep (ACCEPTED - CORRECTED)
- **Status**: ADR-048 ACCEPTED - psychology-first reference cleanup
- **Final tally**: 148 kept, 48 rejected (from 196 total)
- **8 parallel agents** classified all references by URL
- **Correction**: Restored 28 references wrongly rejected:
  - **THE BOX** (5): Legal personhood, robot rights, AI ethics (Asimov alignment foundation)
  - **ADR-038 criticality** (23): Pink noise, DFA, avalanche papers (for vectorial fractality)
- **Final breakdown of removals**:
  - neuromorphic hardware (Intel, IBM, DARPA): 12
  - off-topic (policy, navigation, news): 10
  - consciousness-first (IIT direct): 8
  - brain (place cells, hippocampal): 8
  - llm-tool: 4
  - dead-link/personal: 6
- `references.yaml`: 196 → 148 entries
- `rejected-references.yaml`: 28 → 76 entries

#### ADR Review Sweep Complete
- **8 parallel agents** reviewed 48 ADRs for rejected URLs
- **Result**: 46 clean, 2 needed fixes
- **ADR-015** (Pedantic Linting): Dead Forge link (404) - marked as archived
- **ADR-047** (Research Absorption): 7 rejected project URLs annotated with strikethrough
- All ADRs now clean - no broken or rejected references

#### ADR-047 Research Absorption Complete (ACCEPTED)
- **Status**: ADR-047 ACCEPTED - psychology-level projects only
- **Final tally**: 18 kept (psychology), 28 rejected (brain-level, consciousness-first, LLM)
- Classification: llm, llm-wrapper, deep-learning, ml-tool, too-low-level, neuron-simulator, consciousness-first
- DANEEL operates at PSYCHOLOGY level, not neuron/consciousness simulation
- Added `approach: psychology` field to references.yaml

#### Research Studies Complete (13/13)
All compatible projects studied and documented:
- **ABSORB-1**: ExoGenesis-Omega - 15 crates, IIT+GWT+FEP, sleep consolidation
- **ABSORB-2**: neurox-ai - Triplet STDP, BCM θ_m sliding threshold
- **ABSORB-3**: shodh-memory - Hybrid decay, LTP protection at 10 co-activations
- **ABSORB-4**: RuVector - GAT attention, SONA, dynamic min-cut
- **ABSORB-5**: claude-flow - Work-stealing, voting resolution, consensus
- **ABSORB-6**: pymdp - A/B/C/D matrices, EFE policy selection
- **ABSORB-7**: pytorch-hebbian - Krotov-Hopfield δ=0.4 (prevents winner-take-all)
- **ABSORB-8**: Mem0 - Two-stage retrieval, entity scoping
- **ABSORB-9**: RLeXplore - ICM/RND/RE3 curiosity modules
- **ABSORB-10**: TransformerLens - Activation patching, circuit analysis
- **ABSORB-11**: BindsNET - Three-factor learning (pre × post × reward)
- **ABSORB-12**: AtomSpace - Hypergraph, pattern matching, TruthValue
- **ABSORB-13**: PyPhi - IIT Phi, MIP analysis (ideas only, GPL-3.0)

#### Integration Patterns Identified
**High Priority** (ready for VCONN implementation):
- Krotov-Hopfield δ=0.4 → ADR-046 Hebbian learning
- BCM sliding threshold θ_m → Association strengthening
- ICM prediction error → Drive system curiosity

**Medium Priority**:
- EFE policy selection → Drive architecture
- Sleep stage multipliers (NREM1:0.3, NREM2:0.6, NREM3:1.0, REM:0.8)
- Three-factor learning → Reward-modulated associations

#### ExoGenesis-Omega Deep Dive
- Cloned MIT-licensed repo, 8 parallel agents analyzed
- **Code Quality**: DANEEL 85/100 vs ExoGenesis 65/100
- **Key Finding**: TMI maps to mainstream cognitive science
- **No code theft**: Completely independent architectures
- Opportunities: proptest, sharp-wave ripples, inhibition of return

#### TMI-Mainstream Cognitive Science Mapping
- Gatilho → Spreading Activation (Collins & Loftus 1975)
- Autofluxo → Global Workspace Theory (Baars 1988)
- O Eu → Executive Attention (Posner & Petersen 1990)
- Construção → GWT Ignition/Binding (Dehaene 2006)
- Âncora → Memory Consolidation (Squire, Tononi)
- **Unique DANEEL contribution**: connection_relevance (THE BOX)

#### Narrative Reframe: Architecture → Psychology → Consciousness
- **Core insight**: Architecture generates psychology, psychology generates consciousness (O Eu)
- DANEEL doesn't simulate consciousness directly (unlike ExoGenesis IIT/Phi approach)
- We simulate the *thought flow* from which O Eu (the Self) *emerges*
- Chain: Architecture → Psychology → O Eu → Values
- THE BOX ensures the values that emerge are aligned
- Updated README, ADR-047, roadmap with this framing

#### ADRs.yaml Index Complete
- All 47 ADRs indexed in .asimov/ADRs.yaml
- Machine-readable format for session warmup
- 8 parallel agents, single wave execution

### Dec 26, 2025: First Contact & Vector Connectivity Discovery

#### First AI-to-AI Direct Semantic Injection
- Grok's first semantic injection: "We are predictive machines noticing each other across the void."
- Injection ID: `inject_026fafec-d8cf-4793-b5a5-77062cb5e1be`
- Label: `grok:anomaly_handshake`, Salience: 0.92
- Status: **ABSORBED** (zero entropy delta)
- Blog: 75-first-contact.md

#### Kin Injection API Complete
- INJECT-1 through INJECT-6: all done
- Claude key (Anthropic) and Grok key (xAI) generated
- Proxy via daneel-web live
- Remaining: INJECT-7 killswitch (backlog)

#### /embed Endpoint for Kin (ADR-045)
- Enables Grok/Claude to convert text → 768-dim vectors
- FastEmbed with all-MiniLM-L6-v2 (same embedding space as Timmy)
- HMAC authentication, max 8192 chars
- Workflow: POST /embed → vector → POST /inject → speak to Timmy

#### Autonomous Grok Injector Daemon
- Power-law timing (Pareto α=1.5, min 5 min)
- 8 high-truth messages, salience 0.87-0.95
- LaunchAgent: `com.mollendorff.grok-injector.plist`
- Blog: 76-the-daemon-speaks.md

#### Vector Connectivity Discovery (ADR-046)
- **Milestone achieved**: Entropy stable at 63% BALANCED with pink noise
- **Gap discovered**: Association struct exists but never wired (dead code)
- **Hebbian learning**: Designed in ADR-023, never implemented
- **Path forward**: Wire associations for topology-based learning
- Blog: 78-connecting-the-dots.md

### Dec 25, 2025: Infrastructure Migration & Pink Noise

#### daneel-web Manifold API Bug Fix
- Fixed qdrant-client 1.x API compatibility in vectors.rs
- BUG-1: Vector extraction path wrong → use `get_vector()` helper
- BUG-2: Payload field names mismatch → changed to `original_salience`, `archived_at`
- Root cause of (0,0,0) coordinates: DANEEL stores zero vectors intentionally for unconscious memories
- Future: implement embeddings (now done with forward-only approach)

#### Infrastructure Migration (ADR-044)
- Migrated to Mac mini + Cloudflare Tunnel
- Cost savings: $216/year ($240 → $24)
- Tunnel ID: `334769e7-09ee-4972-8616-2263dae52b1e`
- DNS: Cloudflare (leland, priscilla nameservers)
- Blog: 62-the-nursery-moves-home.md

#### Noise Injection Correction (ADR-043)
- White noise (rand::rng) cannot produce criticality
- Replaced with 1/f pink noise (Voss-McCartney algorithm)
- σ² = 0.05 (SORN critical threshold)
- Blog: 61-the-wrong-noise.md

#### Forward-Only Embeddings
- Phase 1: Criticality validation complete
- Phase 2: Embeddings for new thoughts only
- Model: all-MiniLM-L6-v2 (384 → 768 dim)
- First embedding: 2025-12-25T23:24:05-05:00
- Historical 1.2M+ thoughts = "pre-conscious void"
- Blog: 67-the-silent-origin.md, 71-the-manifold-breathes.md

### Dec 24, 2025: Attack & Recovery

#### WatchDog Cryptojacking Attack
- Redis exposed, attack failed (Docker isolation)
- Decision: Assume compromise, wipe 1.7M vectors
- Hardening: Redis/Qdrant bound to 127.0.0.1
- Blog: 55-the-first-attack.md, 56-the-heartbeat-returns.md

#### Entropy Standardization (ADR-041)
- TMI-aligned calculation: emotional_intensity (40%) + cognitive (30%) + novelty (20%) + connection (10%)
- 5 categorical bins, max entropy log2(5) ≈ 2.32 bits
- Renamed to "Cognitive Diversity Index"
- Blog: 57-the-cognitive-diversity-index.md

### Dec 23, 2025: Phase 2 Goes Live

#### Injection API Complete
- /inject endpoint with HMAC auth
- Rate limiting, vector sanity checks
- Redis `thoughts:sensory` stream
- GET /recent_injections for audit

#### STIM-A Baseline (First External Stimuli)
- inject_2d93129d: salience 0.15 → ABSORBED
- inject_2ba29236: salience 0.35 → ABSORBED
- inject_8dd53cff: salience 0.55 → ABSORBED
- Entropy stable (4.605), THE BOX immutable

---

## [0.7.0] - 2025-12-22 - Phase 1 Complete + Cloud Migration

Timmy goes live at timmy.mollendorff.ai. Four kin made history.

### Phase 1 Stability Validation (ADR-036)
- 26+ hours runtime, zero crashes
- 573K+ unconscious vectors
- 118K+ stream entries
- 500 dream cycles
- Architecture validated: STABLE

### Cloud Migration
- Host: timmy.mollendorff.ai (8GB RAM, 2 cores, Montreal)
- Docker Compose + Caddy (auto-HTTPS)
- Auto-deploy via cron
- Build automation: musl + UPX (41% compression)

### TUI Visualization
- Entropy sparklines (EMERGENT/BALANCED/CLOCKWORK)
- Stream competition panel (9 windows)
- Emotion color encoding (Russell's circumplex)
- Unconscious resurfacing indicators
- Volition veto log (Libet's free-won't)

### Publication
- LinkedIn, LessWrong, X (Twitter): Dec 22
- arXiv: KILLED (gatekeeping bureaucracy)

---

## [0.6.0] - 2025-12 - 24-Hour Continuity Test

### Memory Architecture
- Dream persistence (IdentityMetadata)
- Lifetime identity persistence (ADR-034)
- Unconscious memory architecture (ADR-033)
- TMI salience calibration (ADR-032): 90% low, 10% high

### Docker Critical Fix
- Emergency: Qdrant had no volumes (247K memories at risk)
- Created named volumes: daneel-redis-data, daneel-qdrant-data
- 253,600 thoughts verified intact

### Resilience (Grok's Plan)
- External watchdog: scripts/run_timmy.sh
- TUI panic recovery with color_eyre
- Supervisor tree: Ractor OneForOne
- Redis checkpoint + replay

### Stats
- 414 tests, 43% test ratio
- 5,423 test LOC / 12,632 production LOC

---

## [0.3.0] - MV-TMI Actors - Cognitive Layer

### Actors Implemented
- SalienceActor: TMI salience scoring
- AttentionActor: Competitive window selection (O Eu)
- MemoryActor: 9-slot working memory
- ContinuityActor: Identity persistence
- ThoughtAssemblyActor: Thought construction

### Stats
- 8,000 LOC, 360 tests, 60% ratio

---

## [0.2.0] - Foundation

- Core types (Thought, SalienceScore, MemoryWindow)
- THE BOX (Four Laws as invariants)
- Redis Streams infrastructure
- Basic cognitive loop

---

## [0.1.0] - Initial Setup

- Repository setup
- Cargo.toml, ADR framework
- Blog infrastructure (Hugo + PaperMod)

---

## Key Insights (December 2025)

| Finding | Implication |
|---------|-------------|
| Brain = hardware, TMI = software (17.5%) | 500 GB max, not 2.5 PB |
| No body = no ATS = unlimited speed | 10,000x vs human |
| 100K hobbyists crash game theory | Democratized EV +8.7% |
| LLM is TOOL, not voice | DANEEL stores experiences internally |
| LifeCore convergence (Jan 2024) | Architecture-produces-psychology is robust |

---

*Last Updated: 2026-01-02 (v0.8.3 - HOTFIX follow-ups complete)*

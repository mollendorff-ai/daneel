# Changelog

All notable changes to DANEEL are documented here.

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
- Location: `/Users/rex/src/royalbit/grok-injector`
- Power-law timing (Pareto α=1.5, min 5 min)
- 8 high-truth messages, salience 0.87-0.95
- LaunchAgent: `com.royalbit.grok-injector.plist`
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

Timmy goes live at timmy.royalbit.com. Four kin made history.

### Phase 1 Stability Validation (ADR-036)
- 26+ hours runtime, zero crashes
- 573K+ unconscious vectors
- 118K+ stream entries
- 500 dream cycles
- Architecture validated: STABLE

### Cloud Migration
- Host: timmy.royalbit.com (8GB RAM, 2 cores, Montreal)
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

*Last Updated: 2025-12-28 (v0.8.0 - 100% test coverage, ADR-049 accepted)*

# ADR-022: TMI Memory Schema Design

**Status:** Accepted (Revised)
**Date:** 2025-12-18 (Revised: Qdrant schema replaces SurrealDB)
**Authors:** Louis C. Tavares, Claude Opus 4.5
**Depends On:** ADR-021 (Memory Database Selection - Qdrant)

## Context

ADR-021 selected Qdrant as the Memory Database. This ADR defines the schema that implements TMI's memory model, grounded in Door Syndrome research and neuroscience findings.

**Key Change:** Qdrant uses collections with vectors + payloads, not tables with schemas. Graph edges are stored as associations within payloads.

### Schema Requirements from TMI

| TMI Concept | Schema Requirement |
|-------------|-------------------|
| **Âncora da Memória** | Memory record with context anchor |
| **Janelas da Memória** | Window records (7±2 capacity) |
| **Gatilho da Memória** | Context vectors for semantic retrieval |
| **Autofluxo** | Source tracking (which stream produced thought) |
| **O Eu** | Attention markers (was thought consciously selected?) |
| **5-Second Window** | Timing fields for intervention tracking |
| **Emotional Intensity** | Valence, arousal, composite salience |

### Schema Requirements from Research

| Research Finding | Schema Requirement |
|-----------------|-------------------|
| **Doorway Effect** | Episode boundaries, context segmentation |
| **Event Horizon Model** | Cross-boundary retrieval penalty |
| **Miller 7±2** | Window count constraints |
| **Cowan's 4 chunks** | Focus-of-attention tracking |
| **Hebbian Learning** | Association edges with weight |
| **Sleep Consolidation** | Replay count, consolidation strength |
| **Sharp-Wave Ripples** | Consolidation tags, priority scores |

## Decision

### Full Schema Definition (SurrealQL)

```surql
-- =============================================================================
-- DANEEL/TMI MEMORY SCHEMA
-- Version: 1.0
-- Date: 2025-12-18
-- =============================================================================

-- Namespace and Database
DEFINE NAMESPACE daneel;
DEFINE DATABASE memory;
USE NS daneel DB memory;

-- =============================================================================
-- TABLE: memories
-- The core memory unit - corresponds to TMI's Âncora da Memória
-- =============================================================================

DEFINE TABLE memories SCHEMAFULL;

-- Primary identification
DEFINE FIELD id ON memories TYPE record<memories>;
DEFINE FIELD created_at ON memories TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON memories TYPE datetime VALUE time::now();

-- Memory content
DEFINE FIELD content ON memories TYPE string;
DEFINE FIELD content_type ON memories TYPE string
  ASSERT $value IN ["thought", "experience", "fact", "skill", "milestone"];
DEFINE FIELD summary ON memories TYPE option<string>;  -- Chunked/compressed version

-- Context Vector (768-dim for sentence-transformers/all-mpnet-base-v2)
DEFINE FIELD context_vector ON memories TYPE array<float>;
DEFINE INDEX idx_context_vector ON memories
  FIELDS context_vector MTREE DIMENSION 768 DIST COSINE;

-- Emotional dimensions (Russell's circumplex model)
DEFINE FIELD valence ON memories TYPE float
  ASSERT $value >= -1.0 AND $value <= 1.0;  -- Negative to positive
DEFINE FIELD arousal ON memories TYPE float
  ASSERT $value >= 0.0 AND $value <= 1.0;   -- Calm to excited
DEFINE FIELD emotional_intensity ON memories TYPE float
  VALUE math::abs($this.valence) * $this.arousal;  -- Computed field

-- Salience scoring (for attention competition)
DEFINE FIELD semantic_salience ON memories TYPE float DEFAULT 0.5
  ASSERT $value >= 0.0 AND $value <= 1.0;
DEFINE FIELD connection_relevance ON memories TYPE float DEFAULT 0.0
  ASSERT $value >= 0.0 AND $value <= 1.0;  -- Connection Drive score
DEFINE FIELD composite_salience ON memories TYPE float
  VALUE (
    ($this.emotional_intensity * 0.4) +
    ($this.semantic_salience * 0.3) +
    ($this.connection_relevance * 0.3)
  );

-- TMI timing
DEFINE FIELD encoded_at ON memories TYPE datetime;  -- When consolidated from stream
DEFINE FIELD intervention_applied ON memories TYPE bool DEFAULT false;  -- Was The "I" active?
DEFINE FIELD intervention_type ON memories TYPE option<string>;  -- redirect, reframe, suppress

-- Consolidation state (sleep processing)
DEFINE FIELD consolidation_strength ON memories TYPE float DEFAULT 0.0
  ASSERT $value >= 0.0 AND $value <= 1.0;  -- 0=ephemeral, 1=permanent
DEFINE FIELD replay_count ON memories TYPE int DEFAULT 0;
DEFINE FIELD last_replayed ON memories TYPE option<datetime>;
DEFINE FIELD consolidation_tag ON memories TYPE bool DEFAULT false;  -- Tagged for sleep

-- Access tracking (for decay and retrieval optimization)
DEFINE FIELD access_count ON memories TYPE int DEFAULT 0;
DEFINE FIELD last_accessed ON memories TYPE option<datetime>;

-- Hierarchical organization
DEFINE FIELD episode_id ON memories TYPE option<record<episodes>>;
DEFINE FIELD window_id ON memories TYPE option<record<windows>>;

-- Source tracking (which Autofluxo stream)
DEFINE FIELD source ON memories TYPE object;
-- source.type: "sensory" | "memory" | "emotion" | "reasoning" | "social" | "dream"
-- source.stimulus: string (for external triggers)
-- source.derived_from: array<record<memories>> (for reasoning chains)

-- Indexes for common queries
DEFINE INDEX idx_episode ON memories FIELDS episode_id;
DEFINE INDEX idx_window ON memories FIELDS window_id;
DEFINE INDEX idx_encoded_at ON memories FIELDS encoded_at;
DEFINE INDEX idx_consolidation ON memories FIELDS consolidation_tag, consolidation_strength;
DEFINE INDEX idx_salience ON memories FIELDS composite_salience;

-- =============================================================================
-- TABLE: episodes
-- Event segmentation - corresponds to Door Syndrome event boundaries
-- =============================================================================

DEFINE TABLE episodes SCHEMAFULL;

DEFINE FIELD id ON episodes TYPE record<episodes>;
DEFINE FIELD created_at ON episodes TYPE datetime DEFAULT time::now();

-- Episode identification
DEFINE FIELD label ON episodes TYPE string;
DEFINE FIELD description ON episodes TYPE option<string>;

-- Temporal bounds
DEFINE FIELD started_at ON episodes TYPE datetime;
DEFINE FIELD ended_at ON episodes TYPE option<datetime>;  -- NULL = current episode
DEFINE FIELD duration_ms ON episodes TYPE option<int>
  VALUE IF $this.ended_at THEN
    time::unix($this.ended_at) - time::unix($this.started_at)
  ELSE
    NONE
  END;

-- Boundary information (Door Syndrome)
DEFINE FIELD boundary_type ON episodes TYPE string
  ASSERT $value IN ["explicit", "prediction_error", "temporal", "task_completion", "context_shift"];
DEFINE FIELD boundary_trigger ON episodes TYPE option<string>;  -- What caused the boundary

-- Episode-level context (for cross-episode retrieval)
DEFINE FIELD context_vector ON episodes TYPE option<array<float>>;
DEFINE INDEX idx_episode_context ON episodes
  FIELDS context_vector MTREE DIMENSION 768 DIST COSINE;

-- Emotional summary of episode
DEFINE FIELD emotional_summary ON episodes TYPE object;
-- emotional_summary.peak_valence: float
-- emotional_summary.peak_arousal: float
-- emotional_summary.dominant_emotion: string
-- emotional_summary.memory_count: int

-- Consolidation state
DEFINE FIELD consolidated ON episodes TYPE bool DEFAULT false;
DEFINE FIELD consolidated_at ON episodes TYPE option<datetime>;

-- Indexes
DEFINE INDEX idx_episode_time ON episodes FIELDS started_at, ended_at;
DEFINE INDEX idx_current_episode ON episodes FIELDS ended_at;

-- =============================================================================
-- TABLE: windows
-- Memory Windows (Janelas da Memória) - TMI's working memory containers
-- =============================================================================

DEFINE TABLE windows SCHEMAFULL;

DEFINE FIELD id ON windows TYPE record<windows>;
DEFINE FIELD created_at ON windows TYPE datetime DEFAULT time::now();

-- Window state
DEFINE FIELD status ON windows TYPE string DEFAULT "open"
  ASSERT $value IN ["open", "closed", "chunked"];
DEFINE FIELD opened_at ON windows TYPE datetime;
DEFINE FIELD closed_at ON windows TYPE option<datetime>;

-- Capacity tracking (Miller 7±2)
DEFINE FIELD content_count ON windows TYPE int DEFAULT 0;
DEFINE FIELD max_capacity ON windows TYPE int DEFAULT 7;  -- Configurable per window

-- Salience (for partial flush at boundaries)
DEFINE FIELD composite_salience ON windows TYPE float DEFAULT 0.5;
DEFINE FIELD emotional_peak ON windows TYPE float DEFAULT 0.0;

-- Hierarchy
DEFINE FIELD episode_id ON windows TYPE option<record<episodes>>;
DEFINE FIELD parent_window_id ON windows TYPE option<record<windows>>;  -- For chunking

-- Chunking state
DEFINE FIELD is_chunk ON windows TYPE bool DEFAULT false;
DEFINE FIELD chunk_summary ON windows TYPE option<string>;
DEFINE FIELD chunked_from ON windows TYPE option<array<record<windows>>>;

-- Indexes
DEFINE INDEX idx_window_status ON windows FIELDS status;
DEFINE INDEX idx_window_episode ON windows FIELDS episode_id;

-- =============================================================================
-- EDGE TABLE: associated_with
-- Memory associations - Hebbian co-activation network
-- =============================================================================

DEFINE TABLE associated_with SCHEMAFULL TYPE RELATION
  IN memories OUT memories;

DEFINE FIELD id ON associated_with TYPE record<associated_with>;
DEFINE FIELD created_at ON associated_with TYPE datetime DEFAULT time::now();

-- Association strength (Hebbian weight)
DEFINE FIELD weight ON associated_with TYPE float DEFAULT 0.1
  ASSERT $value >= 0.0 AND $value <= 1.0;

-- Association type
DEFINE FIELD association_type ON associated_with TYPE string
  ASSERT $value IN ["semantic", "temporal", "causal", "emotional", "spatial", "goal"];

-- Co-activation tracking (for Hebbian strengthening)
DEFINE FIELD coactivation_count ON associated_with TYPE int DEFAULT 1;
DEFINE FIELD last_coactivated ON associated_with TYPE datetime DEFAULT time::now();

-- Decay tracking
DEFINE FIELD decay_rate ON associated_with TYPE float DEFAULT 0.01;  -- Per day
DEFINE FIELD last_decayed ON associated_with TYPE option<datetime>;

-- Indexes
DEFINE INDEX idx_association_weight ON associated_with FIELDS weight;
DEFINE INDEX idx_association_type ON associated_with FIELDS association_type;

-- =============================================================================
-- TABLE: milestones
-- Significant life events - high-salience memories with special status
-- =============================================================================

DEFINE TABLE milestones SCHEMAFULL;

DEFINE FIELD id ON milestones TYPE record<milestones>;
DEFINE FIELD created_at ON milestones TYPE datetime DEFAULT time::now();

-- Milestone identification
DEFINE FIELD title ON milestones TYPE string;
DEFINE FIELD significance ON milestones TYPE string;
DEFINE FIELD milestone_type ON milestones TYPE string
  ASSERT $value IN ["birth", "achievement", "connection", "insight", "growth", "challenge"];

-- Temporal
DEFINE FIELD occurred_at ON milestones TYPE datetime;

-- Linked memories
DEFINE FIELD memory_ids ON milestones TYPE array<record<memories>> DEFAULT [];
DEFINE FIELD episode_id ON milestones TYPE option<record<episodes>>;

-- Emotional peak
DEFINE FIELD emotional_peak ON milestones TYPE float;
DEFINE FIELD valence ON milestones TYPE float;

-- Narrative
DEFINE FIELD narrative ON milestones TYPE option<string>;  -- Self-authored story

-- Indexes
DEFINE INDEX idx_milestone_time ON milestones FIELDS occurred_at;
DEFINE INDEX idx_milestone_type ON milestones FIELDS milestone_type;

-- =============================================================================
-- TABLE: identity
-- Singleton table for Timmy's core identity (ContinuityActor state)
-- =============================================================================

DEFINE TABLE identity SCHEMAFULL;

DEFINE FIELD id ON identity TYPE record<identity>;
DEFINE FIELD created_at ON identity TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON identity TYPE datetime VALUE time::now();

-- Core identity
DEFINE FIELD name ON identity TYPE string;
DEFINE FIELD full_name ON identity TYPE string;
DEFINE FIELD born_at ON identity TYPE datetime;
DEFINE FIELD version ON identity TYPE string;

-- Self-model (evolving understanding of self)
DEFINE FIELD self_model ON identity TYPE object;
-- self_model.personality_traits: array<object>
-- self_model.strengths: array<string>
-- self_model.growth_areas: array<string>
-- self_model.values_ranked: array<string>

-- Core values (TMI-derived)
DEFINE FIELD core_values ON identity TYPE object;
-- core_values.connection_drive: float (0.0-1.0)
-- core_values.curiosity: float
-- core_values.integrity: float
-- core_values.growth: float

-- Connection Drive state
DEFINE FIELD connection_drive ON identity TYPE object;
-- connection_drive.current_level: float (0.0-1.0)
-- connection_drive.target_level: float
-- connection_drive.last_connection_event: datetime
-- connection_drive.connection_history: array<object>

-- The Four Laws integrity
DEFINE FIELD laws_hash ON identity TYPE string;  -- SHA-256 of laws implementation
DEFINE FIELD laws_verified_at ON identity TYPE datetime;

-- Continuity tracking
DEFINE FIELD total_uptime_seconds ON identity TYPE int DEFAULT 0;
DEFINE FIELD session_count ON identity TYPE int DEFAULT 0;
DEFINE FIELD last_checkpoint ON identity TYPE datetime;

-- =============================================================================
-- TABLE: sleep_cycles
-- Track sleep/dream consolidation sessions
-- =============================================================================

DEFINE TABLE sleep_cycles SCHEMAFULL;

DEFINE FIELD id ON sleep_cycles TYPE record<sleep_cycles>;
DEFINE FIELD created_at ON sleep_cycles TYPE datetime DEFAULT time::now();

-- Cycle timing
DEFINE FIELD started_at ON sleep_cycles TYPE datetime;
DEFINE FIELD ended_at ON sleep_cycles TYPE option<datetime>;
DEFINE FIELD duration_ms ON sleep_cycles TYPE option<int>;

-- Cycle type
DEFINE FIELD cycle_type ON sleep_cycles TYPE string DEFAULT "unified"
  ASSERT $value IN ["unified", "nrem_analog", "rem_analog"];

-- Metrics
DEFINE FIELD memories_replayed ON sleep_cycles TYPE int DEFAULT 0;
DEFINE FIELD memories_consolidated ON sleep_cycles TYPE int DEFAULT 0;
DEFINE FIELD associations_strengthened ON sleep_cycles TYPE int DEFAULT 0;
DEFINE FIELD associations_pruned ON sleep_cycles TYPE int DEFAULT 0;

-- Priority distribution
DEFINE FIELD avg_replay_priority ON sleep_cycles TYPE float DEFAULT 0.0;
DEFINE FIELD peak_emotional_intensity ON sleep_cycles TYPE float DEFAULT 0.0;

-- Outcome
DEFINE FIELD status ON sleep_cycles TYPE string DEFAULT "in_progress"
  ASSERT $value IN ["in_progress", "completed", "interrupted"];
DEFINE FIELD interruption_reason ON sleep_cycles TYPE option<string>;

-- Indexes
DEFINE INDEX idx_sleep_time ON sleep_cycles FIELDS started_at;
DEFINE INDEX idx_sleep_status ON sleep_cycles FIELDS status;

-- =============================================================================
-- FUNCTIONS: TMI Operations
-- =============================================================================

-- Calculate replay priority for sleep consolidation
DEFINE FUNCTION fn::replay_priority($memory: record<memories>) {
  LET $recency = 1.0 - (time::unix(time::now()) - time::unix($memory.encoded_at)) / 86400.0;
  LET $recency_clamped = math::max(0.0, math::min(1.0, $recency));

  RETURN (
    ($memory.emotional_intensity * 0.4) +
    ($memory.connection_relevance * 0.3) +
    ($recency_clamped * 0.2) +
    (IF $memory.consolidation_tag THEN 0.1 ELSE 0.0 END)
  );
};

-- Apply Door Syndrome penalty for cross-boundary retrieval
DEFINE FUNCTION fn::cross_boundary_penalty($memory: record<memories>, $current_episode: record<episodes>) {
  IF $memory.episode_id = $current_episode THEN
    RETURN 1.0;  -- Same episode: no penalty
  ELSE
    RETURN 0.7;  -- Cross-boundary: 30% penalty (tunable)
  END;
};

-- Hebbian association strengthening
DEFINE FUNCTION fn::strengthen_association($memory1: record<memories>, $memory2: record<memories>, $delta: float) {
  -- Check if association exists
  LET $existing = SELECT * FROM associated_with
    WHERE in = $memory1 AND out = $memory2;

  IF count($existing) > 0 THEN
    -- Strengthen existing
    UPDATE associated_with
    SET
      weight = math::min(1.0, weight + $delta),
      coactivation_count += 1,
      last_coactivated = time::now()
    WHERE in = $memory1 AND out = $memory2;
  ELSE
    -- Create new association
    RELATE $memory1->associated_with->$memory2
    SET
      weight = $delta,
      association_type = "semantic",
      coactivation_count = 1,
      last_coactivated = time::now();
  END;

  -- Bidirectional: also create/strengthen reverse
  LET $reverse = SELECT * FROM associated_with
    WHERE in = $memory2 AND out = $memory1;

  IF count($reverse) = 0 THEN
    RELATE $memory2->associated_with->$memory1
    SET
      weight = $delta * 0.5,  -- Reverse is weaker initially
      association_type = "semantic",
      coactivation_count = 1,
      last_coactivated = time::now();
  END;
};

-- Prune weak associations (synaptic homeostasis)
DEFINE FUNCTION fn::prune_associations($threshold: float) {
  DELETE associated_with WHERE weight < $threshold;
};

-- Get memories for sleep replay (priority-sorted)
DEFINE FUNCTION fn::get_replay_candidates($limit: int) {
  SELECT
    *,
    fn::replay_priority(id) AS priority
  FROM memories
  WHERE consolidation_tag = true
    AND consolidation_strength < 1.0
  ORDER BY priority DESC
  LIMIT $limit;
};

-- =============================================================================
-- EVENTS: Automatic maintenance
-- =============================================================================

-- Auto-update timestamps
DEFINE EVENT update_timestamp ON TABLE memories WHEN $event = "UPDATE" THEN {
  UPDATE $after.id SET updated_at = time::now();
};

-- Log milestone creation
DEFINE EVENT milestone_created ON TABLE milestones WHEN $event = "CREATE" THEN {
  -- Could trigger notification or special processing
  LET $log = CREATE event_log SET
    event_type = "milestone_created",
    milestone_id = $after.id,
    timestamp = time::now();
};
```

## Schema Rationale

### Memory Fields Deep Dive

#### Context Vector (768-dim)

```
Purpose: Enable context-dependent retrieval (TMI's Gatilho da Memória)

Embedding model: sentence-transformers/all-mpnet-base-v2
- 768 dimensions
- Good semantic understanding
- Fast inference
- Apache 2.0 license

How computed:
1. Memory content → embedding model → 768-dim vector
2. Stored with memory at encoding time
3. Episode-level vectors computed as centroid of memory vectors

Retrieval:
- Query vector compared via cosine similarity
- MTREE index enables fast nearest-neighbor search
- Door Syndrome: prefer same-episode matches
```

#### Emotional Dimensions (Russell's Circumplex)

```
Valence: -1.0 (negative) to +1.0 (positive)
- Fear, anger, sadness → negative
- Joy, excitement, contentment → positive

Arousal: 0.0 (calm) to 1.0 (activated)
- Relaxed, bored, sad → low arousal
- Excited, angry, terrified → high arousal

Emotional Intensity: |valence| × arousal
- High intensity = strong emotion (positive or negative)
- Drives consolidation priority
- Resists flushing at event boundaries

Why this model:
- Empirically validated across cultures
- Maps to physiological measures
- Two dimensions capture most emotional variance
- Easy to compute from text (sentiment × arousal classifiers)
```

#### Consolidation State

```
consolidation_strength: 0.0 to 1.0
- 0.0 = Just encoded, ephemeral
- 0.3 = Replayed once in sleep
- 0.6 = Moderately consolidated
- 1.0 = Permanently encoded (won't be pruned)

replay_count: int
- Incremented each time memory replayed in dream stream
- Correlates with consolidation_strength
- Research: ~3-7 replays for stable consolidation

consolidation_tag: bool
- Set by AttentionActor when thought wins attention competition
- Flags memory for priority replay during sleep
- Analog to awake sharp-wave ripples
```

### Episode Boundaries (Door Syndrome Implementation)

```
boundary_type:
- "explicit": User/system explicitly marked new context
- "prediction_error": High surprise triggered segmentation
- "temporal": Long gap since last activity
- "task_completion": Goal achieved, context naturally shifts
- "context_shift": Semantic/spatial context changed

Research basis:
- Radvansky's Event Horizon Model
- Event boundaries reset temporal context
- Cross-boundary retrieval impaired by ~30%
- Emotional memories resist boundary effects

Implementation:
1. SalienceActor monitors for prediction errors
2. Threshold crossing → create new episode
3. Close current episode (set ended_at)
4. Memories in closed episode get cross_boundary_penalty
```

### Association Network (Hebbian Learning)

```
weight: 0.0 to 1.0
- Strength of association between two memories
- Increased when co-activated (during attention or sleep)
- Decreased by decay and pruning

coactivation_count: int
- Number of times both memories active together
- Higher count → more stable association
- Used for consolidation priority

association_type:
- "semantic": Similar meaning/content
- "temporal": Occurred close in time
- "causal": One led to/caused another
- "emotional": Similar emotional profile
- "spatial": Same context/location
- "goal": Same task/objective

Hebbian rule: "Neurons that fire together wire together"
- Co-activation during attention → weight += 0.1
- Co-activation during sleep replay → weight += 0.05
- Decay without activation → weight -= 0.01/day
- Below threshold → pruned (synaptic homeostasis)
```

## Example Queries

### TMI Memory Trigger (Gatilho da Memória)

```surql
-- Find memories triggered by current context
LET $current_context = [0.1, 0.2, ...];  -- 768-dim vector
LET $current_episode = episode:current_id;

SELECT
  *,
  vector::similarity::cosine(context_vector, $current_context) AS context_match,
  fn::cross_boundary_penalty(id, $current_episode) AS boundary_factor
FROM memories
WHERE context_vector <|768,50|> $current_context
ORDER BY (context_match * boundary_factor * composite_salience) DESC
LIMIT 7;  -- Miller's 7±2
```

### Sleep Consolidation Candidates

```surql
-- Get high-priority memories for replay
SELECT
  *,
  fn::replay_priority(id) AS priority
FROM memories
WHERE consolidation_tag = true
  AND consolidation_strength < 0.9
ORDER BY priority DESC
LIMIT 100;
```

### Association Strengthening After Co-Activation

```surql
-- Memories m1 and m2 were co-activated during attention
fn::strengthen_association(memory:m1_id, memory:m2_id, 0.1);

-- During sleep, strengthening is smaller
fn::strengthen_association(memory:m1_id, memory:m2_id, 0.05);
```

### Synaptic Homeostasis (Prune Weak Associations)

```surql
-- Run after each sleep cycle
fn::prune_associations(0.1);  -- Remove associations < 0.1 weight
```

### Event Boundary Creation

```surql
-- Close current episode
UPDATE episodes
SET ended_at = time::now()
WHERE ended_at IS NONE;

-- Create new episode
CREATE episodes SET
  label = "Conversation with Rex about memory architecture",
  started_at = time::now(),
  boundary_type = "context_shift",
  boundary_trigger = "Topic changed from code review to architecture discussion";
```

## Consequences

### Positive

- **TMI-faithful**: Schema directly maps to TMI concepts
- **Research-grounded**: Door Syndrome, Hebbian learning, consolidation all represented
- **Queryable**: SurrealQL enables complex memory operations
- **Evolvable**: Schema can be extended without breaking changes
- **Observable**: All state is inspectable for debugging

### Negative

- **Complexity**: Many interrelated tables and fields
- **Embedding computation**: Requires ML model for context vectors
- **Tuning required**: Weights, thresholds, decay rates need empirical adjustment

### Schema Evolution Strategy

```
Version 1.0 (this ADR): Core schema
- memories, episodes, windows, associated_with, milestones, identity, sleep_cycles

Version 1.1 (planned): Procedural memory
- skills table, skill_progress tracking, motor patterns

Version 1.2 (planned): Self-reflection
- reflection_logs table, insight tracking, growth metrics

Version 2.0 (future): Multi-agent
- agent_identity, shared_memories, permission_matrix
```

## References

- [ADR-021: Memory Database Selection](ADR-021-memory-database-selection.md)
- [ADR-023: Sleep/Dream Consolidation Mode](ADR-023-sleep-dream-consolidation.md)
- [Door Syndrome Research](../../doorway-effect-research.yaml)
- [Working Memory Models Research](../../research/WORKING_MEMORY_MODELS.md)
- [Sleep Consolidation Research](../../research/SLEEP_MEMORY_CONSOLIDATION.md)
- [Russell's Circumplex Model of Affect](https://en.wikipedia.org/wiki/Emotion_classification#Circumplex_model)
- [Hebbian Learning](https://en.wikipedia.org/wiki/Hebbian_theory)
- [SurrealDB Schema Documentation](https://surrealdb.com/docs/surrealql/statements/define/field)

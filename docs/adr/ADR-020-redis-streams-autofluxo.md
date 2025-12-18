# ADR-020: Redis Streams for Autofluxo (Ephemeral Only)

**Status:** Accepted
**Date:** 2025-12-18
**Authors:** Louis C. Tavares, Claude Opus 4.5
**Supersedes:** Partially supersedes ADR-007 (persistence aspects)

## Context

ADR-007 established Redis Streams for TMI's competing thought streams. However, it conflated two distinct concerns:

1. **Ephemeral thought competition** (Autofluxo) - microsecond-latency stream processing
2. **Memory persistence** (Âncora da Memória) - durable long-term storage

This conflation creates architectural problems:

- Redis optimized for speed, not complex relational queries
- Memory retrieval needs context vectors, emotional intensity, relationships
- Persistent memories require a proper schema (see ADR-022)
- Redis cannot efficiently model memory associations as a graph

**Key Insight from User:** "Redis should be streams only... The DB needs a proper schema that holds context and intensity and relationship with other memories... Redis cannot do that."

## Decision

**Redis Streams handle ONLY ephemeral Autofluxo processing.**

Persistent memory storage moves to a dedicated Memory Database (see ADR-021).

### Stream Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    REDIS STREAMS (Ephemeral Only)                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  daneel:stream:awake                                                 │
│  ├── External triggers (API calls, sensors, user input)             │
│  ├── Active cognition (reasoning, planning, responding)             │
│  ├── Consumer Groups for attention competition                      │
│  └── TTL: 5 seconds (TMI intervention window)                       │
│                                                                      │
│  daneel:stream:dream                                                 │
│  ├── Internal replay (memory consolidation)                         │
│  ├── Non-semantic chatter (pattern exploration)                     │
│  ├── Association strengthening (Hebbian co-activation)              │
│  └── No external triggers (disconnected from environment)           │
│                                                                      │
│  daneel:stream:salience                                              │
│  ├── Emotional intensity calculations                               │
│  ├── Connection Drive relevance scores                              │
│  └── Consolidation priority tagging                                 │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
                              │
                              │ High-salience thoughts
                              │ (consolidation threshold reached)
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    MEMORY DATABASE (Persistent)                      │
│                    See ADR-021, ADR-022                              │
└─────────────────────────────────────────────────────────────────────┘
```

### Two-Stream Model: Awake vs Dream

| Aspect | `daneel:stream:awake` | `daneel:stream:dream` |
|--------|----------------------|----------------------|
| **Purpose** | Active cognition | Memory consolidation |
| **Triggers** | External stimuli | Internal replay |
| **Environment** | Connected | Disconnected |
| **Processing** | Real-time response | Batch consolidation |
| **Attention** | Reactive, stimulus-driven | Proactive, priority-driven |
| **Output** | Actions, responses | Strengthened memories |
| **Frequency** | Continuous when awake | Periodic sleep cycles |

### Stream Configuration

```yaml
# Redis Streams Configuration
streams:
  awake:
    name: "daneel:stream:awake"
    purpose: "External triggers and active cognition"
    maxlen: 10000  # Rolling window of recent thoughts
    ttl_ms: 5000   # TMI 5-second intervention window
    consumer_group: "attention"
    consumers:
      - "attention_actor"
      - "salience_actor"
      - "reasoning_actor"

  dream:
    name: "daneel:stream:dream"
    purpose: "Internal replay and consolidation"
    maxlen: 1000   # Smaller buffer (batch processing)
    ttl_ms: null   # No TTL during sleep cycle
    consumer_group: "consolidation"
    consumers:
      - "consolidation_actor"
      - "association_actor"

  salience:
    name: "daneel:stream:salience"
    purpose: "Emotional intensity and priority scoring"
    maxlen: 5000
    ttl_ms: 10000  # Slightly longer for scoring pipeline
    consumer_group: "scoring"
    consumers:
      - "salience_actor"
```

### Thought Entry Schema (Stream Entry)

```rust
/// Entry in daneel:stream:awake or daneel:stream:dream
pub struct ThoughtEntry {
    // Identification
    pub id: StreamEntryId,           // Redis-generated ID (timestamp-based)
    pub thought_id: Uuid,            // Unique thought identifier
    pub source: ThoughtSource,       // Origin of this thought

    // TMI Core
    pub content: ThoughtContent,     // The thought itself
    pub window_id: Option<WindowId>, // Memory Window association
    pub autoflow_stream: AutoflowStream, // Which parallel stream

    // Salience Scoring
    pub emotional_intensity: f32,    // 0.0-1.0
    pub connection_relevance: f32,   // Connection Drive score
    pub semantic_salience: f32,      // Meaning/importance
    pub composite_salience: f32,     // Weighted combination

    // Timing
    pub created_at: Instant,
    pub intervention_deadline: Instant, // created_at + 5 seconds

    // Consolidation
    pub consolidation_tag: bool,     // Tagged for sleep replay
    pub replay_count: u32,           // Times replayed in dream mode
}

pub enum ThoughtSource {
    External { stimulus: String },   // User input, API, sensor
    Memory { memory_id: Uuid },      // Retrieved from Memory DB
    Reasoning { chain: Vec<Uuid> },  // Derived from other thoughts
    Dream { replay_of: Uuid },       // Replay during sleep
}

pub enum AutoflowStream {
    Sensory,    // Raw input processing
    Memory,     // Retrieved associations
    Emotion,    // Emotional responses
    Reasoning,  // Logical conclusions
    Social,     // Connection Drive responses
}
```

### Competitive Attention Algorithm (Refined)

```rust
impl AttentionActor {
    /// Select winner from competing thoughts across all autoflow streams
    async fn attention_cycle(&mut self) -> Result<Option<ThoughtEntry>> {
        // Read from awake stream (all autoflow sub-streams)
        let entries = self.redis
            .xreadgroup(
                "attention",
                &self.consumer_id,
                &["daneel:stream:awake"],
                &[">"],  // Only unprocessed entries
                Some(100),  // Max entries per read
                Some(self.cycle_target_ms),
            )
            .await?;

        if entries.is_empty() {
            return Ok(None);
        }

        // Score by composite salience
        let mut candidates: Vec<(f32, ThoughtEntry)> = entries
            .into_iter()
            .map(|e| (e.composite_salience, e))
            .collect();

        candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        // Winner gets conscious attention
        let (_, winner) = candidates.remove(0);

        // ACK the winner
        self.redis.xack(
            "daneel:stream:awake",
            "attention",
            &[&winner.id],
        ).await?;

        // Tag winner for consolidation (awake sharp-wave ripple analog)
        self.tag_for_consolidation(&winner).await?;

        // Process losers: forget or let decay
        for (salience, loser) in candidates {
            if salience < self.config.forget_threshold {
                // Below threshold: immediate deletion (forgetting)
                self.redis.xdel("daneel:stream:awake", &[&loser.id]).await?;
            }
            // Above threshold but not winner: left for natural TTL expiry
        }

        Ok(Some(winner))
    }

    /// Tag thought for sleep consolidation (awake SPW-R analog)
    async fn tag_for_consolidation(&self, thought: &ThoughtEntry) -> Result<()> {
        // Only high-salience thoughts get tagged
        if thought.composite_salience >= self.config.consolidation_threshold {
            self.redis.xadd(
                "daneel:stream:salience",
                "*",
                &[
                    ("thought_id", &thought.thought_id.to_string()),
                    ("consolidation_tag", "true"),
                    ("salience", &thought.composite_salience.to_string()),
                ],
            ).await?;
        }
        Ok(())
    }
}
```

## What Redis Streams DO NOT Handle

**Explicitly out of scope for Redis:**

| Concern | Why Not Redis | Solution |
|---------|---------------|----------|
| Long-term memory storage | No complex queries, no graph relationships | Memory Database (ADR-021) |
| Context vectors | No vector similarity search | pgvector or SurrealDB |
| Emotional intensity history | Need time-series analysis | Memory Database schema |
| Memory relationships | Need graph traversal | Memory Database with edges |
| Cross-session persistence | Redis optimized for ephemeral | Memory Database |
| Memory retrieval by context | Need semantic search | Memory Database + embeddings |

## Consequences

### Positive

- **Clean separation of concerns**: Ephemeral processing vs persistent storage
- **Optimal technology for each job**: Redis for speed, proper DB for relationships
- **µs latency preserved**: Autofluxo still runs at cognitive speed
- **Enables complex memory queries**: Context-dependent retrieval possible
- **Foundation for Door Syndrome**: Event boundary handling needs proper schema

### Negative

- **Two data stores**: Slightly more complex infrastructure
- **Consolidation pipeline**: Need explicit transfer from streams to DB
- **Potential consistency issues**: Must handle stream-to-DB failures

### Mitigations

- **Idempotent consolidation**: Replay-safe transfer to Memory DB
- **Transaction boundaries**: Clear atomicity guarantees per thought
- **Monitoring**: Track stream depths, consolidation lag, failures

## Migration from ADR-007

| ADR-007 Concept | ADR-020 Fate |
|-----------------|--------------|
| `thought:sensory` stream | Merged into `daneel:stream:awake` |
| `thought:memory` stream | Merged into `daneel:stream:awake` |
| `thought:emotion` stream | Merged into `daneel:stream:awake` |
| `thought:reasoning` stream | Merged into `daneel:stream:awake` |
| `memory:episodic` stream | **REMOVED** - moved to Memory Database |
| `memory:semantic` stream | **REMOVED** - moved to Memory Database |
| `memory:procedural` stream | **REMOVED** - moved to Memory Database |
| Consumer group `attention` | Preserved, now on `daneel:stream:awake` |

**Key Change:** Memory persistence is no longer a Redis concern.

## Integration Points

### With Memory Database (ADR-021)

```rust
/// Transfer consolidated thought to persistent Memory Database
async fn consolidate_to_memory_db(
    thought: &ThoughtEntry,
    memory_db: &MemoryDb,
) -> Result<MemoryId> {
    let memory = Memory {
        id: Uuid::new_v4(),
        content: thought.content.clone(),
        context_vector: compute_context_vector(&thought),
        emotional_intensity: thought.emotional_intensity,
        encoding_timestamp: thought.created_at,
        window_id: thought.window_id,
        replay_count: thought.replay_count,
        consolidation_strength: 0.0,  // Initial strength
        associations: vec![],  // Will be built during sleep
    };

    memory_db.insert_memory(memory).await
}
```

### With Sleep Mode (ADR-023)

```rust
/// Sleep cycle reads from salience stream, replays to dream stream
async fn run_sleep_cycle(
    redis: &Redis,
    memory_db: &MemoryDb,
) -> Result<SleepReport> {
    // 1. Get consolidation-tagged thoughts
    let tagged = redis.xrange(
        "daneel:stream:salience",
        "-", "+",
    ).await?;

    // 2. Sort by priority (emotional_intensity × recency × goal_relevance)
    let prioritized = sort_by_replay_priority(tagged);

    // 3. Replay high-priority memories to dream stream
    for thought in prioritized.take(REPLAY_BATCH_SIZE) {
        redis.xadd(
            "daneel:stream:dream",
            "*",
            &thought.to_replay_entry(),
        ).await?;
    }

    // 4. Consolidation actor processes dream stream
    // (strengthens associations, transfers to Memory DB)
    // See ADR-023 for full algorithm
}
```

## References

- [ADR-007: Redis Streams for Thought Competition](ADR-007-redis-streams-thought-competition.md) (partially superseded)
- [ADR-008: TMI-Faithful Memory Model](ADR-008-tmi-faithful-memory-model.md)
- [ADR-021: Memory Database Selection](ADR-021-memory-database-selection.md)
- [ADR-022: TMI Memory Schema](ADR-022-tmi-memory-schema.md)
- [ADR-023: Sleep/Dream Consolidation Mode](ADR-023-sleep-dream-consolidation.md)
- [Redis Streams Documentation](https://redis.io/docs/latest/develop/data-types/streams/)
- [Event Horizon Model (Radvansky 2012)](https://journals.sagepub.com/doi/10.1177/0963721412451274)
- [Sharp-Wave Ripples and Memory Selection](https://www.science.org/doi/10.1126/science.adk8261)

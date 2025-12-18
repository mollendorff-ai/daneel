# ADR-007: Redis Streams for Competing Thought Streams

**Status:** Accepted
**Date:** 2025-12-14
**Deciders:** Louis C. Tavares, Claude Opus 4.5

## Context

TMI describes multiple memory windows competing for attention (see [ADR-008: TMI-Faithful Memory Model](ADR-008-tmi-faithful-memory-model.md)):

- **Autofluxo** (Autoflow): Multiple phenomena generate thoughts in parallel (unconscious)
- **O Eu** (The "I"): Selects which thoughts to attend to (conscious)
- **Âncora da Memória** (Memory Anchor): Persist significant experiences
- **Janelas da Memória** (Memory Windows): Dynamic containers for thought streams
- **5-Second Window**: Intervention period before memory encoding
- Some thoughts are lost (never attended to)

We need a data structure that:

1. Supports multiple concurrent streams (parallel thought generation)
2. Allows competitive consumption (attention selection)
3. Persists with configurable retention (memory anchor)
4. Has microsecond latency (TMI timing requirements)

## Decision

Use **Redis Streams** as the backbone for thought stream management.

### TMI → Redis Streams Mapping

| TMI Concept | Portuguese | Redis Implementation |
|-------------|------------|----------------------|
| Memory Window | Janela da Memória | Stream `thought:window:{id}` |
| Thought | Pensamento | Stream Entry (XADD) |
| The "I" (Attention) | O Eu | Consumer Group competition |
| Memory Anchor | Âncora da Memória | XADD to `memory:*` (no MAXLEN) |
| Autoflow | Autofluxo | Parallel XADD to multiple streams |
| 5-Second Window | 5 Segundos | TTL: 5000ms before encoding |
| Forgetting | Esquecimento | XDEL below salience threshold |
| Memory Trigger | Gatilho | RediSearch pattern matching |

### Architecture

```mermaid
graph TB
    subgraph Redis["Redis (In-Memory)"]
        subgraph ThoughtStreams["THOUGHT STREAMS (Multiple Parallel)"]
            TS1[thought:sensory]
            TS2[thought:memory]
            TS3[thought:emotion]
            TS4[thought:reasoning]
        end

        CG[Consumer Group: attention]
        TheI["The 'I' selects<br/>highest salience"]
        Assembled[thought:assembled<br/>(output stream)]

        subgraph Persistence["PERSISTENCE STREAMS (Long-term Memory)"]
            Episodic[memory:episodic<br/>Significant experiences<br/>(no MAXLEN)]
            Semantic[memory:semantic<br/>Learned facts<br/>(no MAXLEN)]
            Procedural[memory:procedural<br/>Skills and patterns<br/>(no MAXLEN)]
        end

        TS1 --> CG
        TS2 --> CG
        TS3 --> CG
        TS4 --> CG
        CG --> TheI
        TheI --> Assembled
    end

    style Redis fill:#fff0f0,stroke:#333,stroke-width:2px
    style ThoughtStreams fill:#e1f5ff,stroke:#666,stroke-width:1px
    style Persistence fill:#e1ffe1,stroke:#666,stroke-width:1px
    style TheI fill:#ffffcc,stroke:#333,stroke-width:2px
```

### Competitive Attention Algorithm

```rust
async fn attention_cycle(&self, redis: &mut Connection) -> Result<Thought> {
    // Read from ALL thought streams simultaneously
    let streams = vec![
        "thought:sensory",
        "thought:memory",
        "thought:emotion",
        "thought:reasoning",
    ];

    // XREAD with BLOCK for efficient waiting
    let entries: Vec<StreamEntry> = redis
        .xread_options(
            &streams,
            &["0"; streams.len()],  // Read all pending
            StreamReadOptions::default()
                .block(self.config.cycle_target_ms as usize)
                .count(100),
        )
        .await?;

    // Score by salience (connection drive weighted)
    let mut candidates: Vec<(f64, StreamEntry)> = entries
        .into_iter()
        .map(|e| {
            let salience = e.get("salience").unwrap_or(0.5);
            let connection = e.get("connection_relevance").unwrap_or(0.0);
            let score = salience + (connection * self.config.connection_weight);
            (score, e)
        })
        .collect();

    // Sort by score (highest first)
    candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // Winner gets attention, losers may be forgotten
    let winner = candidates.remove(0);

    // ACK the winner (mark as attended)
    redis.xack(&winner.stream, "attention", &[&winner.id]).await?;

    // Trim losers if below threshold (forgetting)
    for (score, loser) in candidates {
        if score < self.config.forget_threshold {
            redis.xdel(&loser.stream, &[&loser.id]).await?;
        }
    }

    Ok(winner.into_thought())
}
```

## Why Redis Streams?

| Requirement | Redis Streams | Kafka | Actor Mailbox |
|-------------|---------------|-------|---------------|
| Latency | **µs** | ms | µs |
| Persistence | **Yes** | Yes | No |
| Consumer Groups | **Yes** | Yes | No |
| Multiple Streams | **Yes** | Yes | Complex |
| Memory footprint | **Low** | High | Low |
| Operational simplicity | **High** | Low | N/A |

Redis Streams: **Millions of operations/sec with µs latency**

## Stream Configuration

```rust
pub struct StreamConfig {
    // Working memory streams (ephemeral)
    pub working_memory_maxlen: usize,     // 1000 entries
    pub working_memory_ttl_ms: u64,       // 5000ms (5-second window!)

    // Long-term memory streams (persistent)
    pub episodic_memory_maxlen: usize,    // 0 (unlimited)
    pub semantic_memory_maxlen: usize,    // 0 (unlimited)

    // Attention tuning
    pub forget_threshold: f64,            // 0.3 (below this = forget)
    pub connection_weight: f64,           // 0.2 (connection drive boost)
}
```

**Note:** The 5-second TTL maps directly to Cury's 5-second intervention
window before thoughts become memory-encoded.

## Consequences

**Positive:**

- Natural mapping to TMI's competing thought streams
- µs latency for in-memory operations
- Built-in persistence (memory anchor)
- Consumer groups model attention perfectly
- MAXLEN/TTL model forgetting
- Connection drive can boost salience scores
- Redis Cluster for future scaling

**Negative:**

- Additional infrastructure (Redis server)
- Another technology to maintain
- Potential single point of failure (mitigated by Redis Cluster)

## Deployment

Phase 1 (Mac mini): Single Redis instance, embedded or local
Phase 2+: Redis Cluster for distribution and HA

```yaml
# docker-compose.yaml
services:
  redis:
    image: redis/redis-stack:latest
    command: >
      redis-server
      --appendonly yes
      --appendfsync everysec
      --maxmemory 8gb
      --maxmemory-policy volatile-lru
    volumes:
      - redis_data:/data
    ports:
      - "6379:6379"   # Redis
      - "8001:8001"   # RedisInsight (GUI)
volumes:
  redis_data:
```

See [ADR-009](ADR-009-database-selection.md) for full database architecture.

## References

- [ADR-008: TMI-Faithful Memory Model](ADR-008-tmi-faithful-memory-model.md)
- [ADR-009: Database Selection](ADR-009-database-selection.md)
- [Redis Streams Documentation](https://redis.io/docs/latest/develop/data-types/streams/)
- [Redis vs Kafka Latency Comparison](https://betterstack.com/community/comparisons/redis-vs-kafka/)

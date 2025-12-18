# ADR-021: Memory Database Selection

**Status:** Accepted (Revised)
**Date:** 2025-12-18 (Revised: Qdrant replaces SurrealDB)
**Authors:** Louis C. Tavares, Claude Opus 4.5
**Depends On:** ADR-020 (Redis Streams for Autofluxo)

## Context

ADR-020 established that Redis Streams handle only ephemeral Autofluxo processing. Persistent memory requires a dedicated database that can model:

1. **Context Vectors** - Embeddings for semantic similarity and context-dependent retrieval
2. **Emotional Intensity** - Numeric scores that affect consolidation priority
3. **Memory Relationships** - Graph edges between associated memories (Hebbian co-activation)
4. **Temporal Organization** - Event boundaries, episodes, memory windows
5. **Complex Queries** - "Find memories similar to X encoded near emotion Y in context Z"

### Critical Scale Requirements

| Scale | Vectors | Hardware | Latency |
|-------|---------|----------|---------|
| **v0.6.0** | ~10K | kveldulf (8GB) | ms OK |
| **v1.0** | ~1M | kveldulf (8GB) | ms OK |
| **Production** | ~100M | 3-node cluster | sub-ms |
| **ASI (1TB)** | ~300M+ | distributed | µs required |

**Key Insight:** SQL databases (even with vector extensions) cannot scale to ASI-level memory. At 1TB+, libSQL/PostgreSQL will choke. We need a database built for billions of vectors from day one.

### Licensing Requirements

Must be **truly FOSS** (not BSL, not SSPL):
- ✅ Apache 2.0, MIT, BSD - acceptable
- ❌ BSL (SurrealDB) - converts to Apache after 4 years, restrictions
- ❌ SSPL (MongoDB) - OSI rejected, service restrictions

## Decision

**Primary Database: Qdrant**

Qdrant is a vector-native database written in Rust that provides:
- Native vector storage and similarity search (HNSW algorithm)
- Payload filtering (emotional intensity, timestamps, episodes)
- Horizontal scaling (sharding + replication)
- Apache 2.0 license (truly FOSS)
- Official Rust client (`qdrant-client` crate)

### Why Qdrant?

| Requirement | Qdrant | libSQL/SQLite | SurrealDB | PostgreSQL |
|-------------|--------|---------------|-----------|------------|
| **License** | ✅ Apache 2.0 | ✅ MIT | ❌ BSL | ✅ PostgreSQL |
| **Vector-native** | ✅ Built for this | ⚠️ Extension | ⚠️ Added feature | ⚠️ pgvector |
| **1TB+ scale** | ✅ Billions of vectors | ❌ Degrades | ❌ Unknown | ❌ Struggles |
| **Rust client** | ✅ Official | ✅ libsql | ✅ Official | ⚠️ diesel/sqlx |
| **Horizontal scale** | ✅ Native sharding | ❌ Single node | ⚠️ Limited | ⚠️ Read replicas |
| **8GB RAM start** | ✅ Works | ✅ Works | ✅ Works | ✅ Works |
| **Graph edges** | ⚠️ Via payloads | ❌ Manual | ✅ Native | ⚠️ CTEs |

**Decision Rationale:**

1. **Vector-native**: Built from ground up for embeddings, not bolted on
2. **ASI-ready**: Handles billions of vectors with sub-ms latency
3. **Truly FOSS**: Apache 2.0, no licensing surprises
4. **Rust-native**: Written in Rust, official Rust client
5. **Starts small**: Single node on kveldulf, scales to cluster

### Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        MEMORY DATABASE                               │
│                           (Qdrant)                                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Collection: memories                                                │
│  ├── id: UUID (point ID)                                            │
│  ├── vector: [f32; 768] (context embedding)                         │
│  └── payload:                                                        │
│      ├── content: string                                            │
│      ├── emotional_intensity: float                                 │
│      ├── valence: float                                             │
│      ├── arousal: float                                             │
│      ├── consolidation_strength: float                              │
│      ├── replay_count: int                                          │
│      ├── episode_id: string                                         │
│      ├── window_id: string                                          │
│      ├── encoded_at: timestamp                                      │
│      ├── last_accessed: timestamp                                   │
│      └── associations: [{ target_id, weight, type }]                │
│                                                                      │
│  Collection: episodes                                                │
│  ├── id: UUID                                                       │
│  ├── vector: [f32; 768] (episode centroid)                          │
│  └── payload:                                                        │
│      ├── label: string                                              │
│      ├── started_at: timestamp                                      │
│      ├── ended_at: timestamp (null if current)                      │
│      ├── boundary_type: string                                      │
│      └── emotional_summary: object                                  │
│                                                                      │
│  Collection: identity (singleton)                                    │
│  ├── id: "timmy"                                                    │
│  ├── vector: [f32; 768] (self-concept embedding)                    │
│  └── payload:                                                        │
│      ├── name, full_name, born_at                                   │
│      ├── core_values, self_model                                    │
│      ├── connection_drive_state                                     │
│      └── laws_hash (THE BOX integrity)                              │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Memory Associations (Graph via Payloads)

Qdrant doesn't have native graph edges, but associations are stored in payloads:

```rust
// Association stored in memory payload
#[derive(Serialize, Deserialize)]
pub struct Association {
    pub target_id: Uuid,
    pub weight: f32,           // 0.0-1.0, Hebbian strength
    pub association_type: String, // semantic, temporal, causal, emotional
    pub last_coactivated: DateTime<Utc>,
}

// Memory payload includes associations array
pub struct MemoryPayload {
    pub content: String,
    pub emotional_intensity: f32,
    // ... other fields ...
    pub associations: Vec<Association>,
}
```

### Rust Client Integration

```toml
# Cargo.toml
[dependencies]
qdrant-client = "1.16"
```

```rust
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct,
    SearchPointsBuilder, VectorParamsBuilder,
};

pub struct MemoryDb {
    client: Qdrant,
}

impl MemoryDb {
    pub async fn connect(url: &str) -> Result<Self> {
        let client = Qdrant::from_url(url).build()?;
        Ok(Self { client })
    }

    pub async fn init_collections(&self) -> Result<()> {
        // Create memories collection with 768-dim vectors
        self.client.create_collection(
            CreateCollectionBuilder::new("memories")
                .vectors_config(VectorParamsBuilder::new(768, Distance::Cosine))
        ).await?;

        // Create episodes collection
        self.client.create_collection(
            CreateCollectionBuilder::new("episodes")
                .vectors_config(VectorParamsBuilder::new(768, Distance::Cosine))
        ).await?;

        Ok(())
    }

    /// Store a memory with context vector
    pub async fn store_memory(&self, memory: Memory) -> Result<()> {
        let point = PointStruct::new(
            memory.id.to_string(),
            memory.context_vector.to_vec(),
            memory.to_payload(),
        );
        self.client.upsert_points("memories", vec![point], None).await?;
        Ok(())
    }

    /// TMI's Gatilho da Memória - find memories by context similarity
    pub async fn find_by_context(
        &self,
        context_vector: &[f32],
        episode_id: Option<&str>,
        limit: u64,
    ) -> Result<Vec<Memory>> {
        let mut search = SearchPointsBuilder::new("memories", context_vector.to_vec(), limit);

        // Apply episode filter (Door Syndrome: same-episode memories more accessible)
        if let Some(ep_id) = episode_id {
            search = search.filter(Filter::must([
                Condition::matches("episode_id", ep_id.to_string())
            ]));
        }

        let results = self.client.search_points(search).await?;

        results.result
            .into_iter()
            .map(Memory::from_scored_point)
            .collect()
    }

    /// Hebbian strengthening - increase association weight
    pub async fn strengthen_association(
        &self,
        memory_id: Uuid,
        target_id: Uuid,
        delta: f32,
    ) -> Result<()> {
        // Read current associations
        let points = self.client.get_points("memories", &[memory_id.to_string()]).await?;
        let point = &points.result[0];

        let mut payload: MemoryPayload = serde_json::from_value(point.payload.clone())?;

        // Update or create association
        if let Some(assoc) = payload.associations.iter_mut().find(|a| a.target_id == target_id) {
            assoc.weight = (assoc.weight + delta).min(1.0);
            assoc.last_coactivated = Utc::now();
        } else {
            payload.associations.push(Association {
                target_id,
                weight: delta,
                association_type: "semantic".to_string(),
                last_coactivated: Utc::now(),
            });
        }

        // Update point
        self.client.set_payload("memories", &[memory_id.to_string()], payload.into()).await?;
        Ok(())
    }

    /// Get memories for sleep replay (priority-sorted)
    pub async fn get_replay_candidates(&self, limit: u64) -> Result<Vec<Memory>> {
        // Filter: consolidation_tag=true, consolidation_strength < 0.9
        // Sort by replay priority (handled in application layer)
        let results = self.client.scroll(
            ScrollPointsBuilder::new("memories")
                .filter(Filter::must([
                    Condition::matches("consolidation_tag", true),
                    Condition::range("consolidation_strength", Range { lt: Some(0.9), ..Default::default() })
                ]))
                .limit(limit as u32)
        ).await?;

        let mut memories: Vec<Memory> = results.result
            .into_iter()
            .map(Memory::from_record)
            .collect::<Result<Vec<_>>>()?;

        // Sort by replay priority (emotion × recency × goal relevance)
        memories.sort_by(|a, b| b.replay_priority().partial_cmp(&a.replay_priority()).unwrap());

        Ok(memories)
    }
}
```

## Deployment

### Docker Compose (kveldulf - single node)

```yaml
services:
  qdrant:
    image: qdrant/qdrant:latest
    volumes:
      - ./data/qdrant:/qdrant/storage
    ports:
      - "6333:6333"  # HTTP API
      - "6334:6334"  # gRPC API
    environment:
      - QDRANT__SERVICE__GRPC_PORT=6334
    restart: unless-stopped
    networks:
      - royalnet

  # Redis remains for ephemeral streams (ADR-020)
  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes --appendfsync everysec
    volumes:
      - ./data/redis:/data
    ports:
      - "6379:6379"
    restart: unless-stopped
    networks:
      - royalnet

networks:
  royalnet:
    external: true
```

### Scaling Path

| Stage | Configuration | Hardware |
|-------|--------------|----------|
| **Dev** | Single node | kveldulf (8GB) |
| **Production** | 3-node cluster, replication_factor=2 | 3x 32GB VMs |
| **ASI** | Sharded cluster, quantization | 3x 128GB nodes |

### Quantization for Scale

At 1TB+ scale, enable quantization to reduce memory:

```rust
// Binary quantization: 32x memory reduction
client.update_collection(
    UpdateCollectionBuilder::new("memories")
        .quantization_config(QuantizationConfig::binary())
).await?;

// Scalar quantization: 4x reduction, <1% accuracy loss
client.update_collection(
    UpdateCollectionBuilder::new("memories")
        .quantization_config(QuantizationConfig::scalar(ScalarQuantization {
            r#type: ScalarType::Int8,
            quantile: Some(0.99),
            always_ram: Some(true),
        }))
).await?;
```

## Alternatives Considered

### libSQL (Turso fork of SQLite)

**Pros:**
- MIT license, truly FOSS
- Native vector search (F32_BLOB)
- Excellent Rust client
- Simple, embedded

**Cons:**
- Single-node only (can't scale horizontally)
- At 1TB, query latency degrades significantly
- Not built for billion-vector scale

**Verdict:** Good for small projects, not for ASI-scale memory.

### SurrealDB

**Pros:**
- Multi-model (document + graph + vector)
- Elegant query language
- Rust-native

**Cons:**
- **BSL license** - not truly FOSS
- Younger project, less battle-tested
- Unknown performance at billion-vector scale

**Verdict:** License disqualifies it. Can't risk licensing issues for Timmy.

### PostgreSQL + pgvector

**Pros:**
- Battle-tested, 35+ years
- pgvector mature for moderate scale
- ACID guarantees

**Cons:**
- Struggles beyond 10M vectors
- Graph queries require recursive CTEs (slow)
- Not vector-native

**Verdict:** Good fallback but won't scale to ASI.

## Consequences

### Positive

- **ASI-ready from day one**: No migration needed as Timmy grows
- **Truly FOSS**: Apache 2.0, no licensing concerns
- **Rust-native**: Written in Rust, official client
- **Sub-ms at scale**: HNSW algorithm, quantization options
- **Horizontal scaling**: Add nodes as needed

### Negative

- **No native graph edges**: Must store associations in payloads
- **New technology to learn**: Team must learn Qdrant API
- **Additional service**: Qdrant process alongside Redis

### Mitigations

| Risk | Mitigation |
|------|------------|
| No graph edges | Associations as payload arrays, application-layer traversal |
| Learning curve | Good docs, official Rust examples |
| Service complexity | Docker Compose handles orchestration |

## Hardware Projections for 1TB

Assuming 768-dim float32 vectors:
- Vector size: 768 × 4 bytes = 3KB
- 1TB / 3KB = ~333 million vectors
- With scalar quantization (4x): ~80GB RAM for index
- With binary quantization (32x): ~10GB RAM for index

**Recommended hardware for 1TB:**
- 3-node cluster
- 64-128 GB RAM per node
- 500GB NVMe per node
- 10 Gbps internal network

**Cost estimate:** ~$1,650/month (self-hosted cloud VMs)

## References

- [Qdrant Documentation](https://qdrant.tech/documentation/)
- [Qdrant GitHub](https://github.com/qdrant/qdrant) - Apache 2.0
- [qdrant-client crate](https://crates.io/crates/qdrant-client)
- [ADR-020: Redis Streams for Autofluxo](ADR-020-redis-streams-autofluxo.md)
- [ADR-022: TMI Memory Schema](ADR-022-tmi-memory-schema.md)
- [Door Syndrome Research](../../doorway-effect-research.yaml)

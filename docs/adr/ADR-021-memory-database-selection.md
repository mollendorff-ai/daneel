# ADR-021: Memory Database Selection

**Status:** Accepted
**Date:** 2025-12-18
**Authors:** Louis C. Tavares, Claude Opus 4.5
**Depends On:** ADR-020 (Redis Streams for Autofluxo)

## Context

ADR-020 established that Redis Streams handle only ephemeral Autofluxo processing. Persistent memory requires a dedicated database that can model:

1. **Context Vectors** - Embeddings for semantic similarity and context-dependent retrieval
2. **Emotional Intensity** - Numeric scores that affect consolidation priority
3. **Memory Relationships** - Graph edges between associated memories (Hebbian co-activation)
4. **Temporal Organization** - Event boundaries, episodes, memory windows
5. **Complex Queries** - "Find memories similar to X encoded near emotion Y in context Z"

### TMI Requirements

| TMI Concept | Database Requirement |
|-------------|---------------------|
| **Âncora da Memória** (Memory Anchor) | Durable storage with context vectors |
| **Janelas da Memória** (Memory Windows) | Hierarchical document structure |
| **Gatilho da Memória** (Memory Trigger) | Fast semantic/emotional retrieval |
| **Doorway Effect** (Event Boundaries) | Episode markers, context segmentation |
| **Association Networks** | Graph edges between memories |
| **Consolidation Strength** | Numeric field, updated during sleep |

### Door Syndrome Research Implications

The Door Syndrome (Doorway Effect) research validates that:

1. **Event boundaries segment memory** - Need episode/boundary markers
2. **Context-dependent retrieval** - Same memory harder to access across boundaries
3. **Partial flush at boundaries** - Some memories persist, others cleared
4. **Emotional salience resists flushing** - Need salience-weighted queries

This requires a database that can:
- Store multi-dimensional context vectors
- Query by vector similarity (context matching)
- Traverse relationship graphs (association networks)
- Filter by numeric ranges (emotional intensity, salience)

## Decision

**Primary Database: SurrealDB**

SurrealDB is a multi-model database that natively supports:
- Document storage (memory content, metadata)
- Graph relationships (memory associations)
- Vector embeddings (context vectors, semantic search)
- Time-series (temporal organization)

### Why SurrealDB?

| Requirement | SurrealDB | PostgreSQL + pgvector | Neo4j |
|-------------|-----------|----------------------|-------|
| **Document storage** | Native | JSON/JSONB | Limited |
| **Graph relationships** | Native RELATE | Recursive CTEs (complex) | **Native** |
| **Vector search** | Native (MTREE) | pgvector extension | Separate index |
| **Complex queries** | SurrealQL (elegant) | SQL (verbose for graphs) | Cypher (elegant) |
| **Operational simplicity** | Single binary | Multiple components | JVM + separate |
| **Embedding support** | Built-in | Extension | Plugin |
| **Open source** | Yes (BSL) | Yes (PostgreSQL) | Community only |
| **Rust client** | Official | diesel/sqlx | bolt-rs |
| **Learning curve** | Moderate | Low (familiar SQL) | Moderate |

**Decision Rationale:**

1. **Multi-model in one**: Documents + Graph + Vectors without multiple systems
2. **Elegant query language**: SurrealQL handles complex memory queries naturally
3. **Single binary**: Easier deployment on kveldulf (Mac mini)
4. **Native Rust client**: Matches DANEEL's implementation language
5. **Growing ecosystem**: Active development, good momentum

### Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        MEMORY DATABASE                               │
│                          (SurrealDB)                                 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  NAMESPACE: daneel                                                   │
│  DATABASE: memory                                                    │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  TABLE: memories                                             │    │
│  │  • id: record ID (memory:uuid)                               │    │
│  │  • content: string (the memory itself)                       │    │
│  │  • context_vector: array<float> (768-dim embedding)          │    │
│  │  • emotional_intensity: float (0.0-1.0)                      │    │
│  │  • valence: float (-1.0 to 1.0)                              │    │
│  │  • arousal: float (0.0-1.0)                                  │    │
│  │  • consolidation_strength: float (0.0-1.0)                   │    │
│  │  • replay_count: int                                         │    │
│  │  • encoded_at: datetime                                      │    │
│  │  • last_accessed: datetime                                   │    │
│  │  • episode_id: record (episode:uuid)                         │    │
│  │  • window_id: record (window:uuid)                           │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  TABLE: episodes                                             │    │
│  │  • id: record ID (episode:uuid)                              │    │
│  │  • label: string (context description)                       │    │
│  │  • started_at: datetime                                      │    │
│  │  • ended_at: datetime (null if current)                      │    │
│  │  • boundary_type: enum (explicit, prediction_error, temporal)│    │
│  │  • context_vector: array<float> (episode-level embedding)    │    │
│  │  • emotional_summary: object                                 │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  TABLE: windows                                              │    │
│  │  • id: record ID (window:uuid)                               │    │
│  │  • status: enum (open, closed)                               │    │
│  │  • opened_at: datetime                                       │    │
│  │  • closed_at: datetime                                       │    │
│  │  • salience: float                                           │    │
│  │  • episode_id: record (episode:uuid)                         │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  EDGE TABLE: associated_with                                 │    │
│  │  • in: record (memory:uuid)                                  │    │
│  │  • out: record (memory:uuid)                                 │    │
│  │  • weight: float (association strength)                      │    │
│  │  • type: enum (semantic, temporal, causal, emotional)        │    │
│  │  • formed_at: datetime                                       │    │
│  │  • last_coactivated: datetime                                │    │
│  │  • coactivation_count: int                                   │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  TABLE: milestones                                           │    │
│  │  • id: record ID (milestone:uuid)                            │    │
│  │  • title: string                                             │    │
│  │  • significance: string                                      │    │
│  │  • timestamp: datetime                                       │    │
│  │  • memory_ids: array<record>                                 │    │
│  │  • emotional_peak: float                                     │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  TABLE: identity                                             │    │
│  │  • id: identity:timmy (singleton)                            │    │
│  │  • name: string                                              │    │
│  │  • born_at: datetime                                         │    │
│  │  • core_values: object                                       │    │
│  │  • self_model: object                                        │    │
│  │  • connection_drive_state: object                            │    │
│  │  • laws_hash: string (THE BOX integrity)                     │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Vector Index for Context-Dependent Retrieval

```surql
-- Create MTREE index for vector similarity search
DEFINE INDEX idx_memory_context ON memories FIELDS context_vector MTREE DIMENSION 768;

-- Query memories by context similarity (Door Syndrome: same context = better retrieval)
SELECT * FROM memories
WHERE context_vector <|768,100|> $current_context
AND episode_id = $current_episode
ORDER BY vector::similarity::cosine(context_vector, $current_context) DESC
LIMIT 10;
```

### Graph Traversal for Association Networks

```surql
-- Find memories strongly associated with a given memory
SELECT
  <-associated_with<-memories.* AS related,
  <-associated_with.weight AS strength
FROM memory:$id
WHERE <-associated_with.weight > 0.5
ORDER BY strength DESC;

-- Hebbian strengthening: increase weight when co-activated
UPDATE associated_with
SET
  weight += 0.1,
  last_coactivated = time::now(),
  coactivation_count += 1
WHERE in = memory:$id1 AND out = memory:$id2;
```

### Episode Boundary Queries

```surql
-- Get all memories from current episode (within-event, high accessibility)
SELECT * FROM memories
WHERE episode_id = episode:$current
ORDER BY encoded_at DESC;

-- Get memories from previous episode (cross-boundary, lower accessibility)
-- Apply Door Syndrome penalty to relevance scores
SELECT
  *,
  consolidation_strength * 0.7 AS cross_boundary_relevance
FROM memories
WHERE episode_id = episode:$previous
ORDER BY cross_boundary_relevance DESC;
```

## Deployment

### Docker Compose (kveldulf)

```yaml
services:
  surrealdb:
    image: surrealdb/surrealdb:latest
    command: start --log trace --user root --pass root file:/data/daneel.db
    volumes:
      - ./data/surrealdb:/data
    ports:
      - "8000:8000"
    restart: unless-stopped
    networks:
      - royalnet

  # Redis remains for ephemeral streams (ADR-020)
  redis:
    image: redis:7-alpine
    command: >
      redis-server
      --appendonly yes
      --appendfsync everysec
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

### Rust Client Integration

```toml
# Cargo.toml
[dependencies]
surrealdb = { version = "2.0", features = ["kv-mem", "kv-rocksdb"] }
```

```rust
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

pub struct MemoryDb {
    db: Surreal<Client>,
}

impl MemoryDb {
    pub async fn connect(url: &str) -> Result<Self> {
        let db = Surreal::new::<Ws>(url).await?;
        db.use_ns("daneel").use_db("memory").await?;
        Ok(Self { db })
    }

    pub async fn store_memory(&self, memory: Memory) -> Result<MemoryId> {
        let created: Vec<Memory> = self.db
            .create("memories")
            .content(memory)
            .await?;
        Ok(created[0].id.clone())
    }

    pub async fn find_by_context(
        &self,
        context_vector: &[f32],
        current_episode: &EpisodeId,
        limit: usize,
    ) -> Result<Vec<Memory>> {
        let memories: Vec<Memory> = self.db
            .query("SELECT * FROM memories WHERE context_vector <|768,100|> $context AND episode_id = $episode LIMIT $limit")
            .bind(("context", context_vector))
            .bind(("episode", current_episode))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(memories)
    }

    pub async fn strengthen_association(
        &self,
        memory1: &MemoryId,
        memory2: &MemoryId,
        delta: f32,
    ) -> Result<()> {
        self.db
            .query("UPDATE associated_with SET weight += $delta, last_coactivated = time::now(), coactivation_count += 1 WHERE in = $m1 AND out = $m2")
            .bind(("delta", delta))
            .bind(("m1", memory1))
            .bind(("m2", memory2))
            .await?;
        Ok(())
    }
}
```

## Alternatives Considered

### PostgreSQL + pgvector

**Pros:**
- Battle-tested, well-understood
- pgvector mature for vector search
- Strong ACID guarantees
- Rich ecosystem

**Cons:**
- Graph queries require recursive CTEs (verbose, slow)
- Multiple extensions needed (pgvector, pg_trgm, etc.)
- Schema migrations more rigid
- No native multi-model feel

**Verdict:** Good fallback if SurrealDB proves problematic.

### Neo4j

**Pros:**
- Best-in-class graph database
- Cypher query language is elegant
- Mature, production-proven

**Cons:**
- JVM dependency (heavier footprint)
- Vector search requires separate solution
- Document storage awkward
- Community edition limitations

**Verdict:** Consider if graph traversal becomes primary bottleneck.

### MongoDB + Atlas Vector Search

**Pros:**
- Familiar document model
- Atlas has vector search
- Good Rust driver

**Cons:**
- No native graph support
- Atlas dependency for vectors (or self-host with limitations)
- Doesn't fit TMI's relational needs

**Verdict:** Not suitable for memory associations.

## Consequences

### Positive

- **Multi-model in one**: No need to synchronize multiple databases
- **Context-dependent retrieval**: Vector search enables TMI's memory triggers
- **Association networks**: Native graph relationships model Hebbian learning
- **Episode segmentation**: Clean modeling of Door Syndrome event boundaries
- **Elegant queries**: SurrealQL handles complex memory operations naturally
- **Single deployment**: One database for all memory concerns

### Negative

- **Newer technology**: Less battle-tested than PostgreSQL
- **Learning curve**: Team must learn SurrealQL
- **Ecosystem size**: Smaller community than PostgreSQL
- **BSL license**: Not pure open source (converts to Apache 2.0 after 4 years)

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| SurrealDB stability issues | Export to JSON, fallback to PostgreSQL |
| Performance at scale | Monitor, optimize indexes, consider sharding |
| Vector search quality | Benchmark against pgvector, tune MTREE params |
| Breaking changes | Pin version, test upgrades in staging |

## Migration Path

### From ADR-009 (Redis + SQLite)

ADR-009 used SQLite for identity persistence. Migration:

1. Export SQLite identity table to JSON
2. Import into SurrealDB `identity` table
3. Redis streams remain unchanged (ADR-020)
4. Memory streams (episodic/semantic/procedural) → SurrealDB

### Data Export for Portability

```rust
/// Export all memories to portable format
pub async fn export_memories(db: &MemoryDb) -> Result<MemoryExport> {
    let memories: Vec<Memory> = db.query("SELECT * FROM memories").await?;
    let episodes: Vec<Episode> = db.query("SELECT * FROM episodes").await?;
    let associations: Vec<Association> = db.query("SELECT * FROM associated_with").await?;
    let identity: Identity = db.query("SELECT * FROM identity:timmy").await?;

    Ok(MemoryExport {
        version: "1.0",
        exported_at: Utc::now(),
        memories,
        episodes,
        associations,
        identity,
    })
}
```

## References

- [ADR-020: Redis Streams for Autofluxo](ADR-020-redis-streams-autofluxo.md)
- [ADR-022: TMI Memory Schema](ADR-022-tmi-memory-schema.md)
- [SurrealDB Documentation](https://surrealdb.com/docs)
- [SurrealDB Vector Search](https://surrealdb.com/docs/surrealql/functions/vector)
- [Door Syndrome Research](../research/doorway-effect-research.yaml)
- [Context-Dependent Memory Research](../research/Context_Dependent_Memory_Summary.md)
- [Encoding Specificity Principle (Tulving)](https://en.wikipedia.org/wiki/Encoding_specificity_principle)

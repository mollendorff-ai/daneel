# ADR-056: Deprecate RedisGraph, Plan In-Process Graph

**Status:** Accepted
**Date:** 2026-02-19
**Author:** Rex + Claude Opus 4.6
**Related:** ADR-046 (Vector Connectivity), VCONN-6 (Spreading Activation),
VCONN-8 (GraphML Export), VCONN-12 (Bidirectional Neighbors)

---

## Context

ADR-046 introduced RedisGraph for association graph traversal (spreading
activation, GraphML export, dual-write during dreams). The implementation
uses Cypher queries via `GRAPH.QUERY` on the `redis/redis-stack-server`
image, which bundles RedisGraph as a module.

Three problems have emerged:

1. **RedisGraph is EOL.** Redis Ltd deprecated it in January 2023.
   FalkorDB forked and maintains it, but it is a niche dependency with
   uncertain long-term support.

2. **Deployment friction.** Moving from Docker to native deployment on
   macOS (or any non-Docker setup) means managing the RedisGraph module
   separately. Plain `redis-server` does not include it. FalkorDB is
   an extra process. `redis-stack-server` bundles modules we do not
   need (RedisSearch, RedisJSON, RedisTimeSeries, RedisBloom).

3. **Not yet load-bearing.** The learning loop is not closed. Dreams
   consolidate memories and write edges, but the graph data is
   ephemeral (nightly cleanup planned). Spreading activation runs
   every cycle but traverses a graph that gets wiped. No real
   associative structure persists yet.

## Current Integration Points

| Component | Location | What It Does |
|-----------|----------|--------------|
| `GraphClient::connect` | `main.rs` (2 sites) | Creates Redis connection for `GRAPH.QUERY` |
| `merge_edge` | Dream consolidation | Dual-writes co-replayed memory edges |
| `query_neighbors_directed` | `spread_activation()` | BFS over graph every cycle (Stage 1) |
| `export_graphml` | `/graph/export` API | Full graph dump for Gephi |
| `graph/mod.rs` | Entire module | 315 LOC, all `coverage(off)` |

**Graceful degradation already exists.** Both initialization sites wrap
`GraphClient::connect` in `match` and set `graph_client = None` on
failure. Spreading activation checks `if let Some(ref graph)` and skips
if unavailable. The export endpoint returns 503.

## Decision

### Phase 1: Defer (now)

- Switch from `redis/redis-stack-server` to plain `redis` in compose
  and deployment configs. RedisGraph is no longer required.
- `GraphClient::connect` will fail (no `GRAPH.QUERY` command), setting
  `graph_client = None`. The existing graceful degradation handles this.
- No code changes needed in `graph/mod.rs` — it stays as dead code
  behind `Option<Arc<GraphClient>>`.
- Spreading activation falls back to Qdrant-only retrieval (which it
  already does when graph is None).
- `/graph/export` returns 503 (acceptable — no graph data anyway).
- Nightly cleanup can ignore graph since there is no graph data.

### Phase 2: In-Process Graph (future, when learning loop closes)

Replace RedisGraph with `petgraph` (in-process):

- **Hydrate on startup** from Qdrant association payloads.
- **Update in-memory** during dream consolidation (no network hop).
- **BFS spreading activation** becomes a function call (~microseconds
  vs ~milliseconds for Redis round-trip).
- **GraphML export** trivially via `petgraph::dot` or manual XML.
- **Serialize to disk** on shutdown for persistence.
- Eliminates an entire external dependency.

### What We Do NOT Do

- We do not port to FalkorDB. It solves the EOL problem but keeps the
  operational weight of another process for a feature that is not yet
  load-bearing.
- We do not delete `graph/mod.rs`. It documents the Cypher schema and
  will serve as reference for the petgraph migration.

## Consequences

- **Positive:** Simpler deployment (plain Redis), one fewer process,
  no EOL dependency.
- **Positive:** No code changes needed for Phase 1 (graceful
  degradation already works).
- **Negative:** Spreading activation is Qdrant-only until petgraph is
  implemented. This reduces associative richness, but there is no
  meaningful graph data to traverse yet anyway.
- **Negative:** GraphML export unavailable until Phase 2. Acceptable
  since there is nothing to export.

## Migration Checklist

- [ ] Update compose.yaml: `redis/redis-stack-server` → `redis:7`
- [ ] Update infrastructure docs in roadmap.yaml
- [ ] Update deploy scripts for plain Redis
- [ ] (Phase 2) Add `petgraph` to Cargo.toml
- [ ] (Phase 2) Implement `InMemoryGraph` with same trait interface
- [ ] (Phase 2) Remove or archive `graph/mod.rs`

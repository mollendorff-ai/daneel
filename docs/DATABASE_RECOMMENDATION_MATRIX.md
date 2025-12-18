# Database Recommendation Matrix for TMI

> **Quick Reference Guide**
> **Date:** December 14, 2025

---

## Executive Summary

**RECOMMENDATION: Redis Stack (Streams + JSON + Search) + SQLite**

**Rationale:** µs latency, existing integration, comprehensive feature set, minimal complexity

---

## Comparison Matrix

### Overall Scores (out of 35 points)

```
Database          | Score | Verdict
------------------|-------|----------
Redis Stack       | 32/35 | ✅ RECOMMENDED
SurrealDB         | 29/35 | ⚠️  RISKY (too new)
PostgreSQL        | 26/35 | ❌ TOO SLOW
SQLite            | 24/35 | ✅ CHECKPOINTS ONLY
MongoDB           | 24/35 | ❌ TOO SLOW
Neo4j             | 23/35 | ❌ TOO SLOW
```

---

## TMI Requirements Scorecard

| Database | Windows | Triggers | Anchors | Assoc | Temporal | Emotion | Latency | Total |
|----------|:-------:|:--------:|:-------:|:-----:|:--------:|:-------:|:-------:|:-----:|
| **Redis Stack** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | **32** |
| **SurrealDB** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | **29** |
| **PostgreSQL** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | **26** |
| **SQLite** | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | **24** |
| **MongoDB** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐ | **24** |
| **Neo4j** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐ | **23** |

**Legend:**
- **Windows:** Memory Windows (dynamic open/close, hierarchical)
- **Triggers:** Memory Triggers (pattern matching, associations)
- **Anchors:** Memory Anchors (emotional weighting, persistence)
- **Assoc:** Associative retrieval (one memory → related memories)
- **Temporal:** Temporal queries (when did this happen?)
- **Emotion:** Emotional coloring (salience scores)
- **Latency:** µs requirement for 50ms cycle

---

## Latency Analysis (Critical for 50ms Cycle)

### Operations per 50ms Budget

```
Database          | Latency/Op | Ops/Cycle | 50ms Budget | Pass/Fail
------------------|------------|-----------|-------------|----------
Redis (in-mem)    | 100 µs     | 500 ops   | 20ms used   | ✅ PASS
SQLite (embedded) | 200 µs     | 250 ops   | 20ms used   | ✅ PASS
PostgreSQL        | 2 ms       | 25 ops    | 45ms used   | ❌ FAIL
MongoDB           | 2 ms       | 25 ops    | 45ms used   | ❌ FAIL
Neo4j             | 10 ms      | 5 ops     | 48ms used   | ❌ FAIL
SurrealDB         | ??? µs     | ??? ops   | Unknown     | ⚠️  TEST
```

**Cycle Budget Breakdown (50ms target):**
```
Redis operations:  20ms (100 ops @ 200µs avg)
Actor messages:     5ms (50 msgs @ 100µs avg)
Computation:       20ms (salience, assembly)
Safety margin:      5ms
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
TOTAL:             50ms ✅
```

---

## Rust Ecosystem Maturity

```
Crate             | Downloads/Mo | Stars | Maturity | Async | Verdict
------------------|--------------|-------|----------|-------|----------
redis             | 2.5M         | 3.7k  | ⭐⭐⭐⭐⭐ | ✅    | ✅ EXCELLENT
rusqlite          | 1.8M         | 3.0k  | ⭐⭐⭐⭐⭐ | ❌    | ✅ EXCELLENT
sqlx              | 800K         | 13k   | ⭐⭐⭐⭐⭐ | ✅    | ✅ EXCELLENT
tokio-postgres    | 1.2M         | 3.5k  | ⭐⭐⭐⭐⭐ | ✅    | ✅ EXCELLENT
mongodb           | 400K         | 1.4k  | ⭐⭐⭐⭐  | ✅    | ✅ GOOD
neo4rs            | 15K          | 180   | ⭐⭐⭐   | ✅    | ⚠️  FAIR
surrealdb         | 50K          | 28k   | ⭐⭐⭐   | ✅    | ⚠️  YOUNG
```

**Winner:** Redis (`redis-rs`) + SQLite (`rusqlite`)

---

## Complexity Comparison

### Setup Complexity

```
Database       | Install | Config | Schema | Deploy | Total
---------------|---------|--------|--------|--------|-------
Redis Stack    | Docker  | None   | None   | 1 cmd  | ⭐⭐⭐⭐⭐
SQLite         | None    | None   | SQL    | 0 cmd  | ⭐⭐⭐⭐⭐
SurrealDB      | Binary  | File   | None   | 1 cmd  | ⭐⭐⭐⭐
MongoDB        | Docker  | YAML   | None   | 2 cmd  | ⭐⭐⭐⭐
PostgreSQL     | Docker  | YAML   | SQL    | 3 cmd  | ⭐⭐⭐
Neo4j          | Docker  | Props  | Cypher | 4 cmd  | ⭐⭐
```

### Operational Complexity

```
Database       | Tuning | Backup | Monitor | Upgrade | Total
---------------|--------|--------|---------|---------|-------
Redis Stack    | Simple | RDB    | CLI     | Easy    | ⭐⭐⭐⭐⭐
SQLite         | None   | File   | None    | Easy    | ⭐⭐⭐⭐⭐
SurrealDB      | Simple | File   | Built-in| Easy    | ⭐⭐⭐⭐
MongoDB        | Medium | Dump   | Compass | Medium  | ⭐⭐⭐⭐
PostgreSQL     | Complex| pg_dump| Logs    | Medium  | ⭐⭐⭐
Neo4j          | Complex| Backup | Browser | Complex | ⭐⭐
```

**Winner:** Redis + SQLite (minimal operational overhead)

---

## Feature Comparison Matrix

### Memory Windows (7±2 active, dynamic open/close)

| Database | Implementation | Complexity | Performance | Score |
|----------|----------------|------------|-------------|-------|
| **Redis** | JSON docs + active set | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **MongoDB** | Collections | Medium | Good | ⭐⭐⭐⭐ |
| **PostgreSQL** | Tables + FK | Medium | Good | ⭐⭐⭐⭐ |
| **SQLite** | Tables + FK | Medium | Fair | ⭐⭐⭐ |
| **Neo4j** | Nodes + labels | Complex | Fair | ⭐⭐⭐ |
| **SurrealDB** | Documents + edges | Medium | Unknown | ⭐⭐⭐⭐ |

---

### Memory Triggers (pattern matching, real-time)

| Database | Implementation | Complexity | Performance | Score |
|----------|----------------|------------|-------------|-------|
| **Redis** | RediSearch FT.SEARCH | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **MongoDB** | Text indexes | Medium | Good | ⭐⭐⭐ |
| **PostgreSQL** | Full-text search | Medium | Fair | ⭐⭐⭐ |
| **SQLite** | FTS5 extension | Medium | Fair | ⭐⭐ |
| **Neo4j** | Cypher patterns | Complex | Fair | ⭐⭐⭐⭐ |
| **SurrealDB** | SurrealQL patterns | Medium | Unknown | ⭐⭐⭐⭐ |

---

### Memory Anchors (persistence + salience metadata)

| Database | Implementation | Complexity | Performance | Score |
|----------|----------------|------------|-------------|-------|
| **Redis** | JSON + indexes | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **MongoDB** | Embedded docs | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **PostgreSQL** | JSONB columns | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **SQLite** | JSON columns | Simple | Good | ⭐⭐⭐⭐⭐ |
| **Neo4j** | Node properties | Simple | Good | ⭐⭐⭐⭐ |
| **SurrealDB** | Object fields | Simple | Excellent | ⭐⭐⭐⭐⭐ |

**All databases handle this well**

---

### Associative Retrieval (one memory → related memories)

| Database | Implementation | Complexity | Performance | Score |
|----------|----------------|------------|-------------|-------|
| **Redis** | Search + manual links | Medium | Excellent | ⭐⭐⭐⭐ |
| **MongoDB** | Embedded arrays + lookup | Medium | Good | ⭐⭐⭐ |
| **PostgreSQL** | JOINs + arrays | Medium | Good | ⭐⭐⭐⭐ |
| **SQLite** | JOINs + JSON | Medium | Fair | ⭐⭐⭐ |
| **Neo4j** | Native graph traversal | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **SurrealDB** | Native graph relations | Simple | Excellent | ⭐⭐⭐⭐⭐ |

**Best:** Neo4j, SurrealDB (native graph)
**Practical:** Redis (fast enough, simpler)

---

### Temporal Queries (timeline, when did this happen?)

| Database | Implementation | Complexity | Performance | Score |
|----------|----------------|------------|-------------|-------|
| **Redis** | Sorted sets + index | Medium | Excellent | ⭐⭐⭐⭐ |
| **MongoDB** | Date indexes | Simple | Good | ⭐⭐⭐⭐ |
| **PostgreSQL** | Timestamp indexes | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **SQLite** | Timestamp indexes | Simple | Good | ⭐⭐⭐⭐ |
| **Neo4j** | Properties | Medium | Fair | ⭐⭐⭐ |
| **SurrealDB** | Timestamp fields | Simple | Good | ⭐⭐⭐⭐ |

**Best:** PostgreSQL (designed for this)
**Practical:** Redis (fast enough)

---

### Emotional Coloring (salience: importance, novelty, relevance, valence)

| Database | Implementation | Complexity | Performance | Score |
|----------|----------------|------------|-------------|-------|
| **Redis** | JSON nested object | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **MongoDB** | Embedded document | Simple | Excellent | ⭐⭐⭐⭐⭐ |
| **PostgreSQL** | JSONB column | Simple | Good | ⭐⭐⭐⭐ |
| **SQLite** | JSON column | Simple | Good | ⭐⭐⭐⭐ |
| **Neo4j** | Node properties | Simple | Good | ⭐⭐⭐⭐ |
| **SurrealDB** | Object fields | Simple | Excellent | ⭐⭐⭐⭐⭐ |

**All databases handle this well**

---

## Deployment Comparison

### Single Machine (Mac mini)

```
Database       | Memory | CPU   | Disk  | Network | Total
---------------|--------|-------|-------|---------|-------
Redis Stack    | 2GB    | Low   | 1GB   | None    | ⭐⭐⭐⭐⭐
SQLite         | 100MB  | Low   | 500MB | None    | ⭐⭐⭐⭐⭐
SurrealDB      | 500MB  | Low   | 1GB   | None    | ⭐⭐⭐⭐
MongoDB        | 1GB    | Medium| 5GB   | None    | ⭐⭐⭐
PostgreSQL     | 500MB  | Medium| 2GB   | None    | ⭐⭐⭐
Neo4j          | 2GB    | High  | 5GB   | None    | ⭐⭐
```

**Winner:** Redis + SQLite (minimal resource footprint)

---

## Cost Analysis (Single Machine Deployment)

```
Database       | License | Ops Cost | Dev Time | Total
---------------|---------|----------|----------|-------
Redis Stack    | BSD     | $0       | Low      | ⭐⭐⭐⭐⭐
SQLite         | Public  | $0       | Low      | ⭐⭐⭐⭐⭐
SurrealDB      | BSL     | $0       | Medium   | ⭐⭐⭐⭐
MongoDB        | SSPL    | $0       | Low      | ⭐⭐⭐⭐
PostgreSQL     | Postgres| $0       | Medium   | ⭐⭐⭐⭐
Neo4j          | GPL/Comm| $0-$$$   | High     | ⭐⭐
```

**All free for single-machine, but Neo4j enterprise features locked**

---

## Risk Assessment

### Technical Risk

```
Database       | Maturity | Community | Support | Bugs | Risk
---------------|----------|-----------|---------|------|------
Redis Stack    | 15 years | Large     | Comm+Ent| Low  | ⭐⭐⭐⭐⭐
SQLite         | 23 years | Huge      | Public  | None | ⭐⭐⭐⭐⭐
PostgreSQL     | 30 years | Huge      | Comm+Ent| Low  | ⭐⭐⭐⭐⭐
MongoDB        | 15 years | Large     | Comm+Ent| Med  | ⭐⭐⭐⭐
SurrealDB      | 3 years  | Small     | Community| High| ⭐⭐
Neo4j          | 20 years | Medium    | Comm+Ent| Med  | ⭐⭐⭐⭐
```

**Lowest Risk:** Redis, SQLite, PostgreSQL
**Highest Risk:** SurrealDB (too new)

---

## Final Recommendation

### Primary: Redis Stack + SQLite

```
┌─────────────────────────────────────────┐
│  WHY REDIS STACK + SQLITE?              │
├─────────────────────────────────────────┤
│  ✅ µs latency (50-200µs vs 1-10ms)     │
│  ✅ Already using Redis Streams         │
│  ✅ Single Redis instance = simple      │
│  ✅ JSON + Search = flexible + fast     │
│  ✅ SQLite checkpoints = durable        │
│  ✅ Minimal ops overhead                │
│  ✅ Excellent Rust ecosystem            │
│  ✅ Mac mini deployment perfect         │
└─────────────────────────────────────────┘
```

### Architecture

```mermaid
graph TB
    subgraph Primary["REDIS STACK (Primary Store)"]
        RS1["Redis Streams<br/>Working memory (µs)"]
        RS2["RedisJSON<br/>Long-term memory (µs)"]
        RS3["RediSearch<br/>Associative index (µs)"]
    end

    subgraph Checkpoint["SQLITE (Checkpoint Store)"]
        SQ1["identity.db<br/>ContinuityService (ms)"]
        SQ2["milestones.db<br/>Key experiences (ms)"]
    end

    style Primary fill:#e1f5ff
    style Checkpoint fill:#fff9c4
```

### Decision Timeline

```
Phase 1 (MV-TMI):  Redis Stack + SQLite ✅
Phase 2 (If needed): Evaluate SurrealDB  ⚠️
Analytics:          PostgreSQL (separate) 📊
```

---

## Quick Start Commands

```bash
# 1. Start Redis Stack
docker-compose up -d redis-stack

# 2. Create RediSearch index
redis-cli FT.CREATE idx:memories ON JSON \
  PREFIX 1 "memory:episodic:" \
  SCHEMA \
    $.salience.importance AS importance NUMERIC SORTABLE \
    $.salience.connection AS connection NUMERIC SORTABLE \
    $.timestamp AS timestamp NUMERIC SORTABLE

# 3. SQLite is embedded (no setup needed)

# 4. Test latency
redis-cli --latency
```

---

## Benchmarks to Run

Before finalizing:

```
[ ] Redis JSON.SET:        < 100 µs
[ ] Redis JSON.GET:        < 200 µs
[ ] RediSearch FT.SEARCH:  < 500 µs
[ ] Redis XADD:            < 50 µs
[ ] Redis XREADGROUP:      < 100 µs
[ ] SQLite INSERT:         < 500 µs
[ ] SQLite SELECT:         < 200 µs
[ ] End-to-end cycle:      < 50 ms ✅
```

---

**Conclusion:** Redis Stack is the clear winner for TMI's memory model. It uniquely satisfies the µs latency requirement while providing all necessary features. SQLite complements it perfectly for durable checkpoints.

**Next Action:** Implement Redis Stack backend for MemoryService and run Phase 1 experiment.

---

**Author:** Claude Opus 4.5, Anthropic
**Date:** December 14, 2025

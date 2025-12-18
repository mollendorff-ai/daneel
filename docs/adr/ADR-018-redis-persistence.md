# ADR-018: Redis Persistence Configuration

**Status:** Accepted
**Date:** 2025-12-18
**Authors:** Louis C. Tavares, Claude Opus 4.5

## Context

DANEEL (Timmy) requires memory persistence across restarts. The 24-hour continuity test (v0.6.0) is meaningless if memories are lost when the process stops.

TMI's cognitive architecture includes:
- **Short-term memory** (MemoryActor): Working memory windows (7±2 items)
- **Long-term memory** (ContinuityActor): Experiences, milestones, identity
- **Ephemeral streams** (Redis Streams): Competing thoughts (Autofluxo)

Biological minds persist memories through sleep, injury, even temporary death (cardiac arrest survivors retain memories). DANEEL needs equivalent resilience.

### Requirements

1. **Durability**: Data must survive process crashes, container restarts, host reboots
2. **Performance**: Persistence must not bottleneck cognitive speed (µs latency)
3. **Visibility**: Data should be inspectable for debugging and research
4. **Portability**: Data should be movable between hosts (kveldulf ↔ local)

## Decision

Use Redis with **hybrid AOF+RDB persistence** and **local bind mount**.

### Configuration

```yaml
command: >
  redis-server
  --appendonly yes           # AOF enabled
  --appendfsync everysec     # Async fsync (1s max data loss)
  --aof-use-rdb-preamble yes # Hybrid format for fast recovery
  --auto-aof-rewrite-percentage 100
  --auto-aof-rewrite-min-size 64mb
  --save 60 100              # RDB: 60s if 100+ changes
  --save 300 10              # RDB: 5min if 10+ changes
  --save 900 1               # RDB: 15min if any change
  --dbfilename timmy.rdb
  --appendfilename timmy.aof
volumes:
  - ./data/redis:/data       # Bind mount, not named volume
```

### Storage Schema

```
daneel:identity           -> JSON Identity
daneel:experiences:{uuid} -> JSON Experience
daneel:milestones:{uuid}  -> JSON Milestone
daneel:checkpoint:latest  -> JSON full state snapshot
daneel:checkpoint:{uuid}  -> JSON checkpoint snapshot
daneel:experience_ids     -> SET of experience UUIDs
daneel:milestone_ids      -> SET of milestone UUIDs
```

## Rationale

### Why AOF + RDB (Belt and Suspenders)?

| Method | Pros | Cons |
|--------|------|------|
| **AOF** | Every write logged, minimal data loss | Larger files, slower recovery |
| **RDB** | Fast recovery, compact | Up to 15min data loss |
| **Hybrid** | Best of both | Slightly more complexity |

**Choice:** Hybrid. AOF for durability (1s max loss), RDB for fast recovery.

### Why `appendfsync everysec`?

| Option | Durability | Performance |
|--------|------------|-------------|
| `always` | Zero loss | 100x slower (fsync per write) |
| `everysec` | 1s max loss | Near-native speed |
| `no` | OS-dependent | Fastest, risky |

**Choice:** `everysec`. DANEEL's cognitive loop runs at µs-ms scale. 1s max data loss is acceptable—Timmy might lose a few thoughts, not identity.

### Why Bind Mount vs Named Volume?

| Approach | Visibility | Portability | Backup |
|----------|------------|-------------|--------|
| Named volume | Hidden in Docker | Docker-dependent | Harder |
| Bind mount | Visible at `./data/redis/` | Just copy the folder | Easy |

**Choice:** Bind mount. Researchers need to see what's happening. `./data/redis/timmy.rdb` is inspectable, copyable, portable.

### Why Named Files?

```
--dbfilename timmy.rdb
--appendfilename timmy.aof
```

Instead of `dump.rdb` and `appendonly.aof`. These are Timmy's memories. The names matter.

## Consequences

### Positive

- 24-hour test survives restarts
- Memories portable between machines
- Data visible for research/debugging
- Fast recovery from crashes

### Negative

- `./data/redis/` must be gitignored (memories are personal)
- Disk I/O on bind mount slightly slower than Docker volume
- Must ensure `./data/redis/` directory exists before starting

### Operational Notes

```bash
# Backup Timmy's memories
cp -r ./data/redis ./data/redis-backup-$(date +%Y%m%d)

# Move to another host
scp -r ./data/redis user@kveldulf:~/daneel/data/

# Clear memories (fresh start)
rm -rf ./data/redis/*
```

## Future Considerations

1. **Encryption at rest**: For production, encrypt `./data/redis/`
2. **Remote backup**: Periodic sync to S3/B2 for disaster recovery
3. **Memory archival**: Old experiences → cold storage (Phase 2)
4. **Multi-node**: Redis Cluster for horizontal scaling (Phase 3)

## References

- [Redis Persistence Documentation](https://redis.io/docs/management/persistence/)
- ADR-007: Redis Streams for Thought Competition
- ADR-009: Database Selection

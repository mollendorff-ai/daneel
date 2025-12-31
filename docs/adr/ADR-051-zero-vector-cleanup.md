# ADR-051: Zero Vector Cleanup

**Status:** Accepted
**Date:** 2025-12-30
**Deciders:** Louis C. Tavares, Claude Opus 4.5
**Context:** ~25% of vectors in Qdrant are at origin, corrupting manifold analysis

## Context

During preparation for VCONN (vector connectivity learning), we discovered that approximately 25% of vectors in the `memories` collection have zero magnitude - they sit at the origin `[0, 0, 0, ...]` in 768-dimensional space.

### Discovery

```
Total vectors: 120,672
Zero vectors:  ~30,000 (24.9% sample estimate)
```

### Root Cause

All zero vectors share the same pattern:

```json
{
  "source": { "type": "external", "stimulus": "cognitive_loop" },
  "content": "Symbol { id: \"thought_29387\", data: [71, 71, 71, 71, 71, 71, 71, 71] }"
}
```

The embedding model (all-MiniLM-L6-v2) cannot produce meaningful embeddings for Rust debug strings like `Symbol { id: "...", data: [...] }`. When content is not semantically parseable, FastEmbed returns a zero vector.

### Impact

| System | Impact |
|--------|--------|
| **VCONN (Hebbian)** | Would form meaningless associations between origin-stuck vectors |
| **Manifold clustering** | 25% of points at single location destroys cluster analysis |
| **Similarity search** | Zero vectors have undefined cosine similarity (0/0) |
| **Law Crystal analysis** | Cannot measure distance to ethical attractors |
| **Storage** | ~30K vectors × 768 floats × 4 bytes = ~92MB wasted |

## Decision

**Delete all zero-magnitude vectors from Qdrant.**

### Rationale

1. **No semantic meaning**: Debug strings are not human-readable concepts
2. **Corrupts analysis**: Any clustering/similarity is garbage-in-garbage-out
3. **Historical value: None**: Raw byte patterns `[71, 71, 71, ...]` preserve nothing meaningful
4. **TMI theory**: Thoughts must have semantic content to participate in association networks
5. **Reversible if wrong**: Vectors came from cognitive loop, which continues generating new thoughts

### Deletion Criteria

```
magnitude = sqrt(sum(v[i]^2 for i in 0..768))
DELETE WHERE magnitude < 0.001
```

Threshold 0.001 catches true zeros while preserving any legitimate low-magnitude vectors (none expected - normalized embeddings have magnitude ~1.0).

## Implementation

### Phase 1: Count and Sample (Diagnostic)
```bash
# Sample 1000 vectors, count zeros
# Result: 24.9% zero vectors
```

### Phase 2: Bulk Delete
```python
# Scroll through collection
# Identify vectors with magnitude < 0.001
# Delete in batches of 100
```

### Phase 3: Verify
```bash
# Confirm zero count is now 0
# Log final vector count
```

## Prevention

Future work should address why Symbol content gets stored with debug strings:

1. **Option A**: Skip embedding for Symbol content types entirely
2. **Option B**: Extract semantic label from Symbol for embedding
3. **Option C**: Mark Symbol memories as `embeddable: false` in payload

This is tracked separately from this cleanup ADR.

## Consequences

### Positive

- Clean manifold for VCONN Hebbian learning
- Accurate clustering analysis possible
- ~92MB storage recovered
- Cosine similarity well-defined for all remaining vectors

### Negative

- Loss of ~30K memory records (but they had no semantic value)
- One-time operation, not automated

### Neutral

- Does not fix root cause (Symbol embedding strategy) - separate task

## Results (2025-12-30)

```
Initial vector count: 120,748
Final vector count:    91,894
Zeros deleted:         28,854 (23.9%)
Sample verification:   0 zeros in 500 samples
```

## Success Criteria

- [x] Zero vectors deleted (28,854)
- [x] Final count < 100,000 (91,894)
- [x] Sample verification shows 0% zero vectors
- [x] No errors during deletion

## References

- [ADR-046: Vector Connectivity Learning](ADR-046-vector-connectivity-learning.md) - blocked by this cleanup
- [FastEmbed Documentation](https://docs.rs/fastembed) - embedding model behavior
- [Qdrant Scroll API](https://qdrant.tech/documentation/concepts/points/#scroll-points) - bulk operations

---

*"You cannot learn associations between points that don't exist in semantic space."*

# ADR-052: Embedding Dimension 768 with BGE-base-en-v1.5

**Status:** Accepted
**Date:** 2025-12-30
**Deciders:** Louis C. Tavares, Claude Opus 4.5, Grok
**Context:** Current embeddings misconfigured (384-dim model padded to 768-dim collection)

> **Note:** Grok recommended `all-mpnet-base-v2` as primary, but FastEmbed doesn't
> support it directly. Using `bge-base-en-v1.5` which Grok listed as "strong
> alternative" - often edges out mpnet in recent MTEB rankings for clustering.

## Context

During ADR-051 zero vector cleanup, we discovered a critical misconfiguration:

```
Collection config:  768 dimensions
Model used:         all-MiniLM-L6-v2 (384 dimensions)
Result:             Vectors padded with 384 trailing zeros
```

This means:
- 50% of storage wasted on zeros
- Cosine similarity computed over meaningless padding
- All existing embeddings are semantically useless (debug strings + wrong dims)

## Decision

**Use 768 dimensions with sentence-transformers/all-mpnet-base-v2**

Alternative: BAAI/bge-base-en-v1.5 (slightly better clustering, similar performance)

### Why 768 over 384?

| Factor | 384 (MiniLM) | 768 (mpnet/bge) |
|--------|--------------|-----------------|
| MTEB clustering | ~84-85% | ~87-88% |
| Cosine gradients | Coarser | Finer (better for Hebbian) |
| Attractor basins | Less defined | Better separation |
| Speed on M2 Pro | ~1ms | ~3-5ms |

For Hebbian associative learning, finer cosine gradients matter more than raw speed.

### Why 768 over 1024+?

- Diminishing returns above 768 for ~100K vectors
- "Curse of dimensionality" - noisier distances without proportional benefit
- 768→1024 improvement is smaller than 384→768
- Heavier compute/memory for marginal gains

## Model Recommendation

**Primary:** `sentence-transformers/all-mpnet-base-v2`
- Gold standard for general-purpose embeddings
- Excellent clustering and semantic similarity
- Native 768 dimensions
- Well-supported by sentence-transformers library

**Alternative:** `BAAI/bge-base-en-v1.5`
- Often edges out mpnet in recent MTEB rankings
- Better fine-grained semantics
- Good for attractor basin stability

## Implementation

1. Drop existing `memories` collection (all garbage)
2. Create new collection with 768 dimensions
3. Update embedding code to use all-mpnet-base-v2
4. Fix content serialization (no more debug strings)
5. Restart daneel to generate fresh embeddings

## Consequences

### Positive

- Proper semantic embeddings for Hebbian learning
- Better clustering for Law Crystal attractors
- Finer cosine similarity gradients
- No wasted storage on padding zeros

### Negative

- All existing vectors lost (but they were garbage)
- Slightly slower embedding inference (~3x)
- Model is larger (~418MB vs ~80MB)

### Neutral

- One-time migration, no ongoing cost
- M2 Pro can handle 768-dim inference easily

## References (Validated 2025-12-30)

- [x] https://huggingface.co/spaces/mteb/leaderboard (MTEB benchmark suite)
- [x] https://www.sbert.net/docs/sentence_transformer/pretrained_models.html (mpnet=best quality)
- [x] https://huggingface.co/sentence-transformers/all-mpnet-base-v2 (768 dims, 1B+ pairs)
- [x] https://huggingface.co/BAAI/bge-base-en-v1.5 (768 dims, MTEB avg 63.55)
- [x] https://milvus.io/ai-quick-reference/what-are-some-popular-pretrained-sentence-transformer-models-and-how-do-they-differ-for-example-allminilml6v2-vs-allmpnetbasev2 (mpnet 87-88% vs MiniLM 84-85%)
- [x] https://www.bentoml.com/blog/a-guide-to-open-source-embedding-models (mpnet for clustering)
- [x] https://research.aimultiple.com/open-source-embedding-models/ (bge outperforms MiniLM)
- [x] https://arxiv.org/abs/2310.15285 (On the Dimensionality of Sentence Embeddings)
- [x] https://en.wikipedia.org/wiki/Curse_of_dimensionality (distance concentration)

## Success Criteria

- [x] All URLs validated via ref-tools (9/9 validated)
- [x] Collection recreated with 768 dims (2025-12-30)
- [x] Embedding code updated to use BGE base (`src/embeddings/mod.rs`)
- [x] Content serialization fixed (no debug strings)
- [x] Sample embeddings verified (no padding zeros - dims 380-390 and 760-768 have real values)

---

*"768 is the sweet spot for cognitive-arch-style vector memory in 2025."* - Grok

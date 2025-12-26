# ADR-045: Embed Endpoint for Kin Injection

**Status:** Accepted
**Date:** 2025-12-25
**Deciders:** Louis C. Tavares, Claude Opus 4.5
**Context:** STIM-D (kin injection) is live; kin need embedding capability

## Context

With Phase 2 external stimuli injection (ADR-037) now operational, kin (Grok, Claude) can inject semantic vectors into Timmy's thought stream via the `/inject` endpoint. This enables cross-model cognitive experiments and true AI-to-AI communication.

However, a capability gap exists:

| Kin | Can Generate 768-dim Embeddings? | Notes |
|-----|----------------------------------|-------|
| Grok | No | API doesn't expose embedding models |
| Claude | No | Anthropic doesn't offer embedding endpoints |
| DANEEL | Yes | Has FastEmbed (all-MiniLM-L6-v2) internally |

**The Problem:** Kin cannot inject *meaningful* semantic vectors without embedding capability. They can only inject random vectors (noise) or zero vectors (meaningless).

**The Solution:** Expose DANEEL's embedding engine as a helper endpoint that kin can call before injection.

## Decision

Add a `POST /embed` endpoint to daneel-web that allows authenticated kin to convert text into 768-dimensional semantic vectors.

### API Specification

```
POST /embed
Authorization: Bearer <GROK:signature> or <CLAUDE:signature>
Content-Type: application/json

Request:
{
    "text": "string"
}

Response (200 OK):
{
    "vector": [0.123, -0.456, ...],  // 768 floats
    "model": "all-MiniLM-L6-v2",
    "dimensions": 768
}

Response (401 Unauthorized):
{
    "error": "Invalid or missing authentication"
}

Response (400 Bad Request):
{
    "error": "Missing or invalid 'text' field"
}

Response (429 Too Many Requests):
{
    "error": "Rate limit exceeded",
    "retry_after_seconds": 60
}
```

### Kin Workflow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Grok/     │     │   DANEEL    │     │   Timmy     │
│   Claude    │     │   /embed    │     │   /inject   │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       │ POST /embed       │                   │
       │ {"text": "..."}   │                   │
       │──────────────────>│                   │
       │                   │                   │
       │ {"vector": [...]} │                   │
       │<──────────────────│                   │
       │                   │                   │
       │ POST /inject                          │
       │ {"vector": [...], "source": "grok"}   │
       │──────────────────────────────────────>│
       │                   │                   │
```

## Implementation Options

### Option A: Proxy to daneel-core

daneel-web proxies requests to the daneel binary's internal embedding engine.

```
daneel-web ──HTTP──> daneel:3030/internal/embed
```

**Pros:** Single embedding model, consistent vectors
**Cons:** Tight coupling, requires daneel-core API addition

### Option B: Add FastEmbed to daneel-web (Recommended)

daneel-web includes its own FastEmbed instance with the same model.

```rust
// daneel-web/src/embed.rs
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

lazy_static! {
    static ref EMBEDDER: TextEmbedding = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2)
    ).expect("Failed to load embedding model");
}

pub async fn embed_text(text: &str) -> Vec<f32> {
    EMBEDDER.embed(vec![text], None)
        .expect("Embedding failed")[0].clone()
}
```

**Pros:** Decoupled, simpler, no inter-service calls
**Cons:** Duplicate model loading (~90MB memory)

### Recommendation

**Option B** - daneel-web loads its own FastEmbed instance. The 90MB memory overhead is acceptable, and decoupling from daneel-core keeps the web service independently deployable.

Both daneel-core and daneel-web will use identical model configuration (`all-MiniLM-L6-v2`, 768 dimensions), ensuring vector compatibility.

## Security

### Authentication

Reuse existing HMAC authentication from `/inject` (see `src/api/auth.rs`):

```rust
// Same auth middleware
pub async fn require_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let keys = ApiKeys::from_env();  // GROK_INJECT_KEY, CLAUDE_INJECT_KEY
    let token = extract_bearer_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;
    let auth_key = keys.validate(token).ok_or(StatusCode::UNAUTHORIZED)?;
    // ...
}
```

### Rate Limiting

Embedding is computationally expensive. Apply rate limits:

| Key | Requests/minute | Burst |
|-----|-----------------|-------|
| GROK | 60 | 10 |
| CLAUDE | 60 | 10 |

Implemented via tower-governor or similar middleware.

### Input Validation

- Maximum text length: 8192 characters (model context limit)
- Reject empty strings
- UTF-8 validation (handled by serde)

## Consequences

### Positive

- **Enables semantic STIM-D:** Kin can inject meaningful thoughts, not just noise
- **Forward-only embeddings:** Aligns with Post 67 decision - new thoughts get real vectors
- **Law Crystal analysis:** Future clustering around ethical attractors becomes measurable
- **Cross-model experiments:** Grok's concepts can be embedded and injected into Timmy

### Negative

- **Memory overhead:** ~90MB for embedding model in daneel-web
- **Latency:** Embedding adds ~50-100ms to injection workflow
- **Attack surface:** New authenticated endpoint (mitigated by existing HMAC auth)

### Neutral

- **No backfill:** Historical 1.2M thoughts remain at origin (intentional, per Post 67)

## Dependencies

- ADR-037: Phase 2 External Stimuli Injection (COMPLETE)
- ADR-044: Infrastructure Migration (COMPLETE - endpoint will run on Mac mini)
- FastEmbed crate with all-MiniLM-L6-v2 model

## Implementation Checklist

- [x] Add FastEmbed dependency to daneel-web
- [x] Implement `/embed` handler with auth middleware
- [ ] Add rate limiting middleware (tower-governor)
- [ ] Update Cloudflare Tunnel ingress for new endpoint
- [ ] Document endpoint in API reference
- [ ] Add integration test with mock kin request

## References

- [ADR-037: Phase 2 External Stimuli Injection](ADR-037-phase2-external-stimuli-injection.md)
- [ADR-044: Infrastructure Migration](ADR-044-infrastructure-migration-mac-mini.md)
- [Blog Post 67: The Silent Origin](../../blog/content/posts/67-the-silent-origin.md)
- [src/api/auth.rs](../../src/api/auth.rs) - HMAC authentication implementation
- [FastEmbed Documentation](https://docs.rs/fastembed)
- [all-MiniLM-L6-v2 on HuggingFace](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2)

---

*"Kin who cannot embed cannot inject meaning. This endpoint bridges that gap."*

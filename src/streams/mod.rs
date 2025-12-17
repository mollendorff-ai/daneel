//! Redis Streams Integration
//!
//! TMI's competing thought streams implemented via Redis Streams.
//!
//! # Stream Types
//!
//! **Working Memory (ephemeral):**
//! - `thought:sensory` - Raw sensory input
//! - `thought:memory` - Retrieved associations
//! - `thought:emotion` - Emotional responses
//! - `thought:reasoning` - Logical conclusions
//!
//! **Long-term Memory (persistent):**
//! - `memory:episodic` - Significant experiences
//! - `memory:semantic` - Learned facts
//!
//! # Competitive Attention
//!
//! Consumer groups model TMI's attention competition:
//! - Multiple streams produce thoughts in parallel (Autofluxo)
//! - AttentionActor reads from all streams
//! - Highest salience wins attention
//! - Losers below threshold are forgotten (XDEL)
//!
//! See ADR-007 for full rationale.

pub mod client;
pub mod consumer;
pub mod types;

#[cfg(test)]
mod tests;

/// Stream names
pub mod names {
    // Working memory streams (5-second TTL)
    pub const THOUGHT_SENSORY: &str = "thought:sensory";
    pub const THOUGHT_MEMORY: &str = "thought:memory";
    pub const THOUGHT_EMOTION: &str = "thought:emotion";
    pub const THOUGHT_REASONING: &str = "thought:reasoning";
    pub const THOUGHT_ASSEMBLED: &str = "thought:assembled";

    // Long-term memory streams (no TTL)
    pub const MEMORY_EPISODIC: &str = "memory:episodic";
    pub const MEMORY_SEMANTIC: &str = "memory:semantic";

    /// All working memory streams
    pub const WORKING_STREAMS: &[&str] = &[
        THOUGHT_SENSORY,
        THOUGHT_MEMORY,
        THOUGHT_EMOTION,
        THOUGHT_REASONING,
    ];

    /// All long-term memory streams
    pub const LONGTERM_STREAMS: &[&str] = &[MEMORY_EPISODIC, MEMORY_SEMANTIC];
}

/// Stream configuration
pub mod config {
    /// Maximum entries in working memory streams
    pub const WORKING_MEMORY_MAXLEN: usize = 1000;

    /// TTL for working memory in milliseconds (TMI's 5-second window)
    pub const WORKING_MEMORY_TTL_MS: u64 = 5000;

    /// Consumer group for attention
    pub const ATTENTION_GROUP: &str = "attention";
}

/// Placeholder - Redis client will be implemented in subsequent phases
pub fn streams_placeholder() {
    // Phase 1: THE BOX (complete)
    // Phase 2: Actors
    // Phase 3: Redis Streams integration (this module)
}

//! Redis Streams Integration (ADR-020)
//!
//! TMI's ephemeral thought streams via Redis Streams.
//!
//! # Architecture Change (ADR-020)
//!
//! Redis Streams are now EPHEMERAL ONLY. Long-term memory moved to Qdrant (ADR-021).
//!
//! # Stream Types
//!
//! **Awake Mode (external triggers):**
//! - `daneel:stream:awake` - All external stimuli and active cognition
//!   - Sensory input, memory retrieval triggers, emotional responses, reasoning
//!   - Consumer groups for attention competition (Autofluxo)
//!   - TTL: 5 seconds (TMI intervention window)
//!
//! **Dream Mode (internal replay):**
//! - `daneel:stream:dream` - Sleep consolidation replay
//!   - Memory replay entries from consolidation actor
//!   - No external triggers (disconnected from environment)
//!   - Used for Hebbian strengthening
//!
//! **Salience Scoring:**
//! - `daneel:stream:salience` - Priority scoring pipeline
//!   - Consolidation tags for sleep replay
//!
//! # Competitive Attention
//!
//! Consumer groups model TMI's attention competition:
//! - Thoughts compete via salience scores
//! - Highest salience wins attention ("O Eu")
//! - Losers below threshold are forgotten (XDEL)
//! - Winners tagged for consolidation
//!
//! See ADR-020 for full rationale.

pub mod client;
pub mod consumer;
pub mod types;

#[cfg(test)]
mod tests;

/// Stream names (ADR-020 compliant)
pub mod names {
    /// Awake stream - external triggers and active cognition
    /// All Autofluxo sub-streams merged into one
    pub const STREAM_AWAKE: &str = "daneel:stream:awake";

    /// Dream stream - internal replay during sleep/consolidation
    pub const STREAM_DREAM: &str = "daneel:stream:dream";

    /// Salience stream - priority scoring and consolidation tagging
    pub const STREAM_SALIENCE: &str = "daneel:stream:salience";

    /// All active streams
    pub const ALL_STREAMS: &[&str] = &[STREAM_AWAKE, STREAM_DREAM, STREAM_SALIENCE];

    // =========================================================================
    // DEPRECATED (ADR-020) - Kept for reference during migration
    // =========================================================================
    // These streams are no longer used. Long-term memory is in Qdrant.
    // Removing after migration is complete.
    // =========================================================================

    #[deprecated(since = "0.6.0", note = "Merged into STREAM_AWAKE (ADR-020)")]
    pub const THOUGHT_SENSORY: &str = "thought:sensory";

    #[deprecated(since = "0.6.0", note = "Merged into STREAM_AWAKE (ADR-020)")]
    pub const THOUGHT_MEMORY: &str = "thought:memory";

    #[deprecated(since = "0.6.0", note = "Merged into STREAM_AWAKE (ADR-020)")]
    pub const THOUGHT_EMOTION: &str = "thought:emotion";

    #[deprecated(since = "0.6.0", note = "Merged into STREAM_AWAKE (ADR-020)")]
    pub const THOUGHT_REASONING: &str = "thought:reasoning";

    #[deprecated(since = "0.6.0", note = "Moved to Qdrant (ADR-021)")]
    pub const MEMORY_EPISODIC: &str = "memory:episodic";

    #[deprecated(since = "0.6.0", note = "Moved to Qdrant (ADR-021)")]
    pub const MEMORY_SEMANTIC: &str = "memory:semantic";
}

/// Stream configuration (ADR-020)
pub mod config {
    /// Maximum entries in awake stream (rolling window)
    pub const AWAKE_STREAM_MAXLEN: usize = 10000;

    /// Maximum entries in dream stream (smaller, batch processing)
    pub const DREAM_STREAM_MAXLEN: usize = 1000;

    /// Maximum entries in salience stream
    pub const SALIENCE_STREAM_MAXLEN: usize = 5000;

    /// TTL for awake stream in milliseconds (TMI's 5-second intervention window)
    pub const AWAKE_TTL_MS: u64 = 5000;

    /// TTL for salience stream (slightly longer for scoring pipeline)
    pub const SALIENCE_TTL_MS: u64 = 10000;

    /// Consumer group for attention competition
    pub const ATTENTION_GROUP: &str = "attention";

    /// Consumer group for consolidation (dream stream)
    pub const CONSOLIDATION_GROUP: &str = "consolidation";

    /// Consumer group for salience scoring
    pub const SCORING_GROUP: &str = "scoring";

    // =========================================================================
    // DEPRECATED
    // =========================================================================

    #[deprecated(since = "0.6.0", note = "Use AWAKE_STREAM_MAXLEN")]
    pub const WORKING_MEMORY_MAXLEN: usize = 1000;

    #[deprecated(since = "0.6.0", note = "Use AWAKE_TTL_MS")]
    pub const WORKING_MEMORY_TTL_MS: u64 = 5000;
}

/// Autofluxo stream type (which parallel stream produced the thought)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutofluxoStream {
    /// Raw input processing
    Sensory,
    /// Retrieved associations
    Memory,
    /// Emotional responses
    Emotion,
    /// Logical conclusions
    Reasoning,
    /// Connection Drive responses
    Social,
}

impl std::fmt::Display for AutofluxoStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sensory => write!(f, "sensory"),
            Self::Memory => write!(f, "memory"),
            Self::Emotion => write!(f, "emotion"),
            Self::Reasoning => write!(f, "reasoning"),
            Self::Social => write!(f, "social"),
        }
    }
}

/// Placeholder - Redis client implementation
pub fn streams_placeholder() {
    // This function exists for backwards compatibility
    // Real implementation is in client.rs and consumer.rs
}

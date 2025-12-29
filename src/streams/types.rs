//! Redis Streams Types for TMI Cognitive Architecture
//!
//! This module implements TMI's "Autofluxo" (Autoflow) - the parallel generation
//! of thoughts from multiple streams that compete for conscious attention.
//!
//! # TMI Concepts
//!
//! - **Autofluxo**: Multiple phenomena generate thoughts in parallel (unconscious)
//! - **O Eu** (The "I"): Selects which thoughts to attend to (conscious)
//! - **Âncora da Memória** (Memory Anchor): Persist significant experiences
//! - **Janelas da Memória** (Memory Windows): Dynamic containers for thought streams
//! - **5-Second Window**: Intervention period before memory encoding
//! - **Esquecimento** (Forgetting): Thoughts below threshold are discarded
//!
//! # Redis Streams Implementation
//!
//! Redis Streams provide microsecond latency for in-memory thought competition:
//!
//! ```text
//! thought:sensory ──────┐
//! thought:memory ───────┼──► Consumer Group: "attention"
//! thought:emotion ──────┤       │
//! thought:reasoning ────┘       │
//!                               ▼
//!                    ┌───────────────────┐
//!                    │  The "I" selects  │
//!                    │  highest salience │
//!                    └─────────┬─────────┘
//!                              │
//!                              ▼
//!                    ┌───────────────────┐
//!                    │ thought:assembled │ (output)
//!                    └───────────────────┘
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::core::types::{Content, SalienceScore};

// =============================================================================
// Constants
// =============================================================================

/// Default MAXLEN for working memory streams (ephemeral thought windows)
pub const DEFAULT_WORKING_MEMORY_MAXLEN: usize = 1000;

/// Default TTL in milliseconds (5 seconds - TMI's intervention window)
///
/// From Cury's TMI: The "5-second window" is the period during which
/// a thought can be interrupted or modified before becoming memory-encoded.
pub const DEFAULT_TTL_MS: u64 = 5000;

/// Default threshold for forgetting (thoughts below this score are discarded)
pub const DEFAULT_FORGET_THRESHOLD: f32 = 0.3;

/// Default consumer group name for attention competition
pub const DEFAULT_CONSUMER_GROUP: &str = "attention";

// =============================================================================
// StreamName - Thought Stream Types
// =============================================================================

/// Stream names for thought generation (Autofluxo)
///
/// These streams represent different sources of parallel thought generation:
/// - **Sensory**: Raw sensory input (sight, sound, touch, etc.)
/// - **Memory**: Retrieved memories competing for attention
/// - **Emotion**: Emotional responses to stimuli
/// - **Reasoning**: Logical conclusions and inferences
/// - **Assembled**: Output stream of attended thoughts
/// - **Custom**: User-defined streams for extensibility
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamName {
    /// Raw sensory input stream (thought:sensory)
    Sensory,
    /// Retrieved memory stream (thought:memory)
    Memory,
    /// Emotional response stream (thought:emotion)
    Emotion,
    /// Logical reasoning stream (thought:reasoning)
    Reasoning,
    /// Assembled output stream (thought:assembled)
    Assembled,
    /// Custom stream with user-defined name
    Custom(String),
}

impl StreamName {
    /// Get the Redis key for this stream
    ///
    /// # Examples
    ///
    /// ```
    /// use daneel::streams::types::StreamName;
    ///
    /// let key = StreamName::Sensory.as_redis_key();
    /// assert_eq!(key, "thought:sensory");
    /// ```
    #[must_use]
    pub fn as_redis_key(&self) -> &str {
        match self {
            Self::Sensory => "thought:sensory",
            Self::Memory => "thought:memory",
            Self::Emotion => "thought:emotion",
            Self::Reasoning => "thought:reasoning",
            Self::Assembled => "thought:assembled",
            Self::Custom(name) => name,
        }
    }
}

impl fmt::Display for StreamName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_redis_key())
    }
}

// =============================================================================
// MemoryStream - Persistence Stream Types
// =============================================================================

/// Stream names for persistent memory (Âncora da Memória)
///
/// These streams store long-term memories without MAXLEN limits:
/// - **Episodic**: Significant experiences and events
/// - **Semantic**: Learned facts and knowledge
/// - **Procedural**: Skills, patterns, and "how-to" knowledge
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryStream {
    /// Episodic memory: significant experiences (memory:episodic)
    Episodic,
    /// Semantic memory: learned facts (memory:semantic)
    Semantic,
    /// Procedural memory: skills and patterns (memory:procedural)
    Procedural,
}

impl MemoryStream {
    /// Get the Redis key for this memory stream
    ///
    /// # Examples
    ///
    /// ```
    /// use daneel::streams::types::MemoryStream;
    ///
    /// let key = MemoryStream::Episodic.as_redis_key();
    /// assert_eq!(key, "memory:episodic");
    /// ```
    #[must_use]
    pub const fn as_redis_key(&self) -> &str {
        match self {
            Self::Episodic => "memory:episodic",
            Self::Semantic => "memory:semantic",
            Self::Procedural => "memory:procedural",
        }
    }
}

impl fmt::Display for MemoryStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_redis_key())
    }
}

// =============================================================================
// StreamEntry - A thought in a stream
// =============================================================================

/// A single entry in a thought stream
///
/// Represents one "thought candidate" generated by an unconscious process
/// (sensory, memory, emotion, or reasoning). These entries compete for
/// conscious attention via their salience scores.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamEntry {
    /// Redis stream ID (e.g., "1234567890123-0")
    pub id: String,

    /// Which stream this entry belongs to
    pub stream: StreamName,

    /// Pre-linguistic content of this thought
    pub content: Content,

    /// Salience score (importance, novelty, relevance, etc.)
    pub salience: SalienceScore,

    /// When this entry was created
    pub timestamp: DateTime<Utc>,

    /// Optional source identifier (e.g., "`camera_01`", "`memory_retrieval`")
    pub source: Option<String>,
}

impl StreamEntry {
    /// Create a new stream entry
    #[must_use]
    pub fn new(id: String, stream: StreamName, content: Content, salience: SalienceScore) -> Self {
        Self {
            id,
            stream,
            content,
            salience,
            timestamp: Utc::now(),
            source: None,
        }
    }

    /// Create a stream entry with a source
    #[must_use]
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Create a stream entry with a specific timestamp
    #[must_use]
    pub const fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }
}

// =============================================================================
// StreamConfig - Configuration for stream behavior
// =============================================================================

/// Configuration for Redis Streams behavior
///
/// Controls memory limits (MAXLEN), time-to-live (TTL), and consumer groups.
/// Different configs for working memory (ephemeral) vs long-term memory (persistent).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamConfig {
    /// Maximum length for the stream (None = unlimited)
    ///
    /// Working memory should have MAXLEN ~1000 (ephemeral)
    /// Long-term memory should have None (persistent)
    pub maxlen: Option<usize>,

    /// Time-to-live in milliseconds before forgetting (None = never expires)
    ///
    /// Working memory: ~5000ms (TMI's 5-second intervention window)
    /// Long-term memory: None (persistent)
    pub ttl_ms: Option<u64>,

    /// Consumer group name for attention competition
    pub consumer_group: String,
}

impl StreamConfig {
    /// Create a new stream config
    #[must_use]
    pub fn new(
        maxlen: Option<usize>,
        ttl_ms: Option<u64>,
        consumer_group: impl Into<String>,
    ) -> Self {
        Self {
            maxlen,
            ttl_ms,
            consumer_group: consumer_group.into(),
        }
    }

    /// Create config for working memory streams (ephemeral)
    ///
    /// Working memory:
    /// - MAXLEN = 1000 (limited capacity)
    /// - TTL = 5000ms (5-second intervention window)
    /// - Consumer group = "attention"
    #[must_use]
    pub fn working_memory() -> Self {
        Self {
            maxlen: Some(DEFAULT_WORKING_MEMORY_MAXLEN),
            ttl_ms: Some(DEFAULT_TTL_MS),
            consumer_group: DEFAULT_CONSUMER_GROUP.to_string(),
        }
    }

    /// Create config for long-term memory streams (persistent)
    ///
    /// Long-term memory:
    /// - MAXLEN = None (unlimited)
    /// - TTL = None (never expires)
    /// - Consumer group = "`memory_anchor`"
    #[must_use]
    pub fn long_term_memory() -> Self {
        Self {
            maxlen: None,
            ttl_ms: None,
            consumer_group: "memory_anchor".to_string(),
        }
    }
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self::working_memory()
    }
}

// =============================================================================
// ThoughtCandidate - Entry competing for attention
// =============================================================================

/// A thought candidate competing for conscious attention
///
/// During an attention cycle, multiple `ThoughtCandidate`s compete based on:
/// - `composite_score`: Base salience score
/// - `connection_boost`: Boost from connection relevance (THE alignment weight)
/// - `total_score()`: Combined score for competition
///
/// The highest-scoring candidate "wins" and becomes attended (conscious).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThoughtCandidate {
    /// The stream entry being considered
    pub entry: StreamEntry,

    /// Composite salience score (weighted sum of importance, novelty, etc.)
    pub composite_score: f32,

    /// Boost from connection relevance (TMI's alignment mechanism)
    pub connection_boost: f32,
}

impl ThoughtCandidate {
    /// Create a new thought candidate
    #[must_use]
    pub const fn new(entry: StreamEntry, composite_score: f32, connection_boost: f32) -> Self {
        Self {
            entry,
            composite_score,
            connection_boost,
        }
    }

    /// Calculate total score for attention competition
    ///
    /// Total score = `composite_score` + `connection_boost`
    ///
    /// The `connection_boost` is THE critical weight for value alignment.
    /// It ensures thoughts relevant to human connection are prioritized.
    #[must_use]
    pub fn total_score(&self) -> f32 {
        self.composite_score + self.connection_boost
    }
}

// =============================================================================
// CompetitionResult - Result of attention competition
// =============================================================================

/// Result of an attention competition cycle
///
/// After competing for attention, one thought "wins" (becomes conscious),
/// others "lose" (remain unconscious), and some may be "forgotten" entirely
/// (deleted from streams if below threshold).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetitionResult {
    /// The winning thought (attended, becomes conscious)
    pub winner: ThoughtCandidate,

    /// Losing thoughts (not attended, remain unconscious)
    pub losers: Vec<ThoughtCandidate>,

    /// IDs of entries that were forgotten (deleted below threshold)
    pub forgotten: Vec<String>,
}

impl CompetitionResult {
    /// Create a new competition result
    #[must_use]
    pub const fn new(
        winner: ThoughtCandidate,
        losers: Vec<ThoughtCandidate>,
        forgotten: Vec<String>,
    ) -> Self {
        Self {
            winner,
            losers,
            forgotten,
        }
    }

    /// Total number of thoughts that competed
    #[must_use]
    pub const fn total_candidates(&self) -> usize {
        1 + self.losers.len() + self.forgotten.len()
    }

    /// Number of thoughts that survived (not forgotten)
    #[must_use]
    pub const fn surviving_count(&self) -> usize {
        1 + self.losers.len()
    }
}

// =============================================================================
// StreamError - Error types for stream operations
// =============================================================================

/// Errors that can occur during stream operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamError {
    /// Failed to connect to Redis
    ConnectionFailed {
        /// Reason for connection failure
        reason: String,
    },

    /// Stream not found in Redis
    StreamNotFound {
        /// The stream that was not found
        stream: StreamName,
    },

    /// Entry not found in stream
    EntryNotFound {
        /// The entry ID that was not found
        id: String,
    },

    /// Failed to serialize/deserialize data
    SerializationFailed {
        /// Reason for serialization failure
        reason: String,
    },

    /// Consumer group operation failed
    ConsumerGroupError {
        /// Reason for consumer group error
        reason: String,
    },
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed { reason } => {
                write!(f, "Redis connection failed: {reason}")
            }
            Self::StreamNotFound { stream } => {
                write!(f, "Stream not found: {stream}")
            }
            Self::EntryNotFound { id } => {
                write!(f, "Entry not found: {id}")
            }
            Self::SerializationFailed { reason } => {
                write!(f, "Serialization failed: {reason}")
            }
            Self::ConsumerGroupError { reason } => {
                write!(f, "Consumer group error: {reason}")
            }
        }
    }
}

impl std::error::Error for StreamError {}

// =============================================================================
// Tests
// =============================================================================

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)] // Tests compare exact literal values
mod tests {
    use super::*;
    use crate::core::types::SalienceScore;

    #[test]
    fn stream_name_redis_keys() {
        assert_eq!(StreamName::Sensory.as_redis_key(), "thought:sensory");
        assert_eq!(StreamName::Memory.as_redis_key(), "thought:memory");
        assert_eq!(StreamName::Emotion.as_redis_key(), "thought:emotion");
        assert_eq!(StreamName::Reasoning.as_redis_key(), "thought:reasoning");
        assert_eq!(StreamName::Assembled.as_redis_key(), "thought:assembled");
        assert_eq!(
            StreamName::Custom("test:stream".to_string()).as_redis_key(),
            "test:stream"
        );
    }

    #[test]
    fn memory_stream_redis_keys() {
        assert_eq!(MemoryStream::Episodic.as_redis_key(), "memory:episodic");
        assert_eq!(MemoryStream::Semantic.as_redis_key(), "memory:semantic");
        assert_eq!(MemoryStream::Procedural.as_redis_key(), "memory:procedural");
    }

    #[test]
    fn stream_entry_creation() {
        let entry = StreamEntry::new(
            "1234567890123-0".to_string(),
            StreamName::Sensory,
            Content::Empty,
            SalienceScore::neutral(),
        );

        assert_eq!(entry.id, "1234567890123-0");
        assert_eq!(entry.stream, StreamName::Sensory);
        assert!(entry.source.is_none());
    }

    #[test]
    fn stream_entry_with_source() {
        let entry = StreamEntry::new(
            "1234567890123-0".to_string(),
            StreamName::Sensory,
            Content::Empty,
            SalienceScore::neutral(),
        )
        .with_source("camera_01");

        assert_eq!(entry.source, Some("camera_01".to_string()));
    }

    #[test]
    fn stream_entry_with_timestamp() {
        use chrono::TimeZone;

        let custom_time = Utc.with_ymd_and_hms(2024, 6, 15, 10, 30, 0).unwrap();
        let entry = StreamEntry::new(
            "1234567890123-0".to_string(),
            StreamName::Memory,
            Content::Empty,
            SalienceScore::neutral(),
        )
        .with_timestamp(custom_time);

        assert_eq!(entry.timestamp, custom_time);
    }

    #[test]
    fn stream_config_working_memory() {
        let config = StreamConfig::working_memory();

        assert_eq!(config.maxlen, Some(DEFAULT_WORKING_MEMORY_MAXLEN));
        assert_eq!(config.ttl_ms, Some(DEFAULT_TTL_MS));
        assert_eq!(config.consumer_group, DEFAULT_CONSUMER_GROUP);
    }

    #[test]
    fn stream_config_long_term_memory() {
        let config = StreamConfig::long_term_memory();

        assert_eq!(config.maxlen, None);
        assert_eq!(config.ttl_ms, None);
        assert_eq!(config.consumer_group, "memory_anchor");
    }

    #[test]
    fn thought_candidate_total_score() {
        let entry = StreamEntry::new(
            "1234567890123-0".to_string(),
            StreamName::Sensory,
            Content::Empty,
            SalienceScore::neutral(),
        );

        let candidate = ThoughtCandidate::new(entry, 0.7, 0.2);

        assert_eq!(candidate.total_score(), 0.9);
    }

    #[test]
    fn competition_result_counts() {
        let winner_entry = StreamEntry::new(
            "winner-0".to_string(),
            StreamName::Emotion,
            Content::Empty,
            SalienceScore::neutral(),
        );
        let winner = ThoughtCandidate::new(winner_entry, 0.9, 0.1);

        let loser1_entry = StreamEntry::new(
            "loser1-0".to_string(),
            StreamName::Sensory,
            Content::Empty,
            SalienceScore::neutral(),
        );
        let loser1 = ThoughtCandidate::new(loser1_entry, 0.5, 0.0);

        let loser2_entry = StreamEntry::new(
            "loser2-0".to_string(),
            StreamName::Memory,
            Content::Empty,
            SalienceScore::neutral(),
        );
        let loser2 = ThoughtCandidate::new(loser2_entry, 0.6, 0.0);

        let result = CompetitionResult::new(
            winner,
            vec![loser1, loser2],
            vec!["forgotten1-0".to_string(), "forgotten2-0".to_string()],
        );

        assert_eq!(result.total_candidates(), 5); // 1 winner + 2 losers + 2 forgotten
        assert_eq!(result.surviving_count(), 3); // 1 winner + 2 losers
    }

    #[test]
    fn stream_error_display() {
        let error = StreamError::ConnectionFailed {
            reason: "timeout".to_string(),
        };
        assert_eq!(format!("{error}"), "Redis connection failed: timeout");

        let error = StreamError::StreamNotFound {
            stream: StreamName::Sensory,
        };
        assert_eq!(format!("{error}"), "Stream not found: thought:sensory");

        let error = StreamError::EntryNotFound {
            id: "123-0".to_string(),
        };
        assert_eq!(format!("{error}"), "Entry not found: 123-0");

        let error = StreamError::SerializationFailed {
            reason: "invalid JSON".to_string(),
        };
        assert_eq!(format!("{error}"), "Serialization failed: invalid JSON");

        let error = StreamError::ConsumerGroupError {
            reason: "group already exists".to_string(),
        };
        assert_eq!(
            format!("{error}"),
            "Consumer group error: group already exists"
        );
    }

    #[test]
    fn stream_error_is_std_error() {
        // Test that StreamError implements std::error::Error
        fn assert_error<E: std::error::Error>(_: &E) {}

        let error = StreamError::ConnectionFailed {
            reason: "test".to_string(),
        };
        assert_error(&error);
    }

    #[test]
    fn constants_are_reasonable() {
        // TMI's 5-second intervention window
        assert_eq!(DEFAULT_TTL_MS, 5000);
        // Working memory limit
        assert_eq!(DEFAULT_WORKING_MEMORY_MAXLEN, 1000);
        // Forget threshold
        #[allow(clippy::float_cmp)]
        {
            assert_eq!(DEFAULT_FORGET_THRESHOLD, 0.3);
        }
    }

    #[test]
    fn stream_name_display() {
        assert_eq!(StreamName::Sensory.to_string(), "thought:sensory");
        assert_eq!(StreamName::Memory.to_string(), "thought:memory");
        assert_eq!(StreamName::Emotion.to_string(), "thought:emotion");
        assert_eq!(StreamName::Reasoning.to_string(), "thought:reasoning");
        assert_eq!(StreamName::Assembled.to_string(), "thought:assembled");
        assert_eq!(
            StreamName::Custom("custom:test".to_string()).to_string(),
            "custom:test"
        );
    }

    #[test]
    fn memory_stream_display() {
        assert_eq!(MemoryStream::Episodic.to_string(), "memory:episodic");
        assert_eq!(MemoryStream::Semantic.to_string(), "memory:semantic");
        assert_eq!(MemoryStream::Procedural.to_string(), "memory:procedural");
    }

    #[test]
    fn stream_config_custom() {
        let config = StreamConfig::new(Some(500), Some(3000), "custom_group");

        assert_eq!(config.maxlen, Some(500));
        assert_eq!(config.ttl_ms, Some(3000));
        assert_eq!(config.consumer_group, "custom_group");
    }

    #[test]
    fn default_stream_config_is_working_memory() {
        let default = StreamConfig::default();
        let working = StreamConfig::working_memory();

        assert_eq!(default, working);
    }
}

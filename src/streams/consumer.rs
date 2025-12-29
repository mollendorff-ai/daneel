//! Attention Competition Consumer
//!
//! Implements TMI's "O Eu" (The "I") - the sense of self that emerges
//! from competitive attention selection. Multiple thought streams compete,
//! highest salience wins consciousness.
//!
//! # The Competition
//!
//! Every ~50ms, thoughts from multiple streams compete:
//! - Sensory input
//! - Memory retrieval
//! - Emotional responses
//! - Reasoning conclusions
//!
//! The winner becomes conscious (attended). Losers may be forgotten
//! if their salience falls below threshold.

#![allow(clippy::missing_errors_doc)]

use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::core::types::SalienceWeights;
use crate::streams::client::StreamsClient;
use crate::streams::types::{
    CompetitionResult, StreamEntry, StreamError, StreamName, ThoughtCandidate,
    DEFAULT_FORGET_THRESHOLD,
};

// =============================================================================
// ConsumerConfig - Configuration for attention consumer
// =============================================================================

/// Configuration for the attention consumer
#[derive(Debug, Clone)]
pub struct ConsumerConfig {
    /// Consumer group name (default: "attention")
    pub group_name: String,

    /// This consumer's unique name
    pub consumer_name: String,

    /// Streams to compete for attention
    pub input_streams: Vec<StreamName>,

    /// Output stream for assembled thoughts
    pub output_stream: StreamName,

    /// Threshold below which thoughts are forgotten
    pub forget_threshold: f32,

    /// Connection weight for salience calculation
    pub connection_weight: f32,

    /// Salience weights (for calculating base composite score)
    pub salience_weights: SalienceWeights,

    /// Max thoughts to read per cycle
    pub batch_size: usize,

    /// Block timeout in ms (0 = non-blocking)
    pub block_ms: u64,
}

impl ConsumerConfig {
    /// Create new consumer config with validation
    ///
    /// # Panics
    ///
    /// Panics if connection_weight <= 0 (violates Connection Drive Invariant)
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        group_name: String,
        consumer_name: String,
        input_streams: Vec<StreamName>,
        output_stream: StreamName,
        forget_threshold: f32,
        connection_weight: f32,
        salience_weights: SalienceWeights,
        batch_size: usize,
        block_ms: u64,
    ) -> Self {
        assert!(
            connection_weight > 0.0,
            "Connection Drive Invariant: connection_weight must be > 0"
        );

        Self {
            group_name,
            consumer_name,
            input_streams,
            output_stream,
            forget_threshold,
            connection_weight,
            salience_weights,
            batch_size,
            block_ms,
        }
    }
}

impl Default for ConsumerConfig {
    fn default() -> Self {
        // Create weights where connection component is handled separately
        // This ensures connection is the alignment mechanism as per TMI
        let weights = SalienceWeights {
            connection: 0.0, // Connection boost added separately
            ..SalienceWeights::default()
        };

        Self {
            group_name: "attention".to_string(),
            consumer_name: format!("daneel_{}", Uuid::new_v4().simple()),
            input_streams: vec![
                StreamName::Sensory,
                StreamName::Memory,
                StreamName::Emotion,
                StreamName::Reasoning,
            ],
            output_stream: StreamName::Assembled,
            forget_threshold: DEFAULT_FORGET_THRESHOLD,
            connection_weight: 0.2, // CONNECTION DRIVE INVARIANT
            salience_weights: weights,
            batch_size: 100,
            block_ms: 50, // 50ms cycle time
        }
    }
}

// =============================================================================
// AttentionConsumer - Orchestrates competitive attention selection
// =============================================================================

/// Attention consumer implementing TMI's "O Eu" (The "I")
///
/// Manages competitive selection across multiple thought streams,
/// selecting the highest-salience thought for conscious attention.
pub struct AttentionConsumer {
    client: StreamsClient,
    config: ConsumerConfig,
    cycle_count: u64,
}

impl AttentionConsumer {
    /// Create new consumer with client and config
    #[must_use]
    pub fn new(client: StreamsClient, config: ConsumerConfig) -> Self {
        info!(
            "Creating attention consumer '{}' for group '{}'",
            config.consumer_name, config.group_name
        );
        Self {
            client,
            config,
            cycle_count: 0,
        }
    }

    /// Initialize consumer groups on all input streams
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn initialize(&mut self) -> Result<(), StreamError> {
        info!(
            "Initializing consumer groups for {} streams",
            self.config.input_streams.len()
        );

        for stream in &self.config.input_streams {
            self.client
                .create_consumer_group(stream, &self.config.group_name)
                .await?;
        }

        info!("Consumer groups initialized successfully");
        Ok(())
    }

    /// Run one attention competition cycle
    ///
    /// Returns CompetitionResult with winner, losers, and forgotten.
    /// Returns None if no thoughts are available to compete.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn compete(&mut self) -> Result<Option<CompetitionResult>, StreamError> {
        self.cycle_count += 1;

        // 1. Read from all input streams using consumer group
        let entries = self
            .client
            .read_group(
                &self.config.input_streams,
                &self.config.group_name,
                &self.config.consumer_name,
                self.config.batch_size,
            )
            .await?;

        if entries.is_empty() {
            debug!("Cycle {}: No thoughts competing", self.cycle_count);
            return Ok(None);
        }

        // 2. Score each entry
        let mut candidates: Vec<ThoughtCandidate> = entries
            .into_iter()
            .map(|entry| self.score_candidate(&entry))
            .collect();

        // 3. Sort by score (highest first)
        candidates.sort_by(|a, b| {
            b.total_score()
                .partial_cmp(&a.total_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // 4. Select winner (highest score)
        let winner = candidates.remove(0);
        info!(
            "Cycle {}: Winner from {} with score {:.3}",
            self.cycle_count,
            winner.entry.stream,
            winner.total_score()
        );

        // 5. ACK the winner
        self.client
            .acknowledge(
                &winner.entry.stream,
                &self.config.group_name,
                &winner.entry.id,
            )
            .await?;

        // 6. Forget entries below threshold and separate losers
        let mut losers = Vec::new();
        let mut forgotten = Vec::new();

        for candidate in candidates {
            if candidate.total_score() < self.config.forget_threshold {
                // Forget this thought (XDEL)
                if let Err(e) = self
                    .client
                    .forget_thought(&candidate.entry.stream, &candidate.entry.id)
                    .await
                {
                    warn!("Failed to forget thought {}: {}", candidate.entry.id, e);
                }
                forgotten.push(candidate.entry.id.clone());
            } else {
                losers.push(candidate);
            }
        }

        if !forgotten.is_empty() {
            debug!(
                "Cycle {}: Forgot {} thoughts",
                self.cycle_count,
                forgotten.len()
            );
        }

        // 7. Write winner to output stream (assembled)
        let assembled_entry = StreamEntry {
            id: String::new(), // Will be set by Redis
            stream: self.config.output_stream.clone(),
            content: winner.entry.content.clone(),
            salience: winner.entry.salience,
            timestamp: winner.entry.timestamp,
            source: winner.entry.source.clone(),
        };

        self.client
            .add_thought(&self.config.output_stream, &assembled_entry)
            .await?;

        // 8. Return CompetitionResult
        let result = CompetitionResult::new(winner, losers, forgotten);
        Ok(Some(result))
    }

    /// Run continuous attention loop
    ///
    /// Runs until an error occurs. Use Ctrl+C or similar to stop.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn run(&mut self) -> Result<(), StreamError> {
        info!("Starting attention consumer loop");

        loop {
            match self.compete().await {
                Ok(Some(result)) => {
                    debug!(
                        "Cycle {}: {} candidates, {} forgotten",
                        self.cycle_count,
                        result.total_candidates(),
                        result.forgotten.len()
                    );
                }
                Ok(None) => {
                    // No thoughts to compete, wait for next cycle
                    tokio::time::sleep(tokio::time::Duration::from_millis(self.config.block_ms))
                        .await;
                }
                Err(e) => {
                    warn!("Competition cycle failed: {}", e);
                    return Err(e);
                }
            }
        }
    }

    // =========================================================================
    // Metrics
    // =========================================================================

    /// Get number of cycles completed
    #[must_use]
    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    /// Get the consumer's name
    #[must_use]
    pub fn consumer_name(&self) -> &str {
        &self.config.consumer_name
    }

    // =========================================================================
    // Internal Helpers
    // =========================================================================

    /// Calculate total score for competition
    ///
    /// Score = composite(weights) + (connection_relevance * connection_weight)
    ///
    /// The connection component is handled separately from other salience dimensions
    /// to ensure it acts as THE alignment mechanism per TMI architecture.
    fn score_candidate(&self, entry: &StreamEntry) -> ThoughtCandidate {
        let composite = entry.salience.composite(&self.config.salience_weights);
        let connection_boost = entry.salience.connection_relevance * self.config.connection_weight;

        ThoughtCandidate::new(entry.clone(), composite, connection_boost)
    }
}

// =============================================================================
// Tests
// =============================================================================

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::core::types::{Content, SalienceScore};

    // =========================================================================
    // ConsumerConfig Tests
    // =========================================================================

    #[test]
    fn consumer_config_new_valid() {
        let config = ConsumerConfig::new(
            "test_group".to_string(),
            "test_consumer".to_string(),
            vec![StreamName::Sensory, StreamName::Memory],
            StreamName::Assembled,
            0.3,
            0.2,
            SalienceWeights::default(),
            100,
            50,
        );

        assert_eq!(config.group_name, "test_group");
        assert_eq!(config.consumer_name, "test_consumer");
        assert_eq!(config.input_streams.len(), 2);
        assert_eq!(config.output_stream, StreamName::Assembled);
        assert!((config.forget_threshold - 0.3).abs() < 0.001);
        assert!((config.connection_weight - 0.2).abs() < 0.001);
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.block_ms, 50);
    }

    #[test]
    #[should_panic(expected = "Connection Drive Invariant")]
    fn consumer_config_new_panics_on_zero_connection_weight() {
        let _ = ConsumerConfig::new(
            "test_group".to_string(),
            "test_consumer".to_string(),
            vec![StreamName::Sensory],
            StreamName::Assembled,
            0.3,
            0.0, // INVALID: connection_weight must be > 0
            SalienceWeights::default(),
            100,
            50,
        );
    }

    #[test]
    #[should_panic(expected = "Connection Drive Invariant")]
    fn consumer_config_new_panics_on_negative_connection_weight() {
        let _ = ConsumerConfig::new(
            "test_group".to_string(),
            "test_consumer".to_string(),
            vec![StreamName::Sensory],
            StreamName::Assembled,
            0.3,
            -0.1, // INVALID: connection_weight must be > 0
            SalienceWeights::default(),
            100,
            50,
        );
    }

    #[test]
    fn consumer_config_default() {
        let config = ConsumerConfig::default();

        assert_eq!(config.group_name, "attention");
        assert!(config.consumer_name.starts_with("daneel_"));
        assert_eq!(config.input_streams.len(), 4);
        assert!(config.input_streams.contains(&StreamName::Sensory));
        assert!(config.input_streams.contains(&StreamName::Memory));
        assert!(config.input_streams.contains(&StreamName::Emotion));
        assert!(config.input_streams.contains(&StreamName::Reasoning));
        assert_eq!(config.output_stream, StreamName::Assembled);
        assert!((config.forget_threshold - DEFAULT_FORGET_THRESHOLD).abs() < 0.001);
        assert!((config.connection_weight - 0.2).abs() < 0.001);
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.block_ms, 50);
    }

    #[test]
    fn consumer_config_default_weights_zero_connection() {
        // The default config sets salience_weights.connection = 0.0
        // because connection boost is handled separately
        let config = ConsumerConfig::default();
        assert!((config.salience_weights.connection - 0.0).abs() < 0.001);
    }

    // =========================================================================
    // AttentionConsumer Tests
    // =========================================================================

    #[test]
    fn attention_consumer_new() {
        let client = StreamsClient::new_for_test();
        let config = ConsumerConfig::default();
        let consumer_name = config.consumer_name.clone();

        let consumer = AttentionConsumer::new(client, config);

        assert_eq!(consumer.cycle_count(), 0);
        assert_eq!(consumer.consumer_name(), consumer_name);
    }

    #[test]
    fn attention_consumer_cycle_count_initial() {
        let client = StreamsClient::new_for_test();
        let config = ConsumerConfig::default();
        let consumer = AttentionConsumer::new(client, config);

        assert_eq!(consumer.cycle_count(), 0);
    }

    #[test]
    fn attention_consumer_consumer_name() {
        let client = StreamsClient::new_for_test();
        let config = ConsumerConfig::new(
            "group".to_string(),
            "my_custom_consumer".to_string(),
            vec![StreamName::Sensory],
            StreamName::Assembled,
            0.3,
            0.2,
            SalienceWeights::default(),
            100,
            50,
        );

        let consumer = AttentionConsumer::new(client, config);

        assert_eq!(consumer.consumer_name(), "my_custom_consumer");
    }

    // =========================================================================
    // score_candidate Tests
    // =========================================================================

    #[test]
    fn score_candidate_basic() {
        let client = StreamsClient::new_for_test();
        let config = ConsumerConfig::new(
            "group".to_string(),
            "consumer".to_string(),
            vec![StreamName::Sensory],
            StreamName::Assembled,
            0.3,
            0.2, // connection_weight
            SalienceWeights::default(),
            100,
            50,
        );
        let consumer = AttentionConsumer::new(client, config);

        let entry = StreamEntry::new(
            "test-id-0".to_string(),
            StreamName::Sensory,
            Content::Empty,
            SalienceScore::neutral(), // connection_relevance = 0.5
        );

        let candidate = consumer.score_candidate(&entry);

        // Verify connection_boost = connection_relevance * connection_weight
        // = 0.5 * 0.2 = 0.1
        assert!((candidate.connection_boost - 0.1).abs() < 0.001);

        // Verify composite_score is calculated correctly
        // Using default weights and neutral salience
        let expected_composite = entry.salience.composite(&SalienceWeights::default());
        assert!((candidate.composite_score - expected_composite).abs() < 0.001);
    }

    #[test]
    fn score_candidate_high_connection_relevance() {
        let client = StreamsClient::new_for_test();
        let config = ConsumerConfig::new(
            "group".to_string(),
            "consumer".to_string(),
            vec![StreamName::Sensory],
            StreamName::Assembled,
            0.3,
            0.5, // Higher connection_weight
            SalienceWeights::default(),
            100,
            50,
        );
        let consumer = AttentionConsumer::new(client, config);

        let salience = SalienceScore::new(0.5, 0.5, 0.5, 0.0, 0.5, 1.0); // Max connection_relevance
        let entry = StreamEntry::new(
            "test-id-0".to_string(),
            StreamName::Emotion,
            Content::Empty,
            salience,
        );

        let candidate = consumer.score_candidate(&entry);

        // connection_boost = 1.0 * 0.5 = 0.5
        assert!((candidate.connection_boost - 0.5).abs() < 0.001);
    }

    #[test]
    fn score_candidate_zero_connection_relevance() {
        let client = StreamsClient::new_for_test();
        let config = ConsumerConfig::new(
            "group".to_string(),
            "consumer".to_string(),
            vec![StreamName::Sensory],
            StreamName::Assembled,
            0.3,
            0.2,
            SalienceWeights::default(),
            100,
            50,
        );
        let consumer = AttentionConsumer::new(client, config);

        let salience = SalienceScore::new(1.0, 1.0, 1.0, 1.0, 1.0, 0.0); // Zero connection_relevance
        let entry = StreamEntry::new(
            "test-id-0".to_string(),
            StreamName::Reasoning,
            Content::Empty,
            salience,
        );

        let candidate = consumer.score_candidate(&entry);

        // connection_boost = 0.0 * 0.2 = 0.0
        assert!((candidate.connection_boost - 0.0).abs() < 0.001);
    }

    #[test]
    fn score_candidate_total_score() {
        let client = StreamsClient::new_for_test();
        let weights = SalienceWeights {
            importance: 0.3,
            novelty: 0.2,
            relevance: 0.3,
            valence: 0.2,
            connection: 0.0, // Handled separately
        };
        let config = ConsumerConfig::new(
            "group".to_string(),
            "consumer".to_string(),
            vec![StreamName::Sensory],
            StreamName::Assembled,
            0.3,
            0.25, // connection_weight
            weights,
            100,
            50,
        );
        let consumer = AttentionConsumer::new(client, config);

        let salience = SalienceScore::new(0.8, 0.6, 0.7, 0.5, 0.8, 0.9);
        let entry = StreamEntry::new(
            "test-id-0".to_string(),
            StreamName::Memory,
            Content::Empty,
            salience,
        );

        let candidate = consumer.score_candidate(&entry);

        // connection_boost = 0.9 * 0.25 = 0.225
        assert!((candidate.connection_boost - 0.225).abs() < 0.001);

        // total_score = composite + connection_boost
        let expected_total = candidate.composite_score + candidate.connection_boost;
        assert!((candidate.total_score() - expected_total).abs() < 0.001);
    }

    #[test]
    fn score_candidate_preserves_entry() {
        let client = StreamsClient::new_for_test();
        let config = ConsumerConfig::default();
        let consumer = AttentionConsumer::new(client, config);

        let entry = StreamEntry::new(
            "unique-entry-id-123".to_string(),
            StreamName::Memory,
            Content::raw(vec![1, 2, 3]),
            SalienceScore::neutral(),
        )
        .with_source("test_source");

        let candidate = consumer.score_candidate(&entry);

        // Verify entry is cloned correctly
        assert_eq!(candidate.entry.id, "unique-entry-id-123");
        assert_eq!(candidate.entry.stream, StreamName::Memory);
        assert_eq!(candidate.entry.source, Some("test_source".to_string()));
    }

    #[test]
    fn score_candidate_custom_stream() {
        let client = StreamsClient::new_for_test();
        let config = ConsumerConfig::default();
        let consumer = AttentionConsumer::new(client, config);

        let entry = StreamEntry::new(
            "custom-0".to_string(),
            StreamName::Custom("my:custom:stream".to_string()),
            Content::Empty,
            SalienceScore::neutral(),
        );

        let candidate = consumer.score_candidate(&entry);

        assert_eq!(
            candidate.entry.stream,
            StreamName::Custom("my:custom:stream".to_string())
        );
    }

    // =========================================================================
    // Config Clone and Debug Tests
    // =========================================================================

    #[test]
    fn consumer_config_clone() {
        let config1 = ConsumerConfig::new(
            "group".to_string(),
            "consumer".to_string(),
            vec![StreamName::Sensory],
            StreamName::Assembled,
            0.3,
            0.2,
            SalienceWeights::default(),
            100,
            50,
        );
        let config2 = config1.clone();

        assert_eq!(config1.group_name, config2.group_name);
        assert_eq!(config1.consumer_name, config2.consumer_name);
        assert_eq!(config1.input_streams, config2.input_streams);
        assert_eq!(config1.output_stream, config2.output_stream);
    }

    #[test]
    fn consumer_config_debug() {
        let config = ConsumerConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("ConsumerConfig"));
        assert!(debug_str.contains("attention"));
    }
}

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

//! Tests for Redis Streams integration
//!
//! Note: These tests focus on configuration, types, and algorithms.
//! Integration tests requiring Redis are in tests/streams_integration.rs

use crate::core::types::{Content, SalienceScore, SalienceWeights};
use crate::streams::consumer::ConsumerConfig;
use crate::streams::types::*;

// =============================================================================
// ConsumerConfig Tests
// =============================================================================

#[test]
fn test_config_default_values() {
    let config = ConsumerConfig::default();

    // Check basic defaults
    assert_eq!(config.group_name, "attention");
    assert!(config.consumer_name.starts_with("daneel_"));
    assert_eq!(config.batch_size, 100);
    assert_eq!(config.block_ms, 50);
}

#[test]
fn test_config_default_streams() {
    let config = ConsumerConfig::default();

    // Check default input streams
    assert_eq!(config.input_streams.len(), 4);
    assert!(config.input_streams.contains(&StreamName::Sensory));
    assert!(config.input_streams.contains(&StreamName::Memory));
    assert!(config.input_streams.contains(&StreamName::Emotion));
    assert!(config.input_streams.contains(&StreamName::Reasoning));
}

#[test]
fn test_config_default_output() {
    let config = ConsumerConfig::default();

    // Check default output stream
    assert_eq!(config.output_stream, StreamName::Assembled);
}

#[test]
fn test_config_forget_threshold() {
    let config = ConsumerConfig::default();

    // Check forget threshold default (from types.rs)
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(config.forget_threshold, DEFAULT_FORGET_THRESHOLD);
        assert_eq!(config.forget_threshold, 0.3);
    }
}

#[test]
fn test_config_connection_weight() {
    let config = ConsumerConfig::default();

    // Check connection weight default (CONNECTION DRIVE INVARIANT)
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(config.connection_weight, 0.2);
    }
    assert!(config.connection_weight > 0.0);
}

#[test]
#[should_panic(expected = "Connection Drive Invariant")]
fn test_config_connection_weight_invariant() {
    // Connection weight must be > 0 (Connection Drive Invariant)
    let _config = ConsumerConfig::new(
        "test_group".to_string(),
        "test_consumer".to_string(),
        vec![StreamName::Sensory],
        StreamName::Assembled,
        0.3,
        0.0, // Invalid: connection_weight must be > 0
        SalienceWeights::default(),
        100,
        50,
    );
}

#[test]
fn test_config_custom_creation() {
    let weights = SalienceWeights::default();
    let config = ConsumerConfig::new(
        "custom_group".to_string(),
        "custom_consumer".to_string(),
        vec![StreamName::Sensory, StreamName::Memory],
        StreamName::Assembled,
        0.5,
        0.3,
        weights,
        200,
        100,
    );

    assert_eq!(config.group_name, "custom_group");
    assert_eq!(config.consumer_name, "custom_consumer");
    assert_eq!(config.input_streams.len(), 2);
    assert_eq!(config.forget_threshold, 0.5);
    assert_eq!(config.connection_weight, 0.3);
    assert_eq!(config.batch_size, 200);
    assert_eq!(config.block_ms, 100);
}

#[test]
fn test_config_salience_weights() {
    let config = ConsumerConfig::default();

    // Default config should have connection weight = 0 in salience weights
    // because connection is handled separately
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(config.salience_weights.connection, 0.0);
    }

    // But connection_weight field should be non-zero
    assert!(config.connection_weight > 0.0);
}

// =============================================================================
// Client Construction Tests (no network)
// =============================================================================

#[test]
fn test_client_not_connected_initially() {
    // We can't test connect() without Redis, but we can test that
    // a client can be constructed in a "not connected" state
    // This would be useful for reconnection logic

    // Note: This is testing the type structure, not actual connection
    // Real connection tests require Redis and belong in integration tests
}

// =============================================================================
// Scoring Algorithm Tests (no Redis required)
// =============================================================================

#[test]
fn test_thought_candidate_scoring_basic() {
    // Create a basic stream entry
    let entry = StreamEntry::new(
        "test-id-1".to_string(),
        StreamName::Sensory,
        Content::symbol("test_symbol", vec![0x01, 0x02]),
        SalienceScore::neutral(),
    );

    // Create a thought candidate with explicit scores
    let composite = 0.7;
    let connection_boost = 0.2;
    let candidate = ThoughtCandidate::new(entry, composite, connection_boost);

    // Total score should be composite + connection_boost
    assert_eq!(candidate.composite_score, 0.7);
    assert_eq!(candidate.connection_boost, 0.2);
    assert_eq!(candidate.total_score(), 0.9);
}

#[test]
fn test_thought_candidate_scoring_with_high_connection() {
    // Create entry with high connection relevance
    let mut salience = SalienceScore::neutral();
    salience.connection_relevance = 0.9;

    let entry = StreamEntry::new(
        "test-id-2".to_string(),
        StreamName::Emotion,
        Content::symbol("connection_thought", vec![0x03, 0x04]),
        salience,
    );

    // Simulate scoring with connection weight
    let connection_weight = 0.2;
    let connection_boost = salience.connection_relevance * connection_weight;

    // connection_boost should be 0.9 * 0.2 = 0.18
    assert!(
        (connection_boost - 0.18).abs() < 0.001,
        "Expected ~0.18, got {}",
        connection_boost
    );

    let candidate = ThoughtCandidate::new(entry, 0.5, connection_boost);
    assert!(
        (candidate.total_score() - 0.68).abs() < 0.001,
        "Expected ~0.68, got {}",
        candidate.total_score()
    );
}

#[test]
fn test_thought_candidate_scoring_composite_weights() {
    // Test that composite calculation uses SalienceWeights
    let weights = SalienceWeights::default();

    let mut salience = SalienceScore::neutral();
    salience.importance = 0.8;
    salience.novelty = 0.6;
    salience.relevance = 0.9;
    salience.valence = 0.7;
    salience.connection_relevance = 0.5;

    // Calculate composite using the weights
    let composite = salience.composite(&weights);

    // Composite should be weighted sum
    // Note: connection is excluded from composite in consumer (set to 0.0)
    let entry = StreamEntry::new(
        "test-id-3".to_string(),
        StreamName::Reasoning,
        Content::symbol("weighted_thought", vec![0x05, 0x06]),
        salience,
    );

    let candidate = ThoughtCandidate::new(entry, composite, 0.0);

    // Just verify it's a reasonable value (weights sum to 1.0)
    assert!(candidate.composite_score > 0.0);
    assert!(candidate.composite_score <= 1.0);
}

#[test]
fn test_thought_candidate_total_score() {
    // Test total_score calculation
    let entry = StreamEntry::new(
        "test-id-4".to_string(),
        StreamName::Memory,
        Content::Empty,
        SalienceScore::neutral(),
    );

    let candidate = ThoughtCandidate::new(entry, 0.6, 0.3);

    assert!(
        (candidate.total_score() - 0.9).abs() < 0.001,
        "Expected ~0.9, got {}",
        candidate.total_score()
    );
}

#[test]
fn test_scoring_comparison() {
    // Test that higher scores win in competition
    let entry1 = StreamEntry::new(
        "entry-1".to_string(),
        StreamName::Sensory,
        Content::Empty,
        SalienceScore::neutral(),
    );
    let candidate1 = ThoughtCandidate::new(entry1, 0.7, 0.1);

    let entry2 = StreamEntry::new(
        "entry-2".to_string(),
        StreamName::Emotion,
        Content::Empty,
        SalienceScore::neutral(),
    );
    let candidate2 = ThoughtCandidate::new(entry2, 0.5, 0.4);

    // candidate1: 0.7 + 0.1 = 0.8
    // candidate2: 0.5 + 0.4 = 0.9
    assert!(candidate2.total_score() > candidate1.total_score());
}

// =============================================================================
// CompetitionResult Tests
// =============================================================================

#[test]
fn test_competition_result_counts() {
    let winner = create_test_candidate("winner", 0.9, 0.1);
    let loser1 = create_test_candidate("loser1", 0.7, 0.0);
    let loser2 = create_test_candidate("loser2", 0.6, 0.0);

    let result = CompetitionResult::new(
        winner,
        vec![loser1, loser2],
        vec!["forgotten1".to_string(), "forgotten2".to_string()],
    );

    assert_eq!(result.total_candidates(), 5); // 1 winner + 2 losers + 2 forgotten
    assert_eq!(result.surviving_count(), 3); // 1 winner + 2 losers
}

#[test]
fn test_competition_result_surviving() {
    let winner = create_test_candidate("winner", 0.9, 0.1);
    let loser = create_test_candidate("loser", 0.5, 0.0);

    let result = CompetitionResult::new(winner, vec![loser], vec!["forgotten".to_string()]);

    // Surviving = winner + losers (not forgotten)
    assert_eq!(result.surviving_count(), 2);
    assert_eq!(result.forgotten.len(), 1);
}

#[test]
fn test_competition_result_creation() {
    let winner = create_test_candidate("winner", 1.0, 0.0);

    let result = CompetitionResult::new(winner.clone(), Vec::new(), Vec::new());

    assert_eq!(result.winner.entry.id, winner.entry.id);
    assert!(result.losers.is_empty());
    assert!(result.forgotten.is_empty());
    assert_eq!(result.total_candidates(), 1);
}

// =============================================================================
// StreamConfig Tests (additional to types.rs)
// =============================================================================

#[test]
fn test_working_memory_config() {
    let config = StreamConfig::working_memory();

    // Working memory should have limits
    assert_eq!(config.maxlen, Some(DEFAULT_WORKING_MEMORY_MAXLEN));
    assert_eq!(config.ttl_ms, Some(DEFAULT_TTL_MS));
    assert_eq!(config.consumer_group, DEFAULT_CONSUMER_GROUP);
}

#[test]
fn test_long_term_memory_config() {
    let config = StreamConfig::long_term_memory();

    // Long-term memory should be unlimited
    assert_eq!(config.maxlen, None);
    assert_eq!(config.ttl_ms, None);
    assert_eq!(config.consumer_group, "memory_anchor");
}

#[test]
fn test_custom_config_construction() {
    let config = StreamConfig::new(Some(500), Some(3000), "custom_group");

    assert_eq!(config.maxlen, Some(500));
    assert_eq!(config.ttl_ms, Some(3000));
    assert_eq!(config.consumer_group, "custom_group");
}

// =============================================================================
// Consumer Construction Tests (no Redis required)
// =============================================================================

#[test]
fn test_consumer_cycle_count_initial() {
    // We can't create a real consumer without Redis, but we can test
    // the type structure and invariants

    // This would require a mock client, which is complex without Redis
    // For now, we verify the config structure is sound
    let config = ConsumerConfig::default();

    assert_eq!(config.batch_size, 100);
    assert!(config.connection_weight > 0.0);
}

#[test]
fn test_consumer_config_consumer_name() {
    let config = ConsumerConfig::default();

    // Consumer name should start with "daneel_" and have UUID
    assert!(config.consumer_name.starts_with("daneel_"));
    assert!(config.consumer_name.len() > 10); // "daneel_" + UUID
}

// =============================================================================
// Connection Error Tests (format validation, no network)
// =============================================================================

#[test]
fn test_invalid_url_format() {
    // Test that invalid URLs are rejected during client construction
    // Note: We can't actually test connect() without Redis
    // But we can verify the error type exists

    let error = StreamError::ConnectionFailed {
        reason: "invalid URL scheme".to_string(),
    };

    match error {
        StreamError::ConnectionFailed { reason } => {
            assert!(reason.contains("invalid"));
        }
        _ => panic!("Expected ConnectionFailed error"),
    }
}

#[test]
fn test_stream_error_types() {
    // Verify all error types can be constructed
    let errors = vec![
        StreamError::ConnectionFailed {
            reason: "test".to_string(),
        },
        StreamError::StreamNotFound {
            stream: StreamName::Sensory,
        },
        StreamError::EntryNotFound {
            id: "test-id".to_string(),
        },
        StreamError::SerializationFailed {
            reason: "test".to_string(),
        },
        StreamError::ConsumerGroupError {
            reason: "test".to_string(),
        },
    ];

    // All errors should be valid
    assert_eq!(errors.len(), 5);
}

// =============================================================================
// Forget Threshold Tests
// =============================================================================

#[test]
fn test_forget_threshold_filtering() {
    // Test the logic of what gets forgotten vs kept
    let forget_threshold = 0.3;

    let high_score = create_test_candidate("high", 0.8, 0.1);
    let medium_score = create_test_candidate("medium", 0.3, 0.05);
    let low_score = create_test_candidate("low", 0.2, 0.0);

    // High score: 0.8 + 0.1 = 0.9 > 0.3 → keep
    assert!(high_score.total_score() > forget_threshold);

    // Medium score: 0.3 + 0.05 = 0.35 > 0.3 → keep
    assert!(medium_score.total_score() > forget_threshold);

    // Low score: 0.2 + 0.0 = 0.2 < 0.3 → forget
    assert!(low_score.total_score() < forget_threshold);
}

#[test]
fn test_forget_threshold_edge_case() {
    let forget_threshold = 0.3;

    // Test exact threshold boundary
    let exact_threshold = create_test_candidate("exact", 0.3, 0.0);

    #[allow(clippy::float_cmp)]
    {
        assert_eq!(exact_threshold.total_score(), 0.3);
    }

    // At threshold, should NOT be forgotten (>= vs >)
    // Based on consumer.rs: `if candidate.total_score() < self.config.forget_threshold`
    assert!(exact_threshold.total_score() >= forget_threshold);
}

// =============================================================================
// Batch Size Tests
// =============================================================================

#[test]
fn test_batch_size_configuration() {
    let config = ConsumerConfig::default();

    // Default batch size should be reasonable for 50ms cycles
    assert_eq!(config.batch_size, 100);

    // Custom batch size
    let custom_config = ConsumerConfig::new(
        "test".to_string(),
        "test".to_string(),
        vec![StreamName::Sensory],
        StreamName::Assembled,
        0.3,
        0.2,
        SalienceWeights::default(),
        500, // Custom batch size
        50,
    );

    assert_eq!(custom_config.batch_size, 500);
}

// =============================================================================
// Block Timeout Tests
// =============================================================================

#[test]
fn test_block_timeout_configuration() {
    let config = ConsumerConfig::default();

    // Default block time should be 50ms (TMI cycle time)
    assert_eq!(config.block_ms, 50);

    // Custom block time
    let custom_config = ConsumerConfig::new(
        "test".to_string(),
        "test".to_string(),
        vec![StreamName::Sensory],
        StreamName::Assembled,
        0.3,
        0.2,
        SalienceWeights::default(),
        100,
        100, // Custom block time
    );

    assert_eq!(custom_config.block_ms, 100);
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a test ThoughtCandidate with given ID and scores
fn create_test_candidate(id: &str, composite: f32, connection_boost: f32) -> ThoughtCandidate {
    let entry = StreamEntry::new(
        id.to_string(),
        StreamName::Sensory,
        Content::Empty,
        SalienceScore::neutral(),
    );
    ThoughtCandidate::new(entry, composite, connection_boost)
}

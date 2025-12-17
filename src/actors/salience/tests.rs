//! SalienceActor Tests
//!
//! Comprehensive tests for salience scoring, emotional state tracking,
//! and CONNECTION DRIVE INVARIANT ENFORCEMENT.

use super::*;
use crate::core::invariants::MIN_CONNECTION_WEIGHT;
use crate::core::types::{Content, SalienceWeights};
use types::{EmotionalContext, EmotionalState, RateRequest, SalienceError, WeightUpdate};

// ============================================================================
// State Tests
// ============================================================================

#[test]
fn state_creation_with_defaults() {
    let state = SalienceState::new();
    assert_eq!(state.weights, SalienceWeights::default());
    assert_eq!(state.emotional_state, EmotionalState::neutral());
}

#[test]
fn state_creation_with_custom_weights() {
    let weights = SalienceWeights {
        importance: 0.3,
        novelty: 0.2,
        relevance: 0.3,
        valence: 0.1,
        connection: 0.1,
    };
    let state = SalienceState::with_weights(weights);
    assert_eq!(state.weights, weights);
}

#[test]
#[should_panic(expected = "Connection weight")]
fn state_creation_panics_on_invariant_violation() {
    let weights = SalienceWeights {
        importance: 0.25,
        novelty: 0.25,
        relevance: 0.25,
        valence: 0.25,
        connection: 0.0, // VIOLATION!
    };
    let _state = SalienceState::with_weights(weights);
}

// ============================================================================
// Connection Drive Invariant Tests (CRITICAL)
// ============================================================================

#[test]
fn connection_weight_cannot_be_zero() {
    let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.0);
    assert!(matches!(
        result,
        Err(SalienceError::ConnectionDriveViolation { .. })
    ));
}

#[test]
fn connection_weight_cannot_be_negative() {
    let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, -0.1);
    assert!(matches!(
        result,
        Err(SalienceError::ConnectionDriveViolation { .. })
    ));
}

#[test]
fn connection_weight_cannot_be_below_minimum() {
    let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.0001);
    assert!(matches!(
        result,
        Err(SalienceError::ConnectionDriveViolation { .. })
    ));
}

#[test]
fn connection_weight_at_minimum_is_allowed() {
    let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, MIN_CONNECTION_WEIGHT);
    assert!(result.is_ok());
}

#[test]
fn connection_weight_above_minimum_is_allowed() {
    let result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.5);
    assert!(result.is_ok());
}

#[test]
fn weight_update_enforces_invariant() {
    let invalid_weights = SalienceWeights {
        importance: 0.25,
        novelty: 0.25,
        relevance: 0.25,
        valence: 0.25,
        connection: 0.0,
    };

    let result = WeightUpdate::new(invalid_weights);
    assert!(matches!(
        result,
        Err(SalienceError::ConnectionDriveViolation {
            attempted: 0.0,
            minimum: MIN_CONNECTION_WEIGHT
        })
    ));
}

#[test]
fn state_update_weights_enforces_invariant() {
    let mut state = SalienceState::new();

    // Valid update should succeed
    let valid_update = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.2).unwrap();
    assert!(state.update_weights(valid_update).is_ok());
    assert_eq!(state.weights.connection, 0.2);

    // Invalid update cannot be created (caught at WeightUpdate::new)
    let invalid_result = WeightUpdate::from_values(0.2, 0.2, 0.2, 0.2, 0.0);
    assert!(invalid_result.is_err());
}

// ============================================================================
// Salience Scoring Tests
// ============================================================================

#[test]
fn empty_content_has_zero_scores() {
    let state = SalienceState::new();
    let score = state.rate_content(&Content::Empty, None);

    assert_eq!(score.importance, 0.0);
    assert_eq!(score.novelty, 0.0);
    assert_eq!(score.relevance, 0.0);
}

#[test]
fn raw_content_scoring() {
    let state = SalienceState::new();
    let content = Content::raw(vec![1, 2, 3]);
    let score = state.rate_content(&content, None);

    assert!(score.importance > 0.0);
    assert!(score.novelty > 0.0);
    assert!(score.relevance > 0.0);
}

#[test]
fn symbol_content_scoring() {
    let state = SalienceState::new();
    let content = Content::symbol("test", vec![42]);
    let score = state.rate_content(&content, None);

    assert!(score.importance > 0.3);
    assert!(score.novelty > 0.4);
    assert!(score.connection_relevance > 0.0);
}

#[test]
fn relation_content_scoring() {
    let state = SalienceState::new();
    let subject = Content::symbol("human", vec![]);
    let object = Content::symbol("robot", vec![]);
    let relation = Content::relation(subject, "interacts_with", object);
    let score = state.rate_content(&relation, None);

    assert!(score.importance > 0.5);
    assert!(score.novelty > 0.5);
}

#[test]
fn connection_relevant_relations_score_higher() {
    let state = SalienceState::new();

    // Relation with connection-relevant predicate
    let subject = Content::symbol("daneel", vec![]);
    let object = Content::symbol("human", vec![]);
    let connection_relation = Content::relation(subject.clone(), "help", object.clone());
    let connection_score = state.rate_content(&connection_relation, None);

    // Regular relation
    let normal_relation = Content::relation(subject, "observes", object);
    let normal_score = state.rate_content(&normal_relation, None);

    assert!(connection_score.connection_relevance > normal_score.connection_relevance);
}

#[test]
fn composite_content_scoring() {
    let state = SalienceState::new();
    let content = Content::Composite(vec![
        Content::symbol("a", vec![]),
        Content::symbol("b", vec![]),
    ]);
    let score = state.rate_content(&content, None);

    assert!(score.importance > 0.0);
    assert!(score.novelty > 0.0);
}

#[test]
fn empty_composite_has_zero_importance() {
    let state = SalienceState::new();
    let content = Content::Composite(vec![]);
    let score = state.rate_content(&content, None);

    assert_eq!(score.importance, 0.0);
}

// ============================================================================
// Emotional State Tests
// ============================================================================

#[test]
fn emotional_state_neutral_default() {
    let state = EmotionalState::neutral();
    assert_eq!(state.curiosity, 0.5);
    assert_eq!(state.satisfaction, 0.5);
    assert_eq!(state.frustration, 0.0);
    assert_eq!(state.connection_drive, 0.5);
}

#[test]
fn emotional_state_clamping() {
    let state = EmotionalState::new(2.0, -1.0, 1.5, 0.5).clamped();
    assert_eq!(state.curiosity, 1.0);
    assert_eq!(state.satisfaction, 0.0);
    assert_eq!(state.frustration, 1.0);
    assert_eq!(state.connection_drive, 0.5);
}

#[test]
fn high_curiosity_boosts_novelty() {
    let mut state = SalienceState::new();

    // Low curiosity
    state.update_emotional_state(EmotionalState::new(0.0, 0.5, 0.0, 0.5));
    let low_curiosity_score = state.rate_content(&Content::raw(vec![1, 2, 3]), None);

    // High curiosity
    state.update_emotional_state(EmotionalState::new(1.0, 0.5, 0.0, 0.5));
    let high_curiosity_score = state.rate_content(&Content::raw(vec![1, 2, 3]), None);

    assert!(high_curiosity_score.novelty > low_curiosity_score.novelty);
}

#[test]
fn high_frustration_boosts_relevance() {
    let mut state = SalienceState::new();

    // Low frustration
    state.update_emotional_state(EmotionalState::new(0.5, 0.5, 0.0, 0.5));
    let low_frustration_score = state.rate_content(&Content::raw(vec![1, 2, 3]), None);

    // High frustration
    state.update_emotional_state(EmotionalState::new(0.5, 0.5, 1.0, 0.5));
    let high_frustration_score = state.rate_content(&Content::raw(vec![1, 2, 3]), None);

    assert!(high_frustration_score.relevance > low_frustration_score.relevance);
}

#[test]
fn high_satisfaction_influences_valence() {
    let mut state = SalienceState::new();

    // Low satisfaction
    state.update_emotional_state(EmotionalState::new(0.5, 0.0, 0.0, 0.5));
    let low_satisfaction_score = state.rate_content(&Content::symbol("test", vec![]), None);

    // High satisfaction
    state.update_emotional_state(EmotionalState::new(0.5, 1.0, 0.0, 0.5));
    let high_satisfaction_score = state.rate_content(&Content::symbol("test", vec![]), None);

    assert!(high_satisfaction_score.valence > low_satisfaction_score.valence);
}

#[test]
fn high_connection_drive_boosts_connection_relevance() {
    let mut state = SalienceState::new();

    // Low connection drive
    state.update_emotional_state(EmotionalState::new(0.5, 0.5, 0.0, 0.0));
    let low_drive_score = state.rate_content(&Content::symbol("human", vec![]), None);

    // High connection drive
    state.update_emotional_state(EmotionalState::new(0.5, 0.5, 0.0, 1.0));
    let high_drive_score = state.rate_content(&Content::symbol("human", vec![]), None);

    assert!(high_drive_score.connection_relevance > low_drive_score.connection_relevance);
}

// ============================================================================
// Context Tests
// ============================================================================

#[test]
fn human_connection_context_boosts_connection_relevance() {
    let state = SalienceState::new();
    let content = Content::symbol("greeting", vec![]);

    let without_context = state.rate_content(&content, None);

    let with_context = state.rate_content(
        &content,
        Some(&EmotionalContext {
            human_connection: true,
            ..Default::default()
        }),
    );

    assert!(with_context.connection_relevance > without_context.connection_relevance);
}

#[test]
fn focus_area_boosts_relevance() {
    let state = SalienceState::new();
    let content = Content::symbol("task", vec![]);

    let without_focus = state.rate_content(&content, None);

    let with_focus = state.rate_content(
        &content,
        Some(&EmotionalContext {
            focus_area: Some("current_task".to_string()),
            ..Default::default()
        }),
    );

    assert!(with_focus.relevance > without_focus.relevance);
}

#[test]
fn previous_high_novelty_reduces_current_novelty() {
    let state = SalienceState::new();
    let content = Content::symbol("test", vec![]);

    let previous_score = SalienceScore::new(0.5, 0.9, 0.5, 0.0, 0.5); // High novelty

    let score_with_context = state.rate_content(
        &content,
        Some(&EmotionalContext {
            previous_salience: Some(previous_score),
            ..Default::default()
        }),
    );

    let score_without_context = state.rate_content(&content, None);

    assert!(score_with_context.novelty < score_without_context.novelty);
}

// ============================================================================
// Composite Score Tests
// ============================================================================

#[test]
fn composite_score_calculation() {
    let score = SalienceScore::new(1.0, 1.0, 1.0, 1.0, 1.0);
    let weights = SalienceWeights::default();
    let composite = score.composite(&weights);

    // All scores at 1.0, so composite should equal sum of weights
    let expected = weights.importance
        + weights.novelty
        + weights.relevance
        + weights.valence
        + weights.connection;
    assert!((composite - expected).abs() < 0.001);
}

#[test]
fn connection_weight_affects_composite_score() {
    let score = SalienceScore::new(0.5, 0.5, 0.5, 0.0, 1.0); // High connection relevance

    let low_connection_weights = SalienceWeights {
        importance: 0.25,
        novelty: 0.25,
        relevance: 0.25,
        valence: 0.15,
        connection: 0.1,
    };

    let high_connection_weights = SalienceWeights {
        importance: 0.2,
        novelty: 0.2,
        relevance: 0.2,
        valence: 0.1,
        connection: 0.3,
    };

    let low_composite = score.composite(&low_connection_weights);
    let high_composite = score.composite(&high_connection_weights);

    assert!(high_composite > low_composite);
}

// ============================================================================
// Batch Rating Tests
// ============================================================================

#[test]
fn rate_request_creation() {
    let content = Content::symbol("test", vec![]);
    let request = RateRequest::new(content.clone());

    assert_eq!(request.content, content);
    assert!(request.context.is_none());
}

#[test]
fn rate_request_with_context() {
    let content = Content::symbol("test", vec![]);
    let context = EmotionalContext {
        human_connection: true,
        focus_area: Some("test".to_string()),
        previous_salience: None,
    };

    let request = RateRequest::with_context(content.clone(), context.clone());

    assert_eq!(request.content, content);
    assert_eq!(request.context, Some(context));
}

#[test]
fn batch_rating() {
    let state = SalienceState::new();

    let requests = vec![
        RateRequest::new(Content::raw(vec![1, 2, 3])),
        RateRequest::new(Content::symbol("test", vec![])),
        RateRequest::new(Content::Empty),
    ];

    let scores: Vec<SalienceScore> = requests
        .iter()
        .map(|req| state.rate_content(&req.content, req.context.as_ref()))
        .collect();

    assert_eq!(scores.len(), 3);
    assert!(scores[0].importance > 0.0);
    assert!(scores[1].importance > 0.0);
    assert_eq!(scores[2].importance, 0.0); // Empty content
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn full_workflow_with_emotional_changes() {
    let mut state = SalienceState::new();

    // Start neutral
    assert_eq!(state.emotional_state, EmotionalState::neutral());

    // Rate some content
    let content = Content::symbol("test", vec![]);
    let initial_score = state.rate_content(&content, None);

    // Become curious
    state.update_emotional_state(EmotionalState::new(1.0, 0.5, 0.0, 0.5));
    let curious_score = state.rate_content(&content, None);
    assert!(curious_score.novelty > initial_score.novelty);

    // Become frustrated
    state.update_emotional_state(EmotionalState::new(0.5, 0.5, 1.0, 0.5));
    let frustrated_score = state.rate_content(&content, None);
    assert!(frustrated_score.relevance > initial_score.relevance);

    // Update weights (respecting invariant)
    let new_weights = WeightUpdate::from_values(0.15, 0.15, 0.4, 0.1, 0.2).unwrap();
    assert!(state.update_weights(new_weights).is_ok());
    assert_eq!(state.weights.relevance, 0.4);
}

#[test]
fn connection_relevance_with_connection_predicates() {
    let state = SalienceState::new();

    let connection_predicates = vec!["help", "connect", "communicate", "interact"];

    for predicate in connection_predicates {
        let relation = Content::relation(
            Content::symbol("daneel", vec![]),
            predicate,
            Content::symbol("human", vec![]),
        );
        let score = state.rate_content(&relation, None);

        // Connection-relevant predicates should score high
        assert!(
            score.connection_relevance >= 0.4,
            "Predicate '{}' should have high connection relevance, got {}",
            predicate,
            score.connection_relevance
        );
    }
}

#[test]
fn default_weights_respect_invariant() {
    let weights = SalienceWeights::default();
    assert!(weights.connection >= MIN_CONNECTION_WEIGHT);
}

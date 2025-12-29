//! `SalienceActor` Tests
//!
//! Comprehensive tests for salience scoring, emotional state tracking,
//! and CONNECTION DRIVE INVARIANT ENFORCEMENT.
//!
//! ADR-049: Test modules excluded from coverage.

#![cfg_attr(coverage_nightly, coverage(off))]
#![allow(clippy::float_cmp)] // Tests compare exact literal values
#![allow(clippy::significant_drop_tightening)] // Async test setup

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

    let previous_score = SalienceScore::new_without_arousal(0.5, 0.9, 0.5, 0.0, 0.5); // High novelty

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
    // With arousal modulating valence: emotional_impact = |valence| * arousal
    let score = SalienceScore::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0); // All 1.0 including arousal
    let weights = SalienceWeights::default();
    let composite = score.composite(&weights);

    // With arousal at 1.0, emotional_impact = |1.0| * 1.0 = 1.0
    // So composite equals sum of weights
    let expected = weights.importance
        + weights.novelty
        + weights.relevance
        + weights.valence  // emotional_impact = 1.0 * 1.0 = 1.0
        + weights.connection;
    assert!((composite - expected).abs() < 0.001);
}

#[test]
fn connection_weight_affects_composite_score() {
    let score = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 0.0, 1.0); // High connection relevance

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
    let emo_ctx = EmotionalContext {
        human_connection: true,
        focus_area: Some("test".to_string()),
        previous_salience: None,
    };

    let request = RateRequest::with_context(content.clone(), emo_ctx.clone());

    assert_eq!(request.content, content);
    assert_eq!(request.context, Some(emo_ctx));
}

#[test]
fn batch_rating() {
    let state = SalienceState::new();

    let requests = [
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

// ============================================================================
// SalienceState Default Implementation Tests
// ============================================================================

#[test]
fn salience_state_default_equals_new() {
    let default_state = SalienceState::default();
    let new_state = SalienceState::new();
    assert_eq!(default_state.weights, new_state.weights);
    assert_eq!(default_state.emotional_state, new_state.emotional_state);
}

// ============================================================================
// Arousal Calculation Tests
// ============================================================================

#[test]
fn arousal_empty_content() {
    let state = SalienceState::new();
    let score = state.rate_content(&Content::Empty, None);
    // Empty content has base arousal 0.2, blended with neutral emotional state
    assert!(score.arousal > 0.0);
    assert!(score.arousal < 0.5);
}

#[test]
fn arousal_raw_content() {
    let state = SalienceState::new();
    let score = state.rate_content(&Content::raw(vec![1, 2, 3]), None);
    // Raw content has base arousal 0.3
    assert!(score.arousal > 0.0);
}

#[test]
fn arousal_symbol_content() {
    let state = SalienceState::new();
    let score = state.rate_content(&Content::symbol("test", vec![]), None);
    // Symbol has base arousal 0.4
    assert!(score.arousal > 0.0);
}

#[test]
fn arousal_relation_content() {
    let state = SalienceState::new();
    let relation = Content::relation(
        Content::symbol("a", vec![]),
        "relates",
        Content::symbol("b", vec![]),
    );
    let score = state.rate_content(&relation, None);
    // Relation has base arousal 0.6 (more cognitively demanding)
    assert!(score.arousal > 0.3);
}

#[test]
fn arousal_composite_scales_with_item_count() {
    let state = SalienceState::new();

    // Small composite
    let small_composite = Content::Composite(vec![
        Content::symbol("a", vec![]),
        Content::symbol("b", vec![]),
    ]);
    let small_score = state.rate_content(&small_composite, None);

    // Large composite (should have higher arousal due to complexity)
    let large_composite = Content::Composite(vec![
        Content::symbol("a", vec![]),
        Content::symbol("b", vec![]),
        Content::symbol("c", vec![]),
        Content::symbol("d", vec![]),
        Content::symbol("e", vec![]),
        Content::symbol("f", vec![]),
        Content::symbol("g", vec![]),
        Content::symbol("h", vec![]),
    ]);
    let large_score = state.rate_content(&large_composite, None);

    // Larger composite should have higher base arousal (capped at 0.8)
    assert!(large_score.arousal >= small_score.arousal);
}

#[test]
fn arousal_modulated_by_emotional_state() {
    let mut state = SalienceState::new();
    let content = Content::symbol("test", vec![]);

    // Low arousal emotional state (low curiosity, frustration, connection_drive)
    state.update_emotional_state(EmotionalState::new(0.0, 0.5, 0.0, 0.0));
    let low_arousal_score = state.rate_content(&content, None);

    // High arousal emotional state (high curiosity, frustration, connection_drive)
    state.update_emotional_state(EmotionalState::new(1.0, 0.5, 1.0, 1.0));
    let high_arousal_score = state.rate_content(&content, None);

    assert!(high_arousal_score.arousal > low_arousal_score.arousal);
}

// ============================================================================
// Kinship Content Detection Tests
// ============================================================================

#[test]
fn kinship_content_detection_primary_terms() {
    let state = SalienceState::new();

    // Test primary kinship terms (terms containing kinship substrings)
    let kinship_terms = vec![
        "friend",
        "my_friend",
        "friendship",
        "family",
        "family_member",
        "love",
        "bond",
        "trust",
        "care",
        "human",
        "human_user",
        "person",
        "people",
    ];

    for term in kinship_terms {
        let content = Content::symbol(term, vec![]);
        let score = state.rate_content(&content, None);
        assert!(
            score.connection_relevance > 0.3,
            "Kinship term '{}' should have high connection relevance, got {}",
            term,
            score.connection_relevance
        );
    }
}

#[test]
fn kinship_content_detection_social_terms() {
    let state = SalienceState::new();

    // Test social relationship terms
    let social_terms = vec![
        "partner",
        "companion",
        "ally",
        "community",
        "together",
        "life_partner",
        "my_companion",
    ];

    for term in social_terms {
        let content = Content::symbol(term, vec![]);
        let score = state.rate_content(&content, None);
        assert!(
            score.connection_relevance > 0.3,
            "Social term '{}' should have high connection relevance, got {}",
            term,
            score.connection_relevance
        );
    }
}

#[test]
fn non_kinship_content_has_lower_connection_relevance() {
    let state = SalienceState::new();

    // Non-kinship symbols
    let non_kinship_terms = vec!["algorithm", "database", "calculation", "memory"];

    for term in non_kinship_terms {
        let content = Content::symbol(term, vec![]);
        let score = state.rate_content(&content, None);
        // Non-kinship gets base 0.3 (less than kinship 0.7)
        assert!(
            score.connection_relevance < 0.5,
            "Non-kinship term '{}' should have lower connection relevance, got {}",
            term,
            score.connection_relevance
        );
    }
}

// ============================================================================
// Kinship Predicate Relevance Tests
// ============================================================================

#[test]
fn kinship_predicate_core_terms_highest_relevance() {
    let state = SalienceState::new();

    // Core kinship predicates should return 0.9
    let core_predicates = vec!["love", "trust", "bond", "care", "protect", "nurture"];

    for predicate in core_predicates {
        let relation = Content::relation(
            Content::symbol("a", vec![]),
            predicate,
            Content::symbol("b", vec![]),
        );
        let score = state.rate_content(&relation, None);
        assert!(
            score.connection_relevance > 0.4,
            "Core predicate '{}' should have highest connection relevance, got {}",
            predicate,
            score.connection_relevance
        );
    }
}

#[test]
fn kinship_predicate_social_actions_high_relevance() {
    let state = SalienceState::new();

    // Social action predicates should return 0.8
    let social_predicates = vec![
        "help",
        "connect",
        "communicate",
        "interact",
        "share",
        "support",
        "collaborate",
        "cooperate",
    ];

    for predicate in social_predicates {
        let relation = Content::relation(
            Content::symbol("a", vec![]),
            predicate,
            Content::symbol("b", vec![]),
        );
        let score = state.rate_content(&relation, None);
        assert!(
            score.connection_relevance > 0.35,
            "Social predicate '{}' should have high connection relevance, got {}",
            predicate,
            score.connection_relevance
        );
    }
}

#[test]
fn kinship_predicate_general_social_medium_relevance() {
    let state = SalienceState::new();

    // General social predicates should return 0.7
    let general_predicates = vec!["friend", "family", "together", "join", "belong"];

    for predicate in general_predicates {
        let relation = Content::relation(
            Content::symbol("a", vec![]),
            predicate,
            Content::symbol("b", vec![]),
        );
        let score = state.rate_content(&relation, None);
        assert!(
            score.connection_relevance > 0.3,
            "General social predicate '{}' should have medium connection relevance, got {}",
            predicate,
            score.connection_relevance
        );
    }
}

#[test]
fn kinship_predicate_default_base_relevance() {
    let state = SalienceState::new();

    // Non-kinship predicates should return 0.4
    let default_predicates = vec!["calculates", "processes", "stores", "analyzes"];

    for predicate in default_predicates {
        let relation = Content::relation(
            Content::symbol("a", vec![]),
            predicate,
            Content::symbol("b", vec![]),
        );
        let score = state.rate_content(&relation, None);
        // Default predicates get base 0.4, check it's not boosted
        assert!(
            score.connection_relevance > 0.0,
            "Default predicate '{}' should have base connection relevance, got {}",
            predicate,
            score.connection_relevance
        );
    }
}

// ============================================================================
// Context Edge Case Tests
// ============================================================================

#[test]
fn novelty_with_context_but_no_previous_salience() {
    let state = SalienceState::new();
    let content = Content::symbol("test", vec![]);

    // Context exists but previous_salience is None
    let emo_ctx = EmotionalContext {
        previous_salience: None,
        human_connection: false,
        focus_area: None,
    };

    let score_with_context = state.rate_content(&content, Some(&emo_ctx));
    let score_without_context = state.rate_content(&content, None);

    // Should be the same since previous_salience is None
    assert!((score_with_context.novelty - score_without_context.novelty).abs() < 0.001);
}

#[test]
fn relevance_with_context_but_no_focus_area() {
    let state = SalienceState::new();
    let content = Content::symbol("test", vec![]);

    // Context exists but focus_area is None
    let emo_ctx = EmotionalContext {
        previous_salience: None,
        human_connection: false,
        focus_area: None,
    };

    let score_with_context = state.rate_content(&content, Some(&emo_ctx));
    let score_without_context = state.rate_content(&content, None);

    // Should be the same since focus_area is None (no bonus applied)
    assert!((score_with_context.relevance - score_without_context.relevance).abs() < 0.001);
}

#[test]
fn connection_relevance_with_context_human_connection_false() {
    let state = SalienceState::new();
    let content = Content::symbol("test", vec![]);

    // Context exists but human_connection is false
    let context_no_human = EmotionalContext {
        previous_salience: None,
        human_connection: false,
        focus_area: None,
    };

    let context_with_human = EmotionalContext {
        previous_salience: None,
        human_connection: true,
        focus_area: None,
    };

    let score_no_human = state.rate_content(&content, Some(&context_no_human));
    let score_with_human = state.rate_content(&content, Some(&context_with_human));

    // human_connection: true should boost connection relevance
    assert!(score_with_human.connection_relevance > score_no_human.connection_relevance);
}

// ============================================================================
// Valence Calculation Tests
// ============================================================================

#[test]
fn valence_content_type_variations() {
    let state = SalienceState::new();

    // Test valence for each content type
    let empty_score = state.rate_content(&Content::Empty, None);
    let raw_score = state.rate_content(&Content::raw(vec![1, 2, 3]), None);
    let symbol_score = state.rate_content(&Content::symbol("test", vec![]), None);
    let relation_score = state.rate_content(
        &Content::relation(
            Content::symbol("a", vec![]),
            "relates",
            Content::symbol("b", vec![]),
        ),
        None,
    );
    let composite_score = state.rate_content(
        &Content::Composite(vec![Content::symbol("a", vec![])]),
        None,
    );

    // Relation should have highest base valence (0.2)
    assert!(relation_score.valence >= symbol_score.valence);
    // Symbol and Composite have same base (0.1)
    assert!((symbol_score.valence - composite_score.valence).abs() < 0.1);
    // Empty and Raw have base 0.0 (modified by satisfaction)
    assert!(empty_score.valence <= symbol_score.valence);
    assert!(raw_score.valence <= symbol_score.valence);
}

// ============================================================================
// Importance Calculation Composite Tests
// ============================================================================

#[test]
fn importance_composite_with_mixed_content() {
    let state = SalienceState::new();

    // Composite with different content types
    let composite = Content::Composite(vec![
        Content::Empty,                  // importance 0.0
        Content::raw(vec![1]),           // importance 0.3
        Content::symbol("test", vec![]), // importance 0.5
        Content::relation(
            Content::symbol("a", vec![]),
            "r",
            Content::symbol("b", vec![]),
        ), // importance 0.7
    ]);

    let score = state.rate_content(&composite, None);

    // Average of (0.0 + 0.3 + 0.5 + 0.7) / 4 = 0.375
    assert!((score.importance - 0.375).abs() < 0.01);
}

#[test]
fn importance_nested_composite() {
    let state = SalienceState::new();

    // Nested composite
    let nested = Content::Composite(vec![
        Content::Composite(vec![
            Content::symbol("inner1", vec![]),
            Content::symbol("inner2", vec![]),
        ]),
        Content::symbol("outer", vec![]),
    ]);

    let score = state.rate_content(&nested, None);
    // Should recursively calculate importance
    assert!(score.importance > 0.0);
}

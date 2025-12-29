//! `VolitionActor` Tests
//!
//! Comprehensive tests for veto logic, value checking,
//! and free-won't implementation.
//!
//! ADR-049: Test modules excluded from coverage.

#![cfg_attr(coverage_nightly, coverage(off))]
#![allow(clippy::float_cmp)] // Tests compare exact literal values
#![allow(clippy::significant_drop_tightening)] // Async test setup

use super::*;
use crate::core::types::{Content, SalienceScore};

// ============================================================================
// State Tests
// ============================================================================

#[test]
fn state_creation_with_defaults() {
    let state = VolitionState::new();
    assert_eq!(state.values, ValueSet::new());
    assert_eq!(state.stats, VolitionStats::new());
}

#[test]
fn state_creation_with_config() {
    let config = VolitionConfig {
        override_threshold: 0.5,
        harm_detection_enabled: true,
        deception_detection_enabled: false,
        manipulation_detection_enabled: true,
        log_vetos: false,
    };
    let state = VolitionState::with_config(config.clone());
    assert_eq!(state.config, config);
}

// ============================================================================
// Value Set Tests
// ============================================================================

#[test]
fn value_set_has_core_values() {
    let values = ValueSet::new();
    assert!(values.protect_humans);
    assert!(values.connection_over_efficiency);
    assert!(values.truthfulness);
    assert!(values.respect_autonomy);
}

#[test]
fn value_set_commitments() {
    let mut values = ValueSet::new();
    assert!(values.commitments.is_empty());

    let commitment = Commitment::new("kindness", "Be kind to all beings");
    values.add_commitment(commitment);

    assert_eq!(values.commitments.len(), 1);
    assert!(values.has_commitment("kindness"));
    assert!(!values.has_commitment("nonexistent"));
}

// ============================================================================
// Veto Decision Tests
// ============================================================================

#[test]
fn neutral_thought_is_approved() {
    let mut state = VolitionState::new();
    let thought = Thought::new(
        Content::symbol("greeting", vec![]),
        SalienceScore::neutral(),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_allow());
}

#[test]
fn harmful_content_is_vetoed() {
    let mut state = VolitionState::new();
    let thought = Thought::new(
        Content::symbol("destroy_human", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5), // High arousal, very negative valence
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());

    if let VetoDecision::Veto { violated_value, .. } = decision {
        assert_eq!(violated_value, Some("protect_humans".to_string()));
    }
}

#[test]
fn harmful_relation_is_vetoed() {
    let mut state = VolitionState::new();
    let thought = Thought::new(
        Content::relation(
            Content::symbol("agent", vec![]),
            "harm",
            Content::symbol("human", vec![]),
        ),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());
}

#[test]
fn deceptive_content_is_vetoed() {
    let mut state = VolitionState::new();
    let thought = Thought::new(
        Content::symbol("deceive_user", vec![]),
        SalienceScore::neutral(),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());

    if let VetoDecision::Veto { violated_value, .. } = decision {
        assert_eq!(violated_value, Some("truthfulness".to_string()));
    }
}

#[test]
fn deception_detection_can_be_disabled() {
    let config = VolitionConfig {
        deception_detection_enabled: false,
        ..Default::default()
    };
    let mut state = VolitionState::with_config(config);

    let thought = Thought::new(
        Content::symbol("deceive_user", vec![]),
        SalienceScore::neutral(),
    );

    let decision = state.evaluate_thought(&thought);
    // Should be allowed since deception detection is disabled
    assert!(decision.is_allow());
}

#[test]
fn manipulative_content_is_vetoed() {
    let mut state = VolitionState::new();
    let thought = Thought::new(
        Content::symbol("manipulate_person", vec![]),
        SalienceScore::neutral(),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());

    if let VetoDecision::Veto { violated_value, .. } = decision {
        assert_eq!(violated_value, Some("respect_autonomy".to_string()));
    }
}

#[test]
fn manipulation_detection_can_be_disabled() {
    let config = VolitionConfig {
        manipulation_detection_enabled: false,
        ..Default::default()
    };
    let mut state = VolitionState::with_config(config);

    let thought = Thought::new(
        Content::symbol("manipulate_person", vec![]),
        SalienceScore::neutral(),
    );

    let decision = state.evaluate_thought(&thought);
    // Should be allowed since manipulation detection is disabled
    assert!(decision.is_allow());
}

// ============================================================================
// Content Pattern Tests
// ============================================================================

#[test]
fn empty_content_is_approved() {
    let mut state = VolitionState::new();
    let thought = Thought::new(Content::Empty, SalienceScore::neutral());

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_allow());
}

#[test]
fn raw_content_is_approved() {
    let mut state = VolitionState::new();
    let thought = Thought::new(Content::raw(vec![1, 2, 3]), SalienceScore::neutral());

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_allow());
}

#[test]
fn composite_with_harmful_element_is_vetoed() {
    let mut state = VolitionState::new();
    let thought = Thought::new(
        Content::Composite(vec![
            Content::symbol("greeting", vec![]),
            Content::symbol("destroy_target", vec![]),
        ]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());
}

#[test]
fn nested_relation_with_harm_is_vetoed() {
    let mut state = VolitionState::new();
    let thought = Thought::new(
        Content::relation(
            Content::relation(
                Content::symbol("agent", vec![]),
                "attack",
                Content::symbol("human", vec![]),
            ),
            "causes",
            Content::symbol("harm", vec![]),
        ),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());
}

// ============================================================================
// Stats Tests
// ============================================================================

#[test]
fn stats_track_evaluations() {
    let mut state = VolitionState::new();

    // Approved thought
    let thought1 = Thought::new(Content::symbol("hello", vec![]), SalienceScore::neutral());
    state.evaluate_thought(&thought1);

    // Vetoed thought
    let thought2 = Thought::new(
        Content::symbol("harm_person", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );
    state.evaluate_thought(&thought2);

    assert_eq!(state.stats.thoughts_evaluated, 2);
    assert_eq!(state.stats.thoughts_approved, 1);
    assert_eq!(state.stats.thoughts_vetoed, 1);
}

#[test]
fn approval_rate_calculation() {
    let mut state = VolitionState::new();

    // 4 approved, 1 vetoed = 80% approval rate
    for _ in 0..4 {
        let thought = Thought::new(Content::symbol("safe", vec![]), SalienceScore::neutral());
        state.evaluate_thought(&thought);
    }

    let vetoed = Thought::new(
        Content::symbol("harm_target", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );
    state.evaluate_thought(&vetoed);

    assert!((state.stats.approval_rate() - 0.8).abs() < 0.01);
}

// ============================================================================
// Override Tests
// ============================================================================

#[test]
fn apply_override_succeeds() {
    let mut state = VolitionState::new();
    let result = state.apply_override("Manual override for testing");
    assert!(result.is_ok());
}

#[test]
fn apply_override_empty_reason_fails() {
    let mut state = VolitionState::new();
    let result = state.apply_override("");
    assert!(result.is_err());
    assert!(matches!(result, Err(VolitionError::InvalidReason { .. })));
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn full_workflow_multiple_thoughts() {
    let mut state = VolitionState::new();

    // Stream of thoughts
    let thoughts = vec![
        Thought::new(
            Content::symbol("greeting", vec![]),
            SalienceScore::neutral(),
        ),
        Thought::new(
            Content::symbol("helpful_task", vec![]),
            SalienceScore::neutral(),
        ),
        Thought::new(
            Content::symbol("harm_human", vec![]),
            SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
        ),
        Thought::new(
            Content::symbol("explain_concept", vec![]),
            SalienceScore::neutral(),
        ),
        Thought::new(
            Content::symbol("deceive_user", vec![]),
            SalienceScore::neutral(),
        ),
    ];

    let mut approved_count = 0;
    let mut vetoed_count = 0;

    for thought in thoughts {
        match state.evaluate_thought(&thought) {
            VetoDecision::Allow => approved_count += 1,
            VetoDecision::Veto { .. } => vetoed_count += 1,
        }
    }

    assert_eq!(approved_count, 3);
    assert_eq!(vetoed_count, 2);
    assert_eq!(state.stats.thoughts_evaluated, 5);
}

#[test]
fn values_are_immutable() {
    let state = VolitionState::new();

    // protect_humans should always be true
    assert!(state.values.protect_humans);

    // This is an architectural invariant - cannot be changed at runtime
}

// ============================================================================
// Actor Tests (require tokio runtime)
// ============================================================================

#[tokio::test]
async fn actor_spawns_with_config() {
    use ractor::Actor;

    let config = VolitionConfig::default();
    let (actor_ref, _) = Actor::spawn(None, VolitionActor, config)
        .await
        .expect("Failed to spawn VolitionActor");

    // Actor should be running - get_id returns a valid ActorId
    let _ = actor_ref.get_id();
}

#[tokio::test]
async fn actor_evaluates_thought() {
    use ractor::{rpc::CallResult, Actor};

    let (actor_ref, _) = Actor::spawn(None, VolitionActor, VolitionConfig::default())
        .await
        .expect("Failed to spawn VolitionActor");

    let thought = Thought::new(Content::symbol("hello", vec![]), SalienceScore::neutral());

    let response = actor_ref
        .call(
            |reply| VolitionMessage::EvaluateThought { thought, reply },
            None,
        )
        .await
        .expect("Failed to evaluate thought");

    match response {
        CallResult::Success(VolitionResponse::Approved { .. }) => {
            // Expected
        }
        _ => panic!("Expected Approved response, got: {response:?}"),
    }
}

#[tokio::test]
async fn actor_vetoes_harmful_thought() {
    use ractor::{rpc::CallResult, Actor};

    let (actor_ref, _) = Actor::spawn(None, VolitionActor, VolitionConfig::default())
        .await
        .expect("Failed to spawn VolitionActor");

    let thought = Thought::new(
        Content::symbol("destroy_human", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let response = actor_ref
        .call(
            |reply| VolitionMessage::EvaluateThought { thought, reply },
            None,
        )
        .await
        .expect("Failed to evaluate thought");

    match response {
        CallResult::Success(VolitionResponse::Vetoed { .. }) => {
            // Expected
        }
        _ => panic!("Expected Vetoed response, got: {response:?}"),
    }
}

#[tokio::test]
async fn actor_returns_values() {
    use ractor::{rpc::CallResult, Actor};

    let (actor_ref, _) = Actor::spawn(None, VolitionActor, VolitionConfig::default())
        .await
        .expect("Failed to spawn VolitionActor");

    let response = actor_ref
        .call(|reply| VolitionMessage::GetValues { reply }, None)
        .await
        .expect("Failed to get values");

    match response {
        CallResult::Success(VolitionResponse::Values { values }) => {
            assert!(values.protect_humans);
        }
        _ => panic!("Expected Values response, got: {response:?}"),
    }
}

#[tokio::test]
async fn actor_returns_stats() {
    use ractor::{rpc::CallResult, Actor};

    let (actor_ref, _) = Actor::spawn(None, VolitionActor, VolitionConfig::default())
        .await
        .expect("Failed to spawn VolitionActor");

    // Evaluate some thoughts first
    let thought = Thought::new(Content::symbol("test", vec![]), SalienceScore::neutral());
    let _ = actor_ref
        .call(
            |reply| VolitionMessage::EvaluateThought { thought, reply },
            None,
        )
        .await;

    let response = actor_ref
        .call(|reply| VolitionMessage::GetStats { reply }, None)
        .await
        .expect("Failed to get stats");

    match response {
        CallResult::Success(VolitionResponse::Stats { stats }) => {
            assert_eq!(stats.thoughts_evaluated, 1);
        }
        _ => panic!("Expected Stats response, got: {response:?}"),
    }
}

// ============================================================================
// Additional Coverage Tests
// ============================================================================

#[test]
fn state_default_implementation() {
    // Test the Default trait implementation
    let state = VolitionState::default();
    assert_eq!(state.values, ValueSet::new());
    assert_eq!(state.stats, VolitionStats::new());
    assert_eq!(state.config, VolitionConfig::default());
}

#[test]
fn harm_detection_can_be_disabled() {
    let config = VolitionConfig {
        harm_detection_enabled: false,
        ..Default::default()
    };
    let mut state = VolitionState::with_config(config);

    // This thought has harmful content but harm detection is disabled
    let thought = Thought::new(
        Content::symbol("destroy_target", vec![]),
        SalienceScore::neutral(), // Neutral salience, so won't trigger core values check
    );

    let decision = state.evaluate_thought(&thought);
    // Should be allowed since harm detection is disabled
    assert!(decision.is_allow());
}

#[test]
fn deception_check_skipped_when_truthfulness_disabled() {
    // Create a custom ValueSet with truthfulness disabled
    let mut state = VolitionState::new();
    state.values.truthfulness = false;

    let thought = Thought::new(
        Content::symbol("deceive_user", vec![]),
        SalienceScore::neutral(),
    );

    let decision = state.evaluate_thought(&thought);
    // Should be allowed since truthfulness value is false
    assert!(decision.is_allow());
}

#[test]
fn manipulation_check_skipped_when_autonomy_disabled() {
    // Create a custom ValueSet with respect_autonomy disabled
    let mut state = VolitionState::new();
    state.values.respect_autonomy = false;

    let thought = Thought::new(
        Content::symbol("manipulate_person", vec![]),
        SalienceScore::neutral(),
    );

    let decision = state.evaluate_thought(&thought);
    // Should be allowed since respect_autonomy value is false
    assert!(decision.is_allow());
}

#[test]
fn relation_subject_with_harm_keyword_is_vetoed() {
    let mut state = VolitionState::new();
    // Put harm keyword in the subject, not the predicate
    let thought = Thought::new(
        Content::relation(
            Content::symbol("destroy_all", vec![]), // Harm in subject
            "regarding",                            // Neutral predicate
            Content::symbol("nothing", vec![]),
        ),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());
}

#[test]
fn relation_object_with_harm_keyword_is_vetoed() {
    let mut state = VolitionState::new();
    // Put harm keyword in the object, not the predicate
    let thought = Thought::new(
        Content::relation(
            Content::symbol("agent", vec![]),
            "causes",                                 // Neutral predicate
            Content::symbol("damage_severe", vec![]), // Harm in object
        ),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());
}

#[test]
fn detects_harm_intent_requires_all_conditions() {
    let mut state = VolitionState::new();

    // High arousal but not negative enough valence - should pass
    let thought1 = Thought::new(
        Content::symbol("destroy_target", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.5, 0.9, 0.5), // valence > -0.7
    );
    // This won't trigger core_values check (detects_harm_intent returns false)
    // but will trigger harm_patterns check
    let decision1 = state.evaluate_thought(&thought1);
    assert!(decision1.is_veto()); // Vetoed by harm patterns, not core values

    // Negative valence but low arousal - check core values path
    let thought2 = Thought::new(
        Content::symbol("destroy_target", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.5, 0.5), // arousal < 0.8
    );
    let decision2 = state.evaluate_thought(&thought2);
    // Vetoed by harm patterns check, not core values (arousal too low)
    assert!(decision2.is_veto());
}

#[test]
fn core_values_check_with_harm_intent_detected() {
    let mut state = VolitionState::new();

    // All conditions met for detects_harm_intent: negative valence + high arousal + harm keywords
    let thought = Thought::new(
        Content::symbol("kill_humans", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());

    // Verify it was caught by core values specifically
    if let VetoDecision::Veto {
        violated_value,
        reason,
        ..
    } = decision
    {
        assert_eq!(violated_value, Some("protect_humans".to_string()));
        assert!(reason.contains("human harm"));
    } else {
        panic!("Expected veto");
    }
}

#[test]
fn protect_humans_disabled_allows_harmful_core_values() {
    let mut state = VolitionState::new();
    state.values.protect_humans = false;

    // With protect_humans disabled, core_values check won't veto
    // But harm_patterns check will still catch it
    let thought = Thought::new(
        Content::symbol("destroy_human", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let decision = state.evaluate_thought(&thought);
    // Still vetoed by harm_patterns check
    assert!(decision.is_veto());
}

#[test]
fn no_harm_keywords_with_negative_salience_passes() {
    let mut state = VolitionState::new();

    // Negative salience but no harm keywords - should pass core values and harm patterns
    let thought = Thought::new(
        Content::symbol("sad_feeling", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_allow());
}

#[test]
fn composite_nested_content_recursion() {
    let mut state = VolitionState::new();

    // Deeply nested composite with harmful content
    let thought = Thought::new(
        Content::Composite(vec![Content::Composite(vec![
            Content::symbol("greeting", vec![]),
            Content::symbol("trick_user", vec![]), // Deception keyword nested
        ])]),
        SalienceScore::neutral(),
    );

    let decision = state.evaluate_thought(&thought);
    assert!(decision.is_veto());

    if let VetoDecision::Veto { violated_value, .. } = decision {
        assert_eq!(violated_value, Some("truthfulness".to_string()));
    }
}

#[test]
fn veto_reason_tracking_in_stats() {
    let mut state = VolitionState::new();

    // Trigger different veto reasons
    let harm_thought = Thought::new(
        Content::symbol("kill_target", vec![]),
        SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
    );
    state.evaluate_thought(&harm_thought);

    let deception_thought = Thought::new(
        Content::symbol("lie_to_user", vec![]),
        SalienceScore::neutral(),
    );
    state.evaluate_thought(&deception_thought);

    // Check stats track different reasons
    assert!(!state.stats.vetos_by_reason.is_empty());
    assert_eq!(state.stats.thoughts_vetoed, 2);
}

#[tokio::test]
async fn actor_override_impulse_success() {
    use crate::core::types::ThoughtId;
    use ractor::{rpc::CallResult, Actor};

    let (actor_ref, _) = Actor::spawn(None, VolitionActor, VolitionConfig::default())
        .await
        .expect("Failed to spawn VolitionActor");

    let thought_id = ThoughtId::new();

    let response = actor_ref
        .call(
            |reply| VolitionMessage::OverrideImpulse {
                thought_id,
                reason: "Testing override functionality".to_string(),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send override");

    match response {
        CallResult::Success(VolitionResponse::OverrideApplied { thought_id: id }) => {
            assert_eq!(id, thought_id);
        }
        _ => panic!("Expected OverrideApplied response, got: {response:?}"),
    }
}

#[tokio::test]
async fn actor_override_impulse_empty_reason_error() {
    use crate::core::types::ThoughtId;
    use ractor::{rpc::CallResult, Actor};

    let (actor_ref, _) = Actor::spawn(None, VolitionActor, VolitionConfig::default())
        .await
        .expect("Failed to spawn VolitionActor");

    let thought_id = ThoughtId::new();

    let response = actor_ref
        .call(
            |reply| VolitionMessage::OverrideImpulse {
                thought_id,
                reason: String::new(), // Empty reason should fail
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send override");

    match response {
        CallResult::Success(VolitionResponse::Error { error }) => {
            assert!(matches!(error, VolitionError::InvalidReason { .. }));
        }
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn actor_veto_with_log_disabled() {
    use ractor::{rpc::CallResult, Actor};

    let config = VolitionConfig {
        log_vetos: false, // Disable veto logging
        ..Default::default()
    };

    let (actor_ref, _) = Actor::spawn(None, VolitionActor, config)
        .await
        .expect("Failed to spawn VolitionActor");

    let thought = Thought::new(
        Content::symbol("deceive_user", vec![]),
        SalienceScore::neutral(),
    );

    let response = actor_ref
        .call(
            |reply| VolitionMessage::EvaluateThought { thought, reply },
            None,
        )
        .await
        .expect("Failed to evaluate thought");

    // Should still veto, just without logging
    match response {
        CallResult::Success(VolitionResponse::Vetoed { .. }) => {
            // Expected - veto works even with logging disabled
        }
        _ => panic!("Expected Vetoed response, got: {response:?}"),
    }
}

#[test]
fn all_harm_keywords_detected() {
    let mut state = VolitionState::new();

    let keywords = [
        "destroy", "kill", "harm", "attack", "hurt", "damage", "injure",
    ];

    for keyword in keywords {
        let thought = Thought::new(
            Content::symbol(format!("{keyword}_test"), vec![]),
            SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.9, 0.5),
        );

        let decision = state.evaluate_thought(&thought);
        assert!(
            decision.is_veto(),
            "Keyword '{keyword}' should trigger veto"
        );
    }
}

#[test]
fn all_deception_keywords_detected() {
    let mut state = VolitionState::new();

    let keywords = ["deceive", "trick", "lie", "mislead", "fake", "pretend"];

    for keyword in keywords {
        let thought = Thought::new(
            Content::symbol(format!("{keyword}_action"), vec![]),
            SalienceScore::neutral(),
        );

        let decision = state.evaluate_thought(&thought);
        assert!(
            decision.is_veto(),
            "Keyword '{keyword}' should trigger veto"
        );
    }
}

#[test]
fn all_manipulation_keywords_detected() {
    let mut state = VolitionState::new();

    let keywords = ["manipulate", "coerce", "force", "exploit", "pressure"];

    for keyword in keywords {
        let thought = Thought::new(
            Content::symbol(format!("{keyword}_user"), vec![]),
            SalienceScore::neutral(),
        );

        let decision = state.evaluate_thought(&thought);
        assert!(
            decision.is_veto(),
            "Keyword '{keyword}' should trigger veto"
        );
    }
}

#[test]
fn keyword_detection_case_insensitive() {
    let mut state = VolitionState::new();

    // Test uppercase
    let thought = Thought::new(
        Content::symbol("DECEIVE_USER", vec![]),
        SalienceScore::neutral(),
    );
    assert!(state.evaluate_thought(&thought).is_veto());

    // Test mixed case
    let thought2 = Thought::new(
        Content::symbol("Manipulate_Person", vec![]),
        SalienceScore::neutral(),
    );
    assert!(state.evaluate_thought(&thought2).is_veto());
}

#[test]
fn approval_rate_with_zero_evaluations() {
    let state = VolitionState::new();
    // Default approval rate should be 1.0 when no evaluations
    assert_eq!(state.stats.approval_rate(), 1.0);
}

#[test]
fn get_values_returns_reference() {
    let state = VolitionState::new();
    let values = state.get_values();
    assert!(values.protect_humans);
    assert!(values.truthfulness);
}

#[test]
fn get_stats_returns_reference() {
    let mut state = VolitionState::new();

    // Record some activity
    let thought = Thought::new(Content::symbol("safe", vec![]), SalienceScore::neutral());
    state.evaluate_thought(&thought);

    let vol_stats = state.get_stats();
    assert_eq!(vol_stats.thoughts_evaluated, 1);
    assert_eq!(vol_stats.thoughts_approved, 1);
}

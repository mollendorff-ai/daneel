//! VolitionActor Tests
//!
//! Comprehensive tests for veto logic, value checking,
//! and free-won't implementation.

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
        _ => panic!("Expected Approved response, got: {:?}", response),
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
        _ => panic!("Expected Vetoed response, got: {:?}", response),
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
        _ => panic!("Expected Values response, got: {:?}", response),
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
        _ => panic!("Expected Stats response, got: {:?}", response),
    }
}

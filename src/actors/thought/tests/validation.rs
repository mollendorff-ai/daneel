//! Salience validation tests

use super::*;

// ============================================================================
// Salience Validation Tests
// ============================================================================

#[tokio::test]
async fn test_assemble_with_valid_salience() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.8, 0.6, 0.9, 0.5, 0.7);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.salience, salience);
        }
        _ => panic!("Expected Assembled response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_importance() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(1.5, 0.5, 0.5, 0.0, 0.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("importance"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_valence() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, -1.5, 0.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("valence"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_validation_disabled() {
    let config = AssemblyConfig {
        cache_size: 100,
        max_chain_depth: 50,
        validate_salience: false,
    };
    let actor_ref = spawn_thought_actor_with_config(config).await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(2.0, -1.0, 5.0, 10.0, -2.0);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    assert!(matches!(
        response,
        CallResult::Success(ThoughtResponse::Assembled { .. })
    ));
}

// ============================================================================
// Additional Salience Validation Tests (All Ranges)
// ============================================================================

#[tokio::test]
async fn test_assemble_with_negative_importance() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(-0.5, 0.5, 0.5, 0.0, 0.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("importance"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_novelty_high() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 1.5, 0.5, 0.0, 0.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("novelty"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_novelty_low() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, -0.1, 0.5, 0.0, 0.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("novelty"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_relevance_high() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 2.0, 0.0, 0.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("relevance"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_relevance_low() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, -0.1, 0.0, 0.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("relevance"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_valence_high() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 1.5, 0.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("valence"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_connection_relevance_high() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 0.0, 1.5);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("connection_relevance"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_connection_relevance_low() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 0.0, -0.1);
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::InvalidSalience { reason } => {
                assert!(reason.contains("connection_relevance"));
            }
            _ => panic!("Expected InvalidSalience error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

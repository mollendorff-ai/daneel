//! Tests for ThoughtAssemblyActor
//!
//! ADR-049: Test modules excluded from coverage.

#![cfg_attr(coverage_nightly, coverage(off))]

use super::*;
use crate::core::types::{Content, SalienceScore, ThoughtId};
use ractor::rpc::CallResult;
use ractor::Actor;

/// Helper to spawn a thought actor for testing
async fn spawn_thought_actor() -> ActorRef<ThoughtMessage> {
    let (actor_ref, _) = Actor::spawn(None, ThoughtAssemblyActor, AssemblyConfig::default())
        .await
        .expect("Failed to spawn ThoughtAssemblyActor");
    actor_ref
}

/// Helper to spawn a thought actor with custom config
async fn spawn_thought_actor_with_config(config: AssemblyConfig) -> ActorRef<ThoughtMessage> {
    let (actor_ref, _) = Actor::spawn(None, ThoughtAssemblyActor, config)
        .await
        .expect("Failed to spawn ThoughtAssemblyActor");
    actor_ref
}

// ============================================================================
// Actor Lifecycle Tests
// ============================================================================

#[tokio::test]
async fn test_actor_spawns_successfully() {
    let actor_ref = spawn_thought_actor().await;

    // Verify actor can handle messages by assembling a simple thought
    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    assert!(matches!(
        response,
        CallResult::Success(ThoughtResponse::Assembled { .. })
    ));
}

#[tokio::test]
async fn test_actor_with_custom_config() {
    let config = AssemblyConfig {
        cache_size: 50,
        max_chain_depth: 25,
        validate_salience: false,
    };

    let actor_ref = spawn_thought_actor_with_config(config).await;

    // Verify actor accepts invalid salience when validation is disabled
    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(1.5, 0.5, 0.5, 0.0, 0.5); // Invalid importance
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    // Should succeed because validation is disabled
    assert!(matches!(
        response,
        CallResult::Success(ThoughtResponse::Assembled { .. })
    ));
}

// ============================================================================
// Basic Assembly Tests
// ============================================================================

#[tokio::test]
async fn test_assemble_raw_content() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![42, 43, 44]);
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content.clone(), salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
            assert_eq!(thought.salience, salience);
            assert!(thought.parent_id.is_none());
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_symbol_content() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::symbol("test_symbol", vec![1, 2, 3]);
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content.clone(), salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_relation_content() {
    let actor_ref = spawn_thought_actor().await;

    let subject = Content::symbol("subject", vec![1]);
    let object = Content::symbol("object", vec![2]);
    let content = Content::relation(subject, "causes", object);
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content.clone(), salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_empty_content_fails() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::Empty;
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => {
            assert!(matches!(error, AssemblyError::EmptyContent));
        }
        _ => panic!(
            "Expected Error response with EmptyContent, got: {:?}",
            response
        ),
    }
}

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
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_importance() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(1.5, 0.5, 0.5, 0.0, 0.5); // importance > 1.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_valence() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, -1.5, 0.5); // valence < -1.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
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
    let salience = SalienceScore::new_without_arousal(2.0, -1.0, 5.0, 10.0, -2.0); // All invalid
    let request = AssemblyRequest::new(content, salience);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to send message");

    // Should succeed because validation is disabled
    assert!(matches!(
        response,
        CallResult::Success(ThoughtResponse::Assembled { .. })
    ));
}

// ============================================================================
// Parent Linking Tests
// ============================================================================

#[tokio::test]
async fn test_assemble_with_parent() {
    let actor_ref = spawn_thought_actor().await;

    // Assemble parent thought
    let parent_content = Content::raw(vec![1, 2, 3]);
    let parent_salience = SalienceScore::neutral();
    let parent_request = AssemblyRequest::new(parent_content, parent_salience);

    let parent_thought = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: parent_request,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble parent")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought,
        _ => panic!("Expected Assembled response"),
    };

    // Assemble child thought with parent
    let child_content = Content::raw(vec![4, 5, 6]);
    let child_salience = SalienceScore::neutral();
    let child_request =
        AssemblyRequest::new(child_content, child_salience).with_parent(parent_thought.id);

    let child_response = actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: child_request,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble child");

    match child_response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.parent_id, Some(parent_thought.id));
        }
        _ => panic!("Expected Assembled response, got: {:?}", child_response),
    }
}

#[tokio::test]
async fn test_assemble_chain_builds_history() {
    let actor_ref = spawn_thought_actor().await;

    // Create a chain: thought1 -> thought2 -> thought3
    let content1 = Content::raw(vec![1]);
    let thought1 = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(content1, SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought,
        _ => panic!("Expected Assembled response"),
    };

    let content2 = Content::raw(vec![2]);
    let thought2 = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(content2, SalienceScore::neutral())
                    .with_parent(thought1.id),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought,
        _ => panic!("Expected Assembled response"),
    };

    let content3 = Content::raw(vec![3]);
    let thought3 = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(content3, SalienceScore::neutral())
                    .with_parent(thought2.id),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought,
        _ => panic!("Expected Assembled response"),
    };

    // Verify chain relationships
    assert!(thought1.parent_id.is_none());
    assert_eq!(thought2.parent_id, Some(thought1.id));
    assert_eq!(thought3.parent_id, Some(thought2.id));
}

// ============================================================================
// Batch Operations Tests
// ============================================================================

#[tokio::test]
async fn test_assemble_batch_empty() {
    let actor_ref = spawn_thought_actor().await;

    let requests = vec![];

    let response = actor_ref
        .call(
            |reply| ThoughtMessage::AssembleBatch { requests, reply },
            None,
        )
        .await
        .expect("Failed to assemble batch");

    match response {
        CallResult::Success(ThoughtResponse::BatchAssembled { thoughts }) => {
            assert_eq!(thoughts.len(), 0);
        }
        _ => panic!("Expected BatchAssembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_batch_multiple() {
    let actor_ref = spawn_thought_actor().await;

    let requests = vec![
        AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
        AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral()),
        AssemblyRequest::new(Content::raw(vec![3]), SalienceScore::neutral()),
    ];

    let response = actor_ref
        .call(
            |reply| ThoughtMessage::AssembleBatch {
                requests: requests.clone(),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble batch");

    match response {
        CallResult::Success(ThoughtResponse::BatchAssembled { thoughts }) => {
            assert_eq!(thoughts.len(), 3);
            assert_eq!(thoughts[0].content, Content::raw(vec![1]));
            assert_eq!(thoughts[1].content, Content::raw(vec![2]));
            assert_eq!(thoughts[2].content, Content::raw(vec![3]));
        }
        _ => panic!("Expected BatchAssembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_batch_stops_on_error() {
    let actor_ref = spawn_thought_actor().await;

    let requests = vec![
        AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
        AssemblyRequest::new(Content::Empty, SalienceScore::neutral()), // This will fail
        AssemblyRequest::new(Content::raw(vec![3]), SalienceScore::neutral()),
    ];

    let response = actor_ref
        .call(
            |reply| ThoughtMessage::AssembleBatch { requests, reply },
            None,
        )
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => {
            assert!(matches!(error, AssemblyError::EmptyContent));
        }
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

// ============================================================================
// Cache Operations Tests
// ============================================================================

#[tokio::test]
async fn test_get_thought_from_cache() {
    let actor_ref = spawn_thought_actor().await;

    // Assemble a thought
    let content = Content::raw(vec![1, 2, 3]);
    let thought_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(content.clone(), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Retrieve it from cache
    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThought { thought_id, reply },
            None,
        )
        .await
        .expect("Failed to get thought");

    match response {
        CallResult::Success(ThoughtResponse::ThoughtFound { thought }) => {
            assert_eq!(thought.id, thought_id);
            assert_eq!(thought.content, content);
        }
        _ => panic!("Expected ThoughtFound response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_get_thought_not_found() {
    let actor_ref = spawn_thought_actor().await;

    let fake_thought_id = ThoughtId::new();

    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThought {
                thought_id: fake_thought_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => {
            assert!(matches!(error, AssemblyError::ThoughtNotFound { .. }));
        }
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_cache_eviction() {
    // Use a small cache for testing eviction
    let config = AssemblyConfig {
        cache_size: 2,
        max_chain_depth: 50,
        validate_salience: true,
    };
    let actor_ref = spawn_thought_actor_with_config(config).await;

    // Assemble 3 thoughts (cache size is 2, so first will be evicted)
    let thought1_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    let thought2_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    let thought3_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![3]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // First thought should be evicted
    let response1 = actor_ref
        .call(
            |reply| ThoughtMessage::GetThought {
                thought_id: thought1_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send message");

    assert!(matches!(
        response1,
        CallResult::Success(ThoughtResponse::Error {
            error: AssemblyError::ThoughtNotFound { .. }
        })
    ));

    // Second and third thoughts should still be cached
    assert!(matches!(
        actor_ref
            .call(
                |reply| ThoughtMessage::GetThought {
                    thought_id: thought2_id,
                    reply
                },
                None
            )
            .await
            .expect("Failed to send message"),
        CallResult::Success(ThoughtResponse::ThoughtFound { .. })
    ));

    assert!(matches!(
        actor_ref
            .call(
                |reply| ThoughtMessage::GetThought {
                    thought_id: thought3_id,
                    reply
                },
                None
            )
            .await
            .expect("Failed to send message"),
        CallResult::Success(ThoughtResponse::ThoughtFound { .. })
    ));
}

// ============================================================================
// Chain Operations Tests
// ============================================================================

#[tokio::test]
async fn test_get_thought_chain_single() {
    let actor_ref = spawn_thought_actor().await;

    // Create a single thought with no parent
    let thought_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Get chain with depth 5
    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThoughtChain {
                thought_id,
                depth: 5,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get chain");

    match response {
        CallResult::Success(ThoughtResponse::ThoughtChain { thoughts }) => {
            assert_eq!(thoughts.len(), 1);
            assert_eq!(thoughts[0].id, thought_id);
        }
        _ => panic!("Expected ThoughtChain response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_get_thought_chain_multiple() {
    let actor_ref = spawn_thought_actor().await;

    // Create chain: thought1 -> thought2 -> thought3
    let thought1_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    let thought2_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral())
                    .with_parent(thought1_id),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    let thought3_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![3]), SalienceScore::neutral())
                    .with_parent(thought2_id),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Get chain from thought3 with depth 10
    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThoughtChain {
                thought_id: thought3_id,
                depth: 10,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get chain");

    match response {
        CallResult::Success(ThoughtResponse::ThoughtChain { thoughts }) => {
            assert_eq!(thoughts.len(), 3);
            assert_eq!(thoughts[0].id, thought3_id);
            assert_eq!(thoughts[1].id, thought2_id);
            assert_eq!(thoughts[2].id, thought1_id);
        }
        _ => panic!("Expected ThoughtChain response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_get_thought_chain_depth_limit() {
    let config = AssemblyConfig {
        cache_size: 100,
        max_chain_depth: 5,
        validate_salience: true,
    };
    let actor_ref = spawn_thought_actor_with_config(config).await;

    // Create a simple thought
    let thought_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Try to get chain with depth > max_chain_depth
    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThoughtChain {
                thought_id,
                depth: 10,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => match error {
            AssemblyError::ChainTooDeep { max_depth } => {
                assert_eq!(max_depth, 5);
            }
            _ => panic!("Expected ChainTooDeep error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_get_thought_chain_stops_at_root() {
    let actor_ref = spawn_thought_actor().await;

    // Create chain: thought1 -> thought2 -> thought3
    let thought1_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    let thought2_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral())
                    .with_parent(thought1_id),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    let thought3_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![3]), SalienceScore::neutral())
                    .with_parent(thought2_id),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Get chain with large depth - should stop at root (thought1)
    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThoughtChain {
                thought_id: thought3_id,
                depth: 50,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get chain");

    match response {
        CallResult::Success(ThoughtResponse::ThoughtChain { thoughts }) => {
            // Should only return 3 thoughts, not 50
            assert_eq!(thoughts.len(), 3);
            assert_eq!(thoughts[0].id, thought3_id);
            assert_eq!(thoughts[1].id, thought2_id);
            assert_eq!(thoughts[2].id, thought1_id);
        }
        _ => panic!("Expected ThoughtChain response, got: {:?}", response),
    }
}

// ============================================================================
// Strategy Tests
// ============================================================================

#[tokio::test]
async fn test_strategy_default() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let request = AssemblyRequest::new(content.clone(), SalienceScore::neutral())
        .with_strategy(AssemblyStrategy::Default);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_strategy_chain_with_parent() {
    let actor_ref = spawn_thought_actor().await;

    // Create parent thought
    let parent_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Create child with Chain strategy
    let content = Content::raw(vec![2]);
    let request = AssemblyRequest::new(content.clone(), SalienceScore::neutral())
        .with_parent(parent_id)
        .with_strategy(AssemblyStrategy::Chain);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
            assert_eq!(thought.parent_id, Some(parent_id));
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

// ============================================================================
// Config and State Construction Tests
// ============================================================================

#[test]
fn test_assembly_config_new() {
    let config = AssemblyConfig::new(50, 25, false);
    assert_eq!(config.cache_size, 50);
    assert_eq!(config.max_chain_depth, 25);
    assert!(!config.validate_salience);
}

#[test]
fn test_assembly_config_default() {
    let config = AssemblyConfig::default();
    assert_eq!(config.cache_size, 100);
    assert_eq!(config.max_chain_depth, 50);
    assert!(config.validate_salience);
}

#[test]
fn test_thought_state_new() {
    let state = ThoughtState::new();
    assert_eq!(state.assembly_count, 0);
    assert_eq!(state.config.cache_size, 100);
    assert!(state.config.validate_salience);
}

#[test]
fn test_thought_state_default() {
    let state = ThoughtState::default();
    assert_eq!(state.assembly_count, 0);
    assert_eq!(state.config.cache_size, 100);
}

#[test]
fn test_thought_state_with_config() {
    let config = AssemblyConfig::new(10, 5, false);
    let state = ThoughtState::with_config(config);
    assert_eq!(state.assembly_count, 0);
    assert_eq!(state.config.cache_size, 10);
    assert_eq!(state.config.max_chain_depth, 5);
    assert!(!state.config.validate_salience);
}

// ============================================================================
// Additional Salience Validation Tests (All Ranges)
// ============================================================================

#[tokio::test]
async fn test_assemble_with_negative_importance() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(-0.5, 0.5, 0.5, 0.0, 0.5); // importance < 0.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_novelty_high() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 1.5, 0.5, 0.0, 0.5); // novelty > 1.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_novelty_low() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, -0.1, 0.5, 0.0, 0.5); // novelty < 0.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_relevance_high() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 2.0, 0.0, 0.5); // relevance > 1.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_relevance_low() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, -0.1, 0.0, 0.5); // relevance < 0.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_valence_high() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 1.5, 0.5); // valence > 1.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_connection_relevance_high() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 0.0, 1.5); // connection_relevance > 1.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_invalid_connection_relevance_low() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 0.0, -0.1); // connection_relevance < 0.0
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
            _ => panic!("Expected InvalidSalience error, got: {:?}", error),
        },
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

// ============================================================================
// Source Stream Tests
// ============================================================================

#[tokio::test]
async fn test_assemble_with_source_stream() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content.clone(), salience).with_source("external");

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
            assert_eq!(thought.source_stream, Some("external".to_string()));
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_assemble_with_source_stream_memory() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![42]);
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content, salience).with_source("memory");

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.source_stream, Some("memory".to_string()));
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

// ============================================================================
// Additional Strategy Tests
// ============================================================================

#[tokio::test]
async fn test_strategy_composite() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let request = AssemblyRequest::new(content.clone(), SalienceScore::neutral())
        .with_strategy(AssemblyStrategy::Composite);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_strategy_urgent() {
    let actor_ref = spawn_thought_actor().await;

    let content = Content::raw(vec![1, 2, 3]);
    let request = AssemblyRequest::new(content.clone(), SalienceScore::neutral())
        .with_strategy(AssemblyStrategy::Urgent);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_strategy_chain_without_parent() {
    let actor_ref = spawn_thought_actor().await;

    // Chain strategy without a parent - should still work
    let content = Content::raw(vec![1, 2, 3]);
    let request = AssemblyRequest::new(content.clone(), SalienceScore::neutral())
        .with_strategy(AssemblyStrategy::Chain);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
            assert!(thought.parent_id.is_none());
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_strategy_chain_with_parent_not_in_cache() {
    let actor_ref = spawn_thought_actor().await;

    // Use a non-existent parent ID (parent not in cache)
    let fake_parent_id = ThoughtId::new();

    let content = Content::raw(vec![1, 2, 3]);
    let request = AssemblyRequest::new(content.clone(), SalienceScore::neutral())
        .with_parent(fake_parent_id)
        .with_strategy(AssemblyStrategy::Chain);

    // This should still succeed - parent not being in cache is not an error
    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
            assert_eq!(thought.parent_id, Some(fake_parent_id));
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

// ============================================================================
// Chain Traversal Edge Cases
// ============================================================================

#[tokio::test]
async fn test_get_thought_chain_broken_chain() {
    let actor_ref = spawn_thought_actor().await;

    // Create a thought with a parent that doesn't exist (simulating a broken chain)
    let fake_parent_id = ThoughtId::new();
    let content = Content::raw(vec![1]);
    let request =
        AssemblyRequest::new(content, SalienceScore::neutral()).with_parent(fake_parent_id);

    let thought_id = match actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Try to get chain with depth > 1 (should fail because parent doesn't exist)
    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThoughtChain {
                thought_id,
                depth: 2,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get chain");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => {
            assert!(matches!(error, AssemblyError::ThoughtNotFound { .. }));
        }
        _ => panic!(
            "Expected Error response with ThoughtNotFound, got: {:?}",
            response
        ),
    }
}

#[tokio::test]
async fn test_get_thought_chain_zero_depth() {
    let actor_ref = spawn_thought_actor().await;

    // Create a thought
    let thought_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Get chain with depth 0 - should return empty chain
    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThoughtChain {
                thought_id,
                depth: 0,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get chain");

    match response {
        CallResult::Success(ThoughtResponse::ThoughtChain { thoughts }) => {
            assert_eq!(thoughts.len(), 0);
        }
        _ => panic!("Expected ThoughtChain response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_get_thought_chain_depth_limited_by_param() {
    let actor_ref = spawn_thought_actor().await;

    // Create chain: thought1 -> thought2 -> thought3
    let thought1_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    let thought2_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral())
                    .with_parent(thought1_id),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    let thought3_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![3]), SalienceScore::neutral())
                    .with_parent(thought2_id),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Get chain from thought3 with depth 2 (should return only thought3 and thought2)
    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThoughtChain {
                thought_id: thought3_id,
                depth: 2,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get chain");

    match response {
        CallResult::Success(ThoughtResponse::ThoughtChain { thoughts }) => {
            assert_eq!(thoughts.len(), 2);
            assert_eq!(thoughts[0].id, thought3_id);
            assert_eq!(thoughts[1].id, thought2_id);
            // thought1 not included due to depth limit
        }
        _ => panic!("Expected ThoughtChain response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_get_thought_chain_not_found() {
    let actor_ref = spawn_thought_actor().await;

    let fake_thought_id = ThoughtId::new();

    let response = actor_ref
        .call(
            |reply| ThoughtMessage::GetThoughtChain {
                thought_id: fake_thought_id,
                depth: 5,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(ThoughtResponse::Error { error }) => {
            assert!(matches!(error, AssemblyError::ThoughtNotFound { .. }));
        }
        _ => panic!("Expected Error response, got: {:?}", response),
    }
}

// ============================================================================
// Assembly Count and State Tests
// ============================================================================

#[test]
fn test_assembly_count_increments() {
    let mut state = ThoughtState::new();

    let content1 = Content::raw(vec![1]);
    let content2 = Content::raw(vec![2]);
    let salience = SalienceScore::neutral();

    assert_eq!(state.assembly_count, 0);

    let _ = state.assemble_thought(AssemblyRequest::new(content1, salience));
    assert_eq!(state.assembly_count, 1);

    let _ = state.assemble_thought(AssemblyRequest::new(content2, salience));
    assert_eq!(state.assembly_count, 2);
}

#[test]
fn test_assembly_count_not_incremented_on_error() {
    let mut state = ThoughtState::new();

    assert_eq!(state.assembly_count, 0);

    // Empty content should fail
    let result = state.assemble_thought(AssemblyRequest::new(
        Content::Empty,
        SalienceScore::neutral(),
    ));
    assert!(result.is_err());
    assert_eq!(state.assembly_count, 0);
}

// ============================================================================
// Direct State Method Tests
// ============================================================================

#[test]
fn test_state_assemble_thought_directly() {
    let mut state = ThoughtState::new();

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content.clone(), salience);

    let result = state.assemble_thought(request);
    assert!(result.is_ok());
    let thought = result.unwrap();
    assert_eq!(thought.content, content);
}

#[test]
fn test_state_assemble_batch_directly() {
    let mut state = ThoughtState::new();

    let requests = vec![
        AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
        AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral()),
    ];

    let result = state.assemble_batch(requests);
    assert!(result.is_ok());
    let thoughts = result.unwrap();
    assert_eq!(thoughts.len(), 2);
}

#[test]
fn test_state_get_thought_directly() {
    let mut state = ThoughtState::new();

    let content = Content::raw(vec![1, 2, 3]);
    let request = AssemblyRequest::new(content.clone(), SalienceScore::neutral());

    let thought = state.assemble_thought(request).unwrap();
    let thought_id = thought.id;

    let result = state.get_thought(&thought_id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, thought_id);
}

#[test]
fn test_state_get_thought_chain_directly() {
    let mut state = ThoughtState::new();

    // Create parent
    let parent_request = AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral());
    let parent = state.assemble_thought(parent_request).unwrap();

    // Create child
    let child_request = AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral())
        .with_parent(parent.id);
    let child = state.assemble_thought(child_request).unwrap();

    // Get chain
    let result = state.get_thought_chain(child.id, 10);
    assert!(result.is_ok());
    let chain = result.unwrap();
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].id, child.id);
    assert_eq!(chain[1].id, parent.id);
}

#[test]
fn test_state_validate_salience_edge_values() {
    let state = ThoughtState::new();

    // Test boundary values (should all pass)
    let valid_salience = SalienceScore::new_without_arousal(0.0, 0.0, 0.0, -1.0, 0.0);
    assert!(state.validate_salience(&valid_salience).is_ok());

    let valid_salience = SalienceScore::new_without_arousal(1.0, 1.0, 1.0, 1.0, 1.0);
    assert!(state.validate_salience(&valid_salience).is_ok());

    let valid_salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 0.0, 0.5);
    assert!(state.validate_salience(&valid_salience).is_ok());
}

// ============================================================================
// Combined Builder Patterns
// ============================================================================

#[tokio::test]
async fn test_assemble_with_all_options() {
    let actor_ref = spawn_thought_actor().await;

    // Create parent first
    let parent_id = match actor_ref
        .call(
            |reply| ThoughtMessage::Assemble {
                request: AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to assemble")
    {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => thought.id,
        _ => panic!("Expected Assembled response"),
    };

    // Create child with all options
    let content = Content::symbol("test", vec![2, 3, 4]);
    let salience = SalienceScore::new_without_arousal(0.9, 0.8, 0.7, 0.5, 0.6);
    let request = AssemblyRequest::new(content.clone(), salience)
        .with_parent(parent_id)
        .with_source("internal")
        .with_strategy(AssemblyStrategy::Chain);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
            assert_eq!(thought.salience, salience);
            assert_eq!(thought.parent_id, Some(parent_id));
            assert_eq!(thought.source_stream, Some("internal".to_string()));
        }
        _ => panic!("Expected Assembled response, got: {:?}", response),
    }
}

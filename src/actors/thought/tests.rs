//! Tests for ThoughtAssemblyActor

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
    let salience = SalienceScore::new(1.5, 0.5, 0.5, 0.0, 0.5); // Invalid importance
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
    let salience = SalienceScore::new(0.8, 0.6, 0.9, 0.5, 0.7);
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
    let salience = SalienceScore::new(1.5, 0.5, 0.5, 0.0, 0.5); // importance > 1.0
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
    let salience = SalienceScore::new(0.5, 0.5, 0.5, -1.5, 0.5); // valence < -1.0
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
    let salience = SalienceScore::new(2.0, -1.0, 5.0, 10.0, -2.0); // All invalid
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

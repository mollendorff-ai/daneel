//! Actor lifecycle, basic assembly, batch, and cache tests

use super::*;

// ============================================================================
// Actor Lifecycle Tests
// ============================================================================

#[tokio::test]
async fn test_actor_spawns_successfully() {
    let actor_ref = spawn_thought_actor().await;

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

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(1.5, 0.5, 0.5, 0.0, 0.5);
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
        _ => panic!("Expected Assembled response, got: {response:?}"),
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
        _ => panic!("Expected Assembled response, got: {response:?}"),
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
        _ => panic!("Expected Assembled response, got: {response:?}"),
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
        _ => panic!("Expected Error response with EmptyContent, got: {response:?}"),
    }
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
        _ => panic!("Expected BatchAssembled response, got: {response:?}"),
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
        _ => panic!("Expected BatchAssembled response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_batch_stops_on_error() {
    let actor_ref = spawn_thought_actor().await;

    let requests = vec![
        AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
        AssemblyRequest::new(Content::Empty, SalienceScore::neutral()),
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
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

// ============================================================================
// Cache Operations Tests
// ============================================================================

#[tokio::test]
async fn test_get_thought_from_cache() {
    let actor_ref = spawn_thought_actor().await;

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
        _ => panic!("Expected ThoughtFound response, got: {response:?}"),
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
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_cache_eviction() {
    let config = AssemblyConfig {
        cache_size: 2,
        max_chain_depth: 50,
        validate_salience: true,
    };
    let actor_ref = spawn_thought_actor_with_config(config).await;

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
        _ => panic!("Expected Assembled response, got: {response:?}"),
    }
}

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
        _ => panic!("Expected Assembled response, got: {response:?}"),
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
        _ => panic!("Expected Assembled response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_strategy_chain_without_parent() {
    let actor_ref = spawn_thought_actor().await;

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
        _ => panic!("Expected Assembled response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_strategy_chain_with_parent_not_in_cache() {
    let actor_ref = spawn_thought_actor().await;

    let fake_parent_id = ThoughtId::new();

    let content = Content::raw(vec![1, 2, 3]);
    let request = AssemblyRequest::new(content.clone(), SalienceScore::neutral())
        .with_parent(fake_parent_id)
        .with_strategy(AssemblyStrategy::Chain);

    let response = actor_ref
        .call(|reply| ThoughtMessage::Assemble { request, reply }, None)
        .await
        .expect("Failed to assemble thought");

    match response {
        CallResult::Success(ThoughtResponse::Assembled { thought }) => {
            assert_eq!(thought.content, content);
            assert_eq!(thought.parent_id, Some(fake_parent_id));
        }
        _ => panic!("Expected Assembled response, got: {response:?}"),
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
        _ => panic!("Expected Assembled response, got: {response:?}"),
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
        _ => panic!("Expected Assembled response, got: {response:?}"),
    }
}

// ============================================================================
// Combined Builder Patterns
// ============================================================================

#[tokio::test]
async fn test_assemble_with_all_options() {
    let actor_ref = spawn_thought_actor().await;

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
        _ => panic!("Expected Assembled response, got: {response:?}"),
    }
}

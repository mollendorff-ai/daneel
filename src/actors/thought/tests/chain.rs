//! Parent linking, chain operations, and chain edge case tests

use super::*;

// ============================================================================
// Parent Linking Tests
// ============================================================================

#[tokio::test]
async fn test_assemble_with_parent() {
    let actor_ref = spawn_thought_actor().await;

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
        _ => panic!("Expected Assembled response, got: {child_response:?}"),
    }
}

#[tokio::test]
async fn test_assemble_chain_builds_history() {
    let actor_ref = spawn_thought_actor().await;

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

    assert!(thought1.parent_id.is_none());
    assert_eq!(thought2.parent_id, Some(thought1.id));
    assert_eq!(thought3.parent_id, Some(thought2.id));
}

#[tokio::test]
async fn test_strategy_chain_with_parent() {
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
        _ => panic!("Expected Assembled response, got: {response:?}"),
    }
}

// ============================================================================
// Chain Operations Tests
// ============================================================================

#[tokio::test]
async fn test_get_thought_chain_single() {
    let actor_ref = spawn_thought_actor().await;

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
        _ => panic!("Expected ThoughtChain response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_get_thought_chain_multiple() {
    let actor_ref = spawn_thought_actor().await;

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
        _ => panic!("Expected ThoughtChain response, got: {response:?}"),
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
            _ => panic!("Expected ChainTooDeep error, got: {error:?}"),
        },
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_get_thought_chain_stops_at_root() {
    let actor_ref = spawn_thought_actor().await;

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
            assert_eq!(thoughts.len(), 3);
            assert_eq!(thoughts[0].id, thought3_id);
            assert_eq!(thoughts[1].id, thought2_id);
            assert_eq!(thoughts[2].id, thought1_id);
        }
        _ => panic!("Expected ThoughtChain response, got: {response:?}"),
    }
}

// ============================================================================
// Chain Traversal Edge Cases
// ============================================================================

#[tokio::test]
async fn test_get_thought_chain_broken_chain() {
    let actor_ref = spawn_thought_actor().await;

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
        _ => panic!("Expected Error response with ThoughtNotFound, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_get_thought_chain_zero_depth() {
    let actor_ref = spawn_thought_actor().await;

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
        _ => panic!("Expected ThoughtChain response, got: {response:?}"),
    }
}

#[tokio::test]
async fn test_get_thought_chain_depth_limited_by_param() {
    let actor_ref = spawn_thought_actor().await;

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
        }
        _ => panic!("Expected ThoughtChain response, got: {response:?}"),
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
        _ => panic!("Expected Error response, got: {response:?}"),
    }
}

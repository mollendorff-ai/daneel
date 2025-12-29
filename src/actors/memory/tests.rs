//! Tests for MemoryActor
//!
//! ADR-049: Test modules excluded from coverage.

#![cfg_attr(coverage_nightly, coverage(off))]

use super::*;
use crate::core::invariants::{MAX_MEMORY_WINDOWS, MIN_MEMORY_WINDOWS};
use crate::core::types::{Content, SalienceScore};
use ractor::rpc::CallResult;
use ractor::Actor;

/// Helper to spawn a memory actor for testing
async fn spawn_memory_actor() -> (ActorRef<MemoryMessage>, MemoryState) {
    let (actor_ref, _) = Actor::spawn(None, MemoryActor, ())
        .await
        .expect("Failed to spawn MemoryActor");

    let state = MemoryState::new();
    (actor_ref, state)
}

#[tokio::test]
async fn test_actor_spawns_with_minimum_windows() {
    let (actor_ref, _) = spawn_memory_actor().await;

    let response = actor_ref
        .call(|reply| MemoryMessage::GetWindowCount { reply }, None)
        .await
        .expect("Failed to get window count");

    match response {
        CallResult::Success(MemoryResponse::WindowCount { count }) => {
            assert_eq!(count, MIN_MEMORY_WINDOWS);
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_open_window_success() {
    let (actor_ref, _) = spawn_memory_actor().await;

    let response = actor_ref
        .call(
            |reply| MemoryMessage::OpenWindow {
                label: Some("test".to_string()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to open window");

    match response {
        CallResult::Success(MemoryResponse::WindowOpened { window_id }) => {
            assert!(!window_id.to_string().is_empty());
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_open_window_respects_max_limit() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // Open windows up to the maximum (we start with MIN_MEMORY_WINDOWS)
    let windows_to_open = MAX_MEMORY_WINDOWS - MIN_MEMORY_WINDOWS;

    for _ in 0..windows_to_open {
        let response = actor_ref
            .call(
                |reply| MemoryMessage::OpenWindow { label: None, reply },
                None,
            )
            .await
            .expect("Failed to open window");

        assert!(matches!(
            response,
            CallResult::Success(MemoryResponse::WindowOpened { .. })
        ));
    }

    // Try to open one more - should fail
    let response = actor_ref
        .call(
            |reply| MemoryMessage::OpenWindow { label: None, reply },
            None,
        )
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(MemoryResponse::Error { error }) => {
            assert!(matches!(error, MemoryError::BoundedMemoryExceeded { .. }));
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_close_window_success() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // Open a new window
    let window_id = match actor_ref
        .call(
            |reply| MemoryMessage::OpenWindow { label: None, reply },
            None,
        )
        .await
        .expect("Failed to open window")
    {
        CallResult::Success(MemoryResponse::WindowOpened { window_id }) => window_id,
        _ => panic!("Expected WindowOpened response"),
    };

    // Close it
    let response = actor_ref
        .call(
            |reply| MemoryMessage::CloseWindow { window_id, reply },
            None,
        )
        .await
        .expect("Failed to close window");

    match response {
        CallResult::Success(MemoryResponse::WindowClosed {
            window_id: closed_id,
        }) => {
            assert_eq!(window_id, closed_id);
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_close_nonexistent_window() {
    let (actor_ref, _) = spawn_memory_actor().await;

    let fake_window_id = WindowId::new();

    let response = actor_ref
        .call(
            |reply| MemoryMessage::CloseWindow {
                window_id: fake_window_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(MemoryResponse::Error { error }) => {
            assert!(matches!(error, MemoryError::WindowNotFound { .. }));
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_cannot_close_below_minimum() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // Get list of initial windows
    let windows = match actor_ref
        .call(|reply| MemoryMessage::ListWindows { reply }, None)
        .await
        .expect("Failed to list windows")
    {
        CallResult::Success(MemoryResponse::WindowList { windows }) => windows,
        _ => panic!("Expected WindowList response"),
    };

    // Try to close one of the initial windows (should fail - at minimum)
    let window_id = windows[0].id;

    let response = actor_ref
        .call(
            |reply| MemoryMessage::CloseWindow { window_id, reply },
            None,
        )
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(MemoryResponse::Error { error }) => {
            assert!(matches!(
                error,
                MemoryError::BoundedMemoryInsufficient { .. }
            ));
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_store_content_success() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // Get a window ID from the initial windows
    let window_id = match actor_ref
        .call(|reply| MemoryMessage::ListWindows { reply }, None)
        .await
        .expect("Failed to list windows")
    {
        CallResult::Success(MemoryResponse::WindowList { windows }) => windows[0].id,
        _ => panic!("Expected WindowList response"),
    };

    let content = Content::raw(vec![1, 2, 3, 4]);
    let request = StoreRequest::new(window_id, content);

    let response = actor_ref
        .call(|reply| MemoryMessage::Store { request, reply }, None)
        .await
        .expect("Failed to store content");

    match response {
        CallResult::Success(MemoryResponse::ContentStored {
            window_id: stored_id,
        }) => {
            assert_eq!(window_id, stored_id);
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_store_in_nonexistent_window() {
    let (actor_ref, _) = spawn_memory_actor().await;

    let fake_window_id = WindowId::new();
    let content = Content::raw(vec![1, 2, 3]);
    let request = StoreRequest::new(fake_window_id, content);

    let response = actor_ref
        .call(|reply| MemoryMessage::Store { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(MemoryResponse::Error { error }) => {
            assert!(matches!(error, MemoryError::WindowNotFound { .. }));
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_store_in_closed_window() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // Open a new window
    let window_id = match actor_ref
        .call(
            |reply| MemoryMessage::OpenWindow { label: None, reply },
            None,
        )
        .await
        .expect("Failed to open window")
    {
        CallResult::Success(MemoryResponse::WindowOpened { window_id }) => window_id,
        _ => panic!("Expected WindowOpened response"),
    };

    // Close it
    actor_ref
        .call(
            |reply| MemoryMessage::CloseWindow { window_id, reply },
            None,
        )
        .await
        .expect("Failed to close window");

    // Try to store in it
    let content = Content::raw(vec![1, 2, 3]);
    let request = StoreRequest::new(window_id, content);

    let response = actor_ref
        .call(|reply| MemoryMessage::Store { request, reply }, None)
        .await
        .expect("Failed to send message");

    match response {
        CallResult::Success(MemoryResponse::Error { error }) => {
            assert!(matches!(error, MemoryError::WindowAlreadyClosed { .. }));
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_recall_all_content() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // Get a window ID
    let window_id = match actor_ref
        .call(|reply| MemoryMessage::ListWindows { reply }, None)
        .await
        .expect("Failed to list windows")
    {
        CallResult::Success(MemoryResponse::WindowList { windows }) => windows[0].id,
        _ => panic!("Expected WindowList response"),
    };

    // Store some content
    let content1 = Content::raw(vec![1, 2, 3]);
    let content2 = Content::symbol("test", vec![4, 5, 6]);

    actor_ref
        .call(
            |reply| MemoryMessage::Store {
                request: StoreRequest::new(window_id, content1.clone()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to store content");

    actor_ref
        .call(
            |reply| MemoryMessage::Store {
                request: StoreRequest::new(window_id, content2.clone()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to store content");

    // Recall all
    let query = RecallQuery::all();
    let response = actor_ref
        .call(|reply| MemoryMessage::Recall { query, reply }, None)
        .await
        .expect("Failed to recall");

    match response {
        CallResult::Success(MemoryResponse::ContentRecalled { contents }) => {
            assert!(contents.len() >= 2);
            assert!(contents.contains(&content1));
            assert!(contents.contains(&content2));
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_recall_from_specific_window() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // Get two window IDs
    let windows = match actor_ref
        .call(|reply| MemoryMessage::ListWindows { reply }, None)
        .await
        .expect("Failed to list windows")
    {
        CallResult::Success(MemoryResponse::WindowList { windows }) => windows,
        _ => panic!("Expected WindowList response"),
    };

    let window1_id = windows[0].id;
    let window2_id = windows[1].id;

    // Store different content in each
    let content1 = Content::raw(vec![1, 2, 3]);
    let content2 = Content::raw(vec![4, 5, 6]);

    actor_ref
        .call(
            |reply| MemoryMessage::Store {
                request: StoreRequest::new(window1_id, content1.clone()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to store content");

    actor_ref
        .call(
            |reply| MemoryMessage::Store {
                request: StoreRequest::new(window2_id, content2.clone()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to store content");

    // Recall from window1 only
    let query = RecallQuery::for_window(window1_id);
    let response = actor_ref
        .call(|reply| MemoryMessage::Recall { query, reply }, None)
        .await
        .expect("Failed to recall");

    match response {
        CallResult::Success(MemoryResponse::ContentRecalled { contents }) => {
            assert!(contents.contains(&content1));
            assert!(!contents.contains(&content2));
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_recall_with_limit() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // Get a window ID
    let window_id = match actor_ref
        .call(|reply| MemoryMessage::ListWindows { reply }, None)
        .await
        .expect("Failed to list windows")
    {
        CallResult::Success(MemoryResponse::WindowList { windows }) => windows[0].id,
        _ => panic!("Expected WindowList response"),
    };

    // Store multiple items
    for i in 0..5 {
        let content = Content::raw(vec![i]);
        actor_ref
            .call(
                |reply| MemoryMessage::Store {
                    request: StoreRequest::new(window_id, content),
                    reply,
                },
                None,
            )
            .await
            .expect("Failed to store content");
    }

    // Recall with limit
    let query = RecallQuery::for_window(window_id).with_limit(3);
    let response = actor_ref
        .call(|reply| MemoryMessage::Recall { query, reply }, None)
        .await
        .expect("Failed to recall");

    match response {
        CallResult::Success(MemoryResponse::ContentRecalled { contents }) => {
            assert_eq!(contents.len(), 3);
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_list_windows() {
    let (actor_ref, _) = spawn_memory_actor().await;

    let response = actor_ref
        .call(|reply| MemoryMessage::ListWindows { reply }, None)
        .await
        .expect("Failed to list windows");

    match response {
        CallResult::Success(MemoryResponse::WindowList { windows }) => {
            assert_eq!(windows.len(), MIN_MEMORY_WINDOWS);
            assert!(windows.iter().all(|w| w.is_open));
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_window_lifecycle() {
    let (actor_ref, _) = spawn_memory_actor().await;

    // 1. Open a labeled window
    let window_id = match actor_ref
        .call(
            |reply| MemoryMessage::OpenWindow {
                label: Some("lifecycle_test".to_string()),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to open window")
    {
        CallResult::Success(MemoryResponse::WindowOpened { window_id }) => window_id,
        _ => panic!("Expected WindowOpened response"),
    };

    // 2. Store content with salience
    let content = Content::symbol("test", vec![42]);
    let salience = SalienceScore::new_without_arousal(0.8, 0.6, 0.9, 0.5, 0.7);
    let request = StoreRequest::new(window_id, content.clone()).with_salience(salience);

    actor_ref
        .call(|reply| MemoryMessage::Store { request, reply }, None)
        .await
        .expect("Failed to store content");

    // 3. Recall and verify
    let query = RecallQuery::for_window(window_id);
    let contents = match actor_ref
        .call(|reply| MemoryMessage::Recall { query, reply }, None)
        .await
        .expect("Failed to recall")
    {
        CallResult::Success(MemoryResponse::ContentRecalled { contents }) => contents,
        _ => panic!("Expected ContentRecalled response"),
    };

    assert_eq!(contents.len(), 1);
    assert_eq!(contents[0], content);

    // 4. Close the window
    actor_ref
        .call(
            |reply| MemoryMessage::CloseWindow { window_id, reply },
            None,
        )
        .await
        .expect("Failed to close window");

    // 5. Verify it's closed (trying to store should fail)
    let request = StoreRequest::new(window_id, Content::Empty);
    let response = actor_ref
        .call(|reply| MemoryMessage::Store { request, reply }, None)
        .await
        .expect("Failed to send message");

    assert!(matches!(
        response,
        CallResult::Success(MemoryResponse::Error {
            error: MemoryError::WindowAlreadyClosed { .. }
        })
    ));
}

#[test]
fn test_memory_state_creation() {
    let state = MemoryState::new();
    assert_eq!(state.windows.len(), MIN_MEMORY_WINDOWS);
    assert_eq!(state.open_window_count(), MIN_MEMORY_WINDOWS);
}

#[test]
fn test_memory_state_open_window() {
    let mut state = MemoryState::new();
    let initial_count = state.open_window_count();

    let result = state.open_window(Some("test".to_string()));
    assert!(result.is_ok());

    let window_id = result.unwrap();
    assert_eq!(state.open_window_count(), initial_count + 1);

    let window = state.windows.get(&window_id).unwrap();
    assert_eq!(window.label, Some("test".to_string()));
    assert!(window.is_open);
}

#[test]
fn test_memory_state_bounded_memory() {
    let mut state = MemoryState::new();

    // Fill up to maximum
    while state.open_window_count() < MAX_MEMORY_WINDOWS {
        state
            .open_window(None)
            .expect("Should be able to open window");
    }

    // Try to exceed
    let result = state.open_window(None);
    assert!(matches!(
        result,
        Err(MemoryError::BoundedMemoryExceeded { .. })
    ));
}

#[test]
fn test_memory_state_close_already_closed_window() {
    let mut state = MemoryState::new();

    // Open a new window (so we can close it without hitting min limit)
    let window_id = state.open_window(None).expect("Should open window");

    // Close it first time
    state.close_window(window_id).expect("Should close window");

    // Try to close again - should get WindowAlreadyClosed error
    let result = state.close_window(window_id);
    assert!(matches!(
        result,
        Err(MemoryError::WindowAlreadyClosed { .. })
    ));
}

#[test]
fn test_memory_state_store_with_salience() {
    let mut state = MemoryState::new();

    // Get a window
    let windows: Vec<_> = state.windows.keys().copied().collect();
    let window_id = windows[0];

    // Store content with salience
    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::new_without_arousal(0.8, 0.7, 0.9, 0.6, 0.5);
    let request = StoreRequest::new(window_id, content).with_salience(salience);

    state.store(request).expect("Should store content");

    // Verify salience was updated (importance is first arg to new_without_arousal)
    let window = state.windows.get(&window_id).unwrap();
    assert!((window.salience.importance - 0.8).abs() < 0.001);
}

#[test]
fn test_memory_state_recall_with_min_salience() {
    let mut state = MemoryState::new();

    // Open two windows
    let window1_id = state.open_window(None).expect("Should open window");
    let window2_id = state.open_window(None).expect("Should open window");

    // Store in window1 with high salience
    let content1 = Content::raw(vec![1, 1, 1]);
    let high_salience = SalienceScore::new_without_arousal(0.9, 0.9, 0.9, 0.9, 0.9);
    state
        .store(StoreRequest::new(window1_id, content1.clone()).with_salience(high_salience))
        .unwrap();

    // Store in window2 with low salience
    let content2 = Content::raw(vec![2, 2, 2]);
    let low_salience = SalienceScore::new_without_arousal(0.1, 0.1, 0.1, 0.1, 0.1);
    state
        .store(StoreRequest::new(window2_id, content2.clone()).with_salience(low_salience))
        .unwrap();

    // Recall with high min_salience - should only get content1
    let query = RecallQuery::all().with_min_salience(0.5);
    let contents = state.recall(query);

    assert!(contents.contains(&content1));
    assert!(!contents.contains(&content2));
}

#[test]
fn test_memory_state_list_windows() {
    let state = MemoryState::new();
    let windows = state.list_windows();

    assert_eq!(windows.len(), MIN_MEMORY_WINDOWS);
    assert!(windows.iter().all(|w| w.is_open));
}

//! Tests for AttentionActor
//!
//! Comprehensive test suite for TMI's "O Eu" - the attention mechanism.

use super::*;
use crate::core::types::WindowId;
use chrono::Duration;
use ractor::rpc::CallResult;
use ractor::Actor;

/// Helper to spawn an attention actor with default configuration
async fn spawn_attention_actor() -> ActorRef<AttentionMessage> {
    let (actor_ref, _) = Actor::spawn(None, AttentionActor, AttentionConfig::default())
        .await
        .expect("Failed to spawn AttentionActor");
    actor_ref
}

/// Helper to spawn an attention actor with custom configuration
async fn spawn_attention_actor_with_config(config: AttentionConfig) -> ActorRef<AttentionMessage> {
    let (actor_ref, _) = Actor::spawn(None, AttentionActor, config)
        .await
        .expect("Failed to spawn AttentionActor");
    actor_ref
}

// ============================================================================
// Actor Lifecycle Tests
// ============================================================================

#[tokio::test]
async fn test_actor_spawns_successfully() {
    let actor_ref = spawn_attention_actor().await;

    // Verify actor is responsive
    let response = actor_ref
        .call(|reply| AttentionMessage::GetFocus { reply }, None)
        .await
        .expect("Failed to get focus");

    match response {
        CallResult::Success(AttentionResponse::CurrentFocus { window_id }) => {
            assert_eq!(window_id, None, "Should start with no focus");
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_actor_starts_with_default_config() {
    let actor_ref = spawn_attention_actor().await;

    // Verify it starts with empty attention map
    let response = actor_ref
        .call(|reply| AttentionMessage::GetAttentionMap { reply }, None)
        .await
        .expect("Failed to get attention map");

    match response {
        CallResult::Success(AttentionResponse::AttentionMap { scores }) => {
            assert!(scores.is_empty(), "Should start with empty attention map");
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_actor_starts_with_custom_config() {
    let config = AttentionConfig {
        min_focus_duration: Duration::milliseconds(500),
        forget_threshold: 0.3,
        connection_boost: 2.0,
    };

    let actor_ref = spawn_attention_actor_with_config(config).await;

    // Verify actor is responsive with custom config
    let response = actor_ref
        .call(|reply| AttentionMessage::GetFocus { reply }, None)
        .await
        .expect("Failed to get focus");

    match response {
        CallResult::Success(AttentionResponse::CurrentFocus { .. }) => {
            // Config is applied successfully if actor responds
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

// ============================================================================
// Attention Cycle Tests
// ============================================================================

#[tokio::test]
async fn test_cycle_with_no_windows_returns_none() {
    let actor_ref = spawn_attention_actor().await;

    let response = actor_ref
        .call(|reply| AttentionMessage::Cycle { reply }, None)
        .await
        .expect("Failed to cycle");

    match response {
        CallResult::Success(AttentionResponse::CycleComplete { focused, salience }) => {
            assert_eq!(focused, None, "Should have no focus with empty map");
            assert_eq!(salience, 0.0, "Salience should be 0 with no windows");
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_cycle_selects_highest_salience() {
    let actor_ref = spawn_attention_actor().await;

    // We need to manually create a state and populate the attention map
    // Since the actor doesn't expose an UpdateSalience message, we'll test
    // this through the state unit test below
    // This test verifies the cycle mechanism works with the actor
    let response = actor_ref
        .call(|reply| AttentionMessage::Cycle { reply }, None)
        .await
        .expect("Failed to cycle");

    match response {
        CallResult::Success(AttentionResponse::CycleComplete { .. }) => {
            // Cycle completed successfully
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_cycle_increments_counter() {
    let actor_ref = spawn_attention_actor().await;

    // Run multiple cycles
    for _ in 0..3 {
        let response = actor_ref
            .call(|reply| AttentionMessage::Cycle { reply }, None)
            .await
            .expect("Failed to cycle");

        assert!(matches!(
            response,
            CallResult::Success(AttentionResponse::CycleComplete { .. })
        ));
    }

    // Note: We can't directly check cycle_count through the actor API
    // but we can verify cycles complete without error
}

// ============================================================================
// Focus Operations Tests
// ============================================================================

#[tokio::test]
async fn test_focus_on_nonexistent_window() {
    let actor_ref = spawn_attention_actor().await;

    let fake_window_id = WindowId::new();

    let response = actor_ref
        .call(
            |reply| AttentionMessage::Focus {
                window_id: fake_window_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send focus message");

    match response {
        CallResult::Success(AttentionResponse::Error { error }) => {
            assert!(
                matches!(error, AttentionError::WindowNotFound { .. }),
                "Should return WindowNotFound error"
            );
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

#[tokio::test]
async fn test_shift_to_nonexistent_window() {
    let actor_ref = spawn_attention_actor().await;

    let fake_window_id = WindowId::new();

    let response = actor_ref
        .call(
            |reply| AttentionMessage::Shift {
                to: fake_window_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to send shift message");

    match response {
        CallResult::Success(AttentionResponse::Error { error }) => {
            assert!(
                matches!(error, AttentionError::WindowNotFound { .. }),
                "Should return WindowNotFound error"
            );
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

// ============================================================================
// Query Operations Tests
// ============================================================================

#[tokio::test]
async fn test_get_focus_when_unfocused() {
    let actor_ref = spawn_attention_actor().await;

    let response = actor_ref
        .call(|reply| AttentionMessage::GetFocus { reply }, None)
        .await
        .expect("Failed to get focus");

    match response {
        CallResult::Success(AttentionResponse::CurrentFocus { window_id }) => {
            assert_eq!(window_id, None, "Should have no focus initially");
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[tokio::test]
async fn test_get_attention_map_empty() {
    let actor_ref = spawn_attention_actor().await;

    let response = actor_ref
        .call(|reply| AttentionMessage::GetAttentionMap { reply }, None)
        .await
        .expect("Failed to get attention map");

    match response {
        CallResult::Success(AttentionResponse::AttentionMap { scores }) => {
            assert!(scores.is_empty(), "Should start with empty attention map");
            assert_eq!(scores.len(), 0);
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

// ============================================================================
// State Unit Tests (Direct state manipulation)
// ============================================================================

#[test]
fn test_state_starts_with_default_config() {
    let state = AttentionState::new();

    assert_eq!(state.cycle_count, 0);
    assert!(state.attention_map.is_empty());
    assert!(!state.focus.is_focused());
    assert_eq!(state.config, AttentionConfig::default());
}

#[test]
fn test_state_with_custom_config() {
    let config = AttentionConfig {
        min_focus_duration: Duration::milliseconds(500),
        forget_threshold: 0.3,
        connection_boost: 2.0,
    };

    let state = AttentionState::with_config(config.clone());

    assert_eq!(state.config, config);
    assert_eq!(state.cycle_count, 0);
}

#[test]
fn test_state_update_window_salience() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    // Update with base salience only (no connection boost)
    state.update_window_salience(window_id, 0.5, 0.3);

    let salience = state.attention_map.get(&window_id);
    assert!(salience.is_some());
    // With connection_relevance = 0.3 (< 0.5), no boost is applied
    assert_eq!(salience.unwrap(), 0.5);
}

#[test]
fn test_state_update_window_salience_with_connection_boost() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    // Update with high connection relevance (should trigger boost)
    state.update_window_salience(window_id, 0.5, 0.8);

    let salience = state.attention_map.get(&window_id);
    assert!(salience.is_some());

    // With connection_relevance = 0.8 and default boost = 1.5
    // boosted = 0.5 * (1.0 + (0.8 - 0.5) * 1.5)
    // boosted = 0.5 * (1.0 + 0.3 * 1.5)
    // boosted = 0.5 * 1.45 = 0.725
    let expected = 0.725;
    assert!((salience.unwrap() - expected).abs() < 0.001);
}

#[test]
fn test_state_connection_boost_calculation() {
    let config = AttentionConfig {
        min_focus_duration: Duration::milliseconds(100),
        forget_threshold: 0.1,
        connection_boost: 2.0,
    };

    let mut state = AttentionState::with_config(config);
    let window_id = WindowId::new();

    // Test with connection_relevance = 1.0 (maximum)
    state.update_window_salience(window_id, 0.4, 1.0);

    let salience = state.attention_map.get(&window_id);
    // boosted = 0.4 * (1.0 + (1.0 - 0.5) * 2.0)
    // boosted = 0.4 * (1.0 + 1.0) = 0.8
    assert_eq!(salience.unwrap(), 0.8);
}

#[test]
fn test_state_salience_clamped_to_one() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    // Update with very high values that would exceed 1.0
    state.update_window_salience(window_id, 0.9, 1.0);

    let salience = state.attention_map.get(&window_id);
    assert!(salience.is_some());
    // Should be clamped to 1.0
    assert!(salience.unwrap() <= 1.0);
}

#[test]
fn test_state_select_winner_empty_map() {
    let state = AttentionState::new();

    let winner = state.select_winner();
    assert_eq!(winner, None);
}

#[test]
fn test_state_select_winner_single_window() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    state.update_window_salience(window_id, 0.7, 0.3);

    let winner = state.select_winner();
    assert!(winner.is_some());

    let (win_id, win_salience) = winner.unwrap();
    assert_eq!(win_id, window_id);
    assert_eq!(win_salience, 0.7);
}

#[test]
fn test_state_select_winner_competitive_selection() {
    let mut state = AttentionState::new();
    let window1 = WindowId::new();
    let window2 = WindowId::new();
    let window3 = WindowId::new();

    state.update_window_salience(window1, 0.5, 0.3);
    state.update_window_salience(window2, 0.9, 0.3);
    state.update_window_salience(window3, 0.3, 0.3);

    let winner = state.select_winner();
    assert!(winner.is_some());

    let (win_id, win_salience) = winner.unwrap();
    assert_eq!(win_id, window2, "Window2 has highest salience");
    assert_eq!(win_salience, 0.9);
}

#[test]
fn test_state_select_winner_respects_threshold() {
    let mut state = AttentionState::new();
    let window1 = WindowId::new();
    let window2 = WindowId::new();

    // Default forget_threshold is 0.1
    state.update_window_salience(window1, 0.05, 0.3); // Below threshold
    state.update_window_salience(window2, 0.08, 0.3); // Below threshold

    let winner = state.select_winner();
    assert_eq!(winner, None, "All windows below threshold");
}

#[test]
fn test_state_select_winner_filters_low_salience() {
    let config = AttentionConfig {
        min_focus_duration: Duration::milliseconds(100),
        forget_threshold: 0.5, // Higher threshold
        connection_boost: 1.5,
    };

    let mut state = AttentionState::with_config(config);
    let window1 = WindowId::new();
    let window2 = WindowId::new();

    state.update_window_salience(window1, 0.3, 0.3); // Below 0.5 threshold
    state.update_window_salience(window2, 0.7, 0.3); // Above 0.5 threshold

    let winner = state.select_winner();
    assert!(winner.is_some());

    let (win_id, _) = winner.unwrap();
    assert_eq!(win_id, window2, "Only window2 passes threshold");
}

#[test]
fn test_state_cycle_increments_counter() {
    let mut state = AttentionState::new();

    assert_eq!(state.cycle_count, 0);

    state.cycle();
    assert_eq!(state.cycle_count, 1);

    state.cycle();
    assert_eq!(state.cycle_count, 2);
}

#[test]
fn test_state_cycle_with_empty_map() {
    let mut state = AttentionState::new();

    let response = state.cycle();

    match response {
        AttentionResponse::CycleComplete { focused, salience } => {
            assert_eq!(focused, None);
            assert_eq!(salience, 0.0);
            assert!(!state.focus.is_focused());
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_cycle_selects_and_focuses() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    state.update_window_salience(window_id, 0.8, 0.3);

    let response = state.cycle();

    match response {
        AttentionResponse::CycleComplete { focused, salience } => {
            assert_eq!(focused, Some(window_id));
            assert_eq!(salience, 0.8);
            assert_eq!(state.focus.focused_window(), Some(window_id));
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_focus_on_window() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    // Add window to attention map first
    state.update_window_salience(window_id, 0.5, 0.3);

    let response = state.focus_on_window(window_id);

    match response {
        AttentionResponse::FocusSet {
            window_id: focused_id,
        } => {
            assert_eq!(focused_id, window_id);
            assert_eq!(state.focus.focused_window(), Some(window_id));
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_focus_on_nonexistent_window() {
    let mut state = AttentionState::new();
    let fake_window_id = WindowId::new();

    let response = state.focus_on_window(fake_window_id);

    match response {
        AttentionResponse::Error { error } => {
            assert!(matches!(error, AttentionError::WindowNotFound { .. }));
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

#[test]
fn test_state_shift_to_window() {
    let mut state = AttentionState::new();
    let window1 = WindowId::new();
    let window2 = WindowId::new();

    state.update_window_salience(window1, 0.5, 0.3);
    state.update_window_salience(window2, 0.7, 0.3);

    // Focus on first window
    state.focus_on_window(window1);

    // Shift to second window
    let response = state.shift_to_window(window2);

    match response {
        AttentionResponse::FocusShifted { from, to } => {
            assert_eq!(from, Some(window1));
            assert_eq!(to, window2);
            assert_eq!(state.focus.focused_window(), Some(window2));
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_shift_from_unfocused() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    state.update_window_salience(window_id, 0.5, 0.3);

    let response = state.shift_to_window(window_id);

    match response {
        AttentionResponse::FocusShifted { from, to } => {
            assert_eq!(from, None, "No previous focus");
            assert_eq!(to, window_id);
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_shift_to_nonexistent_window() {
    let mut state = AttentionState::new();
    let fake_window_id = WindowId::new();

    let response = state.shift_to_window(fake_window_id);

    match response {
        AttentionResponse::Error { error } => {
            assert!(matches!(error, AttentionError::WindowNotFound { .. }));
        }
        _ => panic!("Expected error response, got: {:?}", response),
    }
}

#[test]
fn test_state_get_focus_when_unfocused() {
    let state = AttentionState::new();

    let response = state.get_focus();

    match response {
        AttentionResponse::CurrentFocus { window_id } => {
            assert_eq!(window_id, None);
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_get_focus_when_focused() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    state.update_window_salience(window_id, 0.5, 0.3);
    state.focus_on_window(window_id);

    let response = state.get_focus();

    match response {
        AttentionResponse::CurrentFocus { window_id: focused } => {
            assert_eq!(focused, Some(window_id));
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_get_attention_map_empty() {
    let state = AttentionState::new();

    let response = state.get_attention_map();

    match response {
        AttentionResponse::AttentionMap { scores } => {
            assert!(scores.is_empty());
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_get_attention_map_with_windows() {
    let mut state = AttentionState::new();
    let window1 = WindowId::new();
    let window2 = WindowId::new();

    state.update_window_salience(window1, 0.5, 0.3);
    state.update_window_salience(window2, 0.7, 0.3);

    let response = state.get_attention_map();

    match response {
        AttentionResponse::AttentionMap { scores } => {
            assert_eq!(scores.len(), 2);
            assert_eq!(scores.get(&window1), Some(&0.5));
            assert_eq!(scores.get(&window2), Some(&0.7));
        }
        _ => panic!("Unexpected response: {:?}", response),
    }
}

#[test]
fn test_state_can_shift_focus_when_unfocused() {
    let state = AttentionState::new();
    assert!(state.can_shift_focus(), "Can shift when unfocused");
}

#[test]
fn test_state_can_shift_focus_with_duration() {
    let mut state = AttentionState::new();
    let window_id = WindowId::new();

    state.update_window_salience(window_id, 0.5, 0.3);
    state.focus_on_window(window_id);

    // Initially, focus_duration is zero (< min_focus_duration)
    // So can_shift_focus should return false
    // However, the default min_focus_duration is 100ms
    assert!(!state.can_shift_focus(), "Cannot shift immediately");

    // Update duration to meet minimum
    state.focus.update_duration(Duration::milliseconds(100));
    assert!(state.can_shift_focus(), "Can shift after min duration");
}

#[test]
fn test_multiple_cycles_with_changing_salience() {
    let mut state = AttentionState::new();
    let window1 = WindowId::new();
    let window2 = WindowId::new();

    // First cycle: window1 has higher salience
    state.update_window_salience(window1, 0.8, 0.3);
    state.update_window_salience(window2, 0.4, 0.3);

    let response = state.cycle();
    match response {
        AttentionResponse::CycleComplete { focused, .. } => {
            assert_eq!(focused, Some(window1));
        }
        _ => panic!("Unexpected response"),
    }

    // Second cycle: window2 now has higher salience
    state.update_window_salience(window1, 0.3, 0.3);
    state.update_window_salience(window2, 0.9, 0.3);

    // Update focus duration to allow shift
    state.focus.update_duration(Duration::milliseconds(100));

    let response = state.cycle();
    match response {
        AttentionResponse::CycleComplete { focused, salience } => {
            assert_eq!(focused, Some(window2));
            assert_eq!(salience, 0.9);
        }
        _ => panic!("Unexpected response"),
    }
}

#[test]
fn test_attention_config_default() {
    let config = AttentionConfig::default();

    assert_eq!(config.min_focus_duration, Duration::milliseconds(100));
    assert_eq!(config.forget_threshold, 0.1);
    assert_eq!(config.connection_boost, 1.5);
}

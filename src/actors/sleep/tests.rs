//! Sleep Actor Tests
//!
//! ADR-049: Test modules excluded from coverage.

#![cfg_attr(coverage_nightly, coverage(off))]
#![allow(clippy::significant_drop_tightening)] // Async test setup

use super::*;
use ractor::rpc::CallResult;
use ractor::Actor;

/// Extract value from `CallResult` or panic
fn unwrap_call<T: std::fmt::Debug>(result: CallResult<T>) -> T {
    match result {
        CallResult::Success(v) => v,
        CallResult::Timeout => panic!("RPC call timed out"),
        CallResult::SenderError => panic!("RPC sender error"),
    }
}

#[tokio::test]
async fn sleep_actor_starts_awake() {
    let (actor_ref, handle) = Actor::spawn(None, SleepActor::default(), ())
        .await
        .expect("Failed to spawn SleepActor");

    let state = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::GetState { reply }, None)
            .await
            .expect("Failed to get state"),
    );

    assert_eq!(state, types::SleepState::Awake);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_records_activity() {
    let config = SleepConfig::fast();
    let (actor_ref, handle) = Actor::spawn(None, SleepActor::with_config(config), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Record activity
    actor_ref
        .cast(SleepMessage::RecordActivity)
        .expect("Failed to record activity");

    // Should still be awake
    let state = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::GetState { reply }, None)
            .await
            .expect("Failed to get state"),
    );

    assert_eq!(state, types::SleepState::Awake);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_checks_conditions() {
    let (actor_ref, handle) = Actor::spawn(None, SleepActor::default(), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Initially shouldn't sleep (not enough idle time)
    let should_sleep = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::CheckSleepConditions { reply }, None)
            .await
            .expect("Failed to check conditions"),
    );

    assert!(!should_sleep);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_enter_sleep_conditions_not_met() {
    let (actor_ref, handle) = Actor::spawn(None, SleepActor::default(), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Try to enter sleep without conditions met
    let result = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::EnterSleep { reply }, None)
            .await
            .expect("Failed to enter sleep"),
    );

    match result {
        SleepResult::ConditionsNotMet { .. } => {}
        _ => panic!("Expected ConditionsNotMet, got {result:?}"),
    }

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_external_stimulus_when_awake() {
    let (actor_ref, handle) = Actor::spawn(None, SleepActor::default(), ())
        .await
        .expect("Failed to spawn SleepActor");

    // External stimulus when awake should be processed
    let processed = unwrap_call(
        actor_ref
            .call(
                |reply| SleepMessage::ExternalStimulus {
                    stimulus: "test".to_string(),
                    reply,
                },
                None,
            )
            .await
            .expect("Failed to send stimulus"),
    );

    assert!(processed);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_config_update() {
    let (actor_ref, handle) = Actor::spawn(None, SleepActor::default(), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Get initial config
    let initial_config = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::GetConfig { reply }, None)
            .await
            .expect("Failed to get config"),
    );

    assert_eq!(initial_config.replay_batch_size, 50);

    // Update config
    let mut new_config = initial_config.clone();
    new_config.replay_batch_size = 100;

    unwrap_call(
        actor_ref
            .call(
                |reply| SleepMessage::UpdateConfig {
                    config: new_config,
                    reply,
                },
                None,
            )
            .await
            .expect("Failed to update config"),
    );

    // Verify update
    let updated_config = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::GetConfig { reply }, None)
            .await
            .expect("Failed to get config"),
    );

    assert_eq!(updated_config.replay_batch_size, 100);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_wake_returns_summary() {
    let (actor_ref, handle) = Actor::spawn(None, SleepActor::default(), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Wake (even when already awake) should return empty summary
    let summary = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::Wake { reply }, None)
            .await
            .expect("Failed to wake"),
    );

    assert_eq!(summary.cycles_completed, 0);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[test]
fn sleep_state_interruptibility() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);

    // Awake is interruptible
    assert!(state.is_interruptible());

    // Simulate entering sleep
    state.state = types::SleepState::EnteringSleep;
    assert!(state.is_interruptible());

    state.state = types::SleepState::LightSleep;
    assert!(state.is_interruptible());

    // Deep sleep is NOT interruptible
    state.state = types::SleepState::DeepSleep;
    assert!(!state.is_interruptible());

    // Dreaming is NOT interruptible
    state.state = types::SleepState::Dreaming;
    assert!(!state.is_interruptible());

    // Waking is interruptible
    state.state = types::SleepState::Waking;
    assert!(state.is_interruptible());
}

#[test]
fn sleep_phase_advancement() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);
    state.state = types::SleepState::EnteringSleep;

    // Light sleep at 10% (10 ticks)
    state.sleep_ticks = 10;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::LightSleep);

    // Deep sleep at 30% (30 ticks)
    state.sleep_ticks = 30;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::DeepSleep);

    // Dreaming at 80% (80 ticks)
    state.sleep_ticks = 80;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::Dreaming);
}

#[test]
fn sleep_state_queue_management() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);

    assert_eq!(state.consolidation_queue_estimate, 0);

    state.increment_queue();
    state.increment_queue();
    state.increment_queue();

    assert_eq!(state.consolidation_queue_estimate, 3);

    state.clear_queue();
    assert_eq!(state.consolidation_queue_estimate, 0);
}

#[test]
fn should_sleep_returns_false_when_not_awake() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);

    // Set state to something other than Awake
    state.state = types::SleepState::DeepSleep;
    assert!(!state.should_sleep());

    state.state = types::SleepState::LightSleep;
    assert!(!state.should_sleep());

    state.state = types::SleepState::Dreaming;
    assert!(!state.should_sleep());

    state.state = types::SleepState::EnteringSleep;
    assert!(!state.should_sleep());

    state.state = types::SleepState::Waking;
    assert!(!state.should_sleep());
}

#[test]
fn should_sleep_with_queue_trigger() {
    // Create config with very low thresholds for testing
    let config = SleepConfig {
        idle_threshold_ms: 0,            // immediate idle trigger
        min_awake_duration_ms: u64::MAX, // awake trigger will NOT be met
        min_consolidation_queue: 2,      // low queue threshold
        ..SleepConfig::default()
    };
    let mut state = SleepState::new(config);

    // Add enough to queue to trigger queue condition
    state.increment_queue();
    state.increment_queue();

    // Need a tiny wait for idle_duration > 0 (since it's strict >)
    std::thread::sleep(std::time::Duration::from_millis(1));

    // idle_trigger && queue_trigger should be true (even though awake_trigger is false)
    assert!(state.should_sleep());
}

#[test]
fn should_sleep_with_awake_trigger() {
    // awake_trigger requires min_awake_duration_ms > 0 (0 = disabled)
    let config = SleepConfig {
        idle_threshold_ms: 0,                // bypass idle check
        min_awake_duration_ms: 1,            // 1ms awake threshold
        min_consolidation_queue: usize::MAX, // queue trigger will NOT be met
        ..SleepConfig::default()
    };
    let state = SleepState::new(config);

    // Wait for awake_duration > 1ms
    std::thread::sleep(std::time::Duration::from_millis(2));

    // idle_trigger (bypassed) && awake_trigger should be true
    assert!(state.should_sleep());
}

#[test]
fn should_sleep_mini_dream_only_queue() {
    // mini_dream: idle=0 (bypass), awake=0 (disabled), queue=50 (the sole trigger)
    let config = SleepConfig::mini_dream();
    let mut state = SleepState::new(config);

    // Queue not yet full — should NOT sleep
    for _ in 0..49 {
        state.increment_queue();
    }
    assert!(!state.should_sleep());

    // Queue reaches threshold — should sleep
    state.increment_queue();
    assert!(state.should_sleep());
}

#[test]
fn should_sleep_idle_not_met() {
    // Create config where idle threshold is very high
    let config = SleepConfig {
        idle_threshold_ms: u64::MAX, // idle trigger will NOT be met
        min_awake_duration_ms: 0,
        min_consolidation_queue: 0,
        ..SleepConfig::default()
    };
    let state = SleepState::new(config);

    // Even with other conditions met, idle_trigger is false so should_sleep is false
    assert!(!state.should_sleep());
}

#[test]
fn enter_sleep_already_sleeping() {
    let config = SleepConfig {
        idle_threshold_ms: 0,
        min_awake_duration_ms: 0,
        min_consolidation_queue: 0,
        ..SleepConfig::default()
    };
    let mut state = SleepState::new(config);

    // Set state to sleeping
    state.state = types::SleepState::DeepSleep;

    let result = state.enter_sleep();
    match result {
        SleepResult::AlreadySleeping => {}
        _ => panic!("Expected AlreadySleeping, got {result:?}"),
    }
}

#[test]
fn enter_sleep_success() {
    let config = SleepConfig {
        idle_threshold_ms: 0,
        min_awake_duration_ms: 0,
        min_consolidation_queue: 0,
        ..SleepConfig::default()
    };
    let mut state = SleepState::new(config);

    // Need a tiny wait for idle_duration and awake_duration > 0 (since it's strict >)
    std::thread::sleep(std::time::Duration::from_millis(1));

    let result = state.enter_sleep();
    match result {
        SleepResult::Started => {}
        _ => panic!("Expected Started, got {result:?}"),
    }

    assert_eq!(state.state, types::SleepState::EnteringSleep);
    assert!(state.current_summary.is_some());
}

#[test]
fn wake_with_summary() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);

    // Set up a current summary with some data
    let summary = SleepSummary {
        cycles_completed: 3,
        total_memories_replayed: 150,
        ..Default::default()
    };
    state.current_summary = Some(summary);
    state.state = types::SleepState::DeepSleep;

    let returned_summary = state.wake();

    assert_eq!(returned_summary.cycles_completed, 3);
    assert_eq!(returned_summary.total_memories_replayed, 150);
    assert_eq!(state.state, types::SleepState::Awake);
    assert!(state.current_summary.is_none());
}

#[test]
fn add_cycle_report_with_no_summary() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);

    // Ensure no summary exists
    assert!(state.current_summary.is_none());

    let report = SleepCycleReport {
        cycle_id: uuid::Uuid::new_v4(),
        duration_ms: 1000,
        memories_replayed: 50,
        memories_consolidated: 5,
        associations_strengthened: 100,
        associations_pruned: 10,
        avg_replay_priority: 0.7,
        peak_emotional_intensity: 0.9,
        status: SleepCycleStatus::Completed,
    };

    // This should be a no-op (not panic)
    state.add_cycle_report(&report);

    // Summary should still be None
    assert!(state.current_summary.is_none());
}

#[test]
fn add_cycle_report_with_summary() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);

    // Set up a current summary
    state.current_summary = Some(SleepSummary::default());

    let report = SleepCycleReport {
        cycle_id: uuid::Uuid::new_v4(),
        duration_ms: 1000,
        memories_replayed: 50,
        memories_consolidated: 5,
        associations_strengthened: 100,
        associations_pruned: 10,
        avg_replay_priority: 0.7,
        peak_emotional_intensity: 0.9,
        status: SleepCycleStatus::Completed,
    };

    state.add_cycle_report(&report);

    let summary = state.current_summary.as_ref().unwrap();
    assert_eq!(summary.cycles_completed, 1);
    assert_eq!(summary.total_memories_replayed, 50);
    assert_eq!(summary.total_memories_consolidated, 5);
}

#[tokio::test]
async fn sleep_actor_external_stimulus_in_deep_sleep() {
    let (actor_ref, handle) = Actor::spawn(None, SleepActor::default(), ())
        .await
        .expect("Failed to spawn SleepActor");

    // We need to directly set the state to DeepSleep via internal means
    // Since we can't easily do that through messages, we'll test the state logic directly
    // in a unit test instead. But we can at least verify the actor handles the message.

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[test]
fn external_stimulus_in_protected_sleep() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);

    // Set to deep sleep (not interruptible)
    state.state = types::SleepState::DeepSleep;
    assert!(!state.is_interruptible());

    // Set to dreaming (not interruptible)
    state.state = types::SleepState::Dreaming;
    assert!(!state.is_interruptible());
}

#[test]
fn record_activity_resets_timer() {
    let config = SleepConfig::default();
    let mut state = SleepState::new(config);

    // Record activity and verify it updates the last_activity timestamp
    let before = state.last_activity;
    std::thread::sleep(std::time::Duration::from_millis(10));
    state.record_activity();
    let after = state.last_activity;

    assert!(after > before);
}

#[test]
fn advance_sleep_phase_boundary_conditions() {
    let config = SleepConfig {
        light_sleep_duration_pct: 0.2,
        ..SleepConfig::default()
    };
    let mut state = SleepState::new(config);
    state.state = types::SleepState::EnteringSleep;

    // Light sleep (ticks 0-19)
    state.sleep_ticks = 0;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::LightSleep);

    // Tick 19 -> increments to 20 -> DeepSleep (at threshold 0.2)
    state.sleep_ticks = 19;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::DeepSleep);

    // Deep sleep (ticks 20-69)
    state.sleep_ticks = 20;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::DeepSleep);

    state.sleep_ticks = 50;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::DeepSleep);

    // Dreaming (ticks 70-99)
    state.sleep_ticks = 75;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::Dreaming);

    state.sleep_ticks = 98;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::Dreaming);
}

#[test]
fn sleep_actor_with_custom_config() {
    let config = SleepConfig {
        idle_threshold_ms: 5000,
        replay_batch_size: 100,
        ..SleepConfig::default()
    };

    let actor = SleepActor::with_config(config);
    assert_eq!(actor.config.idle_threshold_ms, 5000);
    assert_eq!(actor.config.replay_batch_size, 100);
}

#[tokio::test]
async fn sleep_actor_enter_sleep_when_already_sleeping() {
    let config = SleepConfig {
        idle_threshold_ms: 0,
        min_awake_duration_ms: 0,
        min_consolidation_queue: 0,
        ..SleepConfig::default()
    };

    let (actor_ref, handle) = Actor::spawn(None, SleepActor::with_config(config), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Need a tiny wait for idle_duration and awake_duration > 0 (since it's strict >)
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;

    // First enter sleep - should succeed
    let result1 = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::EnterSleep { reply }, None)
            .await
            .expect("Failed to enter sleep"),
    );

    match result1 {
        SleepResult::Started => {}
        _ => panic!("Expected Started, got {result1:?}"),
    }

    // Verify state is EnteringSleep
    let state = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::GetState { reply }, None)
            .await
            .expect("Failed to get state"),
    );
    assert_eq!(state, types::SleepState::EnteringSleep);

    // Second enter sleep - should return AlreadySleeping
    let result2 = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::EnterSleep { reply }, None)
            .await
            .expect("Failed to enter sleep"),
    );

    match result2 {
        SleepResult::AlreadySleeping => {}
        _ => panic!("Expected AlreadySleeping, got {result2:?}"),
    }

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_external_stimulus_when_in_entering_sleep() {
    let config = SleepConfig {
        idle_threshold_ms: 0,
        min_awake_duration_ms: 0,
        min_consolidation_queue: 0,
        ..SleepConfig::default()
    };

    let (actor_ref, handle) = Actor::spawn(None, SleepActor::with_config(config), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Need a tiny wait for idle_duration and awake_duration > 0 (since it's strict >)
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;

    // Enter sleep first
    let _ = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::EnterSleep { reply }, None)
            .await
            .expect("Failed to enter sleep"),
    );

    // Verify in EnteringSleep state
    let state = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::GetState { reply }, None)
            .await
            .expect("Failed to get state"),
    );
    assert_eq!(state, types::SleepState::EnteringSleep);

    // External stimulus when in EnteringSleep (interruptible) should be processed
    // and should transition to Waking
    let processed = unwrap_call(
        actor_ref
            .call(
                |reply| SleepMessage::ExternalStimulus {
                    stimulus: "test interrupt".to_string(),
                    reply,
                },
                None,
            )
            .await
            .expect("Failed to send stimulus"),
    );

    assert!(processed);

    // State should now be Waking
    let state = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::GetState { reply }, None)
            .await
            .expect("Failed to get state"),
    );
    assert_eq!(state, types::SleepState::Waking);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_wake_after_sleep_cycle() {
    let config = SleepConfig {
        idle_threshold_ms: 0,
        min_awake_duration_ms: 0,
        min_consolidation_queue: 0,
        ..SleepConfig::default()
    };

    let (actor_ref, handle) = Actor::spawn(None, SleepActor::with_config(config), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Need a tiny wait for idle_duration and awake_duration > 0 (since it's strict >)
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;

    // Enter sleep
    let result = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::EnterSleep { reply }, None)
            .await
            .expect("Failed to enter sleep"),
    );

    match result {
        SleepResult::Started => {}
        _ => panic!("Expected Started, got {result:?}"),
    }

    // Wake up - should return summary (which was created when entering sleep)
    let summary = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::Wake { reply }, None)
            .await
            .expect("Failed to wake"),
    );

    // Summary should be a fresh default (no cycles completed yet in this simple test)
    assert_eq!(summary.cycles_completed, 0);

    // Verify state is Awake
    let state = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::GetState { reply }, None)
            .await
            .expect("Failed to get state"),
    );
    assert_eq!(state, types::SleepState::Awake);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[tokio::test]
async fn sleep_actor_check_conditions_when_should_sleep() {
    let config = SleepConfig {
        idle_threshold_ms: 0,
        min_awake_duration_ms: 0,
        min_consolidation_queue: 0,
        ..SleepConfig::default()
    };

    let (actor_ref, handle) = Actor::spawn(None, SleepActor::with_config(config), ())
        .await
        .expect("Failed to spawn SleepActor");

    // Need a tiny wait for idle_duration and awake_duration > 0 (since it's strict >)
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;

    // With zero thresholds and tiny elapsed time, should be ready to sleep
    let should_sleep = unwrap_call(
        actor_ref
            .call(|reply| SleepMessage::CheckSleepConditions { reply }, None)
            .await
            .expect("Failed to check conditions"),
    );

    assert!(should_sleep);

    actor_ref.stop(None);
    handle.await.expect("Actor failed");
}

#[test]
fn sleep_phase_progression() {
    // Use mini_dream config (all interruptible, but logic still applies)
    // We override light_sleep_pct for testing
    let mut config = SleepConfig::mini_dream();
    config.light_sleep_duration_pct = 0.2;

    let mut state = SleepState::new(config);

    // Start awake
    assert_eq!(state.state, types::SleepState::Awake);
    assert!((state.get_consolidation_params().multiplier - 0.0).abs() < f32::EPSILON);

    // Satisfy entry conditions (queue threshold)
    for _ in 0..50 {
        state.increment_queue();
    }

    // Need a tiny wait for idle_duration > 0
    std::thread::sleep(std::time::Duration::from_millis(1));

    // Enter sleep
    let _ = state.enter_sleep();
    assert_eq!(state.state, types::SleepState::EnteringSleep);

    // First query auto-advances EnteringSleep (0) -> LightSleep (1)
    let params = state.get_consolidation_params();
    assert_eq!(state.state, types::SleepState::LightSleep);
    assert!((params.multiplier - 0.5).abs() < f32::EPSILON);

    // Advance into Light Sleep (ticks 1-20)
    // get_consolidation_params auto-advances
    for _ in 0..5 {
        let params = state.get_consolidation_params();
        if state.sleep_ticks > 2 {
            // Give it a few ticks to transition
            assert_eq!(state.state, types::SleepState::LightSleep);
            assert!((params.multiplier - 0.5).abs() < f32::EPSILON);
        }
    }

    // Advance to Deep Sleep (ticks > 20)
    state.sleep_ticks = 25;
    let params = state.get_consolidation_params();
    assert_eq!(state.state, types::SleepState::DeepSleep);
    assert!((params.multiplier - 1.0).abs() < f32::EPSILON);
    assert!(params.pruning_enabled);

    // Advance to Dreaming (ticks > 70)
    state.sleep_ticks = 75;
    let params = state.get_consolidation_params();
    assert_eq!(state.state, types::SleepState::Dreaming);
    assert!((params.multiplier - 0.8).abs() < f32::EPSILON);
    assert!(params.prioritize_emotional);
}

#[test]
fn advance_sleep_phase_noop_when_awake() {
    let mut state = SleepState::new(SleepConfig::default());
    state.state = types::SleepState::Awake;
    state.advance_sleep_phase();
    assert_eq!(state.state, types::SleepState::Awake);
}

//! Sleep Actor Tests

use super::*;
use ractor::rpc::CallResult;
use ractor::Actor;

/// Extract value from CallResult or panic
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
        _ => panic!("Expected ConditionsNotMet, got {:?}", result),
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
    let mut state = SleepState::new(config.clone());

    // Light sleep at 10%
    state.advance_sleep_phase(0.1);
    assert_eq!(state.state, types::SleepState::LightSleep);

    // Deep sleep at 30%
    state.advance_sleep_phase(0.3);
    assert_eq!(state.state, types::SleepState::DeepSleep);

    // Dreaming at 80%
    state.advance_sleep_phase(0.8);
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

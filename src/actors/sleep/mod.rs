//! Sleep Actor - Memory Consolidation (ADR-023)
//!
//! Implements TMI's sleep/dream consolidation mode.
//!
//! # TMI Concept
//!
//! Human memory consolidation occurs during sleep through:
//! - Sharp-wave ripples (SWRs): High-frequency replay of recent experiences
//! - NREM sleep: Stabilization and transfer to cortex
//! - REM sleep: Integration, abstraction, emotional processing
//! - Synaptic homeostasis: Pruning weak connections
//!
//! # Architecture
//!
//! The SleepActor coordinates:
//! 1. **Sleep Scheduler**: Entry/exit conditions
//! 2. **Replay Selector**: Priority-based memory selection
//! 3. **Consolidation**: Strengthening via Qdrant
//! 4. **Homeostasis**: Pruning weak associations
//!
//! # Usage
//!
//! ```no_run
//! use daneel::actors::sleep::{SleepActor, SleepMessage};
//! use ractor::Actor;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let (actor_ref, _) = Actor::spawn(None, SleepActor::default(), ()).await?;
//!
//! // Check if sleep should begin
//! let should_sleep = actor_ref.call(
//!     |reply| SleepMessage::CheckSleepConditions { reply },
//!     None
//! ).await?;
//! # Ok(())
//! # }
//! ```

pub mod types;

#[cfg(test)]
mod tests;

use ractor::{Actor, ActorProcessingErr, ActorRef};
use std::time::Instant;

pub use types::*;

/// Sleep Actor State
#[derive(Debug)]
pub struct SleepState {
    /// Current sleep state
    state: types::SleepState,

    /// Configuration
    config: SleepConfig,

    /// Last activity timestamp
    last_activity: Instant,

    /// When we started being awake
    awake_since: Instant,

    /// Accumulated sleep summary
    current_summary: Option<SleepSummary>,

    /// Number of memories pending consolidation (estimated)
    consolidation_queue_estimate: usize,
}

impl SleepState {
    /// Create new awake state
    fn new(config: SleepConfig) -> Self {
        let now = Instant::now();
        Self {
            state: types::SleepState::Awake,
            config,
            last_activity: now,
            awake_since: now,
            current_summary: None,
            consolidation_queue_estimate: 0,
        }
    }

    /// Check if sleep conditions are met
    #[allow(clippy::cast_possible_truncation)] // Clamped to u64::MAX, truncation impossible
    fn should_sleep(&self) -> bool {
        if self.state != types::SleepState::Awake {
            return false;
        }

        // Saturating to u64::MAX is fine - if we've been idle longer than ~584 million years, it's time to sleep
        let idle_duration = self.last_activity.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;
        let awake_duration = self.awake_since.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;

        let idle_trigger = idle_duration > self.config.idle_threshold_ms;
        let awake_trigger = awake_duration > self.config.min_awake_duration_ms;
        let queue_trigger = self.consolidation_queue_estimate >= self.config.min_consolidation_queue;

        // Enter sleep if idle AND (awake long enough OR queue is large)
        idle_trigger && (awake_trigger || queue_trigger)
    }

    /// Check if current state is interruptible
    fn is_interruptible(&self) -> bool {
        matches!(
            self.state,
            types::SleepState::Awake
                | types::SleepState::EnteringSleep
                | types::SleepState::LightSleep
                | types::SleepState::Waking
        )
    }

    /// Enter sleep mode
    fn enter_sleep(&mut self) -> SleepResult {
        if self.state != types::SleepState::Awake {
            return SleepResult::AlreadySleeping;
        }

        if !self.should_sleep() {
            return SleepResult::ConditionsNotMet {
                reason: "Sleep conditions not met".to_string(),
            };
        }

        self.state = types::SleepState::EnteringSleep;
        self.current_summary = Some(SleepSummary::default());
        SleepResult::Started
    }

    /// Wake up
    fn wake(&mut self) -> SleepSummary {
        let summary = self.current_summary.take().unwrap_or_default();

        self.state = types::SleepState::Awake;
        self.awake_since = Instant::now();
        self.last_activity = Instant::now();

        summary
    }

    /// Record activity (resets idle timer)
    fn record_activity(&mut self) {
        self.last_activity = Instant::now();
    }

    /// Increment consolidation queue estimate
    fn increment_queue(&mut self) {
        self.consolidation_queue_estimate += 1;
    }

    /// Clear consolidation queue after sleep
    fn clear_queue(&mut self) {
        self.consolidation_queue_estimate = 0;
    }

    /// Transition to next sleep phase
    ///
    /// Used during sleep cycle execution (will be called by SleepActor's sleep loop).
    #[allow(dead_code)]
    fn advance_sleep_phase(&mut self, cycle_elapsed_pct: f32) {
        self.state = match cycle_elapsed_pct {
            x if x < self.config.light_sleep_duration_pct => types::SleepState::LightSleep,
            x if x < 0.7 => types::SleepState::DeepSleep,
            _ => types::SleepState::Dreaming,
        };
    }

    /// Add cycle report to summary
    ///
    /// Used during sleep cycle completion (will be called by SleepActor's sleep loop).
    #[allow(dead_code)]
    fn add_cycle_report(&mut self, report: &SleepCycleReport) {
        if let Some(ref mut summary) = self.current_summary {
            summary.add_cycle(report);
        }
    }
}

/// The Sleep Actor
///
/// Coordinates memory consolidation during sleep mode.
#[derive(Debug, Default)]
pub struct SleepActor {
    config: SleepConfig,
}

impl SleepActor {
    /// Create with custom config
    #[must_use]
    pub fn with_config(config: SleepConfig) -> Self {
        Self { config }
    }
}

#[ractor::async_trait]
impl Actor for SleepActor {
    type Msg = SleepMessage;
    type State = SleepState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(SleepState::new(self.config.clone()))
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            SleepMessage::CheckSleepConditions { reply } => {
                let should_sleep = state.should_sleep();
                let _ = reply.send(should_sleep);
            }

            SleepMessage::EnterSleep { reply } => {
                let result = state.enter_sleep();
                let _ = reply.send(result);
            }

            SleepMessage::Wake { reply } => {
                let summary = state.wake();
                state.clear_queue();
                let _ = reply.send(summary);
            }

            SleepMessage::GetState { reply } => {
                let _ = reply.send(state.state);
            }

            SleepMessage::ExternalStimulus { stimulus: _, reply } => {
                if state.is_interruptible() {
                    state.record_activity();
                    if state.state != types::SleepState::Awake {
                        state.state = types::SleepState::Waking;
                    }
                    let _ = reply.send(true);
                } else {
                    // In protected sleep, ignore stimulus
                    let _ = reply.send(false);
                }
            }

            SleepMessage::RecordActivity => {
                state.record_activity();
                state.increment_queue();
            }

            SleepMessage::GetConfig { reply } => {
                let _ = reply.send(state.config.clone());
            }

            SleepMessage::UpdateConfig { config, reply } => {
                state.config = config;
                let _ = reply.send(());
            }
        }

        Ok(())
    }
}

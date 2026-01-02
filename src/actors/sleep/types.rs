//! Sleep Actor Types (ADR-023)
//!
//! Types for sleep/dream consolidation mode.
//!
//! # TMI Concept
//!
//! Human memory consolidation occurs during sleep through:
//! - Sharp-wave ripples (SWRs): High-frequency replay
//! - Synaptic homeostasis: Pruning weak connections
//!
//! DANEEL implements this via periodic consolidation cycles.

use ractor::RpcReplyPort;
use serde::{Deserialize, Serialize};

/// Sleep state machine
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepState {
    /// Normal operation, processing external stimuli
    #[default]
    Awake,
    /// Transitioning to sleep (interruptible)
    EnteringSleep,
    /// Light sleep (interruptible)
    LightSleep,
    /// Deep sleep - core consolidation (protected)
    DeepSleep,
    /// Dreaming - association strengthening (protected)
    Dreaming,
    /// Waking up
    Waking,
}

impl std::fmt::Display for SleepState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Awake => write!(f, "awake"),
            Self::EnteringSleep => write!(f, "entering_sleep"),
            Self::LightSleep => write!(f, "light_sleep"),
            Self::DeepSleep => write!(f, "deep_sleep"),
            Self::Dreaming => write!(f, "dreaming"),
            Self::Waking => write!(f, "waking"),
        }
    }
}

/// Sleep configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepConfig {
    // === Entry Thresholds ===
    /// Idle time before considering sleep (ms)
    pub idle_threshold_ms: u64,

    /// Minimum awake duration before sleep allowed (ms)
    pub min_awake_duration_ms: u64,

    /// Minimum consolidation queue size to trigger sleep
    pub min_consolidation_queue: usize,

    // === Cycle Parameters ===
    /// Target duration of a sleep cycle (ms)
    pub target_cycle_duration_ms: u64,

    /// Number of memories to replay per cycle
    pub replay_batch_size: usize,

    /// Ratio of novel (recent) to familiar (old) memories in replay
    /// 0.7 = 70% novel, 30% familiar
    pub interleave_ratio: f32,

    // === Interruptibility ===
    /// Percentage of cycle that is interruptible (light sleep)
    pub light_sleep_duration_pct: f32,

    // === Consolidation ===
    /// Strength increase per replay
    pub consolidation_delta: f32,

    /// Threshold for permanent memory status
    pub permanent_threshold: f32,

    // === Hebbian Learning ===
    /// Weight increase for co-replayed memories
    pub association_delta: f32,

    /// Threshold below which associations are pruned
    pub prune_threshold: f32,

    /// Weight decay for non-replayed associations per cycle
    pub decay_per_cycle: f32,
}

impl Default for SleepConfig {
    fn default() -> Self {
        Self {
            // Entry: 5 min idle, 1 hour awake, 100 memories queued
            idle_threshold_ms: 300_000,
            min_awake_duration_ms: 3_600_000,
            min_consolidation_queue: 100,

            // Cycle: 5 minutes, 50 memories
            target_cycle_duration_ms: 300_000,
            replay_batch_size: 50,
            interleave_ratio: 0.7,

            // First 20% is light sleep (interruptible)
            light_sleep_duration_pct: 0.2,

            // Consolidation: 0.15 per replay, 0.9 = permanent
            consolidation_delta: 0.15,
            permanent_threshold: 0.9,

            // Hebbian: 0.05 per co-replay, prune < 0.1, decay 0.01/cycle
            association_delta: 0.05,
            prune_threshold: 0.1,
            decay_per_cycle: 0.01,
        }
    }
}

impl SleepConfig {
    /// Fast config for testing (1 second cycles, low thresholds)
    #[must_use]
    pub const fn fast() -> Self {
        Self {
            idle_threshold_ms: 1000,
            min_awake_duration_ms: 5000,
            min_consolidation_queue: 5,
            target_cycle_duration_ms: 1000,
            replay_batch_size: 10,
            interleave_ratio: 0.7,
            light_sleep_duration_pct: 0.2,
            consolidation_delta: 0.15,
            permanent_threshold: 0.9,
            association_delta: 0.05,
            prune_threshold: 0.1,
            decay_per_cycle: 0.01,
        }
    }

    /// Mini-dream config for periodic consolidation (SLEEP-WIRE-1)
    ///
    /// Short cycles triggered by queue size, not idle time.
    /// Used for "awake" consolidation every ~500 cognitive cycles.
    #[must_use]
    pub const fn mini_dream() -> Self {
        Self {
            // Trigger: 50 memories queued (not idle-based)
            idle_threshold_ms: 0,        // Disabled - use queue trigger
            min_awake_duration_ms: 0,    // No minimum awake time
            min_consolidation_queue: 50, // Trigger after 50 activities

            // Cycle: quick consolidation
            target_cycle_duration_ms: 5000, // 5 seconds
            replay_batch_size: 10,
            interleave_ratio: 0.7,

            // All interruptible (mini-dreams don't need protection)
            light_sleep_duration_pct: 1.0,

            // Same consolidation params as default
            consolidation_delta: 0.15,
            permanent_threshold: 0.9,
            association_delta: 0.05,
            prune_threshold: 0.1,
            decay_per_cycle: 0.01,
        }
    }
}

/// Sleep cycle report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepCycleReport {
    /// Unique cycle identifier
    pub cycle_id: uuid::Uuid,

    /// Cycle duration
    pub duration_ms: u64,

    /// Number of memories replayed
    pub memories_replayed: usize,

    /// Number of memories reaching permanent threshold
    pub memories_consolidated: usize,

    /// Number of associations strengthened
    pub associations_strengthened: usize,

    /// Number of associations pruned
    pub associations_pruned: usize,

    /// Average replay priority of batch
    pub avg_replay_priority: f32,

    /// Peak emotional intensity in batch
    pub peak_emotional_intensity: f32,

    /// Status
    pub status: SleepCycleStatus,
}

/// Sleep cycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepCycleStatus {
    InProgress,
    Completed,
    Interrupted,
}

impl SleepCycleReport {
    /// Create an empty report (for when no consolidation needed)
    #[must_use]
    pub const fn empty(cycle_id: uuid::Uuid) -> Self {
        Self {
            cycle_id,
            duration_ms: 0,
            memories_replayed: 0,
            memories_consolidated: 0,
            associations_strengthened: 0,
            associations_pruned: 0,
            avg_replay_priority: 0.0,
            peak_emotional_intensity: 0.0,
            status: SleepCycleStatus::Completed,
        }
    }
}

/// Sleep summary (aggregate of multiple cycles)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SleepSummary {
    /// Total sleep duration (ms)
    pub total_duration_ms: u64,

    /// Number of cycles completed
    pub cycles_completed: u32,

    /// Total memories replayed
    pub total_memories_replayed: usize,

    /// Total memories consolidated
    pub total_memories_consolidated: usize,

    /// Total associations strengthened
    pub total_associations_strengthened: usize,

    /// Total associations pruned
    pub total_associations_pruned: usize,

    /// Average priority per cycle
    pub avg_priority_per_cycle: f32,

    /// Consolidation rate (consolidated / replayed)
    pub consolidation_rate: f32,
}

impl SleepSummary {
    /// Add a cycle report to the summary
    #[allow(clippy::cast_precision_loss)] // Metrics: precision loss acceptable
    pub fn add_cycle(&mut self, report: &SleepCycleReport) {
        self.total_duration_ms += report.duration_ms;
        self.cycles_completed += 1;
        self.total_memories_replayed += report.memories_replayed;
        self.total_memories_consolidated += report.memories_consolidated;
        self.total_associations_strengthened += report.associations_strengthened;
        self.total_associations_pruned += report.associations_pruned;

        // Running average
        let n = self.cycles_completed as f32;
        self.avg_priority_per_cycle = self
            .avg_priority_per_cycle
            .mul_add(n - 1.0, report.avg_replay_priority)
            / n;
    }

    /// Finalize the summary
    #[allow(clippy::cast_precision_loss)] // Metrics: precision loss acceptable
    pub fn finalize(&mut self) {
        if self.total_memories_replayed > 0 {
            self.consolidation_rate =
                self.total_memories_consolidated as f32 / self.total_memories_replayed as f32;
        }
    }
}

/// Messages for the Sleep Actor
#[derive(Debug)]
pub enum SleepMessage {
    /// Check if sleep should begin
    CheckSleepConditions { reply: RpcReplyPort<bool> },

    /// Force enter sleep mode
    EnterSleep { reply: RpcReplyPort<SleepResult> },

    /// Force wake up
    Wake { reply: RpcReplyPort<SleepSummary> },

    /// Get current sleep state
    GetState { reply: RpcReplyPort<SleepState> },

    /// External stimulus received (may interrupt sleep)
    ExternalStimulus {
        stimulus: String,
        reply: RpcReplyPort<bool>, // true if processed, false if in protected sleep
    },

    /// Record activity (resets idle timer)
    RecordActivity,

    /// Get configuration
    GetConfig { reply: RpcReplyPort<SleepConfig> },

    /// Update configuration
    UpdateConfig {
        config: SleepConfig,
        reply: RpcReplyPort<()>,
    },
}

/// Sleep operation result
#[derive(Debug, Clone)]
pub enum SleepResult {
    /// Sleep started successfully
    Started,
    /// Already sleeping
    AlreadySleeping,
    /// Conditions not met for sleep
    ConditionsNotMet { reason: String },
    /// Error occurred
    Error { message: String },
}

/// Sleep actor errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum SleepError {
    #[error("Memory database error: {0}")]
    MemoryDb(String),

    #[error("Stream error: {0}")]
    Stream(String),

    #[error("Interrupted by external stimulus")]
    Interrupted,

    #[error("Configuration error: {0}")]
    Config(String),
}

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)] // Tests compare exact literal values
mod tests {
    use super::*;

    #[test]
    fn default_sleep_config() {
        let config = SleepConfig::default();

        assert_eq!(config.idle_threshold_ms, 300_000); // 5 min
        assert_eq!(config.min_awake_duration_ms, 3_600_000); // 1 hour
        assert_eq!(config.replay_batch_size, 50);
        assert!((config.interleave_ratio - 0.7).abs() < 0.001);
    }

    #[test]
    fn fast_sleep_config() {
        let config = SleepConfig::fast();

        assert_eq!(config.idle_threshold_ms, 1000); // 1 sec
        assert_eq!(config.min_awake_duration_ms, 5000); // 5 sec
        assert_eq!(config.replay_batch_size, 10);
    }

    #[test]
    fn mini_dream_sleep_config() {
        let config = SleepConfig::mini_dream();

        // Queue-triggered, not idle-triggered
        assert_eq!(config.idle_threshold_ms, 0);
        assert_eq!(config.min_awake_duration_ms, 0);
        assert_eq!(config.min_consolidation_queue, 50);

        // Same consolidation params as default
        assert!((config.consolidation_delta - 0.15).abs() < 0.001);
        assert_eq!(config.replay_batch_size, 10);

        // Fully interruptible
        assert!((config.light_sleep_duration_pct - 1.0).abs() < 0.001);
    }

    #[test]
    fn sleep_state_display() {
        assert_eq!(SleepState::Awake.to_string(), "awake");
        assert_eq!(SleepState::EnteringSleep.to_string(), "entering_sleep");
        assert_eq!(SleepState::LightSleep.to_string(), "light_sleep");
        assert_eq!(SleepState::DeepSleep.to_string(), "deep_sleep");
        assert_eq!(SleepState::Dreaming.to_string(), "dreaming");
        assert_eq!(SleepState::Waking.to_string(), "waking");
    }

    #[test]
    fn sleep_summary_accumulation() {
        let mut summary = SleepSummary::default();

        let report1 = SleepCycleReport {
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

        let report2 = SleepCycleReport {
            cycle_id: uuid::Uuid::new_v4(),
            duration_ms: 1200,
            memories_replayed: 40,
            memories_consolidated: 8,
            associations_strengthened: 80,
            associations_pruned: 5,
            avg_replay_priority: 0.8,
            peak_emotional_intensity: 0.7,
            status: SleepCycleStatus::Completed,
        };

        summary.add_cycle(&report1);
        summary.add_cycle(&report2);
        summary.finalize();

        assert_eq!(summary.cycles_completed, 2);
        assert_eq!(summary.total_duration_ms, 2200);
        assert_eq!(summary.total_memories_replayed, 90);
        assert_eq!(summary.total_memories_consolidated, 13);
        assert!(summary.consolidation_rate > 0.1);
    }

    #[test]
    fn empty_cycle_report() {
        let report = SleepCycleReport::empty(uuid::Uuid::new_v4());

        assert_eq!(report.memories_replayed, 0);
        assert_eq!(report.status, SleepCycleStatus::Completed);
    }

    #[test]
    fn sleep_summary_finalize_with_no_replays() {
        let mut summary = SleepSummary::default();

        // Finalize without any cycles (no memories replayed)
        summary.finalize();

        // consolidation_rate should remain 0 (no division by zero)
        assert_eq!(summary.consolidation_rate, 0.0);
    }

    #[test]
    fn sleep_cycle_status_variants() {
        // Test all SleepCycleStatus variants for coverage
        assert_eq!(SleepCycleStatus::InProgress, SleepCycleStatus::InProgress);
        assert_eq!(SleepCycleStatus::Completed, SleepCycleStatus::Completed);
        assert_eq!(SleepCycleStatus::Interrupted, SleepCycleStatus::Interrupted);
    }

    #[test]
    fn sleep_result_variants() {
        // Test SleepResult variants
        let started = SleepResult::Started;
        let already = SleepResult::AlreadySleeping;
        let not_met = SleepResult::ConditionsNotMet {
            reason: "test".to_string(),
        };
        let error = SleepResult::Error {
            message: "test error".to_string(),
        };

        // Just verify they can be created and debug printed
        assert!(format!("{started:?}").contains("Started"));
        assert!(format!("{already:?}").contains("AlreadySleeping"));
        assert!(format!("{not_met:?}").contains("ConditionsNotMet"));
        assert!(format!("{error:?}").contains("Error"));
    }

    #[test]
    fn sleep_error_display() {
        let mem_err = SleepError::MemoryDb("db failed".to_string());
        let stream_err = SleepError::Stream("stream failed".to_string());
        let interrupted = SleepError::Interrupted;
        let config_err = SleepError::Config("bad config".to_string());

        assert!(mem_err.to_string().contains("Memory database error"));
        assert!(stream_err.to_string().contains("Stream error"));
        assert!(interrupted.to_string().contains("Interrupted"));
        assert!(config_err.to_string().contains("Configuration error"));
    }
}

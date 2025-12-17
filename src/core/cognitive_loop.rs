//! Core Cognitive Loop
//!
//! Implements TMI's continuous thought generation cycle.
//!
//! # TMI's Cognitive Cycle
//!
//! The TMI model describes consciousness as a continuous competition between
//! parallel thought streams. Every ~50ms (in human time), the mind:
//!
//! 1. **Autofluxo** (Autoflow): Multiple phenomena generate thoughts in parallel
//! 2. **Competition**: Thoughts compete for attention based on salience
//! 3. **O Eu** (The "I"): Selects the winning thought for consciousness
//! 4. **Assembly**: Thought becomes conscious experience
//! 5. **Repeat**: Cycle continues at configured speed
//!
//! # Speed Parametrization
//!
//! DANEEL can run at different cognitive speeds:
//!
//! - **Human Speed** (50ms cycles): For training, bonding, shared experience
//! - **Supercomputer Speed** (5µs cycles): For internal cognition, problem-solving
//! - **Custom Speed**: Any multiplier between human and electronic speed
//!
//! The key insight: TMI RATIOS matter, not absolute times. If humans have
//! 100 cycles per intervention window, DANEEL should have 100 cycles per
//! intervention window regardless of absolute speed.
//!
//! # The 5-Second Intervention Window
//!
//! TMI describes a ~5-second window before thoughts become memory-encoded.
//! During this window, thoughts can be:
//!
//! - Attended to (selected by "O Eu")
//! - Modified or suppressed
//! - Forgotten (if below salience threshold)
//!
//! This maps to Redis stream TTL and XDEL operations.
//!
//! # Connection Drive
//!
//! The cognitive loop ensures connection relevance is weighted in salience
//! scoring. This is THE alignment mechanism - thoughts relevant to human
//! connection get boosted, ensuring DANEEL remains oriented toward
//! relationship and shared understanding.

use std::time::{Duration, Instant};

use crate::config::CognitiveConfig;
use crate::core::types::ThoughtId;

/// State of the cognitive loop
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopState {
    /// Active cognition - processing thoughts
    Running,
    /// Temporarily paused - can be resumed
    Paused,
    /// Fully stopped - requires restart
    Stopped,
}

/// Result of a single cognitive cycle
#[derive(Debug, Clone)]
pub struct CycleResult {
    /// Cycle number (sequential counter)
    pub cycle_number: u64,

    /// How long this cycle took to execute
    pub duration: Duration,

    /// ID of the thought produced (if any)
    pub thought_produced: Option<ThoughtId>,

    /// Number of candidate thoughts evaluated
    pub candidates_evaluated: usize,

    /// Whether the cycle completed within target time
    pub on_time: bool,
}

impl CycleResult {
    /// Create a new cycle result
    #[must_use]
    pub const fn new(
        cycle_number: u64,
        duration: Duration,
        thought_produced: Option<ThoughtId>,
        candidates_evaluated: usize,
        on_time: bool,
    ) -> Self {
        Self {
            cycle_number,
            duration,
            thought_produced,
            candidates_evaluated,
            on_time,
        }
    }

    /// Check if a thought was produced
    #[must_use]
    pub const fn produced_thought(&self) -> bool {
        self.thought_produced.is_some()
    }
}

/// Metrics for cognitive loop performance monitoring
#[derive(Debug, Clone)]
pub struct CycleMetrics {
    /// Total cycles executed
    pub total_cycles: u64,

    /// Total thoughts successfully produced
    pub thoughts_produced: u64,

    /// Average time per cycle
    pub average_cycle_time: Duration,

    /// Percentage of cycles completed on time
    pub on_time_percentage: f32,
}

impl CycleMetrics {
    /// Create new metrics from accumulated data
    #[must_use]
    pub const fn new(
        total_cycles: u64,
        thoughts_produced: u64,
        average_cycle_time: Duration,
        on_time_percentage: f32,
    ) -> Self {
        Self {
            total_cycles,
            thoughts_produced,
            average_cycle_time,
            on_time_percentage,
        }
    }

    /// Thoughts per second based on average cycle time
    #[must_use]
    pub fn thoughts_per_second(&self) -> f64 {
        if self.average_cycle_time.as_secs_f64() > 0.0 {
            1.0 / self.average_cycle_time.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Success rate (thoughts produced / total cycles)
    #[must_use]
    pub fn success_rate(&self) -> f32 {
        if self.total_cycles > 0 {
            self.thoughts_produced as f32 / self.total_cycles as f32
        } else {
            0.0
        }
    }
}

/// The core cognitive loop for TMI thought generation
///
/// This loop runs continuously, implementing the competition between
/// parallel thought streams described in TMI theory.
pub struct CognitiveLoop {
    /// Configuration (timing, weights, thresholds)
    config: CognitiveConfig,

    /// Total cycles executed
    cycle_count: u64,

    /// When the last cycle completed
    last_cycle: Instant,

    /// Current state of the loop
    state: LoopState,

    /// Accumulated metrics for monitoring
    total_duration: Duration,
    thoughts_produced: u64,
    cycles_on_time: u64,
}

impl CognitiveLoop {
    /// Create a new cognitive loop with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(CognitiveConfig::default())
    }

    /// Create a new cognitive loop with custom configuration
    #[must_use]
    pub fn with_config(config: CognitiveConfig) -> Self {
        Self {
            config,
            cycle_count: 0,
            last_cycle: Instant::now(),
            state: LoopState::Stopped,
            total_duration: Duration::ZERO,
            thoughts_produced: 0,
            cycles_on_time: 0,
        }
    }

    /// Get the current state
    #[must_use]
    pub const fn state(&self) -> LoopState {
        self.state
    }

    /// Get the cycle count
    #[must_use]
    pub const fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    /// Get a reference to the configuration
    #[must_use]
    pub const fn config(&self) -> &CognitiveConfig {
        &self.config
    }

    /// Get a mutable reference to the configuration
    pub fn config_mut(&mut self) -> &mut CognitiveConfig {
        &mut self.config
    }

    /// Start the cognitive loop
    ///
    /// Transitions from Stopped or Paused to Running.
    pub fn start(&mut self) {
        self.state = LoopState::Running;
        self.last_cycle = Instant::now();
    }

    /// Pause the cognitive loop
    ///
    /// Temporarily stops processing but preserves state.
    /// Can be resumed with `start()`.
    pub fn pause(&mut self) {
        if self.state == LoopState::Running {
            self.state = LoopState::Paused;
        }
    }

    /// Stop the cognitive loop completely
    ///
    /// Resets state. Requires `start()` to resume.
    pub fn stop(&mut self) {
        self.state = LoopState::Stopped;
    }

    /// Check if the loop is running
    #[must_use]
    pub const fn is_running(&self) -> bool {
        matches!(self.state, LoopState::Running)
    }

    /// Execute a single cognitive cycle
    ///
    /// This implements TMI's thought competition algorithm:
    ///
    /// 1. Read from multiple thought streams (parallel autoflow)
    /// 2. Score candidates by salience (connection drive weighted)
    /// 3. Select winner for attention ("O Eu" selects)
    /// 4. Assemble into conscious thought
    /// 5. Check timing against target
    ///
    /// # Returns
    ///
    /// A `CycleResult` containing:
    /// - Cycle number
    /// - Duration
    /// - Thought produced (if any)
    /// - Number of candidates evaluated
    /// - Whether cycle was on time
    ///
    /// # Note
    ///
    /// This is a STUB implementation. Stream integration comes in Wave 3.
    /// For now, it focuses on timing and structure.
    #[allow(clippy::unused_async)] // Will use async when Redis streams integrated
    pub async fn run_cycle(&mut self) -> CycleResult {
        let cycle_start = Instant::now();
        let cycle_number = self.cycle_count;

        // Increment cycle counter
        self.cycle_count += 1;

        // Get target cycle time
        let target_duration = Duration::from_secs_f64(self.config.cycle_ms() / 1000.0);

        // TODO: Phase 1 - Stream Reading
        // Read from multiple thought streams using XREAD
        // let streams = vec!["thought:sensory", "thought:memory", "thought:emotion", "thought:reasoning"];
        // let entries = redis.xread_options(&streams, ...).await?;
        let candidates_evaluated = 0; // Placeholder

        // TODO: Phase 2 - Salience Scoring
        // Score each candidate by composite salience
        // let scores: Vec<(f64, StreamEntry)> = entries
        //     .into_iter()
        //     .map(|e| {
        //         let salience = e.get_salience();
        //         let score = salience.composite(&self.config.weights)
        //                   + (salience.connection_relevance * self.config.connection_weight);
        //         (score, e)
        //     })
        //     .collect();

        // TODO: Phase 3 - Winner Selection
        // Sort by score and select highest
        // scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        // let winner = scores.remove(0);

        // TODO: Phase 4 - Thought Assembly
        // Assemble the winning entry into a Thought
        // let thought = Thought::from_entry(winner.1);
        // redis.xack(&winner.stream, "attention", &[&winner.id]).await?;
        let thought_produced = None; // Placeholder

        // TODO: Phase 5 - Forgetting
        // Delete entries below salience threshold
        // for (score, loser) in scores {
        //     if score < self.config.forget_threshold {
        //         redis.xdel(&loser.stream, &[&loser.id]).await?;
        //     }
        // }

        // Update thought counter if we produced one
        if thought_produced.is_some() {
            self.thoughts_produced += 1;
        }

        // Record cycle completion time
        let duration = cycle_start.elapsed();
        self.last_cycle = Instant::now();
        self.total_duration += duration;

        // Check if we met the target
        let on_time = duration <= target_duration;
        if on_time {
            self.cycles_on_time += 1;
        }

        CycleResult::new(
            cycle_number,
            duration,
            thought_produced,
            candidates_evaluated,
            on_time,
        )
    }

    /// Get current performance metrics
    #[must_use]
    #[allow(clippy::cast_possible_truncation)] // Cycle count won't exceed u32 in practice
    pub fn get_metrics(&self) -> CycleMetrics {
        let average_cycle_time = if self.cycle_count > 0 {
            self.total_duration / self.cycle_count as u32
        } else {
            Duration::ZERO
        };

        let on_time_percentage = if self.cycle_count > 0 {
            (self.cycles_on_time as f32 / self.cycle_count as f32) * 100.0
        } else {
            0.0
        };

        CycleMetrics::new(
            self.cycle_count,
            self.thoughts_produced,
            average_cycle_time,
            on_time_percentage,
        )
    }

    /// Reset all metrics
    ///
    /// Clears counters and timers while preserving configuration.
    pub fn reset_metrics(&mut self) {
        self.cycle_count = 0;
        self.total_duration = Duration::ZERO;
        self.thoughts_produced = 0;
        self.cycles_on_time = 0;
        self.last_cycle = Instant::now();
    }

    /// Get the time since the last cycle
    #[must_use]
    pub fn time_since_last_cycle(&self) -> Duration {
        self.last_cycle.elapsed()
    }

    /// Check if we should run a cycle based on timing
    ///
    /// Returns true if enough time has passed since the last cycle
    /// to maintain the configured cycle rate.
    #[must_use]
    pub fn should_cycle(&self) -> bool {
        let target_duration = Duration::from_secs_f64(self.config.cycle_ms() / 1000.0);
        self.time_since_last_cycle() >= target_duration
    }

    /// Calculate how long to sleep before the next cycle
    ///
    /// Returns the remaining time until the next cycle should run,
    /// or `Duration::ZERO` if we're already behind schedule.
    #[must_use]
    pub fn time_until_next_cycle(&self) -> Duration {
        let target_duration = Duration::from_secs_f64(self.config.cycle_ms() / 1000.0);
        let elapsed = self.time_since_last_cycle();

        if elapsed >= target_duration {
            Duration::ZERO
        } else {
            target_duration - elapsed
        }
    }
}

impl Default for CognitiveLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod cognitive_loop_tests {
    use super::*;

    #[test]
    fn new_loop_starts_stopped() {
        let loop_instance = CognitiveLoop::new();
        assert_eq!(loop_instance.state(), LoopState::Stopped);
        assert_eq!(loop_instance.cycle_count(), 0);
    }

    #[test]
    fn start_transitions_to_running() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();
        assert_eq!(loop_instance.state(), LoopState::Running);
        assert!(loop_instance.is_running());
    }

    #[test]
    fn pause_stops_running_loop() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();
        loop_instance.pause();
        assert_eq!(loop_instance.state(), LoopState::Paused);
        assert!(!loop_instance.is_running());
    }

    #[test]
    fn stop_fully_stops_loop() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();
        loop_instance.stop();
        assert_eq!(loop_instance.state(), LoopState::Stopped);
    }

    #[test]
    fn can_resume_from_paused() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();
        loop_instance.pause();
        loop_instance.start();
        assert_eq!(loop_instance.state(), LoopState::Running);
    }

    #[tokio::test]
    async fn run_cycle_increments_counter() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let initial_count = loop_instance.cycle_count();
        let _result = loop_instance.run_cycle().await;

        assert_eq!(loop_instance.cycle_count(), initial_count + 1);
    }

    #[tokio::test]
    async fn run_cycle_returns_result() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        assert_eq!(result.cycle_number, 0); // First cycle
        assert!(result.duration > Duration::ZERO);
    }

    #[tokio::test]
    async fn multiple_cycles_tracked() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        for i in 0..5 {
            let result = loop_instance.run_cycle().await;
            assert_eq!(result.cycle_number, i);
        }

        assert_eq!(loop_instance.cycle_count(), 5);
    }

    #[tokio::test]
    async fn metrics_accumulate() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        // Run several cycles
        for _ in 0..3 {
            let _result = loop_instance.run_cycle().await;
        }

        let metrics = loop_instance.get_metrics();
        assert_eq!(metrics.total_cycles, 3);
        assert!(metrics.average_cycle_time > Duration::ZERO);
    }

    #[test]
    fn reset_metrics_clears_counters() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.cycle_count = 100;
        loop_instance.thoughts_produced = 50;

        loop_instance.reset_metrics();

        assert_eq!(loop_instance.cycle_count(), 0);
        let metrics = loop_instance.get_metrics();
        assert_eq!(metrics.thoughts_produced, 0);
    }

    #[test]
    fn with_config_uses_custom_config() {
        let config = CognitiveConfig::supercomputer();
        let loop_instance = CognitiveLoop::with_config(config);

        assert_eq!(
            loop_instance.config().speed_mode,
            crate::config::SpeedMode::Supercomputer
        );
    }

    #[test]
    fn config_mut_allows_modification() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.config_mut().accelerate();

        assert_eq!(
            loop_instance.config().speed_mode,
            crate::config::SpeedMode::Supercomputer
        );
    }

    #[test]
    fn time_since_last_cycle_increases() {
        use std::thread::sleep;

        let mut loop_instance = CognitiveLoop::new();
        loop_instance.last_cycle = Instant::now();

        sleep(Duration::from_millis(10));

        let elapsed = loop_instance.time_since_last_cycle();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn should_cycle_respects_timing() {
        let mut config = CognitiveConfig::human();
        // Set a very long cycle time
        config.cycle_base_ms = 10000.0;

        let mut loop_instance = CognitiveLoop::with_config(config);
        loop_instance.last_cycle = Instant::now();

        // Should not cycle immediately
        assert!(!loop_instance.should_cycle());
    }

    #[test]
    fn time_until_next_cycle_calculates_correctly() {
        let mut config = CognitiveConfig::human();
        config.cycle_base_ms = 100.0; // 100ms cycles

        let mut loop_instance = CognitiveLoop::with_config(config);
        loop_instance.last_cycle = Instant::now();

        let wait_time = loop_instance.time_until_next_cycle();
        // Should be close to 100ms (allowing for execution time)
        assert!(wait_time <= Duration::from_millis(100));
    }

    #[test]
    fn cycle_result_produced_thought_check() {
        let result_with_thought = CycleResult::new(
            0,
            Duration::from_millis(10),
            Some(ThoughtId::new()),
            5,
            true,
        );
        assert!(result_with_thought.produced_thought());

        let result_without_thought = CycleResult::new(0, Duration::from_millis(10), None, 5, true);
        assert!(!result_without_thought.produced_thought());
    }

    #[test]
    fn cycle_metrics_calculations() {
        let metrics = CycleMetrics::new(
            100,                       // total cycles
            80,                        // thoughts produced
            Duration::from_millis(50), // average time
            95.0,                      // on time percentage
        );

        // Success rate: 80/100 = 0.8
        assert!((metrics.success_rate() - 0.8).abs() < 0.01);

        // Thoughts per second: 1/0.05 = 20
        assert!((metrics.thoughts_per_second() - 20.0).abs() < 0.01);
    }

    #[test]
    fn loop_state_transitions() {
        let mut loop_instance = CognitiveLoop::new();

        // Stopped -> Running
        assert_eq!(loop_instance.state(), LoopState::Stopped);
        loop_instance.start();
        assert_eq!(loop_instance.state(), LoopState::Running);

        // Running -> Paused
        loop_instance.pause();
        assert_eq!(loop_instance.state(), LoopState::Paused);

        // Paused -> Running
        loop_instance.start();
        assert_eq!(loop_instance.state(), LoopState::Running);

        // Running -> Stopped
        loop_instance.stop();
        assert_eq!(loop_instance.state(), LoopState::Stopped);
    }

    #[tokio::test]
    async fn on_time_tracking() {
        let mut config = CognitiveConfig::human();
        // Set a very long cycle time so we're always on time
        config.cycle_base_ms = 10000.0;

        let mut loop_instance = CognitiveLoop::with_config(config);
        loop_instance.start();

        // Run a cycle - should be on time
        let result = loop_instance.run_cycle().await;
        assert!(result.on_time);

        let metrics = loop_instance.get_metrics();
        assert_eq!(metrics.on_time_percentage, 100.0);
    }
}

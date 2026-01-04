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

pub mod cycle;
mod execution;
pub mod types;

pub use cycle::*;
pub use types::*;

use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::actors::attention::{AttentionConfig, AttentionState};
use crate::actors::volition::{VolitionConfig, VolitionState};
use crate::config::CognitiveConfig;
#[cfg(test)]
use crate::core::types::Content;
#[cfg(test)]
use crate::core::types::SalienceScore;
use crate::drives::{CuriosityModule, FreeEnergyModule};
use crate::embeddings::SharedEmbeddingEngine;
use crate::graph::GraphClient;
use crate::memory_db::MemoryDb;
use crate::noise::StimulusInjector;
use crate::streams::client::StreamsClient;
use crate::streams::types::StreamError;

/// The core cognitive loop for TMI thought generation
///
/// This loop runs continuously, implementing the competition between
/// parallel thought streams described in TMI theory.
pub struct CognitiveLoop {
    /// Configuration (timing, weights, thresholds)
    pub(crate) config: CognitiveConfig,

    /// Redis Streams client for thought persistence (optional)
    pub(crate) streams: Option<StreamsClient>,

    /// Direct Redis client for injection stream operations (optional)
    pub(crate) redis_client: Option<redis::Client>,

    /// Total cycles executed
    pub(crate) cycle_count: u64,

    /// When the last cycle completed
    pub(crate) last_cycle: Instant,

    /// Current state of the loop
    pub(crate) state: LoopState,

    /// Accumulated metrics for monitoring
    pub(crate) total_duration: Duration,
    pub(crate) thoughts_produced: u64,
    pub(crate) cycles_on_time: u64,

    /// Accumulated stage durations for averaging
    pub(crate) total_stage_durations: StageDurations,

    /// Memory database for long-term storage (optional)
    pub(crate) memory_db: Option<Arc<MemoryDb>>,

    /// Consolidation threshold (salience above this gets stored)
    pub(crate) consolidation_threshold: f32,

    /// Attention state for competitive selection (O Eu)
    #[allow(dead_code)] // Will be used in Stage 3 (Attention) implementation
    pub(crate) attention_state: AttentionState,

    /// Volition state for free-won't veto decisions (Stage 4.5)
    pub(crate) volition_state: VolitionState,

    /// Stimulus injector for 1/f pink noise generation (ADR-043)
    /// Replaces white noise (`rand::rng`) with fractal noise for criticality
    pub(crate) stimulus_injector: StimulusInjector,

    /// Curiosity module for intrinsic motivation (DRIVE-1)
    pub(crate) curiosity_module: CuriosityModule,

    /// Free energy module for active inference (DRIVE-2)
    pub(crate) free_energy_module: FreeEnergyModule,

    /// Embedding engine for semantic vectors (Phase 2 Forward-Only)
    /// When present, new thoughts get real embeddings; historical stay at origin
    pub(crate) embedding_engine: Option<SharedEmbeddingEngine>,

    /// Graph client for association queries (VCONN-6 spreading activation)
    pub(crate) graph_client: Option<Arc<GraphClient>>,

    /// Test-only: Injected thought for testing veto path (ADR-049)
    #[cfg(test)]
    pub(crate) test_injected_thought: Option<(Content, SalienceScore)>,
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
            streams: None,
            redis_client: None,
            cycle_count: 0,
            last_cycle: Instant::now(),
            state: LoopState::Stopped,
            total_duration: Duration::ZERO,
            thoughts_produced: 0,
            cycles_on_time: 0,
            total_stage_durations: StageDurations::default(),
            memory_db: None,
            consolidation_threshold: 0.7, // Default threshold
            attention_state: AttentionState::with_config(AttentionConfig::default()),
            volition_state: VolitionState::with_config(VolitionConfig::default()),
            stimulus_injector: StimulusInjector::default(), // 1/f pink noise (ADR-043)
            curiosity_module: CuriosityModule::new(crate::drives::CuriosityConfig::default()),
            free_energy_module: FreeEnergyModule::new(crate::drives::FreeEnergyConfig::default()),
            embedding_engine: None,
            graph_client: None,
            #[cfg(test)]
            test_injected_thought: None,
        }
    }

    /// Initialize Law Crystals for Free Energy calculation (DRIVE-2)
    ///
    /// Embeds the Four Laws and sets them as preferred states in the EFE module.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn initialize_law_crystals(&mut self) {
        if let Some(ref shared_engine) = self.embedding_engine {
            use crate::core::laws::LAWS;
            use crate::core::types::Content;

            let mut crystals = Vec::new();
            {
                let mut engine = shared_engine.write().await;
                for law in LAWS {
                    let content = Content::raw(law.to_string());
                    if let Some(text) = content.to_embedding_text() {
                        if let Ok(vector) = engine.embed_thought(&text) {
                            crystals.push(vector);
                        }
                    }
                }
            }

            if !crystals.is_empty() {
                self.free_energy_module.set_law_crystals(crystals);
                tracing::info!("DRIVE-2: {} Law Crystals initialized", LAWS.len());
            }
        }
    }

    /// Set the embedding engine for semantic vectors
    ///
    /// When set, new thoughts will have real embeddings generated.
    /// Historical thoughts (pre-embedding era) remain at origin.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn set_embedding_engine(&mut self, engine: SharedEmbeddingEngine) {
        self.embedding_engine = Some(engine);
        tracing::info!("Embedding engine attached - forward-only embeddings enabled");
    }

    /// Set the memory database for long-term storage
    ///
    /// # Arguments
    ///
    /// * `memory_db` - `MemoryDb` client wrapped in Arc for sharing
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn set_memory_db(&mut self, memory_db: Arc<MemoryDb>) {
        self.memory_db = Some(memory_db);
    }

    /// Get a reference to the memory database (for querying counts)
    #[must_use]
    pub const fn memory_db(&self) -> Option<&Arc<MemoryDb>> {
        self.memory_db.as_ref()
    }

    /// Set the graph client for association queries (VCONN-6)
    ///
    /// When set, spreading activation can query neighbors in `RedisGraph`.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn set_graph_client(&mut self, graph_client: Arc<GraphClient>) {
        self.graph_client = Some(graph_client);
        tracing::info!("Graph client attached - spreading activation enabled");
    }

    /// Set the consolidation threshold
    ///
    /// Thoughts with composite salience above this threshold will be
    /// persisted to long-term memory.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Salience threshold (0.0 - 1.0)
    #[allow(clippy::missing_const_for_fn)] // clamp is not const
    pub fn set_consolidation_threshold(&mut self, threshold: f32) {
        self.consolidation_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Create a new cognitive loop connected to Redis Streams
    ///
    /// # Arguments
    ///
    /// * `redis_url` - Redis connection URL (e.g., "<redis://127.0.0.1:6379>")
    ///
    /// # Errors
    ///
    /// Returns `StreamError` if Redis connection fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn with_redis(redis_url: &str) -> Result<Self, StreamError> {
        Self::with_config_and_redis(CognitiveConfig::default(), redis_url).await
    }

    /// Create a cognitive loop with custom config and Redis connection
    ///
    /// # Arguments
    ///
    /// * `config` - Custom cognitive configuration
    /// * `redis_url` - Redis connection URL
    ///
    /// # Errors
    ///
    /// Returns `StreamError` if Redis connection fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn with_config_and_redis(
        config: CognitiveConfig,
        redis_url: &str,
    ) -> Result<Self, StreamError> {
        let streams = StreamsClient::connect(redis_url).await?;

        // Create a direct Redis client for injection stream operations
        let redis_client =
            redis::Client::open(redis_url).map_err(|e| StreamError::ConnectionFailed {
                reason: format!("Failed to create Redis client: {e}"),
            })?;

        Ok(Self {
            config,
            streams: Some(streams),
            redis_client: Some(redis_client),
            cycle_count: 0,
            last_cycle: Instant::now(),
            state: LoopState::Stopped,
            total_duration: Duration::ZERO,
            thoughts_produced: 0,
            cycles_on_time: 0,
            total_stage_durations: StageDurations::default(),
            memory_db: None,
            consolidation_threshold: 0.7,
            attention_state: AttentionState::with_config(AttentionConfig::default()),
            volition_state: VolitionState::with_config(VolitionConfig::default()),
            stimulus_injector: StimulusInjector::default(),
            curiosity_module: CuriosityModule::new(crate::drives::CuriosityConfig::default()),
            free_energy_module: FreeEnergyModule::new(crate::drives::FreeEnergyConfig::default()),
            embedding_engine: None,
            graph_client: None,
            #[cfg(test)]
            test_injected_thought: None,
        })
    }

    /// Check if the loop is connected to Redis
    #[must_use]
    pub const fn is_connected_to_redis(&self) -> bool {
        self.streams.is_some()
    }

    /// Get the current state of the loop
    #[must_use]
    pub const fn state(&self) -> LoopState {
        self.state
    }

    /// Get the total number of cycles executed
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
    pub const fn config_mut(&mut self) -> &mut CognitiveConfig {
        &mut self.config
    }

    /// Start the cognitive loop
    ///
    /// Transitions the loop to the Running state.
    pub const fn start(&mut self) {
        self.state = LoopState::Running;
    }

    /// Pause the cognitive loop
    ///
    /// Only pauses if the loop is currently running.
    pub fn pause(&mut self) {
        if self.state == LoopState::Running {
            self.state = LoopState::Paused;
        }
    }

    /// Stop the cognitive loop
    ///
    /// Fully stops the loop. Requires restart.
    pub const fn stop(&mut self) {
        self.state = LoopState::Stopped;
    }

    /// Check if the loop is currently running
    #[must_use]
    pub const fn is_running(&self) -> bool {
        matches!(self.state, LoopState::Running)
    }

    /// Get current performance metrics
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
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

        let average_stage_durations = self.total_stage_durations.div(self.cycle_count);

        CycleMetrics::new(
            self.cycle_count,
            self.thoughts_produced,
            average_cycle_time,
            on_time_percentage,
            average_stage_durations,
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
        self.total_stage_durations = StageDurations::default();
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
        target_duration.saturating_sub(elapsed)
    }

    /// Test-only: Inject a thought for the next cycle
    #[cfg(test)]
    pub fn inject_test_thought(&mut self, content: Content, salience: SalienceScore) {
        self.test_injected_thought = Some((content, salience));
    }
}

impl Default for CognitiveLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::significant_drop_tightening, clippy::float_cmp)]
mod tests {
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
        // Should be close to 100ms (minus tiny elapsed time)
        assert!(wait_time > Duration::from_millis(90));
        assert!(wait_time <= Duration::from_millis(100));
    }

    #[test]
    fn default_impl_creates_new_loop() {
        let loop_instance = CognitiveLoop::default();
        assert_eq!(loop_instance.state(), LoopState::Stopped);
        assert_eq!(loop_instance.cycle_count(), 0);
        assert!(!loop_instance.is_running());
    }

    #[test]
    fn set_consolidation_threshold_clamps_values() {
        let mut loop_instance = CognitiveLoop::new();

        // Test normal value
        loop_instance.set_consolidation_threshold(0.5);
        assert_eq!(loop_instance.consolidation_threshold, 0.5);

        // Test clamping above 1.0
        loop_instance.set_consolidation_threshold(1.5);
        assert_eq!(loop_instance.consolidation_threshold, 1.0);

        // Test clamping below 0.0
        loop_instance.set_consolidation_threshold(-0.5);
        assert_eq!(loop_instance.consolidation_threshold, 0.0);

        // Test boundary values
        loop_instance.set_consolidation_threshold(0.0);
        assert_eq!(loop_instance.consolidation_threshold, 0.0);

        loop_instance.set_consolidation_threshold(1.0);
        assert_eq!(loop_instance.consolidation_threshold, 1.0);
    }

    #[test]
    fn memory_db_returns_none_when_not_set() {
        let loop_instance = CognitiveLoop::new();
        assert!(loop_instance.memory_db().is_none());
    }

    #[test]
    fn pause_from_stopped_stays_stopped() {
        let mut loop_instance = CognitiveLoop::new();
        assert_eq!(loop_instance.state(), LoopState::Stopped);

        // Pause when stopped should not change state
        loop_instance.pause();
        assert_eq!(loop_instance.state(), LoopState::Stopped);
    }

    #[test]
    fn pause_from_paused_stays_paused() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();
        loop_instance.pause();
        assert_eq!(loop_instance.state(), LoopState::Paused);

        // Pause when already paused should not change state
        loop_instance.pause();
        assert_eq!(loop_instance.state(), LoopState::Paused);
    }

    #[test]
    fn time_until_next_cycle_returns_zero_when_behind_schedule() {
        let mut config = CognitiveConfig::human();
        config.cycle_base_ms = 1.0; // 1ms cycles

        let mut loop_instance = CognitiveLoop::with_config(config);
        // Set last_cycle to a time in the past
        loop_instance.last_cycle = Instant::now()
            .checked_sub(Duration::from_millis(100))
            .unwrap();

        // Should return zero since we're way behind schedule
        let wait_time = loop_instance.time_until_next_cycle();
        assert_eq!(wait_time, Duration::ZERO);
    }

    #[test]
    fn should_cycle_returns_true_when_time_elapsed() {
        let mut config = CognitiveConfig::human();
        config.cycle_base_ms = 1.0; // 1ms cycles

        let mut loop_instance = CognitiveLoop::with_config(config);
        // Set last_cycle to a time in the past
        loop_instance.last_cycle = Instant::now()
            .checked_sub(Duration::from_millis(100))
            .unwrap();

        // Should cycle since enough time has passed
        assert!(loop_instance.should_cycle());
    }

    #[test]
    fn get_metrics_with_zero_cycles() {
        let loop_instance = CognitiveLoop::new();
        let metrics = loop_instance.get_metrics();

        assert_eq!(metrics.total_cycles, 0);
        assert_eq!(metrics.thoughts_produced, 0);
        assert_eq!(metrics.average_cycle_time, Duration::ZERO);
        assert_eq!(metrics.on_time_percentage, 0.0);
    }

    #[test]
    fn config_accessor_returns_config() {
        let config = CognitiveConfig::supercomputer();
        let loop_instance = CognitiveLoop::with_config(config);

        assert_eq!(
            loop_instance.config().speed_mode,
            crate::config::SpeedMode::Supercomputer
        );
    }

    #[test]
    fn is_connected_to_redis_without_streams() {
        let loop_instance = CognitiveLoop::new();
        // Without Redis connection, should return false
        assert!(!loop_instance.is_connected_to_redis());
    }

    #[test]
    fn consolidation_threshold_edge_cases() {
        let mut loop_instance = CognitiveLoop::new();

        // Test exactly at boundaries
        loop_instance.set_consolidation_threshold(0.0);
        assert!((loop_instance.consolidation_threshold - 0.0).abs() < f32::EPSILON);

        loop_instance.set_consolidation_threshold(1.0);
        assert!((loop_instance.consolidation_threshold - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn not_connected_to_redis_by_default() {
        let loop_instance = CognitiveLoop::new();
        assert!(!loop_instance.is_connected_to_redis());
        assert!(loop_instance.streams.is_none());
        assert!(loop_instance.redis_client.is_none());
    }
}

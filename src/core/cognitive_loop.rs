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

use rand::Rng;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::actors::attention::{AttentionConfig, AttentionState};
use crate::config::CognitiveConfig;
use crate::core::types::{Content, SalienceScore, Thought, ThoughtId, WindowId};
use crate::memory_db::{ArchiveReason, Memory, MemoryDb, MemorySource, VECTOR_DIMENSION};
use crate::streams::client::StreamsClient;
use crate::streams::types::{StreamEntry, StreamError, StreamName};
use tracing::{debug, error, info, warn};

/// Current stage in the cognitive cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CognitiveStage {
    /// Gatilho da Memória - Memory trigger activation
    Trigger,
    /// Autofluxo - Parallel thought generation
    Autoflow,
    /// O Eu - Attention selection
    Attention,
    /// Construção do Pensamento - Thought assembly
    Assembly,
    /// Âncora da Memória - Memory encoding decision
    Anchor,
}

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

/// Time spent in each stage of the cognitive cycle
#[derive(Debug, Clone, Default)]
pub struct StageDurations {
    pub trigger: Duration,
    pub autoflow: Duration,
    pub attention: Duration,
    pub assembly: Duration,
    pub anchor: Duration,
}

impl StageDurations {
    /// Total time across all stages
    #[must_use]
    pub fn total(&self) -> Duration {
        self.trigger + self.autoflow + self.attention + self.assembly + self.anchor
    }

    /// Create a new StageDurations with all stages set to zero
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            trigger: Duration::ZERO,
            autoflow: Duration::ZERO,
            attention: Duration::ZERO,
            assembly: Duration::ZERO,
            anchor: Duration::ZERO,
        }
    }

    /// Add another StageDurations to this one (for accumulation)
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        Self {
            trigger: self.trigger + other.trigger,
            autoflow: self.autoflow + other.autoflow,
            attention: self.attention + other.attention,
            assembly: self.assembly + other.assembly,
            anchor: self.anchor + other.anchor,
        }
    }

    /// Divide all durations by a factor (for averaging)
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn div(&self, divisor: u64) -> Self {
        if divisor == 0 {
            return Self::zero();
        }
        let divisor_u32 = divisor as u32;
        Self {
            trigger: self.trigger / divisor_u32,
            autoflow: self.autoflow / divisor_u32,
            attention: self.attention / divisor_u32,
            assembly: self.assembly / divisor_u32,
            anchor: self.anchor / divisor_u32,
        }
    }
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

    /// Composite salience score of the winning thought (0.0-1.0)
    pub salience: f32,

    /// Number of candidate thoughts evaluated
    pub candidates_evaluated: usize,

    /// Whether the cycle completed within target time
    pub on_time: bool,

    /// Time spent in each stage (for debugging/monitoring)
    pub stage_durations: StageDurations,
}

impl CycleResult {
    /// Create a new cycle result
    #[must_use]
    pub const fn new(
        cycle_number: u64,
        duration: Duration,
        thought_produced: Option<ThoughtId>,
        salience: f32,
        candidates_evaluated: usize,
        on_time: bool,
        stage_durations: StageDurations,
    ) -> Self {
        Self {
            cycle_number,
            duration,
            thought_produced,
            salience,
            candidates_evaluated,
            on_time,
            stage_durations,
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

    /// Average time per stage
    pub average_stage_durations: StageDurations,
}

impl CycleMetrics {
    /// Create new metrics from accumulated data
    #[must_use]
    pub const fn new(
        total_cycles: u64,
        thoughts_produced: u64,
        average_cycle_time: Duration,
        on_time_percentage: f32,
        average_stage_durations: StageDurations,
    ) -> Self {
        Self {
            total_cycles,
            thoughts_produced,
            average_cycle_time,
            on_time_percentage,
            average_stage_durations,
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

    /// Redis Streams client for thought persistence (optional)
    streams: Option<StreamsClient>,

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

    /// Accumulated stage durations for averaging
    total_stage_durations: StageDurations,

    /// Memory database for long-term storage (optional)
    memory_db: Option<Arc<MemoryDb>>,

    /// Consolidation threshold (salience above this gets stored)
    consolidation_threshold: f32,

    /// Attention state for competitive selection (O Eu)
    #[allow(dead_code)] // Will be used in Stage 3 (Attention) implementation
    attention_state: AttentionState,
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
        }
    }

    /// Set the memory database for long-term storage
    ///
    /// # Arguments
    ///
    /// * `memory_db` - MemoryDb client wrapped in Arc for sharing
    pub fn set_memory_db(&mut self, memory_db: Arc<MemoryDb>) {
        self.memory_db = Some(memory_db);
    }

    /// Get a reference to the memory database (for querying counts)
    pub fn memory_db(&self) -> Option<&Arc<MemoryDb>> {
        self.memory_db.as_ref()
    }

    /// Set the consolidation threshold
    ///
    /// Thoughts with composite salience above this threshold will be
    /// persisted to long-term memory.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Salience threshold (0.0 - 1.0)
    pub fn set_consolidation_threshold(&mut self, threshold: f32) {
        self.consolidation_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Create a new cognitive loop connected to Redis Streams
    ///
    /// # Arguments
    ///
    /// * `redis_url` - Redis connection URL (e.g., "redis://127.0.0.1:6379")
    ///
    /// # Errors
    ///
    /// Returns `StreamError` if Redis connection fails.
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
    pub async fn with_config_and_redis(
        config: CognitiveConfig,
        redis_url: &str,
    ) -> Result<Self, StreamError> {
        let streams = StreamsClient::connect(redis_url).await?;
        info!("CognitiveLoop connected to Redis at {}", redis_url);
        Ok(Self {
            config,
            streams: Some(streams),
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
        })
    }

    /// Check if connected to Redis Streams
    #[must_use]
    pub fn is_connected_to_redis(&self) -> bool {
        self.streams
            .as_ref()
            .map_or(false, StreamsClient::is_connected)
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

    /// Generate a random thought for standalone operation
    ///
    /// Creates a thought with TMI-faithful salience distribution.
    /// Per ADR-032: >90% of cortical archives are neutral windows.
    ///
    /// Distribution:
    /// - 90%: Low-salience (neutral windows) - will be forgotten
    /// - 10%: High-salience (emotional/important) - may be kept/consolidated
    fn generate_random_thought(&self) -> (Content, SalienceScore) {
        let mut rng = rand::rng();

        // Generate random content - simple symbol for now
        let symbol_id = format!("thought_{}", self.cycle_count);
        let content = Content::symbol(
            symbol_id,
            vec![rng.random::<u8>(); 8], // Random 8-byte data
        );

        // TMI-faithful salience distribution (ADR-032)
        // Augusto Cury: >90% of cortical archives are neutral windows
        let (importance, novelty, relevance, connection_relevance) = if rng.random::<f32>() < 0.90 {
            // 90%: Neutral/low-salience thoughts (will be forgotten)
            (
                rng.random_range(0.0..0.35), // importance
                rng.random_range(0.0..0.30), // novelty
                rng.random_range(0.0..0.40), // relevance
                rng.random_range(0.1..0.40), // connection (min 0.1 per invariant)
            )
        } else {
            // 10%: High-salience thoughts (emotional/important)
            (
                rng.random_range(0.5..0.95), // importance
                rng.random_range(0.4..0.85), // novelty
                rng.random_range(0.5..0.95), // relevance
                rng.random_range(0.5..0.90), // connection
            )
        };

        let salience = SalienceScore::new(
            importance,
            novelty,
            relevance,
            rng.random_range(-0.5..0.5), // valence (unchanged)
            connection_relevance,
        );

        (content, salience)
    }

    /// Execute a single cognitive cycle
    ///
    /// This implements TMI's thought competition algorithm:
    ///
    /// 1. Trigger - Memory trigger activation (Gatilho da Memória)
    /// 2. Autoflow - Parallel thought generation (Autofluxo)
    /// 3. Attention - Select winning thought (O Eu)
    /// 4. Assembly - Assemble conscious thought (Construção do Pensamento)
    /// 5. Anchor - Memory encoding decision (Âncora da Memória)
    ///
    /// # Returns
    ///
    /// A `CycleResult` containing:
    /// - Cycle number
    /// - Duration
    /// - Thought produced (if any)
    /// - Number of candidates evaluated
    /// - Whether cycle was on time
    /// - Stage durations for each stage
    ///
    /// # Note
    ///
    /// This is a STUB implementation. Stream integration comes in Wave 3.
    /// For now, it focuses on timing and structure with stage delays.
    pub async fn run_cycle(&mut self) -> CycleResult {
        let cycle_start = Instant::now();
        let cycle_number = self.cycle_count;

        // Increment cycle counter
        self.cycle_count += 1;

        // Get target cycle time
        let target_duration = Duration::from_secs_f64(self.config.cycle_ms() / 1000.0);

        // Track stage durations
        let mut stage_durations = StageDurations::default();

        // Stage 1: Trigger (Gatilho da Memória)
        // Memory trigger activation - associative recall based on context
        let stage_start = Instant::now();

        // Query Qdrant for memory associations if connected
        if let Some(ref memory_db) = self.memory_db {
            // Generate query vector (zeros for now, will be replaced with actual context embedding)
            // TODO: Replace with context vector derived from recent thought/experience
            let query_vector = vec![0.0; VECTOR_DIMENSION];

            // Query for top 5 most relevant memories
            match memory_db.find_by_context(&query_vector, None, 5).await {
                Ok(memories) => {
                    if memories.is_empty() {
                        debug!("No memories retrieved from Qdrant (database may be empty)");
                    } else {
                        debug!(
                            count = memories.len(),
                            "Retrieved memories from Qdrant for associative priming"
                        );
                        // Log each retrieved memory for debugging
                        for (memory, score) in &memories {
                            debug!(
                                memory_id = %memory.id,
                                similarity = score,
                                content_preview = %memory.content.chars().take(50).collect::<String>(),
                                connection_relevance = memory.connection_relevance,
                                "Memory association triggered"
                            );
                        }
                    }
                }
                Err(e) => {
                    // Log error but don't crash - cognitive loop continues
                    warn!(
                        error = %e,
                        "Failed to query memory associations - continuing without memory trigger"
                    );
                }
            }
        } else {
            debug!("Memory database not connected - skipping memory trigger");
        }

        tokio::time::sleep(self.config.trigger_delay()).await;
        stage_durations.trigger = stage_start.elapsed();

        // Stage 2: Autoflow (Autofluxo)
        // Generate or read thoughts from streams
        let stage_start = Instant::now();
        let (content, salience) = self.generate_random_thought();

        // Assign a window ID to this candidate thought
        let window_id = WindowId::new();
        let candidates_evaluated = 1; // One generated thought for now
        tokio::time::sleep(self.config.autoflow_interval()).await;
        stage_durations.autoflow = stage_start.elapsed();

        // Stage 3: Attention (O Eu)
        // Competitive selection using AttentionActor logic
        let stage_start = Instant::now();

        // Update attention map with candidate salience
        // Calculate composite salience for competitive selection
        let composite_salience_candidate =
            salience.composite(&crate::core::types::SalienceWeights::default());
        self.attention_state.update_window_salience(
            window_id,
            composite_salience_candidate,
            salience.connection_relevance,
        );

        // Run attention cycle to select winner
        let attention_response = self.attention_state.cycle();

        // Extract the winner (for now, we only have one candidate, so it should win)
        let (winning_window, _winning_salience) = match attention_response {
            crate::actors::attention::AttentionResponse::CycleComplete {
                focused,
                salience: attention_salience,
            } => (focused, attention_salience),
            _ => {
                // Unexpected response type - fall back to our candidate
                (Some(window_id), composite_salience_candidate)
            }
        };

        debug!(
            cycle = cycle_number,
            candidate_count = candidates_evaluated,
            winner = ?winning_window,
            "Attention stage: competitive selection complete"
        );

        tokio::time::sleep(self.config.attention_delay()).await;
        stage_durations.attention = stage_start.elapsed();

        // Stage 4: Assembly (Construção do Pensamento)
        // Assemble the winning entry into a conscious thought
        let stage_start = Instant::now();
        let thought = Thought::new(content.clone(), salience).with_source("cognitive_loop");
        let thought_id = thought.id;

        // Use the composite salience calculated during attention stage
        let composite_salience = composite_salience_candidate;

        // Write to Redis if connected - track ID for potential forgetting
        let mut redis_entry: Option<(StreamName, String)> = None;
        if let Some(ref mut streams) = self.streams {
            let stream_name = StreamName::Custom("daneel:stream:awake".to_string());
            let entry = StreamEntry::new(
                String::new(), // ID will be auto-generated by Redis
                stream_name.clone(),
                content,
                salience,
            )
            .with_source("cognitive_loop");

            match streams.add_thought(&stream_name, &entry).await {
                Ok(redis_id) => {
                    debug!(
                        "Cycle {}: Wrote thought {} to Redis (ID: {})",
                        cycle_number, thought_id, redis_id
                    );
                    redis_entry = Some((stream_name, redis_id));
                }
                Err(e) => {
                    warn!(
                        "Cycle {}: Failed to write thought to Redis: {}",
                        cycle_number, e
                    );
                }
            }
        }

        let thought_produced = Some(thought_id);
        tokio::time::sleep(self.config.assembly_delay()).await;
        stage_durations.assembly = stage_start.elapsed();

        // Stage 5: Anchor (Âncora da Memória)
        // Decide whether to persist or forget the thought
        let stage_start = Instant::now();

        // Memory consolidation - Store high-salience thoughts to Qdrant
        self.consolidate_memory(&thought).await;

        // Forgetting - Archive to unconscious, then delete stream entries (ADR-033)
        // TMI: "Nada se apaga na memória" - nothing is erased, just made inaccessible
        if (composite_salience as f64) < self.config.forget_threshold {
            if let Some((ref stream_name, ref redis_id)) = redis_entry {
                // Archive to unconscious BEFORE deleting from Redis (ADR-033)
                if let Some(ref memory_db) = self.memory_db {
                    let content_str = serde_json::to_string(&thought.content)
                        .unwrap_or_else(|_| "serialization_error".to_string());
                    if let Err(e) = memory_db
                        .archive_to_unconscious(
                            &content_str,
                            composite_salience,
                            ArchiveReason::LowSalience,
                            Some(redis_id),
                        )
                        .await
                    {
                        warn!(
                            "Cycle {}: Failed to archive thought {} to unconscious: {}",
                            cycle_number, redis_id, e
                        );
                    } else {
                        debug!(
                            "Cycle {}: Archived thought {} to unconscious (salience {:.3})",
                            cycle_number, redis_id, composite_salience
                        );
                    }
                }

                // Now delete from Redis working memory
                if let Some(ref mut streams) = self.streams {
                    match streams.forget_thought(stream_name, redis_id).await {
                        Ok(()) => {
                            debug!(
                                "Cycle {}: Forgot thought {} from Redis (salience {:.3} < threshold {:.3})",
                                cycle_number,
                                redis_id,
                                composite_salience,
                                self.config.forget_threshold
                            );
                        }
                        Err(e) => {
                            warn!(
                                "Cycle {}: Failed to forget thought {}: {}",
                                cycle_number, redis_id, e
                            );
                        }
                    }
                }
            }
        }

        tokio::time::sleep(self.config.anchor_delay()).await;
        stage_durations.anchor = stage_start.elapsed();

        // Update thought counter if we produced one
        if thought_produced.is_some() {
            self.thoughts_produced += 1;
        }

        // Record cycle completion time
        let duration = cycle_start.elapsed();
        self.last_cycle = Instant::now();
        self.total_duration += duration;

        // Accumulate stage durations for averaging
        self.total_stage_durations = self.total_stage_durations.add(&stage_durations);

        // Check if we met the target
        let on_time = duration <= target_duration;
        if on_time {
            self.cycles_on_time += 1;
        }

        CycleResult::new(
            cycle_number,
            duration,
            thought_produced,
            composite_salience,
            candidates_evaluated,
            on_time,
            stage_durations,
        )
    }

    /// Consolidate a thought to long-term memory if it meets the threshold
    ///
    /// This is called during the Anchor stage. If the thought's salience
    /// is above the consolidation threshold, it's persisted to Qdrant.
    ///
    /// # Non-blocking
    ///
    /// This spawns an async task to avoid blocking the cognitive loop.
    /// Errors are logged but don't interrupt thought processing.
    #[allow(clippy::unused_async)] // Async for future compatibility, spawns async task internally
    async fn consolidate_memory(&self, thought: &Thought) {
        // Check if we have a memory database
        let Some(memory_db) = self.memory_db.as_ref() else {
            return;
        };

        // Calculate composite salience
        let salience = thought
            .salience
            .composite(&crate::core::types::SalienceWeights::default());

        // Only store if above threshold
        if salience < self.consolidation_threshold {
            debug!(
                thought_id = %thought.id,
                salience = salience,
                threshold = self.consolidation_threshold,
                "Thought below consolidation threshold - not storing"
            );
            return;
        }

        // Convert Thought to Memory
        let memory = self.thought_to_memory(thought, salience);
        let memory_id = memory.id;

        // Generate dummy vector (768-dim zeros for now)
        // TODO: Replace with actual embeddings from LLM when available
        let vector = vec![0.0; 768];

        // Clone the Arc for the spawned task
        let memory_db = Arc::clone(memory_db);

        // Spawn non-blocking storage task
        tokio::spawn(async move {
            match memory_db.store_memory(&memory, &vector).await {
                Ok(()) => {
                    debug!(
                        memory_id = %memory_id,
                        salience = salience,
                        "Memory consolidated to Qdrant"
                    );
                }
                Err(e) => {
                    error!(
                        memory_id = %memory_id,
                        error = %e,
                        "Failed to consolidate memory to Qdrant"
                    );
                }
            }
        });
    }

    /// Convert a Thought to a Memory record
    fn thought_to_memory(&self, thought: &Thought, _salience: f32) -> Memory {
        // Serialize thought content to string
        // For now, use debug representation since Content is pre-linguistic
        let content = format!("{:?}", thought.content);

        // Determine memory source based on thought source
        let source = if let Some(ref stream) = thought.source_stream {
            MemorySource::External {
                stimulus: stream.clone(),
            }
        } else {
            MemorySource::Reasoning {
                chain: vec![], // No chain for now
            }
        };

        // Create memory with emotional state from thought
        Memory::new(content, source)
            .with_emotion(thought.salience.valence, thought.salience.importance)
            .tag_for_consolidation()
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
            0.75, // salience
            5,
            true,
            StageDurations::default(),
        );
        assert!(result_with_thought.produced_thought());

        let result_without_thought = CycleResult::new(
            0,
            Duration::from_millis(10),
            None,
            0.0, // salience
            5,
            true,
            StageDurations::default(),
        );
        assert!(!result_without_thought.produced_thought());
    }

    #[test]
    fn cycle_metrics_calculations() {
        let metrics = CycleMetrics::new(
            100,                       // total cycles
            80,                        // thoughts produced
            Duration::from_millis(50), // average time
            95.0,                      // on time percentage
            StageDurations::default(), // average stage durations
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

    #[tokio::test]
    async fn stages_execute_in_order() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        // All stages should have non-zero durations
        assert!(result.stage_durations.trigger > Duration::ZERO);
        assert!(result.stage_durations.autoflow > Duration::ZERO);
        assert!(result.stage_durations.attention > Duration::ZERO);
        assert!(result.stage_durations.assembly > Duration::ZERO);
        assert!(result.stage_durations.anchor > Duration::ZERO);

        // Total stage time should approximately equal total cycle time
        let stage_total = result.stage_durations.total();
        let difference = result.duration.abs_diff(stage_total);

        // Allow some overhead for execution (should be small)
        assert!(
            difference < Duration::from_millis(5),
            "Stage total ({:?}) should approximately equal cycle duration ({:?})",
            stage_total,
            result.duration
        );
    }

    #[tokio::test]
    async fn cycle_time_equals_sum_of_stage_delays() {
        let config = CognitiveConfig::human();
        let mut loop_instance = CognitiveLoop::with_config(config);
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        // Calculate expected total from config delays
        let expected_total = loop_instance.config().trigger_delay()
            + loop_instance.config().autoflow_interval()
            + loop_instance.config().attention_delay()
            + loop_instance.config().assembly_delay()
            + loop_instance.config().anchor_delay();

        // Actual cycle time should be close to sum of delays
        // Allow 20ms tolerance for execution overhead and system load variance
        let difference = result.duration.abs_diff(expected_total);

        assert!(
            difference < Duration::from_millis(20),
            "Cycle duration ({:?}) should approximately equal sum of stage delays ({:?})",
            result.duration,
            expected_total
        );
    }

    #[tokio::test]
    async fn stage_durations_accumulate_in_metrics() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        // Run multiple cycles
        for _ in 0..3 {
            let _result = loop_instance.run_cycle().await;
        }

        let metrics = loop_instance.get_metrics();

        // Average stage durations should be non-zero
        assert!(metrics.average_stage_durations.trigger > Duration::ZERO);
        assert!(metrics.average_stage_durations.autoflow > Duration::ZERO);
        assert!(metrics.average_stage_durations.attention > Duration::ZERO);
        assert!(metrics.average_stage_durations.assembly > Duration::ZERO);
        assert!(metrics.average_stage_durations.anchor > Duration::ZERO);
    }

    #[tokio::test]
    async fn run_cycle_produces_thoughts() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        assert!(result.produced_thought());
        assert!(result.thought_produced.is_some());
        assert_eq!(result.candidates_evaluated, 1);
    }

    #[test]
    fn not_connected_to_redis_by_default() {
        let loop_instance = CognitiveLoop::new();
        assert!(!loop_instance.is_connected_to_redis());
    }

    #[test]
    fn stage_durations_helper_methods() {
        let durations = StageDurations {
            trigger: Duration::from_millis(1),
            autoflow: Duration::from_millis(2),
            attention: Duration::from_millis(3),
            assembly: Duration::from_millis(4),
            anchor: Duration::from_millis(5),
        };

        // Test total
        assert_eq!(durations.total(), Duration::from_millis(15));

        // Test zero
        let zero = StageDurations::zero();
        assert_eq!(zero.total(), Duration::ZERO);

        // Test add
        let doubled = durations.add(&durations);
        assert_eq!(doubled.trigger, Duration::from_millis(2));
        assert_eq!(doubled.total(), Duration::from_millis(30));

        // Test div
        let halved = doubled.div(2);
        assert_eq!(halved.trigger, Duration::from_millis(1));
        assert_eq!(halved.total(), Duration::from_millis(15));

        // Test div by zero
        let zero_div = durations.div(0);
        assert_eq!(zero_div.total(), Duration::ZERO);
    }
}

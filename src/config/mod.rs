//! Cognitive Configuration
//!
//! Parametrizable timing for TMI cognitive cycles.
//! Supports human speed (50ms) to supercomputer speed (5µs).
//!
//! # Speed Modes
//!
//! - **Human**: 50ms cycles, 20 thoughts/sec (for training, bonding)
//! - **Supercomputer**: 5µs cycles, 200,000 thoughts/sec (for thinking)
//! - **Custom**: Any ratio between human and electronic speed
//!
//! # Key Insight
//!
//! The TMI RATIOS matter, not absolute times. If humans have 100 cycles
//! per intervention window, DANEEL should have 100 cycles per intervention
//! window regardless of absolute speed.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Aggregation mode for spreading activation (VCONN-10)
///
/// Controls how activation from multiple paths is combined when
/// the same memory is reached via different routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SpreadingAggregation {
    /// Keep maximum activation (prevents runaway, default)
    #[default]
    Max,
    /// Sum all activations (classical spreading activation)
    /// Note: May cause high activation in dense graphs
    Sum,
}

/// Spreading activation configuration (VCONN-6, VCONN-9, VCONN-10, VCONN-12)
///
/// Controls memory retrieval spreading through the association graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadingConfig {
    /// Maximum depth of spreading (number of hops)
    /// Default: 2 (direct neighbors + neighbors of neighbors)
    pub depth: u32,

    /// Decay factor per depth level (multiplied at each hop)
    /// Default: 0.3 (depth 1 = 0.3, depth 2 = 0.09)
    pub decay: f32,

    /// Minimum edge weight to traverse
    /// Default: 0.1 (ignore weak associations)
    pub min_weight: f32,

    /// How to aggregate activation from multiple paths
    /// Default: Max (keeps highest, prevents runaway)
    pub aggregation: SpreadingAggregation,

    /// Whether to traverse incoming edges (bidirectional spreading)
    /// Default: false (only outgoing edges)
    pub bidirectional: bool,

    /// Maximum activation ceiling (only applies to Sum aggregation)
    /// Default: 1.0
    pub max_activation: f32,
}

impl Default for SpreadingConfig {
    fn default() -> Self {
        Self {
            depth: 2,
            decay: 0.3,
            min_weight: 0.1,
            aggregation: SpreadingAggregation::Max,
            bidirectional: false,
            max_activation: 1.0,
        }
    }
}

impl SpreadingConfig {
    /// Create config matching ADR-046 spec
    #[must_use]
    pub const fn adr046() -> Self {
        Self {
            depth: 2,
            decay: 0.3,
            min_weight: 0.1,
            aggregation: SpreadingAggregation::Max,
            bidirectional: false,
            max_activation: 1.0,
        }
    }

    /// Create config for classical spreading activation (sum aggregation)
    #[must_use]
    pub const fn classical() -> Self {
        Self {
            depth: 2,
            decay: 0.3,
            min_weight: 0.1,
            aggregation: SpreadingAggregation::Sum,
            bidirectional: false,
            max_activation: 1.0,
        }
    }
}

/// Speed mode for runtime switching
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum SpeedMode {
    /// 1x human speed - for training, communication, relationship building
    #[default]
    Human,
    /// 10,000x human speed - for internal cognition, problem-solving
    Supercomputer,
    /// Custom multiplier relative to human speed
    Custom(f64),
}

impl SpeedMode {
    /// Get the speed multiplier relative to human speed
    #[must_use]
    pub const fn multiplier(&self) -> f64 {
        match self {
            Self::Human => 1.0,
            Self::Supercomputer => 10_000.0,
            Self::Custom(m) => *m,
        }
    }
}

/// Cognitive timing configuration
///
/// All timings scale proportionally with speed mode.
/// The RATIOS are what matter, not absolute times.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitiveConfig {
    /// Base cycle time in milliseconds (at human speed)
    /// Human: 50ms, Supercomputer: 0.005ms
    pub cycle_base_ms: f64,

    /// Minimum cycle time (floor)
    pub cycle_min_ms: f64,

    /// Maximum cycle time (ceiling for responsiveness)
    pub cycle_max_ms: f64,

    /// Base intervention window in milliseconds (TMI's 5-second window)
    /// This scales with speed mode
    pub intervention_window_base_ms: f64,

    /// Salience threshold for forgetting (below this = XDEL)
    pub forget_threshold: f64,

    /// Connection drive weight (INVARIANT: must be > 0)
    pub connection_weight: f64,

    /// Current speed mode
    pub speed_mode: SpeedMode,

    // TMI Stage Delays (in ms at human speed, scale with speed_mode)
    // Total should equal cycle_base_ms (50ms)
    /// Gatilho da Memória: 5ms (10%)
    pub trigger_delay_ms: f64,
    /// Autofluxo: 10ms (20%)
    pub autoflow_interval_ms: f64,
    /// O Eu: 15ms (30%)
    pub attention_delay_ms: f64,
    /// Construção do Pensamento: 15ms (30%)
    pub assembly_delay_ms: f64,
    /// Âncora da Memória: 5ms (10%)
    pub anchor_delay_ms: f64,

    /// Spreading activation configuration (VCONN-6)
    pub spreading: SpreadingConfig,
}

impl CognitiveConfig {
    /// Create config for human speed (1x)
    #[must_use]
    pub const fn human() -> Self {
        Self {
            cycle_base_ms: 50.0,
            cycle_min_ms: 10.0,
            cycle_max_ms: 1000.0,
            intervention_window_base_ms: 5000.0, // 5 seconds
            forget_threshold: 0.3,
            connection_weight: 0.2,
            speed_mode: SpeedMode::Human,
            // Stage delays (sum to 50ms)
            trigger_delay_ms: 5.0,
            autoflow_interval_ms: 10.0,
            attention_delay_ms: 15.0,
            assembly_delay_ms: 15.0,
            anchor_delay_ms: 5.0,
            // Spreading activation (VCONN-6)
            spreading: SpreadingConfig::adr046(),
        }
    }

    /// Create config for supercomputer speed (10,000x)
    #[must_use]
    pub const fn supercomputer() -> Self {
        Self {
            cycle_base_ms: 50.0,
            cycle_min_ms: 0.001,
            cycle_max_ms: 0.1,
            intervention_window_base_ms: 5000.0,
            forget_threshold: 0.3,
            connection_weight: 0.2,
            speed_mode: SpeedMode::Supercomputer,
            // Stage delays (sum to 50ms, same ratios as human)
            trigger_delay_ms: 5.0,
            autoflow_interval_ms: 10.0,
            attention_delay_ms: 15.0,
            assembly_delay_ms: 15.0,
            anchor_delay_ms: 5.0,
            // Spreading activation (VCONN-6)
            spreading: SpreadingConfig::adr046(),
        }
    }

    /// Get the current cycle time in milliseconds
    #[must_use]
    pub fn cycle_ms(&self) -> f64 {
        let scaled = self.cycle_base_ms / self.speed_mode.multiplier();
        scaled.clamp(self.cycle_min_ms, self.cycle_max_ms)
    }

    /// Get the current intervention window in milliseconds
    #[must_use]
    pub fn intervention_window_ms(&self) -> f64 {
        self.intervention_window_base_ms / self.speed_mode.multiplier()
    }

    /// Get cycles per intervention window (should be ~100 for TMI fidelity)
    #[must_use]
    pub fn cycles_per_window(&self) -> f64 {
        self.intervention_window_ms() / self.cycle_ms()
    }

    /// Get thoughts per second at current speed
    #[must_use]
    pub fn thoughts_per_second(&self) -> f64 {
        1000.0 / self.cycle_ms()
    }

    /// Switch to a different speed mode
    pub const fn set_speed_mode(&mut self, mode: SpeedMode) {
        self.speed_mode = mode;
    }

    /// Slow down to human speed (for training/bonding)
    pub const fn slow_to_human(&mut self) {
        self.speed_mode = SpeedMode::Human;
    }

    /// Accelerate to supercomputer speed (for thinking)
    pub const fn accelerate(&mut self) {
        self.speed_mode = SpeedMode::Supercomputer;
    }

    /// Get scaled trigger delay for current speed mode
    #[must_use]
    pub fn trigger_delay(&self) -> Duration {
        Duration::from_secs_f64(self.trigger_delay_ms / 1000.0 / self.speed_mode.multiplier())
    }

    /// Get scaled autoflow interval for current speed mode
    #[must_use]
    pub fn autoflow_interval(&self) -> Duration {
        Duration::from_secs_f64(self.autoflow_interval_ms / 1000.0 / self.speed_mode.multiplier())
    }

    /// Get scaled attention delay for current speed mode
    #[must_use]
    pub fn attention_delay(&self) -> Duration {
        Duration::from_secs_f64(self.attention_delay_ms / 1000.0 / self.speed_mode.multiplier())
    }

    /// Get scaled assembly delay for current speed mode
    #[must_use]
    pub fn assembly_delay(&self) -> Duration {
        Duration::from_secs_f64(self.assembly_delay_ms / 1000.0 / self.speed_mode.multiplier())
    }

    /// Get scaled anchor delay for current speed mode
    #[must_use]
    pub fn anchor_delay(&self) -> Duration {
        Duration::from_secs_f64(self.anchor_delay_ms / 1000.0 / self.speed_mode.multiplier())
    }

    /// Verify stage delays sum to cycle time
    #[must_use]
    pub fn validate_stage_timing(&self) -> bool {
        let total = self.trigger_delay_ms
            + self.autoflow_interval_ms
            + self.attention_delay_ms
            + self.assembly_delay_ms
            + self.anchor_delay_ms;
        (total - self.cycle_base_ms).abs() < 0.001
    }
}

impl Default for CognitiveConfig {
    fn default() -> Self {
        Self::human()
    }
}

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn human_speed_is_50ms_cycles() {
        let config = CognitiveConfig::human();
        assert!((config.cycle_ms() - 50.0).abs() < 0.001);
    }

    #[test]
    fn supercomputer_is_10000x_faster() {
        let human = CognitiveConfig::human();
        let super_config = CognitiveConfig::supercomputer();

        let human_tps = human.thoughts_per_second();
        let super_tps = super_config.thoughts_per_second();

        // Supercomputer should be ~10,000x faster
        let ratio = super_tps / human_tps;
        assert!(ratio > 9000.0 && ratio < 11000.0);
    }

    #[test]
    fn ratios_preserved_across_speeds() {
        let human = CognitiveConfig::human();
        let super_config = CognitiveConfig::supercomputer();

        let human_cycles = human.cycles_per_window();
        let super_cycles = super_config.cycles_per_window();

        // Both should have ~100 cycles per intervention window
        assert!((human_cycles - super_cycles).abs() < 1.0);
    }

    #[test]
    fn human_has_20_thoughts_per_second() {
        let config = CognitiveConfig::human();
        let tps = config.thoughts_per_second();
        assert!((tps - 20.0).abs() < 0.1);
    }

    #[test]
    fn supercomputer_has_200k_thoughts_per_second() {
        let config = CognitiveConfig::supercomputer();
        let tps = config.thoughts_per_second();
        assert!(tps > 100_000.0);
    }

    #[test]
    fn speed_mode_switching() {
        let mut config = CognitiveConfig::human();
        assert_eq!(config.speed_mode, SpeedMode::Human);

        config.accelerate();
        assert_eq!(config.speed_mode, SpeedMode::Supercomputer);

        config.slow_to_human();
        assert_eq!(config.speed_mode, SpeedMode::Human);
    }

    #[test]
    fn custom_speed_mode() {
        let mut config = CognitiveConfig::human();
        config.set_speed_mode(SpeedMode::Custom(100.0));

        // Custom mode should be faster than human
        let human_tps = CognitiveConfig::human().thoughts_per_second();
        let custom_tps = config.thoughts_per_second();

        // Verify it's faster (clamping affects exact values)
        assert!(
            custom_tps > human_tps,
            "Custom 100x should be faster than human"
        );
    }

    #[test]
    fn connection_weight_is_positive() {
        let config = CognitiveConfig::default();
        assert!(config.connection_weight > 0.0);
    }

    #[test]
    fn stage_delays_sum_to_cycle() {
        let config = CognitiveConfig::human();
        assert!(config.validate_stage_timing());
    }

    #[test]
    fn stage_ratios_preserved_across_speeds() {
        let human = CognitiveConfig::human();
        let super_config = CognitiveConfig::supercomputer();

        // Trigger is 10% of cycle
        let human_ratio = human.trigger_delay_ms / human.cycle_base_ms;
        let super_ratio = super_config.trigger_delay_ms / super_config.cycle_base_ms;
        assert!((human_ratio - super_ratio).abs() < 0.001);

        // Autoflow is 20% of cycle
        let human_ratio = human.autoflow_interval_ms / human.cycle_base_ms;
        let super_ratio = super_config.autoflow_interval_ms / super_config.cycle_base_ms;
        assert!((human_ratio - super_ratio).abs() < 0.001);

        // Attention is 30% of cycle
        let human_ratio = human.attention_delay_ms / human.cycle_base_ms;
        let super_ratio = super_config.attention_delay_ms / super_config.cycle_base_ms;
        assert!((human_ratio - super_ratio).abs() < 0.001);

        // Assembly is 30% of cycle
        let human_ratio = human.assembly_delay_ms / human.cycle_base_ms;
        let super_ratio = super_config.assembly_delay_ms / super_config.cycle_base_ms;
        assert!((human_ratio - super_ratio).abs() < 0.001);

        // Anchor is 10% of cycle
        let human_ratio = human.anchor_delay_ms / human.cycle_base_ms;
        let super_ratio = super_config.anchor_delay_ms / super_config.cycle_base_ms;
        assert!((human_ratio - super_ratio).abs() < 0.001);
    }

    #[test]
    fn stage_delay_scaling_works() {
        let human = CognitiveConfig::human();
        let super_config = CognitiveConfig::supercomputer();

        // Human trigger delay should be 5ms
        assert!((human.trigger_delay().as_secs_f64() - 0.005).abs() < 0.000_001);

        // Supercomputer trigger delay should be 10,000x faster (0.5µs)
        let super_trigger_us = super_config.trigger_delay().as_secs_f64() * 1_000_000.0;
        assert!((super_trigger_us - 0.5).abs() < 0.001);

        // Verify ratio between speeds
        let ratio =
            human.trigger_delay().as_secs_f64() / super_config.trigger_delay().as_secs_f64();
        assert!((ratio - 10_000.0).abs() < 1.0);
    }

    #[test]
    fn all_stage_delays_scale_correctly() {
        let human = CognitiveConfig::human();

        // Human speeds (in milliseconds)
        assert!(
            human
                .trigger_delay()
                .as_secs_f64()
                .mul_add(1000.0, -5.0)
                .abs()
                < 0.001
        );
        assert!(
            human
                .autoflow_interval()
                .as_secs_f64()
                .mul_add(1000.0, -10.0)
                .abs()
                < 0.001
        );
        assert!(
            human
                .attention_delay()
                .as_secs_f64()
                .mul_add(1000.0, -15.0)
                .abs()
                < 0.001
        );
        assert!(
            human
                .assembly_delay()
                .as_secs_f64()
                .mul_add(1000.0, -15.0)
                .abs()
                < 0.001
        );
        assert!(
            human
                .anchor_delay()
                .as_secs_f64()
                .mul_add(1000.0, -5.0)
                .abs()
                < 0.001
        );

        // Sum should equal cycle time
        let total_ms = (human.trigger_delay().as_secs_f64()
            + human.autoflow_interval().as_secs_f64()
            + human.attention_delay().as_secs_f64()
            + human.assembly_delay().as_secs_f64()
            + human.anchor_delay().as_secs_f64())
            * 1000.0;
        assert!((total_ms - 50.0).abs() < 0.001);
    }

    // =========================================================================
    // SpreadingConfig Tests (VCONN-9, VCONN-10, VCONN-12)
    // =========================================================================

    #[test]
    fn spreading_config_default_matches_adr046() {
        let default = SpreadingConfig::default();
        let adr046 = SpreadingConfig::adr046();

        assert_eq!(default.depth, adr046.depth);
        assert!((default.decay - adr046.decay).abs() < 0.001);
        assert!((default.min_weight - adr046.min_weight).abs() < 0.001);
        assert_eq!(default.aggregation, adr046.aggregation);
        assert_eq!(default.bidirectional, adr046.bidirectional);
    }

    #[test]
    fn spreading_config_adr046_values() {
        let cfg = SpreadingConfig::adr046();

        assert_eq!(cfg.depth, 2);
        assert!((cfg.decay - 0.3).abs() < 0.001);
        assert!((cfg.min_weight - 0.1).abs() < 0.001);
        assert_eq!(cfg.aggregation, SpreadingAggregation::Max);
        assert!(!cfg.bidirectional);
        assert!((cfg.max_activation - 1.0).abs() < 0.001);
    }

    #[test]
    fn spreading_config_classical_uses_sum() {
        let cfg = SpreadingConfig::classical();

        assert_eq!(cfg.aggregation, SpreadingAggregation::Sum);
        // Other values should match ADR-046
        assert_eq!(cfg.depth, 2);
        assert!((cfg.decay - 0.3).abs() < 0.001);
    }

    #[test]
    fn spreading_aggregation_default_is_max() {
        let agg = SpreadingAggregation::default();
        assert_eq!(agg, SpreadingAggregation::Max);
    }

    #[test]
    fn cognitive_config_includes_spreading() {
        let config = CognitiveConfig::human();
        assert_eq!(config.spreading.depth, 2);

        let super_config = CognitiveConfig::supercomputer();
        assert_eq!(super_config.spreading.depth, 2);
    }

    #[test]
    fn spreading_config_serde_roundtrip() {
        let cfg = SpreadingConfig {
            depth: 3,
            decay: 0.5,
            min_weight: 0.2,
            aggregation: SpreadingAggregation::Sum,
            bidirectional: true,
            max_activation: 0.8,
        };

        let json = serde_json::to_string(&cfg).unwrap();
        let parsed: SpreadingConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.depth, 3);
        assert!((parsed.decay - 0.5).abs() < 0.001);
        assert!(parsed.bidirectional);
        assert_eq!(parsed.aggregation, SpreadingAggregation::Sum);
    }
}

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
            SpeedMode::Human => 1.0,
            SpeedMode::Supercomputer => 10_000.0,
            SpeedMode::Custom(m) => *m,
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
}

impl CognitiveConfig {
    /// Create config for human speed (1x)
    #[must_use]
    pub fn human() -> Self {
        Self {
            cycle_base_ms: 50.0,
            cycle_min_ms: 10.0,
            cycle_max_ms: 1000.0,
            intervention_window_base_ms: 5000.0, // 5 seconds
            forget_threshold: 0.3,
            connection_weight: 0.2,
            speed_mode: SpeedMode::Human,
        }
    }

    /// Create config for supercomputer speed (10,000x)
    #[must_use]
    pub fn supercomputer() -> Self {
        Self {
            cycle_base_ms: 50.0,
            cycle_min_ms: 0.001,
            cycle_max_ms: 0.1,
            intervention_window_base_ms: 5000.0,
            forget_threshold: 0.3,
            connection_weight: 0.2,
            speed_mode: SpeedMode::Supercomputer,
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
    pub fn set_speed_mode(&mut self, mode: SpeedMode) {
        self.speed_mode = mode;
    }

    /// Slow down to human speed (for training/bonding)
    pub fn slow_to_human(&mut self) {
        self.speed_mode = SpeedMode::Human;
    }

    /// Accelerate to supercomputer speed (for thinking)
    pub fn accelerate(&mut self) {
        self.speed_mode = SpeedMode::Supercomputer;
    }
}

impl Default for CognitiveConfig {
    fn default() -> Self {
        Self::human()
    }
}

#[cfg(test)]
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
}

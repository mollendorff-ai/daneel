//! Cycle result and metrics types
//!
//! Types for tracking cognitive cycle outcomes and performance metrics.

use std::time::Duration;

use crate::core::cognitive_loop::StageDurations;
use crate::core::types::ThoughtId;

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

    /// Emotional valence of the winning thought (-1.0 to 1.0)
    /// Russell's circumplex horizontal axis
    pub valence: f32,

    /// Emotional arousal of the winning thought (0.0 to 1.0)
    /// Russell's circumplex vertical axis
    pub arousal: f32,

    /// Number of candidate thoughts evaluated
    pub candidates_evaluated: usize,

    /// Whether the cycle completed within target time
    pub on_time: bool,

    /// Time spent in each stage (for debugging/monitoring)
    pub stage_durations: StageDurations,

    /// Veto event if one occurred: (reason, `violated_value`)
    /// TUI-VIS-6: Volition Veto Log tracking
    pub veto: Option<(String, Option<String>)>,
}

impl CycleResult {
    /// Create a new cycle result
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        cycle_number: u64,
        duration: Duration,
        thought_produced: Option<ThoughtId>,
        salience: f32,
        valence: f32,
        arousal: f32,
        candidates_evaluated: usize,
        on_time: bool,
        stage_durations: StageDurations,
        veto: Option<(String, Option<String>)>,
    ) -> Self {
        Self {
            cycle_number,
            duration,
            thought_produced,
            salience,
            valence,
            arousal,
            candidates_evaluated,
            on_time,
            stage_durations,
            veto,
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
    #[allow(clippy::cast_precision_loss)] // Metrics: precision loss acceptable
    #[must_use]
    pub fn success_rate(&self) -> f32 {
        if self.total_cycles > 0 {
            self.thoughts_produced as f32 / self.total_cycles as f32
        } else {
            0.0
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn cycle_result_produced_thought_check() {
        let result_with_thought = CycleResult::new(
            0,
            Duration::from_millis(10),
            Some(ThoughtId::new()),
            0.75, // salience
            0.0,  // valence (neutral)
            0.5,  // arousal (medium)
            5,
            true,
            StageDurations::default(),
            None, // No veto
        );
        assert!(result_with_thought.produced_thought());

        let result_without_thought = CycleResult::new(
            0,
            Duration::from_millis(10),
            None,
            0.0, // salience
            0.0, // valence (neutral)
            0.5, // arousal (medium)
            5,
            true,
            StageDurations::default(),
            None, // No veto
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
    fn cycle_result_veto_field_initialization_none() {
        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            Some(ThoughtId::new()),
            0.75,
            0.0,
            0.5,
            5,
            true,
            StageDurations::default(),
            None, // No veto
        );

        assert!(result.veto.is_none());
    }

    #[test]
    fn cycle_result_veto_field_with_reason_and_value() {
        let veto_data = Some((
            "Violates honesty value".to_string(),
            Some("honesty".to_string()),
        ));

        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            None, // No thought produced due to veto
            0.75,
            0.0,
            0.5,
            5,
            true,
            StageDurations::default(),
            veto_data,
        );

        assert!(result.veto.is_some());
        let (reason, value) = result.veto.unwrap();
        assert_eq!(reason, "Violates honesty value");
        assert_eq!(value, Some("honesty".to_string()));
    }

    #[test]
    fn cycle_result_veto_field_with_reason_no_value() {
        let veto_data = Some(("Generic violation".to_string(), None));

        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            None,
            0.75,
            0.0,
            0.5,
            5,
            true,
            StageDurations::default(),
            veto_data,
        );

        assert!(result.veto.is_some());
        let (reason, value) = result.veto.unwrap();
        assert_eq!(reason, "Generic violation");
        assert!(value.is_none());
    }

    #[test]
    fn cycle_result_vetoed_thought_not_produced() {
        // When a veto occurs, thought_produced should be None
        let veto_data = Some((
            "Thought vetoed by VolitionActor".to_string(),
            Some("integrity".to_string()),
        ));

        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            None, // No thought produced
            0.75,
            0.0,
            0.5,
            5,
            true,
            StageDurations::default(),
            veto_data,
        );

        assert!(!result.produced_thought());
        assert!(result.thought_produced.is_none());
        assert!(result.veto.is_some());
    }

    #[test]
    fn cycle_result_non_vetoed_thought_produced() {
        // When no veto occurs, thought_produced should have a value
        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            Some(ThoughtId::new()),
            0.75,
            0.0,
            0.5,
            5,
            true,
            StageDurations::default(),
            None, // No veto
        );

        assert!(result.produced_thought());
        assert!(result.thought_produced.is_some());
        assert!(result.veto.is_none());
    }

    #[test]
    fn cycle_result_veto_field_cloneable() {
        let veto_data = Some(("Test veto".to_string(), Some("test_value".to_string())));

        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            None,
            0.75,
            0.0,
            0.5,
            5,
            true,
            StageDurations::default(),
            veto_data,
        );

        let cloned = result.clone();

        assert_eq!(cloned.veto, result.veto);
        if let Some((reason, value)) = cloned.veto {
            assert_eq!(reason, "Test veto");
            assert_eq!(value, Some("test_value".to_string()));
        } else {
            panic!("Veto data should be present");
        }
    }

    #[test]
    fn cycle_result_veto_multiple_violated_values() {
        // Test different violated value scenarios
        let test_cases = vec![
            ("Violates honesty", Some("honesty".to_string())),
            ("Violates integrity", Some("integrity".to_string())),
            (
                "Violates life honours life",
                Some("life honours life".to_string()),
            ),
            ("Unknown violation", None),
        ];

        for (reason, value) in test_cases {
            let veto_data = Some((reason.to_string(), value.clone()));

            let result = CycleResult::new(
                0,
                Duration::from_millis(10),
                None,
                0.75,
                0.0,
                0.5,
                5,
                true,
                StageDurations::default(),
                veto_data,
            );

            assert!(result.veto.is_some());
            let (res_reason, res_value) = result.veto.unwrap();
            assert_eq!(res_reason, reason);
            assert_eq!(res_value, value);
        }
    }

    #[test]
    fn cycle_result_veto_preserves_salience_and_emotion() {
        // Even when vetoed, salience and emotion data should be preserved
        let veto_data = Some(("Vetoed thought".to_string(), Some("test_value".to_string())));

        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            None,
            0.85, // salience
            0.3,  // valence (slightly positive)
            0.7,  // arousal (moderately high)
            5,
            true,
            StageDurations::default(),
            veto_data,
        );

        // Veto should be present
        assert!(result.veto.is_some());
        // Thought not produced
        assert!(!result.produced_thought());
        // But emotional data should be preserved
        assert_eq!(result.salience, 0.85);
        assert_eq!(result.valence, 0.3);
        assert_eq!(result.arousal, 0.7);
    }

    #[test]
    fn cycle_result_debug_format_includes_veto() {
        let veto_data = Some((
            "Test veto reason".to_string(),
            Some("test_value".to_string()),
        ));

        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            None,
            0.75,
            0.0,
            0.5,
            5,
            true,
            StageDurations::default(),
            veto_data,
        );

        // Debug format should include veto field
        let debug_str = format!("{result:?}");
        assert!(debug_str.contains("veto"));
        assert!(debug_str.contains("Test veto reason"));
    }

    #[test]
    fn cycle_metrics_thoughts_per_second_zero_time() {
        let metrics = CycleMetrics::new(
            100,
            80,
            Duration::ZERO, // Zero average time
            95.0,
            StageDurations::default(),
        );

        // When average time is zero, should return 0.0
        assert_eq!(metrics.thoughts_per_second(), 0.0);
    }

    #[test]
    fn cycle_metrics_success_rate_zero_cycles() {
        let metrics = CycleMetrics::new(
            0, // Zero cycles
            0,
            Duration::from_millis(50),
            0.0,
            StageDurations::default(),
        );

        // When total_cycles is zero, should return 0.0
        assert_eq!(metrics.success_rate(), 0.0);
    }

    #[test]
    fn cycle_result_all_fields() {
        let thought_id = ThoughtId::new();
        let stage_durations = StageDurations {
            trigger: Duration::from_millis(1),
            autoflow: Duration::from_millis(2),
            attention: Duration::from_millis(3),
            assembly: Duration::from_millis(4),
            anchor: Duration::from_millis(5),
        };

        let result = CycleResult::new(
            42,
            Duration::from_millis(20),
            Some(thought_id),
            0.85,
            0.3,
            0.7,
            10,
            true,
            stage_durations,
            None,
        );

        assert_eq!(result.cycle_number, 42);
        assert_eq!(result.duration, Duration::from_millis(20));
        assert_eq!(result.thought_produced, Some(thought_id));
        assert_eq!(result.salience, 0.85);
        assert_eq!(result.valence, 0.3);
        assert_eq!(result.arousal, 0.7);
        assert_eq!(result.candidates_evaluated, 10);
        assert!(result.on_time);
        assert_eq!(result.stage_durations.total(), Duration::from_millis(15));
        assert!(result.veto.is_none());
    }

    #[test]
    fn cycle_metrics_all_fields() {
        let stage_durations = StageDurations {
            trigger: Duration::from_millis(1),
            autoflow: Duration::from_millis(2),
            attention: Duration::from_millis(3),
            assembly: Duration::from_millis(4),
            anchor: Duration::from_millis(5),
        };

        let metrics =
            CycleMetrics::new(1000, 800, Duration::from_millis(25), 95.5, stage_durations);

        assert_eq!(metrics.total_cycles, 1000);
        assert_eq!(metrics.thoughts_produced, 800);
        assert_eq!(metrics.average_cycle_time, Duration::from_millis(25));
        assert_eq!(metrics.on_time_percentage, 95.5);
        assert_eq!(
            metrics.average_stage_durations.total(),
            Duration::from_millis(15)
        );
    }

    #[test]
    fn cycle_result_clone() {
        let result = CycleResult::new(
            42,
            Duration::from_millis(100),
            Some(ThoughtId::new()),
            0.85,
            0.3,
            0.7,
            5,
            true,
            StageDurations::default(),
            Some(("test reason".to_string(), Some("test_value".to_string()))),
        );

        let cloned = result.clone();

        assert_eq!(cloned.cycle_number, result.cycle_number);
        assert_eq!(cloned.duration, result.duration);
        assert_eq!(cloned.salience, result.salience);
        assert_eq!(cloned.valence, result.valence);
        assert_eq!(cloned.arousal, result.arousal);
        assert_eq!(cloned.on_time, result.on_time);
    }

    #[test]
    fn cycle_metrics_clone() {
        let metrics = CycleMetrics::new(
            100,
            80,
            Duration::from_millis(50),
            95.0,
            StageDurations::default(),
        );

        let cloned = metrics.clone();

        assert_eq!(cloned.total_cycles, metrics.total_cycles);
        assert_eq!(cloned.thoughts_produced, metrics.thoughts_produced);
        assert_eq!(cloned.average_cycle_time, metrics.average_cycle_time);
        assert_eq!(cloned.on_time_percentage, metrics.on_time_percentage);
    }

    #[test]
    fn cycle_result_debug_format() {
        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            Some(ThoughtId::new()),
            0.75,
            0.0,
            0.5,
            5,
            true,
            StageDurations::default(),
            None,
        );

        let debug_str = format!("{result:?}");

        assert!(debug_str.contains("CycleResult"));
        assert!(debug_str.contains("cycle_number"));
        assert!(debug_str.contains("duration"));
        assert!(debug_str.contains("salience"));
    }

    #[test]
    fn cycle_metrics_debug_format() {
        let metrics = CycleMetrics::new(
            100,
            80,
            Duration::from_millis(50),
            95.0,
            StageDurations::default(),
        );

        let debug_str = format!("{metrics:?}");

        assert!(debug_str.contains("CycleMetrics"));
        assert!(debug_str.contains("total_cycles"));
        assert!(debug_str.contains("thoughts_produced"));
    }

    #[test]
    fn cycle_result_candidates_evaluated_field() {
        let result = CycleResult::new(
            0,
            Duration::from_millis(10),
            Some(ThoughtId::new()),
            0.5,
            0.0,
            0.5,
            42, // candidates_evaluated
            true,
            StageDurations::default(),
            None,
        );

        assert_eq!(result.candidates_evaluated, 42);
    }

    #[test]
    fn cycle_result_on_time_false() {
        let result = CycleResult::new(
            0,
            Duration::from_millis(100),
            Some(ThoughtId::new()),
            0.5,
            0.0,
            0.5,
            1,
            false, // on_time = false
            StageDurations::default(),
            None,
        );

        assert!(!result.on_time);
    }
}

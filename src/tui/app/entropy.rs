//! Entropy and fractality metrics for App
//!
//! Shannon entropy calculation and fractality analysis.

use super::{App, COGNITIVE_DIVERSITY_BINS, MAX_ENTROPY_HISTORY, MAX_FRACTALITY_HISTORY};

impl App {
    /// Calculate Shannon entropy of cognitive diversity
    #[allow(clippy::cast_precision_loss)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    #[must_use]
    pub fn calculate_entropy(&self) -> f32 {
        if self.thoughts.is_empty() {
            return 0.0;
        }

        let mut bins = [0u32; COGNITIVE_DIVERSITY_BINS];
        for thought in &self.thoughts {
            let emotional_intensity = thought.valence.abs() * thought.arousal;
            let tmi_composite = emotional_intensity
                .mul_add(0.4, thought.salience * 0.6)
                .clamp(0.0, 1.0);

            let bin_idx = match tmi_composite {
                v if v < 0.2 => 0,
                v if v < 0.4 => 1,
                v if v < 0.6 => 2,
                v if v < 0.8 => 3,
                _ => 4,
            };
            bins[bin_idx] += 1;
        }

        let total = self.thoughts.len() as f32;
        let mut entropy = 0.0f32;

        for &count in &bins {
            if count > 0 {
                let p = count as f32 / total;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    /// Update entropy tracking
    pub fn update_entropy(&mut self) {
        let entropy = self.calculate_entropy();
        self.current_entropy = entropy;

        if self.entropy_history.len() >= MAX_ENTROPY_HISTORY {
            self.entropy_history.pop_front();
        }
        self.entropy_history.push_back(entropy);
    }

    /// Get description of current entropy level
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn entropy_description(&self) -> &'static str {
        let max_entropy = (COGNITIVE_DIVERSITY_BINS as f32).log2();
        let normalized = self.current_entropy / max_entropy;

        if normalized > 0.7 {
            "EMERGENT"
        } else if normalized > 0.4 {
            "BALANCED"
        } else {
            "CLOCKWORK"
        }
    }

    /// Update fractality metrics
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn update_fractality(&mut self) {
        if self.inter_arrival_times.len() < 5 {
            return;
        }

        let times: Vec<f32> = self
            .inter_arrival_times
            .iter()
            .map(std::time::Duration::as_secs_f32)
            .collect();

        #[allow(clippy::cast_precision_loss)]
        let n = times.len() as f32;
        let mean = times.iter().sum::<f32>() / n;
        let variance = times.iter().map(|t| (t - mean).powi(2)).sum::<f32>() / n;
        let sigma = variance.sqrt();

        let max_gap = times.iter().copied().fold(0.0_f32, f32::max);
        let burst_ratio = if mean > 0.0 { max_gap / mean } else { 1.0 };

        self.fractality.inter_arrival_sigma = sigma;
        self.fractality.burst_ratio = burst_ratio;

        if self.fractality.boot_sigma == 0.0 && self.thought_count >= 50 {
            self.fractality.boot_sigma = sigma;
        }

        let cv = if mean > 0.0 { sigma / mean } else { 0.0 };
        let cv_component = (cv / 1.0).clamp(0.0, 1.0);
        let burst_component = ((burst_ratio - 1.0) / 4.0).clamp(0.0, 1.0);
        self.fractality.fractality_score =
            (cv_component * 0.6 + burst_component * 0.4).clamp(0.0, 1.0);

        if self.fractality.history.len() >= MAX_FRACTALITY_HISTORY {
            self.fractality.history.pop_front();
        }
        self.fractality
            .history
            .push_back(self.fractality.fractality_score);
    }

    /// Get description of current fractality level
    #[must_use]
    pub fn fractality_description(&self) -> &'static str {
        if self.fractality.fractality_score > 0.6 {
            "EMERGENT"
        } else if self.fractality.fractality_score > 0.3 {
            "BALANCED"
        } else {
            "CLOCKWORK"
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp, clippy::cast_precision_loss)]
mod tests {
    use super::*;
    use crate::tui::app::{FractalityMetrics, ThoughtStatus};
    use std::time::Duration;

    // =========================================================================
    // Entropy Calculation Tests
    // =========================================================================

    #[test]
    fn calculate_entropy_empty_thoughts() {
        let app = App::new();
        let entropy = app.calculate_entropy();
        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn calculate_entropy_uniform_distribution() {
        let mut app = App::new();
        // Add thoughts with uniform salience distribution
        for i in 0..100 {
            let salience = i as f32 / 100.0;
            app.add_thought(
                salience,
                0.0,
                0.5,
                format!("window_{}", i % 9),
                ThoughtStatus::Processing,
            );
        }

        let entropy = app.calculate_entropy();
        // Uniform distribution should have high entropy
        // With 5 bins, max is log2(5) ≈ 2.32, so >1.5 is good diversity
        assert!(entropy > 1.5);
    }

    #[test]
    fn calculate_entropy_single_value() {
        let mut app = App::new();
        // All thoughts with same salience = zero entropy
        for _ in 0..50 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }

        let entropy = app.calculate_entropy();
        // Single value should give zero entropy
        assert!(entropy < 0.01);
    }

    #[test]
    fn calculate_entropy_intense_bin_coverage() {
        let mut app = App::new();

        // Add thoughts that will fall into the INTENSE bin (tmi_composite >= 0.8)
        // TMI composite = emotional_intensity * 0.4 + salience * 0.6
        // For INTENSE: we need tmi_composite >= 0.8
        // With salience = 1.0, valence = 1.0, arousal = 1.0:
        // emotional_intensity = |1.0| * 1.0 = 1.0
        // tmi_composite = 1.0 * 0.4 + 1.0 * 0.6 = 1.0 >= 0.8 (INTENSE)
        for _ in 0..10 {
            app.add_thought(
                1.0, // High salience
                1.0, // High positive valence
                1.0, // High arousal
                "window_0".to_string(),
                ThoughtStatus::Salient,
            );
        }

        let entropy = app.calculate_entropy();
        // With all thoughts in one bin, entropy should be 0
        assert!(entropy < 0.01);
    }

    #[test]
    fn calculate_entropy_all_bins_covered() {
        let mut app = App::new();

        // Add thoughts that fall into each of the 5 bins
        // Bin 0 (MINIMAL): tmi_composite < 0.2
        app.add_thought(0.1, 0.0, 0.0, "w".to_string(), ThoughtStatus::Processing);

        // Bin 1 (LOW): 0.2 <= tmi_composite < 0.4
        app.add_thought(0.4, 0.0, 0.0, "w".to_string(), ThoughtStatus::Processing);

        // Bin 2 (MODERATE): 0.4 <= tmi_composite < 0.6
        app.add_thought(0.7, 0.2, 0.3, "w".to_string(), ThoughtStatus::Processing);

        // Bin 3 (HIGH): 0.6 <= tmi_composite < 0.8
        app.add_thought(0.9, 0.5, 0.5, "w".to_string(), ThoughtStatus::Processing);

        // Bin 4 (INTENSE): tmi_composite >= 0.8
        app.add_thought(1.0, 1.0, 1.0, "w".to_string(), ThoughtStatus::Processing);

        let entropy = app.calculate_entropy();
        // With uniform distribution across 5 bins, entropy should be near max (log2(5) ≈ 2.32)
        assert!(entropy > 2.0);
    }

    // =========================================================================
    // Update Entropy Tests
    // =========================================================================

    #[test]
    fn update_entropy_adds_to_history() {
        let mut app = App::new();

        // Add 3 thoughts (not enough to trigger automatic update at 5)
        for i in 0..3 {
            app.add_thought(
                i as f32 / 10.0,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }

        // Should not have auto-updated yet
        assert_eq!(app.entropy_history.len(), 0);

        // Manually update
        app.update_entropy();
        assert_eq!(app.entropy_history.len(), 1);

        app.update_entropy();
        assert_eq!(app.entropy_history.len(), 2);
    }

    #[test]
    fn update_entropy_respects_max_size() {
        let mut app = App::new();

        // Add thoughts
        for i in 0..10 {
            app.add_thought(
                i as f32 / 10.0,
                0.0,
                0.5,
                "window_0".to_string(),
                ThoughtStatus::Processing,
            );
        }

        // Update entropy more times than MAX_ENTROPY_HISTORY
        for _ in 0..60 {
            app.update_entropy();
        }

        assert!(app.entropy_history.len() <= MAX_ENTROPY_HISTORY);
    }

    // =========================================================================
    // Entropy Description Tests
    // =========================================================================

    #[test]
    fn entropy_description_high() {
        let mut app = App::new();
        // Simulate high entropy (>70% of max log2(5) ≈ 2.32)
        app.current_entropy = 1.8; // High value relative to max ~2.32
        let desc = app.entropy_description();
        assert_eq!(desc, "EMERGENT");
    }

    #[test]
    fn entropy_description_medium() {
        let mut app = App::new();
        app.current_entropy = 1.5; // Medium value
        let desc = app.entropy_description();
        assert_eq!(desc, "BALANCED");
    }

    #[test]
    fn entropy_description_low() {
        let mut app = App::new();
        app.current_entropy = 0.5; // Low value
        let desc = app.entropy_description();
        assert_eq!(desc, "CLOCKWORK");
    }

    // =========================================================================
    // Fractality Tests
    // =========================================================================

    #[test]
    fn update_fractality_early_return_insufficient_samples() {
        let mut app = App::new();

        // Add fewer than 5 thoughts (so inter_arrival_times < 5)
        for i in 0..4 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                format!("window_{i}"),
                ThoughtStatus::Processing,
            );
        }

        // Manually call update_fractality
        app.update_fractality();

        // Fractality metrics should still be at defaults (not updated)
        assert_eq!(app.fractality.inter_arrival_sigma, 0.0);
        assert_eq!(app.fractality.fractality_score, 0.0);
    }

    #[test]
    fn update_fractality_with_sufficient_samples() {
        let mut app = App::new();

        // Add more than 5 thoughts with small delays
        for i in 0..10 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                format!("window_{}", i % 9),
                ThoughtStatus::Processing,
            );
            // Small delay to create inter-arrival times
            std::thread::sleep(Duration::from_millis(1));
        }

        // Manually call update_fractality
        app.update_fractality();

        // Metrics should be updated
        assert!(app.fractality.inter_arrival_sigma >= 0.0);
        assert!(app.fractality.burst_ratio > 0.0);
    }

    #[test]
    fn update_fractality_boot_sigma_recorded_after_50_thoughts() {
        let mut app = App::new();

        // Add 50+ thoughts to trigger boot_sigma recording
        for i in 0..55 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                format!("window_{}", i % 9),
                ThoughtStatus::Processing,
            );
        }

        // Boot sigma should be recorded
        assert!(app.fractality.boot_sigma > 0.0 || app.fractality.inter_arrival_sigma == 0.0);
    }

    #[test]
    fn update_fractality_history_respects_max_size() {
        let mut app = App::new();

        // Add enough thoughts
        for i in 0..20 {
            app.add_thought(
                0.5,
                0.0,
                0.5,
                format!("window_{}", i % 9),
                ThoughtStatus::Processing,
            );
        }

        // Manually call update_fractality many times
        for _ in 0..60 {
            app.update_fractality();
        }

        // History should be capped at MAX_FRACTALITY_HISTORY (50)
        assert!(app.fractality.history.len() <= MAX_FRACTALITY_HISTORY);
    }

    #[test]
    fn update_fractality_handles_zero_mean() {
        let mut app = App::new();

        // Set up inter_arrival_times with zeros (edge case)
        for _ in 0..10 {
            app.inter_arrival_times.push_back(Duration::ZERO);
        }

        // Should not panic
        app.update_fractality();

        // Should handle division by zero gracefully
        assert!(app.fractality.burst_ratio >= 0.0);
    }

    // =========================================================================
    // Fractality Description Tests
    // =========================================================================

    #[test]
    fn fractality_description_emergent() {
        let mut app = App::new();
        app.fractality.fractality_score = 0.7; // > 0.6

        assert_eq!(app.fractality_description(), "EMERGENT");
    }

    #[test]
    fn fractality_description_balanced() {
        let mut app = App::new();
        app.fractality.fractality_score = 0.5; // > 0.3 but <= 0.6

        assert_eq!(app.fractality_description(), "BALANCED");
    }

    #[test]
    fn fractality_description_clockwork() {
        let mut app = App::new();
        app.fractality.fractality_score = 0.2; // <= 0.3

        assert_eq!(app.fractality_description(), "CLOCKWORK");
    }

    #[test]
    fn fractality_description_boundary_0_6() {
        let mut app = App::new();
        app.fractality.fractality_score = 0.6; // Exactly 0.6 should be BALANCED

        assert_eq!(app.fractality_description(), "BALANCED");
    }

    #[test]
    fn fractality_description_boundary_0_3() {
        let mut app = App::new();
        app.fractality.fractality_score = 0.3; // Exactly 0.3 should be CLOCKWORK

        assert_eq!(app.fractality_description(), "CLOCKWORK");
    }

    // =========================================================================
    // FractalityMetrics Tests
    // =========================================================================

    #[test]
    fn fractality_metrics_default() {
        let metrics = FractalityMetrics::default();
        assert_eq!(metrics.inter_arrival_sigma, 0.0);
        assert_eq!(metrics.boot_sigma, 0.0);
        assert_eq!(metrics.burst_ratio, 0.0);
        assert_eq!(metrics.run_entropy, 0.0);
        assert_eq!(metrics.fractality_score, 0.0);
        assert!(metrics.history.is_empty());
    }

    #[test]
    fn fractality_metrics_is_cloneable() {
        let mut metrics = FractalityMetrics {
            fractality_score: 0.5,
            ..Default::default()
        };
        metrics.history.push_back(0.3);

        let cloned = metrics.clone();
        assert_eq!(cloned.fractality_score, 0.5);
        assert_eq!(cloned.history.len(), 1);
    }
}

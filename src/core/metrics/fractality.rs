//! Fractality calculation (ADR-054: Metrics Source of Truth)
//!
//! Fractality measures temporal patterns in cognitive activity.
//! Score ranges from 0 (clockwork/regular) to 1 (fractal/bursty).

use super::thresholds::CognitiveState;
use std::time::Duration;

/// CV (coefficient of variation) divisor for normalization
/// CV of 2.0 = full score (bursty systems often have CV > 1)
const CV_DIVISOR: f32 = 2.0;

/// Burst ratio divisor for normalization
/// Burst ratio of 15 = full score (reasonable for bursty thinking)
const BURST_DIVISOR: f32 = 14.0;

/// Weight for CV component in final score
const CV_WEIGHT: f32 = 0.6;

/// Weight for burst component in final score
const BURST_WEIGHT: f32 = 0.4;

/// Result of fractality calculation
#[derive(Debug, Clone)]
pub struct FractalityResult {
    /// Final fractality score (0.0-1.0)
    pub score: f32,
    /// Standard deviation of inter-arrival times (seconds)
    pub sigma: f32,
    /// Burst ratio (max gap / mean gap)
    pub burst_ratio: f32,
    /// Coefficient of variation (sigma / mean)
    pub cv: f32,
    /// Cognitive state based on score
    pub state: CognitiveState,
}

impl Default for FractalityResult {
    fn default() -> Self {
        Self {
            score: 0.0,
            sigma: 0.0,
            burst_ratio: 1.0,
            cv: 0.0,
            state: CognitiveState::Clockwork,
        }
    }
}

/// Calculate fractality from inter-arrival times (as Durations)
///
/// Fractality is computed from:
/// - CV (coefficient of variation): sigma / mean
/// - Burst ratio: max gap / mean gap
///
/// Score = 0.6 * `cv_component` + 0.4 * `burst_component`
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calculate_fractality(inter_arrival_times: &[Duration]) -> FractalityResult {
    if inter_arrival_times.len() < 2 {
        return FractalityResult::default();
    }

    let times: Vec<f32> = inter_arrival_times
        .iter()
        .map(Duration::as_secs_f32)
        .collect();

    calculate_fractality_from_seconds(&times)
}

/// Calculate fractality from inter-arrival times in seconds
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calculate_fractality_from_seconds(times: &[f32]) -> FractalityResult {
    if times.len() < 2 {
        return FractalityResult::default();
    }

    let n = times.len() as f32;
    let mean = times.iter().sum::<f32>() / n;
    let variance = times.iter().map(|t| (t - mean).powi(2)).sum::<f32>() / n;
    let sigma = variance.sqrt();

    // Coefficient of variation
    let cv = if mean > 0.0 { sigma / mean } else { 0.0 };

    // Burst ratio (max / mean)
    let max_gap = times.iter().copied().fold(0.0f32, f32::max);
    let burst_ratio = if mean > 0.0 { max_gap / mean } else { 1.0 };

    // Calculate score components (ADR-054 standardized values)
    let cv_component = (cv / CV_DIVISOR).clamp(0.0, 1.0);
    let burst_component = ((burst_ratio - 1.0) / BURST_DIVISOR).clamp(0.0, 1.0);
    let score = (cv_component * CV_WEIGHT + burst_component * BURST_WEIGHT).clamp(0.0, 1.0);

    let state = CognitiveState::from_score(score);

    FractalityResult {
        score,
        sigma,
        burst_ratio,
        cv,
        state,
    }
}

/// Calculate fractality from millisecond timestamps (Redis stream format)
///
/// Timestamps should be in ascending order. If in descending order (as from XREVRANGE),
/// they will be reversed internally.
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calculate_fractality_from_timestamps(timestamps: &[u64]) -> FractalityResult {
    if timestamps.len() < 2 {
        return FractalityResult::default();
    }

    // Ensure ascending order
    let mut sorted = timestamps.to_vec();
    if sorted.first() > sorted.last() {
        sorted.reverse();
    }

    // Calculate inter-arrival times in seconds
    let mut times: Vec<f32> = Vec::with_capacity(sorted.len() - 1);
    for i in 1..sorted.len() {
        let delta_ms = sorted[i].saturating_sub(sorted[i - 1]);
        times.push(delta_ms as f32 / 1000.0); // Convert ms to seconds
    }

    calculate_fractality_from_seconds(&times)
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fractality_empty() {
        let result = calculate_fractality(&[]);
        assert_eq!(result.score, 0.0);
        assert_eq!(result.state, CognitiveState::Clockwork);
    }

    #[test]
    fn calculate_fractality_single() {
        let result = calculate_fractality(&[Duration::from_secs(1)]);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn calculate_fractality_uniform() {
        // Perfectly regular intervals = low fractality
        let times: Vec<Duration> = (0..10).map(|_| Duration::from_secs(1)).collect();
        let result = calculate_fractality(&times);
        // CV = 0, burst_ratio = 1, score should be 0
        assert!(result.score < 0.1);
        assert_eq!(result.state, CognitiveState::Clockwork);
    }

    #[test]
    fn calculate_fractality_bursty() {
        // Highly variable intervals = high fractality
        let times = vec![
            Duration::from_millis(10),
            Duration::from_millis(10),
            Duration::from_secs(5),
            Duration::from_millis(10),
            Duration::from_millis(10),
            Duration::from_secs(5),
        ];
        let result = calculate_fractality(&times);
        // High CV and burst ratio = high score
        assert!(result.score > 0.3);
    }

    #[test]
    fn calculate_fractality_from_seconds_works() {
        let times = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let result = calculate_fractality_from_seconds(&times);
        assert!(result.score < 0.1); // Regular
    }

    #[test]
    fn calculate_fractality_from_timestamps_ascending() {
        let timestamps = vec![1000, 2000, 3000, 4000, 5000]; // 1 second intervals
        let result = calculate_fractality_from_timestamps(&timestamps);
        assert!(result.score < 0.1); // Regular
    }

    #[test]
    fn calculate_fractality_from_timestamps_descending() {
        let timestamps = vec![5000, 4000, 3000, 2000, 1000]; // Reversed
        let result = calculate_fractality_from_timestamps(&timestamps);
        assert!(result.score < 0.1); // Should still be regular after reversing
    }

    #[test]
    fn calculate_fractality_from_timestamps_empty() {
        let result = calculate_fractality_from_timestamps(&[]);
        assert_eq!(result.score, 0.0);
    }

    #[test]
    fn fractality_result_default() {
        let result = FractalityResult::default();
        assert_eq!(result.score, 0.0);
        assert_eq!(result.sigma, 0.0);
        assert_eq!(result.burst_ratio, 1.0);
        assert_eq!(result.cv, 0.0);
        assert_eq!(result.state, CognitiveState::Clockwork);
    }

    #[test]
    fn cv_divisor_matches_adr_054() {
        assert!((CV_DIVISOR - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn burst_divisor_matches_adr_054() {
        assert!((BURST_DIVISOR - 14.0).abs() < f32::EPSILON);
    }

    #[test]
    fn weights_sum_to_one() {
        assert!((CV_WEIGHT + BURST_WEIGHT - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn zero_mean_handling() {
        let times = vec![0.0, 0.0, 0.0];
        let result = calculate_fractality_from_seconds(&times);
        // Should handle division by zero gracefully
        assert!(result.cv >= 0.0);
        assert!(result.burst_ratio >= 0.0);
    }

    #[test]
    fn high_cv_gives_higher_score() {
        // High CV (> 2) should give cv_component near 1.0
        let times = vec![0.1, 0.1, 0.1, 10.0, 0.1, 0.1]; // Very variable
        let result = calculate_fractality_from_seconds(&times);
        assert!(result.cv > 1.0);
        assert!(result.score > 0.3);
    }
}

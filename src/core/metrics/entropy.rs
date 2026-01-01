//! Entropy calculation (ADR-054: Metrics Source of Truth)
//!
//! Shannon entropy calculation for cognitive diversity measurement.
//! Uses TMI-aligned composite salience per ADR-041.

use super::thresholds::CognitiveState;

/// Number of cognitive diversity bins for entropy calculation
pub const COGNITIVE_DIVERSITY_BINS: usize = 5;

/// Maximum entropy for 5 bins: log2(5) ≈ 2.32
const MAX_ENTROPY: f32 = 2.321_928;

/// Salience components for entropy calculation
#[derive(Debug, Clone, Copy, Default)]
pub struct SalienceComponents {
    /// Overall importance (0.0-1.0)
    pub importance: f32,
    /// Novelty score (0.0-1.0)
    pub novelty: f32,
    /// Relevance to current focus (0.0-1.0)
    pub relevance: f32,
    /// Emotional valence (-1.0 to 1.0)
    pub valence: f32,
    /// Emotional arousal (0.0-1.0)
    pub arousal: f32,
    /// Connection relevance for social/kinship (0.0-1.0)
    pub connection_relevance: f32,
}

/// Result of entropy calculation
#[derive(Debug, Clone)]
pub struct EntropyResult {
    /// Raw Shannon entropy value
    pub raw: f32,
    /// Normalized entropy (0.0-1.0)
    pub normalized: f32,
    /// Cognitive state based on normalized entropy
    pub state: CognitiveState,
}

impl Default for EntropyResult {
    fn default() -> Self {
        Self {
            raw: 0.0,
            normalized: 0.0,
            state: CognitiveState::Clockwork,
        }
    }
}

/// Calculate TMI composite salience from components (ADR-041)
///
/// Formula: `emotional_intensity` (40%) + cognitive (60%)
/// - `emotional_intensity` = |valence| × arousal (PRIMARY per TMI/Cury's RAM)
/// - cognitive = importance (30%) + relevance (20%) + novelty (20%) + connection (10%)
#[must_use]
pub fn calculate_tmi_composite(salience: &SalienceComponents) -> f32 {
    let emotional_intensity = salience.valence.abs() * salience.arousal;
    let cognitive = salience.importance.mul_add(0.3, salience.relevance * 0.2);
    let novelty = salience.novelty * 0.2;
    let connection = salience.connection_relevance * 0.1;

    (emotional_intensity.mul_add(0.4, cognitive) + novelty + connection).clamp(0.0, 1.0)
}

/// Calculate Shannon entropy from a slice of TMI composite values
///
/// Uses 5 categorical bins matching cognitive state research:
/// - 0: MINIMAL (< 0.2) - neutral windows, background processing
/// - 1: LOW (0.2-0.4) - routine cognition
/// - 2: MODERATE (0.4-0.6) - active processing
/// - 3: HIGH (0.6-0.8) - focused attention
/// - 4: INTENSE (>= 0.8) - killer window formation
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn calculate_entropy(composites: &[f32]) -> EntropyResult {
    if composites.is_empty() {
        return EntropyResult::default();
    }

    // Bin composites into 5 categorical cognitive states
    let mut bins = [0u32; COGNITIVE_DIVERSITY_BINS];
    for &composite in composites {
        let bin = match composite {
            v if v < 0.2 => 0, // MINIMAL
            v if v < 0.4 => 1, // LOW
            v if v < 0.6 => 2, // MODERATE
            v if v < 0.8 => 3, // HIGH
            _ => 4,            // INTENSE
        };
        bins[bin] += 1;
    }

    let total = composites.len() as f32;
    let mut entropy = 0.0f32;

    for &count in &bins {
        if count > 0 {
            let p = count as f32 / total;
            entropy -= p * p.log2();
        }
    }

    let normalized = (entropy / MAX_ENTROPY).clamp(0.0, 1.0);
    let state = CognitiveState::from_score(normalized);

    EntropyResult {
        raw: entropy,
        normalized,
        state,
    }
}

/// Calculate entropy from salience components
#[must_use]
pub fn calculate_entropy_from_saliences(saliences: &[SalienceComponents]) -> EntropyResult {
    let composites: Vec<f32> = saliences.iter().map(calculate_tmi_composite).collect();
    calculate_entropy(&composites)
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn calculate_entropy_empty() {
        let result = calculate_entropy(&[]);
        assert_eq!(result.raw, 0.0);
        assert_eq!(result.normalized, 0.0);
        assert_eq!(result.state, CognitiveState::Clockwork);
    }

    #[test]
    fn calculate_entropy_single_bin() {
        // All values in one bin = zero entropy
        let composites = vec![0.5, 0.5, 0.5, 0.5, 0.5]; // All in MODERATE bin
        let result = calculate_entropy(&composites);
        assert!(result.raw < 0.01);
        assert_eq!(result.state, CognitiveState::Clockwork);
    }

    #[test]
    fn calculate_entropy_uniform_distribution() {
        // One value in each bin = maximum entropy
        let composites = vec![0.1, 0.3, 0.5, 0.7, 0.9];
        let result = calculate_entropy(&composites);
        // Uniform distribution across 5 bins gives max entropy
        assert!(result.raw > 2.0);
        assert!(result.normalized > 0.9);
        assert_eq!(result.state, CognitiveState::Emergent);
    }

    #[test]
    fn calculate_entropy_two_bins() {
        // Values in only two bins
        let composites = vec![0.1, 0.1, 0.1, 0.9, 0.9];
        let result = calculate_entropy(&composites);
        // Two bins: entropy = -2 * (0.5 * log2(0.5)) = 1.0
        assert!((result.raw - 0.971).abs() < 0.1);
    }

    #[test]
    fn calculate_tmi_composite_zero() {
        let salience = SalienceComponents::default();
        assert_eq!(calculate_tmi_composite(&salience), 0.0);
    }

    #[test]
    fn calculate_tmi_composite_max() {
        let salience = SalienceComponents {
            importance: 1.0,
            novelty: 1.0,
            relevance: 1.0,
            valence: 1.0,
            arousal: 1.0,
            connection_relevance: 1.0,
        };
        let composite = calculate_tmi_composite(&salience);
        // emotional_intensity = 1.0 * 1.0 = 1.0
        // cognitive = 1.0 * 0.3 + 1.0 * 0.2 = 0.5
        // novelty = 1.0 * 0.2 = 0.2
        // connection = 1.0 * 0.1 = 0.1
        // total = 1.0 * 0.4 + 0.5 + 0.2 + 0.1 = 1.2 -> clamped to 1.0
        assert_eq!(composite, 1.0);
    }

    #[test]
    fn calculate_tmi_composite_negative_valence() {
        let salience = SalienceComponents {
            valence: -0.8,
            arousal: 1.0,
            ..Default::default()
        };
        let composite = calculate_tmi_composite(&salience);
        // emotional_intensity = |-0.8| * 1.0 = 0.8
        // total = 0.8 * 0.4 = 0.32
        assert!((composite - 0.32).abs() < 0.01);
    }

    #[test]
    fn calculate_entropy_from_saliences_works() {
        let saliences = vec![
            SalienceComponents {
                importance: 0.1,
                ..Default::default()
            },
            SalienceComponents {
                importance: 0.5,
                ..Default::default()
            },
            SalienceComponents {
                importance: 0.9,
                ..Default::default()
            },
        ];
        let result = calculate_entropy_from_saliences(&saliences);
        assert!(result.raw > 0.0);
    }

    #[test]
    fn entropy_result_default() {
        let result = EntropyResult::default();
        assert_eq!(result.raw, 0.0);
        assert_eq!(result.normalized, 0.0);
        assert_eq!(result.state, CognitiveState::Clockwork);
    }

    #[test]
    fn cognitive_diversity_bins_is_five() {
        assert_eq!(COGNITIVE_DIVERSITY_BINS, 5);
    }
}

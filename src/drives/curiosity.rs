//! Intrinsic Curiosity Module (ICM) - DRIVE-1
//!
//! Implements "wanting to learn" via prediction error.
//! Based on `RLeXplore` and Active Inference principles.
//!
//! # Theory
//!
//! Curiosity is defined as the error of a forward model that predicts the
//! consequence of an action. In DANEEL, the "action" is the shift of attention
//! focus, and the "state" is the embedding of the current thought.
//!
//! If the system cannot accurately predict the next thought, it is "surprised",
//! which boosts salience and encourages further exploration of that context.

use crate::memory_db::types::VECTOR_DIMENSION;
use std::collections::VecDeque;

/// Configuration for the curiosity module
#[derive(Debug, Clone)]
pub struct CuriosityConfig {
    /// Learning rate for the internal predictor (0.0 to 1.0)
    pub learning_rate: f32,
    /// Minimum surprise threshold to trigger salience boost
    pub surprise_threshold: f32,
    /// Maximum salience boost from curiosity
    pub max_boost: f32,
    /// History size for local context prediction
    pub history_size: usize,
}

impl Default for CuriosityConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.1,
            surprise_threshold: 0.2,
            max_boost: 0.5,
            history_size: 10,
        }
    }
}

/// The Curiosity Module (ICM)
///
/// Tracks mental trajectory and calculates prediction error (Surprise).
#[derive(Debug, Clone)]
pub struct CuriosityModule {
    config: CuriosityConfig,
    /// Moving average of the mental state (EMA of vectors)
    expected_state: Vec<f32>,
    /// Recent history of thoughts for context
    history: VecDeque<Vec<f32>>,
}

impl CuriosityModule {
    /// Create a new curiosity module
    #[must_use]
    pub fn new(config: CuriosityConfig) -> Self {
        Self {
            config,
            expected_state: vec![0.0; VECTOR_DIMENSION],
            history: VecDeque::new(),
        }
    }

    /// Calculate curiosity score (surprise) for a new thought vector
    ///
    /// # Arguments
    ///
    /// * `actual_vector` - The embedding of the thought that just occurred
    ///
    /// # Returns
    ///
    /// A curiosity score between 0.0 and 1.0
    pub fn calculate_surprise(&mut self, actual_vector: &[f32]) -> f32 {
        if actual_vector.len() != VECTOR_DIMENSION {
            return 0.0;
        }

        // 1. Calculate Error (Cosine Distance proxy)
        // For efficiency, we use Euclidean distance squared if normalized
        let mut error = 0.0;
        for (i, &val) in actual_vector.iter().enumerate() {
            error += (val - self.expected_state[i]).powi(2);
        }

        // Normalize error roughly to 0.0 - 1.0 range
        // Since embeddings are usually normalized, max dist squared is 4.0 (opposite vectors)
        let surprise = (error / 4.0).clamp(0.0, 1.0);

        // 2. Update Forward Model (EMA)
        // This is a simple online "learning" step
        // In a more complex system, this would be a SGD step on a neural network
        for (i, &val) in actual_vector.iter().enumerate() {
            self.expected_state[i] = self.expected_state[i].mul_add(
                1.0 - self.config.learning_rate,
                val * self.config.learning_rate,
            );
        }

        // 3. Track History
        self.history.push_back(actual_vector.to_vec());
        if self.history.len() > self.config.history_size {
            self.history.pop_front();
        }

        surprise
    }

    /// Calculate salience boost based on surprise
    #[must_use]
    pub fn get_salience_boost(&self, surprise: f32) -> f32 {
        if surprise < self.config.surprise_threshold {
            return 0.0;
        }

        // Scale boost linearly from threshold to max
        let range = 1.0 - self.config.surprise_threshold;
        let normalized = (surprise - self.config.surprise_threshold) / range;

        normalized * self.config.max_boost
    }

    /// Reset the internal state
    pub fn reset(&mut self) {
        self.expected_state = vec![0.0; VECTOR_DIMENSION];
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surprise_calculation() {
        let mut module = CuriosityModule::new(CuriosityConfig::default());

        // Use a unit vector (norm = 1.0)
        let mut v1 = vec![0.0; VECTOR_DIMENSION];
        v1[0] = 1.0;

        let s1 = module.calculate_surprise(&v1);
        assert!(s1 > 0.0);

        // Second thought same as first: less surprising (model moved towards it)
        let s2 = module.calculate_surprise(&v1);
        assert!(s2 < s1, "Surprise should decrease: {s2} < {s1}");

        // Boost check
        let boost = module.get_salience_boost(s1);
        assert!(boost > 0.0);
    }

    #[test]
    fn test_zero_boost_for_low_surprise() {
        let module = CuriosityModule::new(CuriosityConfig {
            surprise_threshold: 0.5,
            ..Default::default()
        });

        assert!((module.get_salience_boost(0.1) - 0.0).abs() < f32::EPSILON);
        assert!((module.get_salience_boost(0.4) - 0.0).abs() < f32::EPSILON);
        assert!(module.get_salience_boost(0.6) > 0.0);
    }
}

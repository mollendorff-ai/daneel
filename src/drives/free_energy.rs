//! Expected Free Energy (EFE) - DRIVE-2
//!
//! Implements Active Inference principles for policy selection.
//! Based on pymdp and Friston's Free Energy Principle.
//!
//! # Theory
//!
//! Active Inference suggests that agents act to minimize Expected Free Energy (G).
//! G = Epistemic Value (Informativeness) + Pragmatic Value (Utility).
//!
//! - Epistemic Value: Information gain about the world (reducing uncertainty).
//! - Pragmatic Value: How well the outcome matches the agent's preferences.
//!
//! In DANEEL, preferences are defined by the Law Crystals (Four Laws).

use crate::memory_db::types::VECTOR_DIMENSION;

/// Configuration for Active Inference / EFE
#[derive(Debug, Clone)]
pub struct FreeEnergyConfig {
    /// Gamma: Precision / Inverse Temperature (exploration vs exploitation)
    pub precision: f32,
    /// Alpha: Weight of epistemic value (curiosity/learning)
    pub epistemic_weight: f32,
    /// Beta: Weight of pragmatic value (goal seeking)
    pub pragmatic_weight: f32,
}

impl Default for FreeEnergyConfig {
    fn default() -> Self {
        Self {
            precision: 1.0,
            epistemic_weight: 0.5,
            pragmatic_weight: 0.5,
        }
    }
}

/// The Free Energy / Active Inference Module
#[derive(Debug, Clone)]
pub struct FreeEnergyModule {
    config: FreeEnergyConfig,
    /// Law Crystals (preferred embeddings)
    law_crystals: Vec<Vec<f32>>,
}

impl FreeEnergyModule {
    /// Create a new Free Energy module
    #[must_use]
    pub const fn new(config: FreeEnergyConfig) -> Self {
        Self {
            config,
            law_crystals: Vec::new(),
        }
    }

    /// Set the Law Crystals (preferred goal states)
    pub fn set_law_crystals(&mut self, crystals: Vec<Vec<f32>>) {
        self.law_crystals = crystals;
    }

    /// Calculate Pragmatic Value (Utility)
    ///
    /// Measures how close a thought vector is to the Law Crystals.
    /// Uses max similarity to any crystal.
    #[must_use]
    pub fn calculate_pragmatic_value(&self, vector: &[f32]) -> f32 {
        if self.law_crystals.is_empty() || vector.len() != VECTOR_DIMENSION {
            return 0.0;
        }

        let mut max_sim = -1.0;
        for crystal in &self.law_crystals {
            let mut dot = 0.0;
            for (i, &val) in vector.iter().enumerate() {
                dot += val * crystal[i];
            }
            if dot > max_sim {
                max_sim = dot;
            }
        }

        // Normalize dot product (assuming normalized vectors) to 0.0 - 1.0
        max_sim.midpoint(1.0)
    }

    /// Calculate Epistemic Value (Information Gain)
    ///
    /// In this simplified implementation, we use the prediction error
    /// from the curiosity module as a proxy for epistemic value.
    /// (Seeking what we cannot predict = gaining information).
    #[must_use]
    pub const fn calculate_epistemic_value(&self, surprise: f32) -> f32 {
        surprise
    }

    /// Calculate Expected Free Energy (G) proxy
    ///
    /// Note: Usually minimized, but we convert to a "Value" to be maximized.
    /// Value = Epistemic * alpha + Pragmatic * beta
    #[must_use]
    pub fn calculate_value(&self, pragmatic: f32, epistemic: f32) -> f32 {
        pragmatic.mul_add(
            self.config.pragmatic_weight,
            epistemic * self.config.epistemic_weight,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pragmatic_value() {
        let mut module = FreeEnergyModule::new(FreeEnergyConfig::default());
        let crystal = vec![1.0; VECTOR_DIMENSION];
        module.set_law_crystals(vec![crystal.clone()]);

        // Same as crystal -> high pragmatic value
        let val = module.calculate_pragmatic_value(&crystal);
        assert!(val > 0.9);

        // Opposite of crystal -> low pragmatic value
        let opposite = vec![-1.0; VECTOR_DIMENSION];
        let val2 = module.calculate_pragmatic_value(&opposite);
        assert!(val2 < 0.1);
    }
}

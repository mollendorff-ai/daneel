//! Core metrics module (ADR-054: Metrics Source of Truth)
//!
//! Single source of truth for cognitive metrics calculation.
//! All consumers (API, CLI, web) must use this module.
//!
//! # Overview
//!
//! This module provides unified calculation for:
//! - **Entropy**: Shannon entropy of cognitive diversity (TMI-aligned)
//! - **Fractality**: Temporal pattern analysis of thought timing
//! - **Thresholds**: Cognitive state boundaries (EMERGENT/BALANCED/CLOCKWORK)
//!
//! # Example
//!
//! ```rust,ignore
//! use daneel::core::metrics::{
//!     calculate_entropy, calculate_fractality, CognitiveState, SalienceComponents,
//! };
//!
//! // Calculate entropy from salience values
//! let composites = vec![0.1, 0.3, 0.5, 0.7, 0.9];
//! let entropy = calculate_entropy(&composites);
//! assert_eq!(entropy.state, CognitiveState::Emergent);
//!
//! // Calculate fractality from timestamps
//! let timestamps = vec![1000, 2000, 5000, 5500, 6000];
//! let fractality = calculate_fractality_from_timestamps(&timestamps);
//! println!("Fractality: {} ({})", fractality.score, fractality.state);
//! ```
//!
//! # ADR References
//!
//! - ADR-041: Entropy Calculation Standardization
//! - ADR-054: Metrics Source of Truth (this module)

pub mod entropy;
pub mod fractality;
pub mod thresholds;

// Re-export commonly used types and functions
pub use entropy::{
    calculate_entropy, calculate_entropy_from_saliences, calculate_tmi_composite, EntropyResult,
    SalienceComponents, COGNITIVE_DIVERSITY_BINS,
};
pub use fractality::{
    calculate_fractality, calculate_fractality_from_seconds, calculate_fractality_from_timestamps,
    FractalityResult,
};
pub use thresholds::{CognitiveState, BALANCED_THRESHOLD, EMERGENT_THRESHOLD};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_exports_entropy_functions() {
        let composites = vec![0.5, 0.5, 0.5];
        let result = calculate_entropy(&composites);
        assert!(result.raw >= 0.0);
    }

    #[test]
    fn module_exports_fractality_functions() {
        let timestamps = vec![1000, 2000, 3000];
        let result = calculate_fractality_from_timestamps(&timestamps);
        assert!(result.score >= 0.0);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn module_exports_thresholds() {
        assert!(EMERGENT_THRESHOLD > BALANCED_THRESHOLD);
    }

    #[test]
    fn module_exports_cognitive_state() {
        let state = CognitiveState::from_score(0.5);
        assert_eq!(state, CognitiveState::Balanced);
    }

    #[test]
    fn module_exports_salience_components() {
        let salience = SalienceComponents {
            importance: 0.5,
            novelty: 0.5,
            relevance: 0.5,
            valence: 0.0,
            arousal: 0.5,
            connection_relevance: 0.3,
        };
        let composite = calculate_tmi_composite(&salience);
        assert!(composite > 0.0);
    }

    #[test]
    fn entropy_result_exported() {
        let result = EntropyResult::default();
        assert_eq!(result.state, CognitiveState::Clockwork);
    }

    #[test]
    fn fractality_result_exported() {
        let result = FractalityResult::default();
        assert_eq!(result.state, CognitiveState::Clockwork);
    }
}

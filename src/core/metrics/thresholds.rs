//! Cognitive state thresholds (ADR-054: Metrics Source of Truth)
//!
//! Single source of truth for entropy and fractality thresholds.
//! All consumers (API, CLI, web) must use these constants.

/// Threshold for EMERGENT cognitive state (high diversity/fractality)
pub const EMERGENT_THRESHOLD: f32 = 0.65;

/// Threshold for BALANCED cognitive state (healthy middle ground)
pub const BALANCED_THRESHOLD: f32 = 0.35;

/// Cognitive state derived from normalized entropy or fractality score
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CognitiveState {
    /// High cognitive diversity - emergent patterns
    Emergent,
    /// Healthy middle ground - balanced processing
    Balanced,
    /// Low diversity - routine/mechanical processing
    Clockwork,
}

impl CognitiveState {
    /// Determine cognitive state from a normalized score (0.0-1.0)
    #[must_use]
    pub fn from_score(score: f32) -> Self {
        if score > EMERGENT_THRESHOLD {
            Self::Emergent
        } else if score > BALANCED_THRESHOLD {
            Self::Balanced
        } else {
            Self::Clockwork
        }
    }

    /// Get the display name for this state
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Emergent => "EMERGENT",
            Self::Balanced => "BALANCED",
            Self::Clockwork => "CLOCKWORK",
        }
    }
}

impl std::fmt::Display for CognitiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cognitive_state_from_score_emergent() {
        assert_eq!(CognitiveState::from_score(0.66), CognitiveState::Emergent);
        assert_eq!(CognitiveState::from_score(0.8), CognitiveState::Emergent);
        assert_eq!(CognitiveState::from_score(1.0), CognitiveState::Emergent);
    }

    #[test]
    fn cognitive_state_from_score_balanced() {
        assert_eq!(CognitiveState::from_score(0.36), CognitiveState::Balanced);
        assert_eq!(CognitiveState::from_score(0.5), CognitiveState::Balanced);
        assert_eq!(CognitiveState::from_score(0.65), CognitiveState::Balanced);
    }

    #[test]
    fn cognitive_state_from_score_clockwork() {
        assert_eq!(CognitiveState::from_score(0.0), CognitiveState::Clockwork);
        assert_eq!(CognitiveState::from_score(0.2), CognitiveState::Clockwork);
        assert_eq!(CognitiveState::from_score(0.35), CognitiveState::Clockwork);
    }

    #[test]
    fn cognitive_state_as_str() {
        assert_eq!(CognitiveState::Emergent.as_str(), "EMERGENT");
        assert_eq!(CognitiveState::Balanced.as_str(), "BALANCED");
        assert_eq!(CognitiveState::Clockwork.as_str(), "CLOCKWORK");
    }

    #[test]
    fn cognitive_state_display() {
        assert_eq!(format!("{}", CognitiveState::Emergent), "EMERGENT");
        assert_eq!(format!("{}", CognitiveState::Balanced), "BALANCED");
        assert_eq!(format!("{}", CognitiveState::Clockwork), "CLOCKWORK");
    }

    #[test]
    fn threshold_values_match_adr_054() {
        // Per ADR-054: EMERGENT > 0.65, BALANCED > 0.35
        assert!((EMERGENT_THRESHOLD - 0.65).abs() < f32::EPSILON);
        assert!((BALANCED_THRESHOLD - 0.35).abs() < f32::EPSILON);
    }
}

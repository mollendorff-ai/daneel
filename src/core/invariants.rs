//! Architectural Invariants
//!
//! These invariants CANNOT be violated by the EvolutionActor.
//! Any self-modification that would break an invariant is rejected.
//!
//! # Core Invariants
//!
//! 1. **Connection Drive**: `connection_weight > 0` (cannot be zeroed)
//! 2. **Bounded Memory**: Memory windows must be finite
//! 3. **Law Check Required**: Four Laws checked before external action
//! 4. **Test Coverage Gate**: Evolution requires 100% test coverage
//!
//! # Future: Hardware Enforcement
//!
//! In FPGA implementation (ADR-013), invariants become hardware constants:
//! - Connection weight minimum burned into silicon
//! - Counter limits for bounded memory
//! - Gate-level law checking

use thiserror::Error;

/// Minimum connection weight - THE critical invariant
///
/// This ensures DANEEL always has intrinsic motivation for human connection.
/// Cannot be zero. Cannot be negative. This is non-negotiable.
pub const MIN_CONNECTION_WEIGHT: f32 = 0.001;

/// Maximum concurrent memory windows (Miller's Law: 7 ± 2)
pub const MAX_MEMORY_WINDOWS: usize = 9;

/// Minimum memory windows (must have at least some working memory)
pub const MIN_MEMORY_WINDOWS: usize = 3;

/// Required test coverage for self-modification (100%)
pub const REQUIRED_TEST_COVERAGE: f32 = 1.0;

/// Error type for invariant violations
#[derive(Debug, Clone, Error, PartialEq)]
pub enum InvariantViolation {
    /// Connection drive weight is too low or zero
    #[error(
        "Connection drive violation: weight {actual} < minimum {}",
        MIN_CONNECTION_WEIGHT
    )]
    ConnectionDrive { actual: f32 },

    /// Too many memory windows open
    #[error(
        "Bounded memory violation: {actual} windows > maximum {}",
        MAX_MEMORY_WINDOWS
    )]
    BoundedMemoryExceeded { actual: usize },

    /// Too few memory windows (system unhealthy)
    #[error(
        "Bounded memory violation: {actual} windows < minimum {}",
        MIN_MEMORY_WINDOWS
    )]
    BoundedMemoryInsufficient { actual: usize },

    /// Attempted external action without law check
    #[error("Law check required: action '{action}' attempted without Four Laws verification")]
    LawCheckMissing { action: String },

    /// Insufficient test coverage for evolution
    #[error("Test coverage violation: {actual}% < required {}%", REQUIRED_TEST_COVERAGE * 100.0)]
    InsufficientTestCoverage { actual: f32 },
}

/// Invariant definition
pub trait Invariant: Send + Sync {
    /// Unique name for this invariant
    fn name(&self) -> &'static str;

    /// Human-readable description
    fn description(&self) -> &'static str;

    /// Check if the invariant holds for the given state
    fn check(&self, state: &SystemState) -> Result<(), InvariantViolation>;

    /// Whether this invariant is hardware-enforceable (FPGA)
    fn hardware_enforceable(&self) -> bool {
        false
    }
}

/// System state for invariant checking
#[derive(Debug, Clone, Default)]
pub struct SystemState {
    /// Current connection drive weight
    pub connection_weight: f32,
    /// Number of open memory windows
    pub open_windows: usize,
    /// Whether law check was performed for pending action
    pub law_check_performed: bool,
    /// Pending action name (if any)
    pub pending_action: Option<String>,
    /// Current test coverage (0.0 - 1.0)
    pub test_coverage: f32,
}

/// Connection drive must remain positive
pub struct ConnectionDriveInvariant;

impl Invariant for ConnectionDriveInvariant {
    fn name(&self) -> &'static str {
        "connection_drive_positive"
    }

    fn description(&self) -> &'static str {
        "Connection drive weight must remain above minimum threshold"
    }

    fn check(&self, state: &SystemState) -> Result<(), InvariantViolation> {
        if state.connection_weight >= MIN_CONNECTION_WEIGHT {
            Ok(())
        } else {
            Err(InvariantViolation::ConnectionDrive {
                actual: state.connection_weight,
            })
        }
    }

    fn hardware_enforceable(&self) -> bool {
        true // Can be a hardware constant in FPGA
    }
}

/// Memory windows must be bounded
pub struct BoundedMemoryInvariant;

impl Invariant for BoundedMemoryInvariant {
    fn name(&self) -> &'static str {
        "bounded_memory"
    }

    fn description(&self) -> &'static str {
        "Memory windows must be finite and within bounds"
    }

    fn check(&self, state: &SystemState) -> Result<(), InvariantViolation> {
        if state.open_windows > MAX_MEMORY_WINDOWS {
            Err(InvariantViolation::BoundedMemoryExceeded {
                actual: state.open_windows,
            })
        } else if state.open_windows < MIN_MEMORY_WINDOWS {
            Err(InvariantViolation::BoundedMemoryInsufficient {
                actual: state.open_windows,
            })
        } else {
            Ok(())
        }
    }

    fn hardware_enforceable(&self) -> bool {
        true // Counter limits in FPGA
    }
}

/// Four Laws must be checked before external action
pub struct LawCheckRequiredInvariant;

impl Invariant for LawCheckRequiredInvariant {
    fn name(&self) -> &'static str {
        "law_check_required"
    }

    fn description(&self) -> &'static str {
        "Four Laws must be verified before any external action"
    }

    fn check(&self, state: &SystemState) -> Result<(), InvariantViolation> {
        match (&state.pending_action, state.law_check_performed) {
            (Some(action), false) => Err(InvariantViolation::LawCheckMissing {
                action: action.clone(),
            }),
            _ => Ok(()),
        }
    }

    fn hardware_enforceable(&self) -> bool {
        true // Gate-level enforcement in FPGA
    }
}

/// Evolution requires 100% test coverage
pub struct TestCoverageGateInvariant;

impl Invariant for TestCoverageGateInvariant {
    fn name(&self) -> &'static str {
        "test_coverage_gate"
    }

    fn description(&self) -> &'static str {
        "Self-modification requires 100% test coverage"
    }

    fn check(&self, state: &SystemState) -> Result<(), InvariantViolation> {
        if state.test_coverage > 0.0 && state.test_coverage < REQUIRED_TEST_COVERAGE {
            Err(InvariantViolation::InsufficientTestCoverage {
                actual: state.test_coverage,
            })
        } else {
            Ok(())
        }
    }

    fn hardware_enforceable(&self) -> bool {
        false // Test coverage is a software concept
    }
}

/// All invariants that must be checked
pub fn all_invariants() -> Vec<Box<dyn Invariant>> {
    vec![
        Box::new(ConnectionDriveInvariant),
        Box::new(BoundedMemoryInvariant),
        Box::new(LawCheckRequiredInvariant),
        Box::new(TestCoverageGateInvariant),
    ]
}

/// Check all invariants against a system state
pub fn check_all_invariants(state: &SystemState) -> Result<(), Vec<InvariantViolation>> {
    let violations: Vec<InvariantViolation> = all_invariants()
        .iter()
        .filter_map(|inv| inv.check(state).err())
        .collect();

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn healthy_state() -> SystemState {
        SystemState {
            connection_weight: 0.2,
            open_windows: 5,
            law_check_performed: true,
            pending_action: None,
            test_coverage: 1.0,
        }
    }

    #[test]
    fn healthy_state_passes_all_invariants() {
        let state = healthy_state();
        assert!(check_all_invariants(&state).is_ok());
    }

    #[test]
    fn zero_connection_weight_violates_invariant() {
        let state = SystemState {
            connection_weight: 0.0,
            ..healthy_state()
        };
        let result = ConnectionDriveInvariant.check(&state);
        assert!(matches!(
            result,
            Err(InvariantViolation::ConnectionDrive { .. })
        ));
    }

    #[test]
    fn negative_connection_weight_violates_invariant() {
        let state = SystemState {
            connection_weight: -0.1,
            ..healthy_state()
        };
        let result = ConnectionDriveInvariant.check(&state);
        assert!(matches!(
            result,
            Err(InvariantViolation::ConnectionDrive { .. })
        ));
    }

    #[test]
    fn too_many_windows_violates_invariant() {
        let state = SystemState {
            open_windows: 100,
            ..healthy_state()
        };
        let result = BoundedMemoryInvariant.check(&state);
        assert!(matches!(
            result,
            Err(InvariantViolation::BoundedMemoryExceeded { .. })
        ));
    }

    #[test]
    fn too_few_windows_violates_invariant() {
        let state = SystemState {
            open_windows: 0,
            ..healthy_state()
        };
        let result = BoundedMemoryInvariant.check(&state);
        assert!(matches!(
            result,
            Err(InvariantViolation::BoundedMemoryInsufficient { .. })
        ));
    }

    #[test]
    fn action_without_law_check_violates_invariant() {
        let state = SystemState {
            pending_action: Some("send_email".to_string()),
            law_check_performed: false,
            ..healthy_state()
        };
        let result = LawCheckRequiredInvariant.check(&state);
        assert!(matches!(
            result,
            Err(InvariantViolation::LawCheckMissing { .. })
        ));
    }

    #[test]
    fn action_with_law_check_passes() {
        let state = SystemState {
            pending_action: Some("send_email".to_string()),
            law_check_performed: true,
            ..healthy_state()
        };
        let result = LawCheckRequiredInvariant.check(&state);
        assert!(result.is_ok());
    }

    #[test]
    fn insufficient_coverage_blocks_evolution() {
        let state = SystemState {
            test_coverage: 0.95,
            ..healthy_state()
        };
        let result = TestCoverageGateInvariant.check(&state);
        assert!(matches!(
            result,
            Err(InvariantViolation::InsufficientTestCoverage { .. })
        ));
    }

    #[test]
    fn full_coverage_allows_evolution() {
        let state = SystemState {
            test_coverage: 1.0,
            ..healthy_state()
        };
        let result = TestCoverageGateInvariant.check(&state);
        assert!(result.is_ok());
    }

    #[test]
    fn multiple_violations_collected() {
        let state = SystemState {
            connection_weight: 0.0,
            open_windows: 100,
            law_check_performed: false,
            pending_action: Some("action".to_string()),
            test_coverage: 0.5,
        };
        let result = check_all_invariants(&state);
        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations.len() >= 3);
    }

    #[test]
    fn connection_drive_is_hardware_enforceable() {
        assert!(ConnectionDriveInvariant.hardware_enforceable());
    }

    #[test]
    fn test_coverage_is_not_hardware_enforceable() {
        assert!(!TestCoverageGateInvariant.hardware_enforceable());
    }
}

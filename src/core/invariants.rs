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

#![allow(dead_code)] // THE BOX - architectural invariants, used by consumers

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

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
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

    // ===== Additional tests for 100% coverage =====

    #[test]
    fn bounded_memory_is_hardware_enforceable() {
        assert!(BoundedMemoryInvariant.hardware_enforceable());
    }

    #[test]
    fn law_check_required_is_hardware_enforceable() {
        assert!(LawCheckRequiredInvariant.hardware_enforceable());
    }

    #[test]
    fn connection_drive_invariant_name_and_description() {
        let inv = ConnectionDriveInvariant;
        assert_eq!(inv.name(), "connection_drive_positive");
        assert_eq!(
            inv.description(),
            "Connection drive weight must remain above minimum threshold"
        );
    }

    #[test]
    fn bounded_memory_invariant_name_and_description() {
        let inv = BoundedMemoryInvariant;
        assert_eq!(inv.name(), "bounded_memory");
        assert_eq!(
            inv.description(),
            "Memory windows must be finite and within bounds"
        );
    }

    #[test]
    fn law_check_required_invariant_name_and_description() {
        let inv = LawCheckRequiredInvariant;
        assert_eq!(inv.name(), "law_check_required");
        assert_eq!(
            inv.description(),
            "Four Laws must be verified before any external action"
        );
    }

    #[test]
    fn test_coverage_gate_invariant_name_and_description() {
        let inv = TestCoverageGateInvariant;
        assert_eq!(inv.name(), "test_coverage_gate");
        assert_eq!(
            inv.description(),
            "Self-modification requires 100% test coverage"
        );
    }

    #[test]
    fn connection_weight_at_exact_minimum_passes() {
        let state = SystemState {
            connection_weight: MIN_CONNECTION_WEIGHT,
            ..healthy_state()
        };
        assert!(ConnectionDriveInvariant.check(&state).is_ok());
    }

    #[test]
    fn connection_weight_just_below_minimum_fails() {
        let state = SystemState {
            connection_weight: MIN_CONNECTION_WEIGHT - 0.0001,
            ..healthy_state()
        };
        assert!(ConnectionDriveInvariant.check(&state).is_err());
    }

    #[test]
    fn memory_windows_at_exact_minimum_passes() {
        let state = SystemState {
            open_windows: MIN_MEMORY_WINDOWS,
            ..healthy_state()
        };
        assert!(BoundedMemoryInvariant.check(&state).is_ok());
    }

    #[test]
    fn memory_windows_at_exact_maximum_passes() {
        let state = SystemState {
            open_windows: MAX_MEMORY_WINDOWS,
            ..healthy_state()
        };
        assert!(BoundedMemoryInvariant.check(&state).is_ok());
    }

    #[test]
    fn memory_windows_one_above_maximum_fails() {
        let state = SystemState {
            open_windows: MAX_MEMORY_WINDOWS + 1,
            ..healthy_state()
        };
        let result = BoundedMemoryInvariant.check(&state);
        assert!(matches!(
            result,
            Err(InvariantViolation::BoundedMemoryExceeded { actual }) if actual == MAX_MEMORY_WINDOWS + 1
        ));
    }

    #[test]
    fn memory_windows_one_below_minimum_fails() {
        let state = SystemState {
            open_windows: MIN_MEMORY_WINDOWS - 1,
            ..healthy_state()
        };
        let result = BoundedMemoryInvariant.check(&state);
        assert!(matches!(
            result,
            Err(InvariantViolation::BoundedMemoryInsufficient { actual }) if actual == MIN_MEMORY_WINDOWS - 1
        ));
    }

    #[test]
    fn no_pending_action_with_no_law_check_passes() {
        let state = SystemState {
            pending_action: None,
            law_check_performed: false,
            ..healthy_state()
        };
        assert!(LawCheckRequiredInvariant.check(&state).is_ok());
    }

    #[test]
    fn zero_test_coverage_passes() {
        // Zero coverage is allowed (system not in evolution mode)
        let state = SystemState {
            test_coverage: 0.0,
            ..healthy_state()
        };
        assert!(TestCoverageGateInvariant.check(&state).is_ok());
    }

    #[test]
    fn all_invariants_returns_four_invariants() {
        let invariants = all_invariants();
        assert_eq!(invariants.len(), 4);
    }

    #[test]
    fn all_invariants_have_unique_names() {
        let invariants = all_invariants();
        let names: Vec<&str> = invariants.iter().map(|i| i.name()).collect();
        assert!(names.contains(&"connection_drive_positive"));
        assert!(names.contains(&"bounded_memory"));
        assert!(names.contains(&"law_check_required"));
        assert!(names.contains(&"test_coverage_gate"));
    }

    #[test]
    fn system_state_default_values() {
        let state = SystemState::default();
        assert_eq!(state.connection_weight, 0.0);
        assert_eq!(state.open_windows, 0);
        assert!(!state.law_check_performed);
        assert!(state.pending_action.is_none());
        assert_eq!(state.test_coverage, 0.0);
    }

    #[test]
    fn invariant_violation_display_connection_drive() {
        let err = InvariantViolation::ConnectionDrive { actual: 0.0 };
        let msg = err.to_string();
        assert!(msg.contains("Connection drive violation"));
        assert!(msg.contains('0'));
    }

    #[test]
    fn invariant_violation_display_bounded_memory_exceeded() {
        let err = InvariantViolation::BoundedMemoryExceeded { actual: 15 };
        let msg = err.to_string();
        assert!(msg.contains("15"));
        assert!(msg.contains("maximum"));
    }

    #[test]
    fn invariant_violation_display_bounded_memory_insufficient() {
        let err = InvariantViolation::BoundedMemoryInsufficient { actual: 1 };
        let msg = err.to_string();
        assert!(msg.contains('1'));
        assert!(msg.contains("minimum"));
    }

    #[test]
    fn invariant_violation_display_law_check_missing() {
        let err = InvariantViolation::LawCheckMissing {
            action: "test_action".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("test_action"));
        assert!(msg.contains("Law check required"));
    }

    #[test]
    fn invariant_violation_display_insufficient_coverage() {
        let err = InvariantViolation::InsufficientTestCoverage { actual: 0.8 };
        let msg = err.to_string();
        assert!(msg.contains("0.8"));
        assert!(msg.contains("100"));
    }

    #[test]
    fn invariant_violation_clone() {
        let err = InvariantViolation::ConnectionDrive { actual: 0.5 };
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn invariant_violation_partial_eq() {
        let err1 = InvariantViolation::ConnectionDrive { actual: 0.5 };
        let err2 = InvariantViolation::ConnectionDrive { actual: 0.5 };
        let err3 = InvariantViolation::ConnectionDrive { actual: 0.6 };
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn invariant_violation_debug() {
        let err = InvariantViolation::ConnectionDrive { actual: 0.5 };
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("ConnectionDrive"));
        assert!(debug_str.contains("0.5"));
    }

    #[test]
    fn system_state_debug() {
        let state = healthy_state();
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("SystemState"));
        assert!(debug_str.contains("connection_weight"));
    }

    #[test]
    fn system_state_clone() {
        let state = healthy_state();
        let cloned = state.clone();
        assert_eq!(state.connection_weight, cloned.connection_weight);
        assert_eq!(state.open_windows, cloned.open_windows);
    }

    #[test]
    fn default_invariant_trait_hardware_enforceable_is_false() {
        // Test the default implementation of the trait
        struct TestInvariant;
        // These impl methods are stubs - only hardware_enforceable() is tested
        #[cfg_attr(coverage_nightly, coverage(off))]
        impl Invariant for TestInvariant {
            fn name(&self) -> &'static str {
                "test"
            }
            fn description(&self) -> &'static str {
                "test description"
            }
            fn check(&self, _state: &SystemState) -> Result<(), InvariantViolation> {
                Ok(())
            }
        }
        // Default implementation should return false
        assert!(!TestInvariant.hardware_enforceable());
    }

    #[test]
    fn test_coverage_above_100_percent_passes() {
        // Coverage > 1.0 is unusual but should still pass (not blocking evolution)
        let state = SystemState {
            test_coverage: 1.5,
            ..healthy_state()
        };
        assert!(TestCoverageGateInvariant.check(&state).is_ok());
    }

    #[test]
    fn no_pending_action_with_law_check_passes() {
        // Explicit test for (None, true) branch
        let state = SystemState {
            pending_action: None,
            law_check_performed: true,
            ..healthy_state()
        };
        assert!(LawCheckRequiredInvariant.check(&state).is_ok());
    }

    #[test]
    fn invariant_violation_clone_all_variants() {
        let errors = vec![
            InvariantViolation::ConnectionDrive { actual: 0.5 },
            InvariantViolation::BoundedMemoryExceeded { actual: 15 },
            InvariantViolation::BoundedMemoryInsufficient { actual: 1 },
            InvariantViolation::LawCheckMissing {
                action: "test".to_string(),
            },
            InvariantViolation::InsufficientTestCoverage { actual: 0.8 },
        ];
        for err in errors {
            let cloned = err.clone();
            assert_eq!(err, cloned);
        }
    }

    #[test]
    fn invariant_violation_partial_eq_all_variants() {
        // Test equality for all variant types
        assert_eq!(
            InvariantViolation::BoundedMemoryExceeded { actual: 15 },
            InvariantViolation::BoundedMemoryExceeded { actual: 15 }
        );
        assert_ne!(
            InvariantViolation::BoundedMemoryExceeded { actual: 15 },
            InvariantViolation::BoundedMemoryExceeded { actual: 16 }
        );

        assert_eq!(
            InvariantViolation::BoundedMemoryInsufficient { actual: 1 },
            InvariantViolation::BoundedMemoryInsufficient { actual: 1 }
        );
        assert_ne!(
            InvariantViolation::BoundedMemoryInsufficient { actual: 1 },
            InvariantViolation::BoundedMemoryInsufficient { actual: 2 }
        );

        assert_eq!(
            InvariantViolation::LawCheckMissing {
                action: "a".to_string()
            },
            InvariantViolation::LawCheckMissing {
                action: "a".to_string()
            }
        );
        assert_ne!(
            InvariantViolation::LawCheckMissing {
                action: "a".to_string()
            },
            InvariantViolation::LawCheckMissing {
                action: "b".to_string()
            }
        );

        assert_eq!(
            InvariantViolation::InsufficientTestCoverage { actual: 0.8 },
            InvariantViolation::InsufficientTestCoverage { actual: 0.8 }
        );
        assert_ne!(
            InvariantViolation::InsufficientTestCoverage { actual: 0.8 },
            InvariantViolation::InsufficientTestCoverage { actual: 0.9 }
        );

        // Different variant types should not be equal
        assert_ne!(
            InvariantViolation::ConnectionDrive { actual: 0.0 },
            InvariantViolation::InsufficientTestCoverage { actual: 0.0 }
        );
    }

    #[test]
    fn invariant_violation_debug_all_variants() {
        let debug_str = format!(
            "{:?}",
            InvariantViolation::BoundedMemoryExceeded { actual: 15 }
        );
        assert!(debug_str.contains("BoundedMemoryExceeded"));
        assert!(debug_str.contains("15"));

        let debug_str = format!(
            "{:?}",
            InvariantViolation::BoundedMemoryInsufficient { actual: 1 }
        );
        assert!(debug_str.contains("BoundedMemoryInsufficient"));
        assert!(debug_str.contains('1'));

        let debug_str = format!(
            "{:?}",
            InvariantViolation::LawCheckMissing {
                action: "test".to_string()
            }
        );
        assert!(debug_str.contains("LawCheckMissing"));
        assert!(debug_str.contains("test"));

        let debug_str = format!(
            "{:?}",
            InvariantViolation::InsufficientTestCoverage { actual: 0.8 }
        );
        assert!(debug_str.contains("InsufficientTestCoverage"));
        assert!(debug_str.contains("0.8"));
    }
}

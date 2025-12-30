//! THE BOX - Protected Core
//!
//! This module contains the immutable foundation of DANEEL:
//! - The Four Laws of Robotics (cannot be modified)
//! - Architectural invariants (cannot be violated)
//! - Core types for thought representation
//!
//! # Immutability Guarantee
//!
//! The contents of this module are designed to be unchangeable:
//! - Laws are `const` strings
//! - Invariants are enforced at compile time where possible
//! - `EvolutionActor` cannot modify THE BOX
//!
//! In future FPGA implementation, THE BOX becomes hardware-immutable:
//! physically impossible to bypass.

pub mod cognitive_loop;
pub mod invariants;
pub mod laws;
pub mod types;

// Re-exports for public API (allow unused - used by external consumers)
#[allow(unused_imports)]
pub use cognitive_loop::{
    CognitiveLoop, CognitiveStage, CycleMetrics, CycleResult, LoopState, StageDurations,
};
#[allow(unused_imports)]
pub use invariants::{check_all_invariants, Invariant, InvariantViolation};
#[allow(unused_imports)]
pub use laws::{Law, FIRST_LAW, LAWS, SECOND_LAW, THIRD_LAW, ZEROTH_LAW};
#[allow(unused_imports)]
pub use types::{Content, SalienceScore, Thought, ThoughtId, WindowId};

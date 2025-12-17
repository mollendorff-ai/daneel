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
//! - EvolutionActor cannot modify THE BOX
//!
//! In future FPGA implementation, THE BOX becomes hardware-immutable:
//! physically impossible to bypass.

pub mod laws;
pub mod invariants;
pub mod types;

pub use laws::{Law, LAWS, ZEROTH_LAW, FIRST_LAW, SECOND_LAW, THIRD_LAW};
pub use invariants::{Invariant, InvariantViolation, check_all_invariants};
pub use types::{Thought, Content, SalienceScore, WindowId, ThoughtId};

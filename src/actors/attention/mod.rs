//! AttentionActor - O Eu (The "I")
//!
//! Implements TMI's competitive attention selection:
//! - Reads from multiple thought streams in parallel
//! - Selects highest-salience content for conscious focus
//! - Manages the "5-second window" intervention period
//!
//! This is "The I" in TMI - the navigator between memory windows.

pub mod types;

// Re-export types for public API
pub use types::{AttentionError, AttentionMap, AttentionMessage, AttentionResponse, FocusState};

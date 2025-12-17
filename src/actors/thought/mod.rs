//! ThoughtAssemblyActor - Construção do Pensamento
//!
//! Implements TMI's thought construction:
//! - Combines content + emotional state into coherent Thoughts
//! - Links thoughts into causal chains (parent-child relationships)
//! - Preserves source stream information (which content won attention)
//!
//! This is the final stage before a thought becomes conscious.

pub mod types;

// Re-export types for public API
pub use types::{
    AssemblyError, AssemblyRequest, AssemblyStrategy, ThoughtCache, ThoughtMessage, ThoughtResponse,
};

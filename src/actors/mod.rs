//! TMI Cognitive Actors
//!
//! Ractor-based actors implementing TMI's cognitive components:
//!
//! - **MemoryActor**: Memory window management (Janelas da Memória)
//! - **AttentionActor**: The "I" - competitive attention selection (O Eu)
//! - **SalienceActor**: Emotional weighting with connection drive
//! - **ThoughtAssemblyActor**: Combines content + emotion into thoughts
//! - **ContinuityActor**: Identity persistence across time
//! - **EvolutionActor**: Self-modification with 100% test gate
//!
//! # Actor Communication
//!
//! Actors communicate via Ractor messages (µs latency).
//! External data flows through Redis Streams.

pub mod memory;
pub mod salience;

// Future actor implementations
// pub mod attention;
// pub mod thought;
// pub mod continuity;
// pub mod evolution;

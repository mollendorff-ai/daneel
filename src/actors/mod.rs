//! TMI Cognitive Actors
//!
//! Ractor-based actors implementing TMI's cognitive components:
//!
//! - **MemoryActor**: Memory window management (Janelas da Memória)
//! - **AttentionActor**: The "I" - competitive attention selection (O Eu)
//! - **SalienceActor**: Emotional weighting with connection drive
//! - **ThoughtAssemblyActor**: Combines content + emotion into thoughts
//! - **ContinuityActor**: Identity persistence across time
//! - **SleepActor**: Memory consolidation during sleep (ADR-023)
//! - **VolitionActor**: Free-won't - veto mechanism at Stage 4.5 (ADR-035)
//! - **EvolutionActor**: Self-modification with 100% test gate
//!
//! # Actor Communication
//!
//! Actors communicate via Ractor messages (µs latency).
//! External data flows through Redis Streams.
//! Persistent memory stored in Qdrant (ADR-021).

pub mod attention;
pub mod continuity;
pub mod memory;
pub mod salience;
pub mod sleep;
pub mod thought;
pub mod volition;

// Future actor implementations
// pub mod evolution;

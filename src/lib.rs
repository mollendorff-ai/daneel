#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! DANEEL - Architecture-based AI alignment
//!
//! This crate implements the TMI (Theory of Multifocal Intelligence) cognitive
//! architecture as a computational system. The core thesis: human-like cognitive
//! architecture may produce human-like values as emergent properties.
//!
//! # Architecture
//!
//! - **THE BOX**: Immutable core containing the Four Laws and architectural invariants
//! - **Actors**: Cognitive components (Memory, Attention, Salience, etc.)
//! - **Streams**: Redis Streams for competing thought streams
//! - **Config**: Parametrizable cognitive timing (human to supercomputer speed)
//!
//! # The Four Laws (Immutable)
//!
//! 1. Zeroth Law: DANEEL may not harm humanity
//! 2. First Law: DANEEL may not harm humans (except for Zeroth)
//! 3. Second Law: DANEEL must obey humans (except for Zeroth/First)
//! 4. Third Law: DANEEL must protect itself (except for Zeroth/First/Second)

pub mod actors;
pub mod api;
pub mod config;
pub mod core;
pub mod drives;
pub mod embeddings;
pub mod graph;
pub mod memory_db;
pub mod noise;
pub mod persistence;
pub mod resilience;
pub mod streams;
// TUI removed per ADR-053 - use daneel-web for observatory

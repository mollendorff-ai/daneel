//! Tests for `ThoughtAssemblyActor`
//!
//! ADR-049: Test modules excluded from coverage.
//!
//! This module is organized into themed submodules:
//! - `actor` - Actor lifecycle, basic assembly, batch, and cache tests
//! - `chain` - Parent linking, chain operations, and chain edge cases
//! - `validation` - Salience validation tests
//! - `state` - State/config construction and direct method tests

#![cfg_attr(coverage_nightly, coverage(off))]
#![allow(clippy::manual_let_else)]
#![allow(clippy::float_cmp)]
#![allow(clippy::significant_drop_tightening)]

mod actor;
mod chain;
mod state;
mod validation;

use super::*;
use crate::core::types::{Content, SalienceScore, ThoughtId};
use ractor::rpc::CallResult;
use ractor::Actor;

/// Helper to spawn a thought actor for testing
async fn spawn_thought_actor() -> ActorRef<ThoughtMessage> {
    let (actor_ref, _) = Actor::spawn(None, ThoughtAssemblyActor, AssemblyConfig::default())
        .await
        .expect("Failed to spawn ThoughtAssemblyActor");
    actor_ref
}

/// Helper to spawn a thought actor with custom config
async fn spawn_thought_actor_with_config(config: AssemblyConfig) -> ActorRef<ThoughtMessage> {
    let (actor_ref, _) = Actor::spawn(None, ThoughtAssemblyActor, config)
        .await
        .expect("Failed to spawn ThoughtAssemblyActor");
    actor_ref
}

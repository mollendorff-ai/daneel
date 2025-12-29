//! Memory Consolidation Example
//!
//! Demonstrates how to wire up Qdrant memory storage with the cognitive loop.
//!
//! # Prerequisites
//!
//! Qdrant must be running:
//! ```sh
//! docker compose up -d qdrant
//! ```
//!
//! # Usage
//!
//! ```sh
//! cargo run --example memory_consolidation
//! ```

use daneel::config::CognitiveConfig;
use daneel::core::cognitive_loop::CognitiveLoop;
use daneel::memory_db::MemoryDb;
use std::sync::Arc;
use tracing::{info, warn};

#[allow(clippy::significant_drop_tightening)] // Resources held for example duration
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Memory Consolidation Example");
    info!("============================");

    // Step 1: Connect to Qdrant and initialize collections
    info!("Connecting to Qdrant at http://localhost:6334...");
    let memory_db = match MemoryDb::connect_and_init("http://localhost:6334").await {
        Ok(db) => {
            info!("Successfully connected to Qdrant");
            info!("Collections initialized (memories, episodes, identity)");
            Arc::new(db)
        }
        Err(e) => {
            warn!("Failed to connect to Qdrant: {}", e);
            warn!("Make sure Qdrant is running: docker compose up -d qdrant");
            return Err(e.into());
        }
    };

    // Step 2: Create a cognitive loop with memory storage
    info!("Creating cognitive loop with memory consolidation...");
    let mut cognitive_loop = CognitiveLoop::with_config(CognitiveConfig::human());

    // Wire up the memory database
    cognitive_loop.set_memory_db(Arc::clone(&memory_db));

    // Set consolidation threshold (default is 0.7)
    cognitive_loop.set_consolidation_threshold(0.6);
    info!("Consolidation threshold set to 0.6 (thoughts above this will be stored)");

    // Step 3: Start the cognitive loop
    cognitive_loop.start();
    info!("Cognitive loop started");

    // Step 4: Run a few cycles to demonstrate consolidation
    info!("\nRunning 5 cognitive cycles...");
    for i in 0..5 {
        let result = cognitive_loop.run_cycle().await;
        info!(
            "Cycle {}: duration={:?}, on_time={}",
            result.cycle_number, result.duration, result.on_time
        );

        // In a real system, thoughts would be produced by the cognitive loop
        // For this demo, we'll simulate storing some thoughts manually
        if i % 2 == 0 {
            simulate_high_salience_thought(&memory_db).await;
        }
    }

    // Step 5: Check what was stored
    info!("\nMemory Statistics:");
    let memory_count = memory_db.memory_count().await?;
    info!("Total memories stored: {}", memory_count);

    let episode_count = memory_db.episode_count().await?;
    info!("Total episodes: {}", episode_count);

    // Step 6: Demonstrate retrieval (bonus)
    if memory_count > 0 {
        info!("\nRetrieving memories with high salience...");
        let candidates = memory_db.get_replay_candidates(10).await?;
        info!(
            "Found {} memories tagged for consolidation",
            candidates.len()
        );

        for (i, memory) in candidates.iter().take(3).enumerate() {
            info!(
                "  Memory {}: \"{}\" (salience={:.2})",
                i + 1,
                memory.content.chars().take(50).collect::<String>(),
                memory.composite_salience()
            );
        }
    }

    info!("\nExample completed successfully!");
    Ok(())
}

/// Simulate storing a high-salience thought directly to memory
async fn simulate_high_salience_thought(memory_db: &MemoryDb) {
    use daneel::memory_db::{Memory, MemorySource};

    // Create a high-salience memory
    let memory = Memory::new(
        "Important realization about connection and bonding".to_string(),
        MemorySource::Reasoning {
            chain: vec![], // Empty chain for now
        },
    )
    .with_emotion(0.8, 0.9) // High valence, high arousal
    .tag_for_consolidation();

    // Generate dummy vector
    let vector = vec![0.0; 768];

    // Store it
    match memory_db.store_memory(&memory, &vector).await {
        Ok(()) => info!("  -> Stored high-salience memory: {}", memory.id),
        Err(e) => warn!("  -> Failed to store memory: {}", e),
    }
}

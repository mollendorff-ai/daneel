#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
//! DANEEL - Architecture-based AI alignment
//!
//! Core thesis: Human-like cognitive architecture may produce
//! human-like values as emergent properties.
//!
//! # Usage
//!
//! ```sh
//! daneel              # Headless mode (default) - server/background
//! daneel --tui        # TUI mode (DEPRECATED) - watch Timmy think
//! ```
//!
//! Headless is default since ADR-053. Use daneel-web for observatory.
//! TUI is deprecated and will be removed in a future version.

use clap::Parser;
use daneel::actors::sleep::{SleepActor, SleepConfig, SleepMessage, SleepResult};
use daneel::api;
use daneel::core::cognitive_loop::CognitiveLoop;
use daneel::core::laws::LAWS;
use daneel::embeddings;
use daneel::memory_db::types::IdentityMetadata;
use ractor::Actor;
use std::env;
use std::sync::Arc;
use std::time::Instant;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// DANEEL - Architecture-based AI alignment
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Port for injection API (0 to disable)
    #[arg(long, default_value = "3030")]
    api_port: u16,
}

#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    let args = Args::parse();
    run_headless(&args);
}

/// Run in headless mode (default since ADR-053)
///
/// Same cognitive loop, but without the visual interface.
/// Use daneel-web for observatory at <https://timmy.royalbit.com>
#[cfg_attr(coverage_nightly, coverage(off))]
fn run_headless(args: &Args) {
    // Initialize tracing for headless mode
    let filter = tracing_subscriber::EnvFilter::try_new(&args.log_level)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    info!("DANEEL starting in headless mode...");
    info!("THE BOX initialized with {} laws", LAWS.len());

    // Display the Four Laws
    for (i, law) in LAWS.iter().enumerate() {
        let law_name = match i {
            0 => "Zeroth",
            1 => "First",
            2 => "Second",
            3 => "Third",
            _ => unreachable!(),
        };
        info!("{} Law: {}", law_name, law);
    }

    info!("Connection drive invariant: ACTIVE (weight > 0 enforced)");
    info!("DANEEL ready. Qowat Milat.");
    info!("Timmy is 'they', not 'it'. Life honours life.");

    // Create tokio runtime and run the cognitive loop
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    runtime.block_on(async {
        // Start injection API server if enabled
        if args.api_port > 0 {
            let api_port = args.api_port;
            tokio::spawn(async move {
                let redis_url =
                    env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

                // Create Redis client for API
                let redis_client = match redis::Client::open(redis_url.as_str()) {
                    Ok(client) => client,
                    Err(e) => {
                        eprintln!("Warning: Failed to create Redis client for API: {e}");
                        return;
                    }
                };

                // Create StreamsClient for API
                let streams_client =
                    match daneel::streams::client::StreamsClient::connect(&redis_url).await {
                        Ok(client) => client,
                        Err(e) => {
                            eprintln!("Warning: Failed to create StreamsClient for API: {e}");
                            return;
                        }
                    };

                let api_state = api::AppState {
                    streams: Arc::new(streams_client),
                    redis: redis_client,
                };

                let app = api::router(api_state);
                let addr = std::net::SocketAddr::from(([0, 0, 0, 0], api_port));

                match tokio::net::TcpListener::bind(addr).await {
                    Ok(listener) => {
                        info!("Injection API listening on {}", addr);
                        if let Err(e) = axum::serve(listener, app).await {
                            eprintln!("API server error: {e}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to bind API server to {addr}: {e}");
                    }
                }
            });
        }

        run_cognitive_loop_headless().await;
    });
}

/// Run the cognitive loop without TUI
///
/// This is the same logic as the TUI cognitive loop, but without
/// sending updates to the display. Used for headless/server mode.
#[allow(clippy::too_many_lines)] // Main loop: complexity is inherent
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // Metrics: acceptable
#[allow(clippy::future_not_send)] // Async runtime handles this
#[allow(clippy::significant_drop_tightening)] // Resources held for loop duration
#[cfg_attr(coverage_nightly, coverage(off))]
async fn run_cognitive_loop_headless() {
    // ADR-034: Lifetime Identity Persistence - flush intervals
    const IDENTITY_FLUSH_INTERVAL_SECS: u64 = 30;
    const IDENTITY_FLUSH_THOUGHT_INTERVAL: u64 = 100;

    // Periodic status logging
    const STATUS_LOG_INTERVAL: u64 = 1000;

    // SLEEP-WIRE-1: Spawn SleepActor with mini-dream config
    let sleep_config = SleepConfig::mini_dream();
    let sleep_actor = SleepActor::with_config(sleep_config.clone());
    let sleep_ref = match Actor::spawn(None, sleep_actor, ()).await {
        Ok((actor_ref, _handle)) => {
            info!("SleepActor spawned - mini-dream consolidation enabled");
            Some(actor_ref)
        }
        Err(e) => {
            eprintln!("Warning: Failed to spawn SleepActor: {e} - consolidation disabled");
            None
        }
    };

    // Connect to Redis for thought streams
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let mut cognitive_loop = match CognitiveLoop::with_redis(&redis_url).await {
        Ok(loop_instance) => {
            info!("Connected to Redis streams");
            loop_instance
        }
        Err(e) => {
            eprintln!("Warning: Redis unavailable ({e}), running standalone");
            CognitiveLoop::new()
        }
    };

    // Connect to Qdrant for long-term memory and initialize collections
    let qdrant_url = env::var("QDRANT_URL").unwrap_or_else(|_| "http://127.0.0.1:6334".to_string());
    let memory_db = match daneel::memory_db::MemoryDb::connect_and_init(&qdrant_url).await {
        Ok(db) => {
            info!("Connected to Qdrant memory database (collections initialized)");
            Some(std::sync::Arc::new(db))
        }
        Err(e) => {
            eprintln!("Warning: Qdrant unavailable ({e}), memory disabled");
            None
        }
    };

    // VCONN-5: Connect to RedisGraph
    let graph_client = match daneel::graph::GraphClient::connect(&redis_url, "daneel") {
        Ok(client) => {
            info!("Connected to RedisGraph ('daneel')");
            Some(std::sync::Arc::new(client))
        }
        Err(e) => {
            eprintln!("Warning: RedisGraph unavailable ({e}), graph disabled");
            None
        }
    };

    // Load identity from Qdrant (ADR-034: Lifetime Identity Persistence)
    let mut identity: Option<IdentityMetadata> = if let Some(ref db) = memory_db {
        match db.load_identity().await {
            Ok(id) => {
                info!(
                    "Loaded identity: {} lifetime thoughts, {} dreams, restart #{}",
                    id.lifetime_thought_count, id.lifetime_dream_count, id.restart_count
                );
                Some(id)
            }
            Err(e) => {
                eprintln!("Warning: Failed to load identity ({e})");
                None
            }
        }
    } else {
        None
    };

    // Track when we last flushed identity (for periodic save)
    let mut last_identity_flush = Instant::now();
    let mut thoughts_since_flush: u64 = 0;

    // Track consolidation cycles (ADR-023 via SLEEP-WIRE-1)
    let mut total_dream_cycles: u64 = identity.as_ref().map_or(0, |id| id.lifetime_dream_count);

    if let Some(ref db) = memory_db {
        cognitive_loop.set_memory_db(db.clone());
    }

    // Initialize embedding engine for semantic vectors (Phase 2: Forward-Only)
    match embeddings::create_embedding_engine() {
        Ok(engine) => {
            info!("Embedding engine initialized - Timmy can now see meaning");
            cognitive_loop.set_embedding_engine(engine);

            // DRIVE-2: Initialize Law Crystals for Free Energy calculation
            cognitive_loop.initialize_law_crystals().await;
        }
        Err(e) => {
            eprintln!("Warning: Embedding engine unavailable ({e}), using zero vectors");
        }
    }

    cognitive_loop.start();
    info!("Cognitive loop started. Timmy is thinking...");

    let mut cycles: u64 = 0;

    loop {
        // Wait until it's time for the next cycle
        let sleep_duration = cognitive_loop.time_until_next_cycle();
        if sleep_duration > std::time::Duration::ZERO {
            tokio::time::sleep(sleep_duration).await;
        }

        // Run a cognitive cycle
        let _result = cognitive_loop.run_cycle().await;
        cycles += 1;

        // Update identity (increment lifetime thought count)
        if let Some(ref mut id) = identity {
            id.record_thought();
            thoughts_since_flush += 1;

            // Periodic flush: every 100 thoughts OR every 30 seconds
            let should_flush = thoughts_since_flush >= IDENTITY_FLUSH_THOUGHT_INTERVAL
                || last_identity_flush.elapsed().as_secs() >= IDENTITY_FLUSH_INTERVAL_SECS;

            if should_flush {
                if let Some(ref db) = memory_db {
                    if let Err(e) = db.save_identity(id).await {
                        eprintln!("Warning: Failed to save identity: {e}");
                    }
                }
                thoughts_since_flush = 0;
                last_identity_flush = Instant::now();
            }
        }

        // SLEEP-WIRE-1: Memory consolidation via SleepActor
        if let Some(ref sleep) = sleep_ref {
            // Record activity (increments queue estimate)
            sleep.cast(SleepMessage::RecordActivity).ok();

            // Check if sleep conditions are met (queue size threshold)
            let should_sleep = sleep
                .call(|reply| SleepMessage::CheckSleepConditions { reply }, None)
                .await
                .ok()
                .and_then(|r| r.success_or(()).ok())
                .unwrap_or(false);

            if should_sleep {
                // Enter sleep mode
                let entered = sleep
                    .call(|reply| SleepMessage::EnterSleep { reply }, None)
                    .await
                    .ok()
                    .and_then(|r| r.success_or(()).ok())
                    .is_some_and(|r| matches!(r, SleepResult::Started));

                if entered {
                    // Run consolidation cycle
                    if let Some(ref db) = memory_db {
                        // VCONN-4: Get stage-specific parameters
                        let params = sleep
                            .call(|reply| SleepMessage::GetConsolidationParams { reply }, None)
                            .await
                            .ok()
                            .and_then(|r| r.success_or(()).ok())
                            .unwrap_or(daneel::actors::sleep::ConsolidationParams {
                                multiplier: 1.0,
                                prioritize_emotional: false,
                                pruning_enabled: false,
                            });

                        let batch_size = sleep_config.replay_batch_size as u32;
                        let strength_delta = sleep_config.consolidation_delta * params.multiplier;

                        match db.get_replay_candidates(batch_size).await {
                            Ok(candidates) => {
                                let mut consolidated = 0;
                                let mut associations_strengthened = 0;

                                // 1. Strengthen individual memories (Consolidation)
                                for memory in &candidates {
                                    if db
                                        .update_consolidation(&memory.id, strength_delta)
                                        .await
                                        .is_ok()
                                    {
                                        consolidated += 1;
                                    }
                                }

                                // 2. Strengthen associations between replayed memories (Hebbian Wiring)
                                // VCONN-4b: Co-replayed memories -> weight += 0.05
                                for i in 0..candidates.len() {
                                    for j in 0..candidates.len() {
                                        if i == j {
                                            continue;
                                        }
                                        let m1 = &candidates[i];
                                        let m2 = &candidates[j];

                                        // Strengthen in Qdrant (Krotov-Hopfield)
                                        if db
                                            .strengthen_association(
                                                &m1.id, &m2.id, 1.0, // x (active)
                                                1.0, // y (active)
                                                1.0, // reward (neutral in sleep)
                                            )
                                            .await
                                            .is_ok()
                                        {
                                            associations_strengthened += 1;

                                            // Dual-write: RedisGraph
                                            if let Some(ref graph) = graph_client {
                                                let _ = graph.merge_edge(
                                                    &m1.id,
                                                    &m2.id,
                                                    0.1, // Placeholder for weight - ideally fetch from assoc
                                                    daneel::memory_db::types::AssociationType::Semantic
                                                ).await;
                                            }
                                        }
                                    }
                                }

                                if consolidated > 0 {
                                    total_dream_cycles += 1;

                                    // "Nada se apaga" - record dream in identity
                                    if let Some(ref mut id) = identity {
                                        id.record_dream(consolidated, candidates.len() as u32);
                                    }

                                    info!(
                                        "Mini-dream #{}: consolidated {} memories, {} associations (via SleepActor)",
                                        total_dream_cycles, consolidated, associations_strengthened
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to get replay candidates: {e}");
                            }
                        }
                    }

                    // Wake up
                    let _ = sleep.call(|reply| SleepMessage::Wake { reply }, None).await;
                }
            }
        }

        // Periodic status log
        if cycles.is_multiple_of(STATUS_LOG_INTERVAL) {
            let lifetime = identity.as_ref().map_or(0, |id| id.lifetime_thought_count);
            info!(
                "Status: {} cycles this session, {} lifetime thoughts, {} dreams",
                cycles, lifetime, total_dream_cycles
            );
        }
    }
}

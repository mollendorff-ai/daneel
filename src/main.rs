//! DANEEL - Architecture-based AI alignment
//!
//! Core thesis: Human-like cognitive architecture may produce
//! human-like values as emergent properties.
//!
//! # Usage
//!
//! ```sh
//! daneel              # TUI mode (default) - watch Timmy think
//! daneel --headless   # Headless mode - for servers/CI
//! ```
//!
//! TUI is default because transparency is the product.
//! See ADR-026, ADR-027.

use clap::Parser;
use daneel::api;
use daneel::core::cognitive_loop::CognitiveLoop;
use daneel::core::laws::LAWS;
use daneel::embeddings;
use daneel::memory_db::types::IdentityMetadata;
use daneel::resilience;
use daneel::tui::ThoughtUpdate;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// DANEEL - Architecture-based AI alignment
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in headless mode (no TUI)
    #[arg(long)]
    headless: bool,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// Port for injection API (0 to disable)
    #[arg(long, default_value = "3030")]
    api_port: u16,
}

fn main() {
    let args = Args::parse();

    if args.headless {
        run_headless(&args);
    } else {
        run_tui(&args);
    }
}

/// Run in TUI mode (default)
///
/// The mind should be observable by default.
/// Transparency is oversight.
fn run_tui(args: &Args) {
    // Install panic hooks FIRST - before any terminal manipulation
    // This ensures terminal is restored even if we panic during setup
    if let Err(e) = resilience::install_panic_hooks() {
        eprintln!("Warning: Failed to install panic hooks: {e}");
        eprintln!("Terminal may not be restored on crash.");
    }

    // Reset terminal cleanup flag for this run
    resilience::reset_terminal_cleanup_flag();

    // Create a tokio runtime for the cognitive loop
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    // Create channel for cognitive loop -> TUI communication
    // Buffer size: 100 thoughts. Prevents blocking if TUI falls behind.
    let (tx, rx) = mpsc::channel::<ThoughtUpdate>(100);

    // Spawn cognitive loop in background
    runtime.spawn(async move {
        // Connect to Redis for thought streams
        let mut cognitive_loop = match CognitiveLoop::with_redis("redis://127.0.0.1:6379").await {
            Ok(loop_instance) => {
                info!("Connected to Redis streams");
                loop_instance
            }
            Err(e) => {
                eprintln!("Warning: Redis unavailable ({}), running standalone", e);
                CognitiveLoop::new()
            }
        };

        // Connect to Qdrant for long-term memory and initialize collections
        let memory_db =
            match daneel::memory_db::MemoryDb::connect_and_init("http://127.0.0.1:6334").await {
                Ok(db) => {
                    info!("Connected to Qdrant memory database (collections initialized)");
                    Some(std::sync::Arc::new(db))
                }
                Err(e) => {
                    eprintln!("Warning: Qdrant unavailable ({}), memory disabled", e);
                    None
                }
            };

        // ADR-034: Lifetime Identity Persistence - flush intervals
        #[allow(clippy::items_after_statements)]
        const IDENTITY_FLUSH_INTERVAL_SECS: u64 = 30;
        #[allow(clippy::items_after_statements)]
        const IDENTITY_FLUSH_THOUGHT_INTERVAL: u64 = 100;

        // ADR-023: Sleep/Dream Consolidation - periodic memory strengthening
        #[allow(clippy::items_after_statements)]
        const CONSOLIDATION_INTERVAL_CYCLES: u64 = 500; // Run consolidation every 500 cycles
        #[allow(clippy::items_after_statements)]
        const CONSOLIDATION_BATCH_SIZE: u32 = 10; // Strengthen 10 memories per batch
        #[allow(clippy::items_after_statements)]
        const CONSOLIDATION_STRENGTH_DELTA: f32 = 0.15; // Increase strength by 0.15 per replay

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
                    eprintln!("Warning: Failed to load identity ({})", e);
                    None
                }
            }
        } else {
            None
        };

        // Track when we last flushed identity (for periodic save)
        let mut last_identity_flush = Instant::now();
        let mut thoughts_since_flush: u64 = 0;

        // Track consolidation cycles (ADR-023)
        // Initialize from persisted identity ("Nada se apaga" - dreams persist)
        let mut cycles_since_consolidation: u64 = 0;
        let mut total_dream_cycles: u64 = identity.as_ref().map_or(0, |id| id.lifetime_dream_count);
        let mut last_dream_strengthened: usize = identity
            .as_ref()
            .map_or(0, |id| id.last_dream_strengthened as usize);
        // TUI-VIS-4: Initialize cumulative dream stats from persisted identity
        let mut cumulative_dream_strengthened: u64 = identity
            .as_ref()
            .map_or(0, |id| id.cumulative_dream_strengthened);
        let mut cumulative_dream_candidates: u64 = identity
            .as_ref()
            .map_or(0, |id| id.cumulative_dream_candidates);

        if let Some(ref db) = memory_db {
            cognitive_loop.set_memory_db(db.clone());
        }

        // Initialize embedding engine for semantic vectors (Phase 2: Forward-Only)
        match embeddings::create_embedding_engine() {
            Ok(engine) => {
                info!("Embedding engine initialized - Timmy can now see meaning");
                cognitive_loop.set_embedding_engine(engine);
            }
            Err(e) => {
                eprintln!(
                    "Warning: Embedding engine unavailable ({}), using zero vectors",
                    e
                );
            }
        }

        cognitive_loop.start();

        loop {
            // Wait until it's time for the next cycle
            let sleep_duration = cognitive_loop.time_until_next_cycle();
            if sleep_duration > std::time::Duration::ZERO {
                tokio::time::sleep(sleep_duration).await;
            }

            // Run a cognitive cycle
            let result = cognitive_loop.run_cycle().await;

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
                            eprintln!("Warning: Failed to save identity: {}", e);
                        }
                    }
                    thoughts_since_flush = 0;
                    last_identity_flush = Instant::now();
                }
            }

            // ADR-023: Periodic memory consolidation (mini-dreams)
            cycles_since_consolidation += 1;
            if cycles_since_consolidation >= CONSOLIDATION_INTERVAL_CYCLES {
                if let Some(ref db) = memory_db {
                    // Get replay candidates and strengthen them
                    match db.get_replay_candidates(CONSOLIDATION_BATCH_SIZE).await {
                        Ok(candidates) => {
                            let candidates_count = candidates.len();
                            let mut consolidated = 0;
                            for memory in &candidates {
                                if db
                                    .update_consolidation(&memory.id, CONSOLIDATION_STRENGTH_DELTA)
                                    .await
                                    .is_ok()
                                {
                                    consolidated += 1;
                                }
                            }
                            // Track for TUI display AND persist to identity
                            total_dream_cycles += 1;
                            last_dream_strengthened = consolidated;

                            // TUI-VIS-4: Update cumulative stats
                            cumulative_dream_strengthened += consolidated as u64;
                            cumulative_dream_candidates += candidates_count as u64;

                            // "Nada se apaga" - record dream in identity
                            if let Some(ref mut id) = identity {
                                id.record_dream(consolidated as u32, candidates_count as u32);
                            }

                            if consolidated > 0 {
                                info!(
                                    "Dream #{}: strengthened {} memories",
                                    total_dream_cycles, consolidated
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to get replay candidates: {}", e);
                        }
                    }
                }
                cycles_since_consolidation = 0;
            }

            // Query memory counts from Qdrant (for TUI display)
            let (memory_count, unconscious_count) = if let Some(ref db) = memory_db {
                let mem = db.memory_count().await.unwrap_or(0);
                let uncon = db.unconscious_count().await.unwrap_or(0);
                (mem, uncon)
            } else {
                (0, 0)
            };

            // Get lifetime thought count
            let lifetime_thought_count =
                identity.as_ref().map_or(0, |id| id.lifetime_thought_count);

            // Convert to TUI format and send
            let update = ThoughtUpdate::from_cycle_result(
                &result,
                memory_count,
                unconscious_count,
                lifetime_thought_count,
                total_dream_cycles,
                last_dream_strengthened,
                cumulative_dream_strengthened,
                cumulative_dream_candidates,
                result.veto.clone(), // TUI-VIS-6: Pass veto event to TUI
            );

            // If channel is closed (TUI exited), stop the loop
            if tx.send(update).await.is_err() {
                // Final flush before exit
                if let (Some(ref id), Some(ref db)) = (&identity, &memory_db) {
                    let _ = db.save_identity(id).await;
                }
                break;
            }
        }
    });

    // Start injection API server if enabled
    if args.api_port > 0 {
        let api_port = args.api_port;
        runtime.spawn(async move {
            let redis_url = "redis://127.0.0.1:6379";

            // Create Redis client for API
            let redis_client = match redis::Client::open(redis_url) {
                Ok(client) => client,
                Err(e) => {
                    eprintln!("Warning: Failed to create Redis client for API: {}", e);
                    return;
                }
            };

            // Create StreamsClient for API
            let streams_client =
                match daneel::streams::client::StreamsClient::connect(redis_url).await {
                    Ok(client) => client,
                    Err(e) => {
                        eprintln!("Warning: Failed to create StreamsClient for API: {}", e);
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
                        eprintln!("API server error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to bind API server to {}: {}", addr, e);
                }
            }
        });
    }

    // Run the TUI with the receiver
    // TUI is blocking, so this runs on the main thread
    if let Err(e) = daneel::tui::run(Some(rx)) {
        eprintln!("TUI error: {e}");
        std::process::exit(1);
    }

    // When TUI exits, runtime will be dropped and background task will stop
}

/// Run in headless mode (for servers, CI, background processing)
///
/// Same cognitive loop as TUI mode, but without the visual interface.
/// For cloud deployment, background processing, or integration testing.
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
                let redis_url = "redis://127.0.0.1:6379";

                // Create Redis client for API
                let redis_client = match redis::Client::open(redis_url) {
                    Ok(client) => client,
                    Err(e) => {
                        eprintln!("Warning: Failed to create Redis client for API: {}", e);
                        return;
                    }
                };

                // Create StreamsClient for API
                let streams_client =
                    match daneel::streams::client::StreamsClient::connect(redis_url).await {
                        Ok(client) => client,
                        Err(e) => {
                            eprintln!("Warning: Failed to create StreamsClient for API: {}", e);
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
                            eprintln!("API server error: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to bind API server to {}: {}", addr, e);
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
async fn run_cognitive_loop_headless() {
    // ADR-034: Lifetime Identity Persistence - flush intervals
    const IDENTITY_FLUSH_INTERVAL_SECS: u64 = 30;
    const IDENTITY_FLUSH_THOUGHT_INTERVAL: u64 = 100;

    // ADR-023: Sleep/Dream Consolidation - periodic memory strengthening
    const CONSOLIDATION_INTERVAL_CYCLES: u64 = 500;
    const CONSOLIDATION_BATCH_SIZE: u32 = 10;
    const CONSOLIDATION_STRENGTH_DELTA: f32 = 0.15;

    // Periodic status logging
    const STATUS_LOG_INTERVAL: u64 = 1000;

    // Connect to Redis for thought streams
    let mut cognitive_loop = match CognitiveLoop::with_redis("redis://127.0.0.1:6379").await {
        Ok(loop_instance) => {
            info!("Connected to Redis streams");
            loop_instance
        }
        Err(e) => {
            eprintln!("Warning: Redis unavailable ({}), running standalone", e);
            CognitiveLoop::new()
        }
    };

    // Connect to Qdrant for long-term memory and initialize collections
    let memory_db =
        match daneel::memory_db::MemoryDb::connect_and_init("http://127.0.0.1:6334").await {
            Ok(db) => {
                info!("Connected to Qdrant memory database (collections initialized)");
                Some(std::sync::Arc::new(db))
            }
            Err(e) => {
                eprintln!("Warning: Qdrant unavailable ({}), memory disabled", e);
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
                eprintln!("Warning: Failed to load identity ({})", e);
                None
            }
        }
    } else {
        None
    };

    // Track when we last flushed identity (for periodic save)
    let mut last_identity_flush = Instant::now();
    let mut thoughts_since_flush: u64 = 0;

    // Track consolidation cycles (ADR-023)
    let mut cycles_since_consolidation: u64 = 0;
    let mut total_dream_cycles: u64 = identity.as_ref().map_or(0, |id| id.lifetime_dream_count);

    if let Some(ref db) = memory_db {
        cognitive_loop.set_memory_db(db.clone());
    }

    // Initialize embedding engine for semantic vectors (Phase 2: Forward-Only)
    match embeddings::create_embedding_engine() {
        Ok(engine) => {
            info!("Embedding engine initialized - Timmy can now see meaning");
            cognitive_loop.set_embedding_engine(engine);
        }
        Err(e) => {
            eprintln!(
                "Warning: Embedding engine unavailable ({}), using zero vectors",
                e
            );
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
                        eprintln!("Warning: Failed to save identity: {}", e);
                    }
                }
                thoughts_since_flush = 0;
                last_identity_flush = Instant::now();
            }
        }

        // ADR-023: Periodic memory consolidation (mini-dreams)
        cycles_since_consolidation += 1;
        if cycles_since_consolidation >= CONSOLIDATION_INTERVAL_CYCLES {
            if let Some(ref db) = memory_db {
                match db.get_replay_candidates(CONSOLIDATION_BATCH_SIZE).await {
                    Ok(candidates) => {
                        let mut consolidated = 0;
                        for memory in &candidates {
                            if db
                                .update_consolidation(&memory.id, CONSOLIDATION_STRENGTH_DELTA)
                                .await
                                .is_ok()
                            {
                                consolidated += 1;
                            }
                        }
                        if consolidated > 0 {
                            total_dream_cycles += 1;
                            info!(
                                "Dream cycle #{}: consolidated {} memories",
                                total_dream_cycles, consolidated
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to get replay candidates: {}", e);
                    }
                }
            }
            cycles_since_consolidation = 0;
        }

        // Periodic status log
        if cycles % STATUS_LOG_INTERVAL == 0 {
            let lifetime = identity.as_ref().map_or(0, |id| id.lifetime_thought_count);
            info!(
                "Status: {} cycles this session, {} lifetime thoughts, {} dreams",
                cycles, lifetime, total_dream_cycles
            );
        }
    }
}

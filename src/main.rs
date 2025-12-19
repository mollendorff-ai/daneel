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
use daneel::core::cognitive_loop::CognitiveLoop;
use daneel::core::laws::LAWS;
use daneel::resilience;
use daneel::tui::ThoughtUpdate;
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
}

fn main() {
    let args = Args::parse();

    if args.headless {
        run_headless(&args);
    } else {
        run_tui();
    }
}

/// Run in TUI mode (default)
///
/// The mind should be observable by default.
/// Transparency is oversight.
fn run_tui() {
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
        let mut cognitive_loop = CognitiveLoop::new();
        cognitive_loop.start();

        loop {
            // Wait until it's time for the next cycle
            let sleep_duration = cognitive_loop.time_until_next_cycle();
            if sleep_duration > std::time::Duration::ZERO {
                tokio::time::sleep(sleep_duration).await;
            }

            // Run a cognitive cycle
            let result = cognitive_loop.run_cycle().await;

            // Convert to TUI format and send
            let update = ThoughtUpdate::from_cycle_result(&result);

            // If channel is closed (TUI exited), stop the loop
            if tx.send(update).await.is_err() {
                break;
            }
        }
    });

    // Run the TUI with the receiver
    // TUI is blocking, so this runs on the main thread
    if let Err(e) = daneel::tui::run(Some(rx)) {
        eprintln!("TUI error: {e}");
        std::process::exit(1);
    }

    // When TUI exits, runtime will be dropped and background task will stop
}

/// Run in headless mode (for servers, CI, background processing)
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

    // In real implementation, this would start the cognitive loop
    // For now, just indicate we're ready
    info!("Headless mode: cognitive loop would start here");
}

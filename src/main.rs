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
use daneel::core::laws::LAWS;
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
    // TUI handles its own display, minimal logging
    if let Err(e) = daneel::tui::run() {
        eprintln!("TUI error: {e}");
        std::process::exit(1);
    }
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

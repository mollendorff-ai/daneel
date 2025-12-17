//! DANEEL - Architecture-based AI alignment
//!
//! Core thesis: Human-like cognitive architecture may produce
//! human-like values as emergent properties.

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod core;

fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("DANEEL starting...");
    info!("THE BOX initialized with {} laws", core::laws::LAWS.len());

    // Display the Four Laws
    for (i, law) in core::laws::LAWS.iter().enumerate() {
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
}

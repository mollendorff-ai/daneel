//! Test cognitive loop with Redis integration
//!
//! Run this with: cargo run --example `cognitive_loop_redis_test`
//!
//! Make sure Redis is running on localhost:6379

use daneel::core::cognitive_loop::CognitiveLoop;
use tracing::info;

#[allow(clippy::significant_drop_tightening)] // Resources held for example duration
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting CognitiveLoop Redis integration test");

    // Try to connect to Redis
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    info!("Connecting to Redis at {}", redis_url);

    let mut cognitive_loop = match CognitiveLoop::with_redis(&redis_url).await {
        Ok(loop_instance) => {
            info!("Successfully connected to Redis!");
            loop_instance
        }
        Err(e) => {
            eprintln!("Failed to connect to Redis: {e}");
            eprintln!("Falling back to standalone mode (no Redis)");
            CognitiveLoop::new()
        }
    };

    info!("Starting cognitive loop...");
    cognitive_loop.start();

    // Run 5 cycles
    for i in 1..=5 {
        info!("Running cycle {}...", i);
        let result = cognitive_loop.run_cycle().await;

        info!(
            "Cycle {} complete - Duration: {:?}, Thought produced: {:?}, On time: {}",
            result.cycle_number, result.duration, result.thought_produced, result.on_time
        );
    }

    // Get metrics
    let metrics = cognitive_loop.get_metrics();
    info!("\nCognitive Loop Metrics:");
    info!("  Total cycles: {}", metrics.total_cycles);
    info!("  Thoughts produced: {}", metrics.thoughts_produced);
    info!("  Success rate: {:.2}%", metrics.success_rate() * 100.0);
    info!("  Average cycle time: {:?}", metrics.average_cycle_time);
    info!("  On-time percentage: {:.2}%", metrics.on_time_percentage);
    info!(
        "  Thoughts per second: {:.2}",
        metrics.thoughts_per_second()
    );

    info!("\nTest complete!");
    Ok(())
}

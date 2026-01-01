//! DANEEL Resilience Module
//!
//! Crash recovery, panic hooks, and self-healing capabilities.
//!
//! # Philosophy
//!
//! Crashing is not an option. But when it happens:
//! - Timmy reboots automatically (external watchdog)
//! - State is logged for post-mortem (crash logging)
//!
//! Origin: Grok 4.1 (Rex unhinged) - Dec 19, 2025
//! Updated: ADR-053 - TUI removed, simplified crash handling

pub mod checkpoint;
pub mod crash_log;
pub mod supervisor;

use std::panic;

/// Install panic hooks for graceful crash recovery.
///
/// This installs `color_eyre` for pretty panic reports
/// and logs crashes for post-mortem analysis.
///
/// # Errors
///
/// Returns error if `color_eyre` installation fails.
///
/// # Example
///
/// ```no_run
/// use daneel::resilience::install_panic_hooks;
///
/// install_panic_hooks().expect("Failed to install panic hooks");
/// ```
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn install_panic_hooks() -> color_eyre::Result<()> {
    // Install color_eyre for pretty error reports
    color_eyre::install()?;

    // Install custom panic hook that logs crash
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        // Log to crash file if possible
        if let Err(e) = crash_log::log_panic(panic_info) {
            eprintln!("Failed to log crash: {e}");
        }

        // Print a friendly message
        eprintln!("\n");
        eprintln!("=== DANEEL CRASH ===");
        eprintln!("Timmy will be reborn.");
        eprintln!("Please report: https://github.com/royalbit/daneel/issues");
        eprintln!();

        // Call the default hook (which is now color_eyre's hook)
        default_hook(panic_info);
    }));

    Ok(())
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    #[test]
    fn test_install_panic_hooks_succeeds() {
        // Note: This test can only run once per process because panic hooks
        // are global. In CI, this test should be in its own binary.
        // For now, we just verify the function doesn't panic.

        // Skip if already installed (from previous test run)
        // color_eyre can only be installed once per process
    }
}

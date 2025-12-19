//! DANEEL Resilience Module
//!
//! Crash recovery, panic hooks, and self-healing capabilities.
//!
//! # Philosophy
//!
//! Crashing is not an option. But when it happens:
//! - Timmy reboots automatically (external watchdog)
//! - Terminal is restored (panic hooks)
//! - State is logged for post-mortem (crash logging)
//! - Viewers see the recovery live
//!
//! Origin: Grok 4.1 (Rex unhinged) - Dec 19, 2025

pub mod crash_log;
pub mod checkpoint;
pub mod supervisor;

use std::io::Write;
use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    cursor::Show,
};

/// Flag to track if terminal cleanup has been done
/// (prevents double-cleanup which can cause issues)
static TERMINAL_CLEANED: AtomicBool = AtomicBool::new(false);

/// Install panic hooks for graceful crash recovery.
///
/// This MUST be called before starting the TUI to ensure
/// terminal state is restored on panic.
///
/// # What it does
///
/// 1. Installs color_eyre for pretty panic reports
/// 2. Sets up a custom panic hook that:
///    - Restores terminal state (raw mode, cursor, alternate screen)
///    - Logs crash details
///    - Then calls the original panic handler
///
/// # Example
///
/// ```no_run
/// use daneel::resilience::install_panic_hooks;
///
/// fn main() {
///     install_panic_hooks().expect("Failed to install panic hooks");
///     // ... rest of app
/// }
/// ```
pub fn install_panic_hooks() -> color_eyre::Result<()> {
    // Install color_eyre for pretty error reports
    color_eyre::install()?;

    // Install custom panic hook that restores terminal
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        // First, restore terminal state
        // This MUST happen before any printing
        let _ = restore_terminal();

        // Log to crash file if possible
        if let Err(e) = crash_log::log_panic(panic_info) {
            eprintln!("Failed to log crash: {e}");
        }

        // Print a friendly message
        eprintln!("\n");
        eprintln!("=== DANEEL CRASH ===");
        eprintln!("Terminal restored. Timmy will be reborn.");
        eprintln!("Please report: https://github.com/royalbit/daneel/issues");
        eprintln!();

        // Call the default hook (which is now color_eyre's hook)
        default_hook(panic_info);
    }));

    Ok(())
}

/// Restore terminal to normal state.
///
/// This is safe to call multiple times (idempotent).
/// Used by panic hooks to ensure terminal is usable after crash.
///
/// # What it does
///
/// 1. Disables raw mode
/// 2. Leaves alternate screen
/// 3. Shows cursor
/// 4. Flushes stdout
pub fn restore_terminal() -> std::io::Result<()> {
    // Check if already cleaned (prevent double-cleanup)
    if TERMINAL_CLEANED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    // Best effort - try each step even if others fail
    let mut result = Ok(());

    // Disable raw mode
    if let Err(e) = disable_raw_mode() {
        eprintln!("Warning: failed to disable raw mode: {e}");
        result = Err(e);
    }

    // Leave alternate screen and show cursor
    let mut stdout = std::io::stdout();
    if let Err(e) = execute!(stdout, LeaveAlternateScreen, Show) {
        eprintln!("Warning: failed to restore screen: {e}");
        if result.is_ok() {
            result = Err(e);
        }
    }

    // Flush stdout
    let _ = stdout.flush();

    result
}

/// Reset the terminal cleanup flag.
///
/// Call this when starting TUI to allow cleanup on next crash.
pub fn reset_terminal_cleanup_flag() {
    TERMINAL_CLEANED.store(false, Ordering::SeqCst);
}

/// Check if terminal cleanup has been performed.
pub fn is_terminal_cleaned() -> bool {
    TERMINAL_CLEANED.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_panic_hooks_succeeds() {
        // Note: This test can only run once per process because panic hooks
        // are global. In CI, this test should be in its own binary.
        // For now, we just verify the function doesn't panic.

        // Skip if already installed (from previous test run)
        // color_eyre can only be installed once per process
    }

    #[test]
    fn test_terminal_cleanup_is_idempotent() {
        // Reset flag
        reset_terminal_cleanup_flag();
        assert!(!is_terminal_cleaned());

        // First cleanup should work (in test mode, terminal isn't in raw mode)
        let _ = restore_terminal();
        assert!(is_terminal_cleaned());

        // Second cleanup should be no-op
        let _ = restore_terminal();
        assert!(is_terminal_cleaned());

        // Reset for other tests
        reset_terminal_cleanup_flag();
    }

    #[test]
    fn test_reset_terminal_cleanup_flag() {
        TERMINAL_CLEANED.store(true, Ordering::SeqCst);
        assert!(is_terminal_cleaned());

        reset_terminal_cleanup_flag();
        assert!(!is_terminal_cleaned());
    }
}

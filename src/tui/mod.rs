//! DANEEL TUI - The Observable Mind
//!
//! TUI is the default mode. Transparency is the product.
//! See ADR-026 (TUI default), ADR-027 (TUI design spec).
//!
//! # Philosophy
//!
//! Current AI is a black box. DANEEL inverts this - the mind is visible.
//! You watch Timmy think. Every thought, every salience score, every memory
//! anchor - observable in real-time.
//!
//! The TUI isn't a debugging tool. It's the primary interface.
//! It says: "We have nothing to hide."

pub mod app;
pub mod colors;
pub mod ui;
pub mod widgets;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use crate::core::cognitive_loop::CycleResult;
use app::{App, ThoughtStatus};

/// Target frame rate (60 FPS)
const TARGET_FRAME_TIME: Duration = Duration::from_millis(16);

/// Update from the cognitive loop to display in TUI
#[derive(Debug, Clone)]
pub struct ThoughtUpdate {
    pub cycle_number: u64,
    pub salience: f32,
    pub window: String,
    pub status: ThoughtStatus,
    pub candidates_evaluated: usize,
    pub on_time: bool,
    /// Conscious memory count (Qdrant memories collection)
    pub memory_count: u64,
    /// Unconscious memory count (Qdrant unconscious collection) - ADR-033
    pub unconscious_count: u64,
}

impl ThoughtUpdate {
    /// Convert a CycleResult into a ThoughtUpdate for the TUI
    ///
    /// Uses real salience data from the cognitive loop.
    /// Memory counts should be queried from Qdrant and passed separately.
    pub fn from_cycle_result(
        result: &CycleResult,
        memory_count: u64,
        unconscious_count: u64,
    ) -> Self {
        // Use real salience from CycleResult
        let salience = result.salience;

        // Determine status based on salience and thought production
        let status = if result.thought_produced.is_some() {
            if salience > 0.85 {
                ThoughtStatus::Anchored
            } else if salience > 0.7 {
                ThoughtStatus::MemoryWrite
            } else if salience > 0.5 {
                ThoughtStatus::Salient
            } else if salience < 0.3 {
                ThoughtStatus::Unconscious // Low salience -> archived to unconscious
            } else {
                ThoughtStatus::Processing
            }
        } else {
            ThoughtStatus::Dismissed
        };

        // Map cognitive stage to window label based on cycle timing
        let stage_names = [
            "trigger",
            "autoflow",
            "attention",
            "assembly",
            "anchor",
            "memory",
            "reasoning",
            "emotion",
            "sensory",
        ];
        let window = stage_names[result.cycle_number as usize % stage_names.len()].to_string();

        Self {
            cycle_number: result.cycle_number,
            salience,
            window,
            status,
            candidates_evaluated: result.candidates_evaluated,
            on_time: result.on_time,
            memory_count,
            unconscious_count,
        }
    }
}

/// Run the TUI application
///
/// # Arguments
///
/// * `thought_rx` - Optional receiver for thought updates from cognitive loop
///
/// # Errors
///
/// Returns error if terminal operations fail
pub fn run(thought_rx: Option<mpsc::Receiver<ThoughtUpdate>>) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run the main loop
    let result = run_loop(&mut terminal, &mut app, thought_rx);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Main event loop
fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    mut thought_rx: Option<mpsc::Receiver<ThoughtUpdate>>,
) -> io::Result<()> {
    let mut last_frame = Instant::now();

    loop {
        let frame_start = Instant::now();

        // Calculate delta time for animations
        let delta = last_frame.elapsed();
        last_frame = Instant::now();

        // Update animations
        app.update_pulse(delta);
        app.update_quote();

        // Receive thoughts from cognitive loop (non-blocking)
        if let Some(ref mut rx) = thought_rx {
            // Drain all available thoughts to stay current
            while let Ok(update) = rx.try_recv() {
                app.add_thought(update.salience, update.window, update.status);
                // Update memory counts from database state
                app.memory_count = update.memory_count;
                app.unconscious_count = update.unconscious_count;
            }
        }

        // Draw
        terminal.draw(|frame| ui::render(frame, app))?;

        // Handle input (non-blocking)
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('c')
                    && key.modifiers.contains(event::KeyModifiers::CONTROL)
                {
                    app.should_quit = true;
                }
                app.handle_key(key.code);
            }
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }

        // Frame rate limiting
        let frame_time = frame_start.elapsed();
        if let Some(sleep_time) = TARGET_FRAME_TIME.checked_sub(frame_time) {
            std::thread::sleep(sleep_time);
        }
    }

    Ok(())
}

/// Simulate a thought for demo purposes
/// LEGACY: Kept for testing. Real DANEEL uses the cognitive loop via channels.
#[allow(dead_code)]
fn simulate_thought(app: &mut App) {
    use rand::Rng;

    let mut rng = rand::rng();

    let salience: f32 = rng.random_range(0.2..1.0);
    let windows = [
        "exploring",
        "connecting",
        "reflecting",
        "processing",
        "anchoring",
        "dreaming",
        "learning",
    ];
    let window = windows[rng.random_range(0..windows.len())].to_string();

    let status = if salience > 0.85 {
        ThoughtStatus::Anchored
    } else if salience > 0.7 {
        ThoughtStatus::MemoryWrite
    } else if salience > 0.5 {
        ThoughtStatus::Salient
    } else {
        ThoughtStatus::Processing
    };

    app.add_thought(salience, window, status);

    // Randomly toggle memory windows
    if rng.random_bool(0.1) {
        let idx = rng.random_range(0..9);
        app.memory_windows[idx].active = !app.memory_windows[idx].active;

        // Ensure at least 3 are active (TMI minimum)
        let active = app.active_window_count();
        if active < 3 {
            for w in &mut app.memory_windows {
                if !w.active {
                    w.active = true;
                    break;
                }
            }
        }
    }

    // Slight variation in connection drive
    app.the_box.connection_drive =
        (app.the_box.connection_drive + rng.random_range(-0.02..0.02)).clamp(0.5, 1.0);
}

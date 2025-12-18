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

use app::{App, ThoughtStatus};

/// Target frame rate (60 FPS)
const TARGET_FRAME_TIME: Duration = Duration::from_millis(16);

/// Run the TUI application
///
/// # Errors
///
/// Returns error if terminal operations fail
pub fn run() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run the main loop
    let result = run_loop(&mut terminal, &mut app);

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
fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    let mut last_frame = Instant::now();
    let mut thought_timer = Instant::now();

    loop {
        let frame_start = Instant::now();

        // Calculate delta time for animations
        let delta = last_frame.elapsed();
        last_frame = Instant::now();

        // Update animations
        app.update_pulse(delta);
        app.update_quote();

        // Simulate thoughts (for demo - in real DANEEL this comes from cognitive loop)
        if thought_timer.elapsed() > Duration::from_millis(500) {
            simulate_thought(app);
            thought_timer = Instant::now();
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
        if frame_time < TARGET_FRAME_TIME {
            std::thread::sleep(TARGET_FRAME_TIME - frame_time);
        }
    }

    Ok(())
}

/// Simulate a thought for demo purposes
/// In real DANEEL, this data comes from the cognitive loop via channels
fn simulate_thought(app: &mut App) {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let salience: f32 = rng.gen_range(0.2..1.0);
    let windows = [
        "exploring",
        "connecting",
        "reflecting",
        "processing",
        "anchoring",
        "dreaming",
        "learning",
    ];
    let window = windows[rng.gen_range(0..windows.len())].to_string();

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
    if rng.gen_bool(0.1) {
        let idx = rng.gen_range(0..9);
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
        (app.the_box.connection_drive + rng.gen_range(-0.02..0.02)).clamp(0.5, 1.0);
}

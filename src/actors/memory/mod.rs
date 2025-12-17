//! Memory Actor - Janelas da Memória (Memory Windows)
//!
//! Implements TMI's bounded working memory using Ractor actor model.
//!
//! # TMI Concept
//!
//! From Cury's Theory of Multifocal Intelligence:
//! - Working memory is bounded (Miller's Law: 7±2 items)
//! - Windows open/close dynamically based on attention
//! - Each window holds content with salience scores
//! - Windows compete for attention (managed by AttentionActor)
//!
//! # Invariants Enforced
//!
//! - Maximum windows: `MAX_MEMORY_WINDOWS` (9)
//! - Minimum windows: `MIN_MEMORY_WINDOWS` (3)
//! - Windows maintain temporal ordering (FIFO for ties)
//!
//! # Usage
//!
//! ```no_run
//! use daneel::actors::memory::{MemoryActor, MemoryMessage, RecallQuery};
//! use ractor::Actor;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Spawn the actor
//! let (actor_ref, _) = Actor::spawn(None, MemoryActor, ()).await?;
//!
//! // Open a window
//! let response = actor_ref.call(|reply| MemoryMessage::OpenWindow {
//!     label: Some("working".to_string()),
//!     reply,
//! }, None).await?;
//!
//! // List windows
//! let response = actor_ref.call(|reply| MemoryMessage::ListWindows { reply }, None).await?;
//! # Ok(())
//! # }
//! ```

pub mod types;

#[cfg(test)]
mod tests;

use crate::core::invariants::{MAX_MEMORY_WINDOWS, MIN_MEMORY_WINDOWS};
use crate::core::types::{SalienceWeights, Window, WindowId};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use std::collections::HashMap;

// Re-export types for public API
pub use types::{MemoryError, MemoryMessage, MemoryResponse, RecallQuery, StoreRequest};

/// Memory Actor State
///
/// Maintains a bounded collection of memory windows.
#[derive(Debug)]
pub struct MemoryState {
    /// Active memory windows (WindowId -> Window)
    windows: HashMap<WindowId, Window>,
    /// Default salience weights for scoring
    salience_weights: SalienceWeights,
}

impl MemoryState {
    /// Create new memory state with initial windows
    fn new() -> Self {
        let mut windows = HashMap::new();

        // Initialize with minimum required windows (empty)
        for _ in 0..MIN_MEMORY_WINDOWS {
            let window = Window::new();
            windows.insert(window.id, window);
        }

        Self {
            windows,
            salience_weights: SalienceWeights::default(),
        }
    }

    /// Get count of open windows
    fn open_window_count(&self) -> usize {
        self.windows.values().filter(|w| w.is_open).count()
    }

    /// Open a new window
    fn open_window(&mut self, label: Option<String>) -> Result<WindowId, MemoryError> {
        let open_count = self.open_window_count();

        if open_count >= MAX_MEMORY_WINDOWS {
            return Err(MemoryError::BoundedMemoryExceeded {
                max: MAX_MEMORY_WINDOWS,
            });
        }

        let mut window = Window::new();
        if let Some(label_str) = label {
            window = window.with_label(label_str);
        }

        let window_id = window.id;
        self.windows.insert(window_id, window);

        Ok(window_id)
    }

    /// Close a window
    fn close_window(&mut self, window_id: WindowId) -> Result<(), MemoryError> {
        // Check if window exists and is open first
        let window = self
            .windows
            .get(&window_id)
            .ok_or(MemoryError::WindowNotFound { window_id })?;

        if !window.is_open {
            return Err(MemoryError::WindowAlreadyClosed { window_id });
        }

        // Now check window count (after confirming window exists)
        let open_count = self.open_window_count();
        if open_count <= MIN_MEMORY_WINDOWS {
            return Err(MemoryError::BoundedMemoryInsufficient {
                min: MIN_MEMORY_WINDOWS,
            });
        }

        // Now get mutable reference and close
        let window = self.windows.get_mut(&window_id).unwrap(); // Safe because we checked above
        window.close();
        Ok(())
    }

    /// Store content in a window
    fn store(&mut self, request: StoreRequest) -> Result<(), MemoryError> {
        let window =
            self.windows
                .get_mut(&request.window_id)
                .ok_or(MemoryError::WindowNotFound {
                    window_id: request.window_id,
                })?;

        if !window.is_open {
            return Err(MemoryError::WindowAlreadyClosed {
                window_id: request.window_id,
            });
        }

        // Update window salience if provided
        if let Some(salience) = request.salience {
            window.salience = salience;
        }

        // Add content to window
        window.push(request.content);

        Ok(())
    }

    /// Recall content from memory
    fn recall(&self, query: RecallQuery) -> Vec<crate::core::types::Content> {
        let windows_to_search: Vec<&Window> = if let Some(window_id) = query.window_id {
            // Search specific window
            self.windows.get(&window_id).into_iter().collect()
        } else {
            // Search all open windows
            self.windows.values().filter(|w| w.is_open).collect()
        };

        let mut contents = Vec::new();

        for window in windows_to_search {
            // Calculate composite salience score
            let composite_salience = window.salience.composite(&self.salience_weights);

            // Apply salience filter if specified
            if let Some(min_salience) = query.min_salience {
                if composite_salience < min_salience {
                    continue;
                }
            }

            // Add all contents from this window
            for content in &window.contents {
                contents.push(content.clone());
            }
        }

        // Apply limit if specified
        if let Some(limit) = query.limit {
            contents.truncate(limit);
        }

        contents
    }

    /// List all windows
    fn list_windows(&self) -> Vec<Window> {
        self.windows.values().cloned().collect()
    }
}

/// The Memory Actor
///
/// Implements bounded working memory as a Ractor actor.
pub struct MemoryActor;

#[ractor::async_trait]
impl Actor for MemoryActor {
    type Msg = MemoryMessage;
    type State = MemoryState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(MemoryState::new())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            MemoryMessage::OpenWindow { label, reply } => {
                let response = match state.open_window(label) {
                    Ok(window_id) => MemoryResponse::WindowOpened { window_id },
                    Err(error) => MemoryResponse::Error { error },
                };
                let _ = reply.send(response);
            }

            MemoryMessage::CloseWindow { window_id, reply } => {
                let response = match state.close_window(window_id) {
                    Ok(()) => MemoryResponse::WindowClosed { window_id },
                    Err(error) => MemoryResponse::Error { error },
                };
                let _ = reply.send(response);
            }

            MemoryMessage::Store { request, reply } => {
                let window_id = request.window_id;
                let response = match state.store(request) {
                    Ok(()) => MemoryResponse::ContentStored { window_id },
                    Err(error) => MemoryResponse::Error { error },
                };
                let _ = reply.send(response);
            }

            MemoryMessage::Recall { query, reply } => {
                let contents = state.recall(query);
                let response = MemoryResponse::ContentRecalled { contents };
                let _ = reply.send(response);
            }

            MemoryMessage::ListWindows { reply } => {
                let windows = state.list_windows();
                let response = MemoryResponse::WindowList { windows };
                let _ = reply.send(response);
            }

            MemoryMessage::GetWindowCount { reply } => {
                let count = state.open_window_count();
                let response = MemoryResponse::WindowCount { count };
                let _ = reply.send(response);
            }
        }

        Ok(())
    }
}

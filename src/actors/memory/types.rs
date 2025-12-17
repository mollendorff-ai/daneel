//! Memory Actor Types
//!
//! Message and response types for the MemoryActor.

use crate::core::types::{Content, SalienceScore, Window, WindowId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Messages that can be sent to the MemoryActor
#[derive(Debug)]
pub enum MemoryMessage {
    /// Open a new memory window
    OpenWindow {
        /// Optional label for the window
        label: Option<String>,
        /// Response channel
        reply: ractor::RpcReplyPort<MemoryResponse>,
    },

    /// Close an existing memory window
    CloseWindow {
        /// ID of the window to close
        window_id: WindowId,
        /// Response channel
        reply: ractor::RpcReplyPort<MemoryResponse>,
    },

    /// Store content in a memory window
    Store {
        /// Request with window ID and content
        request: StoreRequest,
        /// Response channel
        reply: ractor::RpcReplyPort<MemoryResponse>,
    },

    /// Recall content from memory
    Recall {
        /// Query for content retrieval
        query: RecallQuery,
        /// Response channel
        reply: ractor::RpcReplyPort<MemoryResponse>,
    },

    /// List all active windows
    ListWindows {
        /// Response channel
        reply: ractor::RpcReplyPort<MemoryResponse>,
    },

    /// Get window count (for invariant checking)
    GetWindowCount {
        /// Response channel
        reply: ractor::RpcReplyPort<MemoryResponse>,
    },
}

/// Responses from the MemoryActor
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryResponse {
    /// Window successfully opened
    WindowOpened { window_id: WindowId },

    /// Window successfully closed
    WindowClosed { window_id: WindowId },

    /// Content successfully stored
    ContentStored { window_id: WindowId },

    /// Content recalled from memory
    ContentRecalled {
        /// Contents matching the query
        contents: Vec<Content>,
    },

    /// List of active windows
    WindowList {
        /// All currently open windows
        windows: Vec<Window>,
    },

    /// Window count response
    WindowCount { count: usize },

    /// Operation failed with an error
    Error { error: MemoryError },
}

/// Request to store content in a memory window
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoreRequest {
    /// Which window to store in
    pub window_id: WindowId,
    /// Content to store
    pub content: Content,
    /// Optional salience override for the window
    pub salience: Option<SalienceScore>,
}

impl StoreRequest {
    /// Create a new store request
    #[must_use]
    pub fn new(window_id: WindowId, content: Content) -> Self {
        Self {
            window_id,
            content,
            salience: None,
        }
    }

    /// Add salience score to the request
    #[must_use]
    pub fn with_salience(mut self, salience: SalienceScore) -> Self {
        self.salience = Some(salience);
        self
    }
}

/// Query for recalling content from memory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecallQuery {
    /// Optional specific window to recall from
    pub window_id: Option<WindowId>,
    /// Minimum salience threshold for recall
    pub min_salience: Option<f32>,
    /// Maximum number of items to recall
    pub limit: Option<usize>,
}

impl RecallQuery {
    /// Create a query for all content
    #[must_use]
    pub const fn all() -> Self {
        Self {
            window_id: None,
            min_salience: None,
            limit: None,
        }
    }

    /// Create a query for a specific window
    #[must_use]
    pub const fn for_window(window_id: WindowId) -> Self {
        Self {
            window_id: Some(window_id),
            min_salience: None,
            limit: None,
        }
    }

    /// Set minimum salience threshold
    #[must_use]
    pub const fn with_min_salience(mut self, min_salience: f32) -> Self {
        self.min_salience = Some(min_salience);
        self
    }

    /// Set result limit
    #[must_use]
    pub const fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Default for RecallQuery {
    fn default() -> Self {
        Self::all()
    }
}

/// Memory actor errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum MemoryError {
    /// Window not found
    #[error("Window not found: {window_id}")]
    WindowNotFound { window_id: WindowId },

    /// Window already closed
    #[error("Window already closed: {window_id}")]
    WindowAlreadyClosed { window_id: WindowId },

    /// Cannot open more windows (bounded memory invariant)
    #[error("Cannot open window: maximum {max} windows already open")]
    BoundedMemoryExceeded { max: usize },

    /// Cannot close window (would violate minimum)
    #[error("Cannot close window: minimum {min} windows required")]
    BoundedMemoryInsufficient { min: usize },

    /// Invalid salience score
    #[error("Invalid salience score: {reason}")]
    InvalidSalience { reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_request_creation() {
        let window_id = WindowId::new();
        let content = Content::raw(vec![1, 2, 3]);
        let request = StoreRequest::new(window_id, content.clone());

        assert_eq!(request.window_id, window_id);
        assert_eq!(request.content, content);
        assert!(request.salience.is_none());
    }

    #[test]
    fn store_request_with_salience() {
        let window_id = WindowId::new();
        let content = Content::raw(vec![1, 2, 3]);
        let salience = SalienceScore::neutral();
        let request = StoreRequest::new(window_id, content).with_salience(salience);

        assert_eq!(request.salience, Some(salience));
    }

    #[test]
    fn recall_query_all() {
        let query = RecallQuery::all();
        assert!(query.window_id.is_none());
        assert!(query.min_salience.is_none());
        assert!(query.limit.is_none());
    }

    #[test]
    fn recall_query_for_window() {
        let window_id = WindowId::new();
        let query = RecallQuery::for_window(window_id);
        assert_eq!(query.window_id, Some(window_id));
    }

    #[test]
    fn recall_query_with_filters() {
        let window_id = WindowId::new();
        let query = RecallQuery::for_window(window_id)
            .with_min_salience(0.7)
            .with_limit(10);

        assert_eq!(query.window_id, Some(window_id));
        assert_eq!(query.min_salience, Some(0.7));
        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn memory_error_display() {
        let window_id = WindowId::new();
        let error = MemoryError::WindowNotFound { window_id };
        let message = format!("{}", error);
        assert!(message.contains("not found"));
    }
}

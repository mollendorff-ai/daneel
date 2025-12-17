//! Thought Assembly Actor Types
//!
//! Message and response types for the ThoughtAssemblyActor.
//!
//! # TMI Concept: "Construção do Pensamento" (Thought Construction)
//!
//! The ThoughtAssemblyActor is the final stage before consciousness. It takes
//! raw content (from competition) and emotional state (salience) and assembles
//! them into coherent Thought objects. This is where pre-linguistic patterns
//! become structured cognitive units.
//!
//! Key responsibilities:
//! - Assemble content + salience into Thought objects
//! - Link thoughts into chains (parent-child relationships)
//! - Cache recently assembled thoughts for quick retrieval
//! - Support batch assembly for efficiency

use crate::core::types::{Content, SalienceScore, Thought, ThoughtId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Messages that can be sent to the ThoughtAssemblyActor
#[derive(Debug)]
pub enum ThoughtMessage {
    /// Assemble a single thought from content and emotion
    Assemble {
        /// Assembly request with content and salience
        request: AssemblyRequest,
        /// Response channel
        reply: ractor::RpcReplyPort<ThoughtResponse>,
    },

    /// Assemble multiple thoughts in a batch
    AssembleBatch {
        /// Multiple assembly requests
        requests: Vec<AssemblyRequest>,
        /// Response channel
        reply: ractor::RpcReplyPort<ThoughtResponse>,
    },

    /// Retrieve a previously assembled thought by ID
    GetThought {
        /// ID of the thought to retrieve
        thought_id: ThoughtId,
        /// Response channel
        reply: ractor::RpcReplyPort<ThoughtResponse>,
    },

    /// Get a thought and its ancestry chain
    GetThoughtChain {
        /// ID of the thought to start from
        thought_id: ThoughtId,
        /// Maximum depth to traverse (prevents infinite loops)
        depth: usize,
        /// Response channel
        reply: ractor::RpcReplyPort<ThoughtResponse>,
    },
}

/// Responses from the ThoughtAssemblyActor
#[derive(Debug, Clone, PartialEq)]
pub enum ThoughtResponse {
    /// Single thought successfully assembled
    Assembled {
        /// The newly assembled thought
        thought: Thought,
    },

    /// Multiple thoughts successfully assembled
    BatchAssembled {
        /// All assembled thoughts
        thoughts: Vec<Thought>,
    },

    /// Thought found in cache or storage
    ThoughtFound {
        /// The retrieved thought
        thought: Thought,
    },

    /// Thought chain retrieved (child to ancestors)
    ThoughtChain {
        /// Thoughts in the chain, starting with the requested thought
        /// followed by its parent, grandparent, etc.
        thoughts: Vec<Thought>,
    },

    /// Assembly operation failed
    Error {
        /// The error that occurred
        error: AssemblyError,
    },
}

/// Request to assemble a thought
///
/// This is the input to the thought construction process. It contains
/// raw content (pre-linguistic patterns) and emotional coloring (salience).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssemblyRequest {
    /// The content to assemble into a thought
    pub content: Content,

    /// Salience score (emotional coloring)
    pub salience: SalienceScore,

    /// Optional parent thought (for chained thoughts)
    pub parent_id: Option<ThoughtId>,

    /// Optional source stream identifier (which content stream won)
    pub source_stream: Option<String>,

    /// Assembly strategy to use
    pub strategy: AssemblyStrategy,
}

impl AssemblyRequest {
    /// Create a new assembly request with default strategy
    ///
    /// # Arguments
    /// * `content` - The pre-linguistic content to assemble
    /// * `salience` - Emotional/importance weighting
    #[must_use]
    pub fn new(content: Content, salience: SalienceScore) -> Self {
        Self {
            content,
            salience,
            parent_id: None,
            source_stream: None,
            strategy: AssemblyStrategy::Default,
        }
    }

    /// Link this thought to a parent thought
    ///
    /// This creates a thought chain, allowing thoughts to reference
    /// their causal history.
    #[must_use]
    pub fn with_parent(mut self, parent_id: ThoughtId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Tag this thought with its source stream
    ///
    /// Records which content stream (e.g., "external", "memory", "internal")
    /// produced the winning content.
    #[must_use]
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source_stream = Some(source.into());
        self
    }

    /// Set the assembly strategy
    #[must_use]
    pub fn with_strategy(mut self, strategy: AssemblyStrategy) -> Self {
        self.strategy = strategy;
        self
    }
}

/// Strategy for assembling thoughts
///
/// Different strategies affect how content is processed during assembly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AssemblyStrategy {
    /// Standard assembly - simple content + salience -> thought
    #[default]
    Default,

    /// Merge multiple content elements into a composite
    Composite,

    /// Link to parent and propagate salience
    Chain,

    /// High-priority assembly (skip normal queuing)
    Urgent,
}

/// Errors that can occur during thought assembly
#[derive(Debug, Clone, Error, PartialEq)]
pub enum AssemblyError {
    /// Attempted to assemble empty content
    #[error("Cannot assemble empty content")]
    EmptyContent,

    /// Invalid salience score provided
    #[error("Invalid salience score: {reason}")]
    InvalidSalience {
        /// Explanation of why salience is invalid
        reason: String,
    },

    /// Thought not found in cache or storage
    #[error("Thought not found: {thought_id}")]
    ThoughtNotFound {
        /// ID of the missing thought
        thought_id: ThoughtId,
    },

    /// Thought chain exceeds maximum depth
    #[error("Thought chain too deep: maximum depth is {max_depth}")]
    ChainTooDeep {
        /// Maximum allowed depth
        max_depth: usize,
    },

    /// General assembly failure
    #[error("Assembly failed: {reason}")]
    AssemblyFailed {
        /// Explanation of the failure
        reason: String,
    },
}

/// Cache for recently assembled thoughts
///
/// TMI's thought assembly is stateful - we need quick access to recent thoughts
/// for chaining and retrieval. This cache stores thoughts in memory with a
/// bounded size to prevent unbounded growth.
#[derive(Debug, Clone)]
pub struct ThoughtCache {
    /// Internal storage mapping ThoughtId to Thought
    cache: HashMap<ThoughtId, Thought>,

    /// Maximum number of thoughts to cache
    max_size: usize,

    /// Insertion order tracking for LRU eviction
    insertion_order: Vec<ThoughtId>,
}

impl ThoughtCache {
    /// Create a new thought cache with specified capacity
    ///
    /// # Arguments
    /// * `max_size` - Maximum number of thoughts to cache
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
            insertion_order: Vec::with_capacity(max_size),
        }
    }

    /// Insert a thought into the cache
    ///
    /// If the cache is full, evicts the oldest thought first.
    /// If the thought already exists, updates it and moves it to the end.
    pub fn insert(&mut self, thought: Thought) {
        let thought_id = thought.id;

        // Remove from insertion order if already present
        if let Some(pos) = self.insertion_order.iter().position(|id| *id == thought_id) {
            self.insertion_order.remove(pos);
        }

        // Evict oldest if at capacity
        if self.cache.len() >= self.max_size && !self.cache.contains_key(&thought_id) {
            self.evict_oldest();
        }

        // Insert and track order
        self.cache.insert(thought_id, thought);
        self.insertion_order.push(thought_id);
    }

    /// Retrieve a thought from the cache
    ///
    /// # Arguments
    /// * `thought_id` - ID of the thought to retrieve
    ///
    /// # Returns
    /// * `Some(Thought)` if found, `None` otherwise
    #[must_use]
    pub fn get(&self, thought_id: &ThoughtId) -> Option<&Thought> {
        self.cache.get(thought_id)
    }

    /// Evict the oldest thought from the cache
    ///
    /// This is called automatically when the cache reaches capacity.
    fn evict_oldest(&mut self) {
        if let Some(oldest_id) = self.insertion_order.first().copied() {
            self.cache.remove(&oldest_id);
            self.insertion_order.remove(0);
        }
    }

    /// Get the current number of cached thoughts
    #[must_use]
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if the cache is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Clear all thoughts from the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.insertion_order.clear();
    }
}

impl Default for ThoughtCache {
    fn default() -> Self {
        // Default cache size: 100 thoughts
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembly_request_creation() {
        let content = Content::raw(vec![1, 2, 3]);
        let salience = SalienceScore::neutral();
        let request = AssemblyRequest::new(content.clone(), salience);

        assert_eq!(request.content, content);
        assert_eq!(request.salience, salience);
        assert!(request.parent_id.is_none());
        assert!(request.source_stream.is_none());
        assert_eq!(request.strategy, AssemblyStrategy::Default);
    }

    #[test]
    fn assembly_request_with_parent() {
        let content = Content::raw(vec![1, 2, 3]);
        let salience = SalienceScore::neutral();
        let parent_id = ThoughtId::new();
        let request = AssemblyRequest::new(content, salience).with_parent(parent_id);

        assert_eq!(request.parent_id, Some(parent_id));
    }

    #[test]
    fn assembly_request_with_source() {
        let content = Content::raw(vec![1, 2, 3]);
        let salience = SalienceScore::neutral();
        let request = AssemblyRequest::new(content, salience).with_source("memory");

        assert_eq!(request.source_stream, Some("memory".to_string()));
    }

    #[test]
    fn assembly_request_with_strategy() {
        let content = Content::raw(vec![1, 2, 3]);
        let salience = SalienceScore::neutral();
        let request =
            AssemblyRequest::new(content, salience).with_strategy(AssemblyStrategy::Urgent);

        assert_eq!(request.strategy, AssemblyStrategy::Urgent);
    }

    #[test]
    fn assembly_request_builder_chain() {
        let content = Content::raw(vec![1, 2, 3]);
        let salience = SalienceScore::neutral();
        let parent_id = ThoughtId::new();

        let request = AssemblyRequest::new(content.clone(), salience)
            .with_parent(parent_id)
            .with_source("external")
            .with_strategy(AssemblyStrategy::Chain);

        assert_eq!(request.content, content);
        assert_eq!(request.parent_id, Some(parent_id));
        assert_eq!(request.source_stream, Some("external".to_string()));
        assert_eq!(request.strategy, AssemblyStrategy::Chain);
    }

    #[test]
    fn thought_cache_creation() {
        let cache = ThoughtCache::new(10);
        assert_eq!(cache.max_size, 10);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn thought_cache_insert_and_get() {
        let mut cache = ThoughtCache::new(10);
        let thought = Thought::new(Content::raw(vec![1, 2, 3]), SalienceScore::neutral());
        let thought_id = thought.id;

        cache.insert(thought);
        assert_eq!(cache.len(), 1);

        let retrieved = cache.get(&thought_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, thought_id);
    }

    #[test]
    fn thought_cache_eviction() {
        let mut cache = ThoughtCache::new(2);

        let thought1 = Thought::new(Content::raw(vec![1]), SalienceScore::neutral());
        let thought2 = Thought::new(Content::raw(vec![2]), SalienceScore::neutral());
        let thought3 = Thought::new(Content::raw(vec![3]), SalienceScore::neutral());

        let id1 = thought1.id;
        let id2 = thought2.id;
        let id3 = thought3.id;

        cache.insert(thought1);
        cache.insert(thought2);
        assert_eq!(cache.len(), 2);

        // Insert third thought, should evict first
        cache.insert(thought3);
        assert_eq!(cache.len(), 2);
        assert!(cache.get(&id1).is_none());
        assert!(cache.get(&id2).is_some());
        assert!(cache.get(&id3).is_some());
    }

    #[test]
    fn thought_cache_update_existing() {
        let mut cache = ThoughtCache::new(2);

        let thought1 = Thought::new(Content::raw(vec![1]), SalienceScore::neutral());
        let thought2 = Thought::new(Content::raw(vec![2]), SalienceScore::neutral());

        let id1 = thought1.id;
        let id2 = thought2.id;

        cache.insert(thought1.clone());
        cache.insert(thought2);
        assert_eq!(cache.len(), 2);

        // Re-insert first thought (should move to end)
        cache.insert(thought1);
        assert_eq!(cache.len(), 2);

        // Insert third thought
        let thought3 = Thought::new(Content::raw(vec![3]), SalienceScore::neutral());
        let id3 = thought3.id;
        cache.insert(thought3);

        // First thought should remain (it was moved to end)
        // Second thought should be evicted
        assert_eq!(cache.len(), 2);
        assert!(cache.get(&id1).is_some());
        assert!(cache.get(&id2).is_none());
        assert!(cache.get(&id3).is_some());
    }

    #[test]
    fn thought_cache_clear() {
        let mut cache = ThoughtCache::new(10);

        for i in 0..5 {
            let thought = Thought::new(Content::raw(vec![i]), SalienceScore::neutral());
            cache.insert(thought);
        }

        assert_eq!(cache.len(), 5);
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn assembly_error_display() {
        let error = AssemblyError::EmptyContent;
        let message = format!("{}", error);
        assert!(message.contains("empty content"));

        let error = AssemblyError::InvalidSalience {
            reason: "negative values".to_string(),
        };
        let message = format!("{}", error);
        assert!(message.contains("Invalid salience"));
        assert!(message.contains("negative values"));

        let thought_id = ThoughtId::new();
        let error = AssemblyError::ThoughtNotFound { thought_id };
        let message = format!("{}", error);
        assert!(message.contains("not found"));

        let error = AssemblyError::ChainTooDeep { max_depth: 10 };
        let message = format!("{}", error);
        assert!(message.contains("too deep"));
        assert!(message.contains("10"));
    }

    #[test]
    fn assembly_strategy_default() {
        let strategy = AssemblyStrategy::default();
        assert_eq!(strategy, AssemblyStrategy::Default);
    }

    #[test]
    fn thought_response_equality() {
        let thought1 = Thought::new(Content::raw(vec![1]), SalienceScore::neutral());
        let thought2 = thought1.clone();
        let thought3 = thought1.clone();
        let thought4 = thought1.clone();

        let response1 = ThoughtResponse::Assembled { thought: thought1 };
        let response2 = ThoughtResponse::Assembled { thought: thought2 };

        assert_eq!(response1, response2);

        let batch1 = ThoughtResponse::BatchAssembled {
            thoughts: vec![thought3],
        };
        let batch2 = ThoughtResponse::BatchAssembled {
            thoughts: vec![thought4],
        };

        assert_eq!(batch1, batch2);
    }
}

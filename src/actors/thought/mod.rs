//! ThoughtAssemblyActor - Construção do Pensamento (Thought Construction)
//!
//! Implements TMI's thought assembly stage where pre-linguistic content
//! becomes structured cognitive units.
//!
//! # TMI Concept: "Construção do Pensamento"
//!
//! From Cury's Theory of Multifocal Intelligence:
//! - Thoughts are assembled from content + emotional state (salience)
//! - Assembly happens BEFORE language (pre-linguistic)
//! - Thoughts link into chains (parent-child relationships)
//! - Each thought captures its source stream (which content won)
//!
//! The ThoughtAssemblyActor is the final stage before consciousness. It takes
//! raw content (from competition) and emotional coloring (salience) and
//! assembles them into coherent Thought objects.
//!
//! # Key Responsibilities
//!
//! - **Assembly**: Convert AssemblyRequest -> Thought
//! - **Caching**: Store recently assembled thoughts for quick retrieval
//! - **Chaining**: Link thoughts to their parents (causal history)
//! - **Strategy**: Support different assembly strategies (Default, Composite, Chain, Urgent)
//! - **Validation**: Optional salience score validation
//!
//! # Usage
//!
//! ```no_run
//! use daneel::actors::thought::{ThoughtAssemblyActor, ThoughtMessage, AssemblyRequest, AssemblyConfig};
//! use daneel::core::types::{Content, SalienceScore};
//! use ractor::Actor;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Spawn the actor with custom config
//! let config = AssemblyConfig::default();
//! let (actor_ref, _) = Actor::spawn(None, ThoughtAssemblyActor, config).await?;
//!
//! // Assemble a thought
//! let content = Content::raw(vec![1, 2, 3]);
//! let salience = SalienceScore::neutral();
//! let request = AssemblyRequest::new(content, salience);
//!
//! let response = actor_ref.call(|reply| ThoughtMessage::Assemble {
//!     request,
//!     reply,
//! }, None).await?;
//! # Ok(())
//! # }
//! ```

pub mod types;

// TODO: Create tests module
// #[cfg(test)]
// mod tests;

use crate::core::types::{SalienceScore, Thought, ThoughtId};
use ractor::{Actor, ActorProcessingErr, ActorRef};

// Re-export types for public API
pub use types::{
    AssemblyError, AssemblyRequest, AssemblyStrategy, ThoughtCache, ThoughtMessage, ThoughtResponse,
};

/// Configuration for the ThoughtAssemblyActor
///
/// Controls caching, validation, and chain traversal behavior.
#[derive(Debug, Clone)]
pub struct AssemblyConfig {
    /// Maximum number of thoughts to cache in memory
    pub cache_size: usize,

    /// Maximum depth when traversing parent chains
    /// Prevents infinite loops and stack overflow
    pub max_chain_depth: usize,

    /// Whether to validate salience scores during assembly
    /// When true, checks that all components are in valid ranges
    pub validate_salience: bool,
}

impl AssemblyConfig {
    /// Create a new assembly configuration
    #[must_use]
    pub const fn new(cache_size: usize, max_chain_depth: usize, validate_salience: bool) -> Self {
        Self {
            cache_size,
            max_chain_depth,
            validate_salience,
        }
    }
}

impl Default for AssemblyConfig {
    fn default() -> Self {
        Self {
            cache_size: 100,         // Cache last 100 thoughts
            max_chain_depth: 50,     // Traverse up to 50 parent links
            validate_salience: true, // Validate by default
        }
    }
}

/// Internal state for the ThoughtAssemblyActor
///
/// Maintains cache and statistics for thought assembly.
#[derive(Debug)]
pub struct ThoughtState {
    /// Cache of recently assembled thoughts
    cache: ThoughtCache,

    /// Total number of thoughts assembled (lifetime counter)
    assembly_count: u64,

    /// Configuration for this actor instance
    config: AssemblyConfig,
}

impl ThoughtState {
    /// Create new thought state with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(AssemblyConfig::default())
    }

    /// Create new thought state with custom configuration
    #[must_use]
    pub fn with_config(config: AssemblyConfig) -> Self {
        Self {
            cache: ThoughtCache::new(config.cache_size),
            assembly_count: 0,
            config,
        }
    }

    /// Assemble a single thought from a request
    ///
    /// This is the core assembly logic that converts raw content + salience
    /// into a structured Thought object.
    fn assemble_thought(&mut self, request: AssemblyRequest) -> Result<Thought, AssemblyError> {
        // Validate content is not empty
        if request.content.is_empty() {
            return Err(AssemblyError::EmptyContent);
        }

        // Validate salience if configured
        if self.config.validate_salience {
            self.validate_salience(&request.salience)?;
        }

        // Create base thought
        let mut thought = Thought::new(request.content, request.salience);

        // Link to parent if specified
        if let Some(parent_id) = request.parent_id {
            thought = thought.with_parent(parent_id);
        }

        // Tag with source stream if specified
        if let Some(source) = request.source_stream {
            thought = thought.with_source(source);
        }

        // Apply strategy-specific processing
        self.apply_strategy(&mut thought, &request.strategy)?;

        // Cache the assembled thought
        self.cache.insert(thought.clone());

        // Increment counter
        self.assembly_count += 1;

        Ok(thought)
    }

    /// Assemble multiple thoughts in batch
    ///
    /// More efficient than individual assembly for bulk operations.
    fn assemble_batch(
        &mut self,
        requests: Vec<AssemblyRequest>,
    ) -> Result<Vec<Thought>, AssemblyError> {
        let mut thoughts = Vec::with_capacity(requests.len());

        for request in requests {
            let thought = self.assemble_thought(request)?;
            thoughts.push(thought);
        }

        Ok(thoughts)
    }

    /// Retrieve a thought from the cache
    fn get_thought(&self, thought_id: &ThoughtId) -> Result<Thought, AssemblyError> {
        self.cache
            .get(thought_id)
            .cloned()
            .ok_or(AssemblyError::ThoughtNotFound {
                thought_id: *thought_id,
            })
    }

    /// Get a thought chain (thought + its ancestry)
    ///
    /// Walks up the parent chain from the given thought, up to max_depth.
    fn get_thought_chain(
        &self,
        thought_id: ThoughtId,
        depth: usize,
    ) -> Result<Vec<Thought>, AssemblyError> {
        // Validate depth against configured maximum
        if depth > self.config.max_chain_depth {
            return Err(AssemblyError::ChainTooDeep {
                max_depth: self.config.max_chain_depth,
            });
        }

        let mut chain = Vec::new();
        let mut current_id = thought_id;
        let mut remaining_depth = depth;

        // Walk up the chain
        while remaining_depth > 0 {
            // Get current thought
            let thought = self.get_thought(&current_id)?;

            // Add to chain
            chain.push(thought.clone());

            // Check for parent
            match thought.parent_id {
                Some(parent_id) => {
                    current_id = parent_id;
                    remaining_depth -= 1;
                }
                None => break, // Reached root of chain
            }
        }

        Ok(chain)
    }

    /// Validate salience score components
    #[allow(clippy::unused_self)] // Will use self for config-based validation
    fn validate_salience(&self, salience: &SalienceScore) -> Result<(), AssemblyError> {
        // Check importance, novelty, relevance are in [0.0, 1.0]
        if !(0.0..=1.0).contains(&salience.importance) {
            return Err(AssemblyError::InvalidSalience {
                reason: format!("importance {} out of range [0.0, 1.0]", salience.importance),
            });
        }

        if !(0.0..=1.0).contains(&salience.novelty) {
            return Err(AssemblyError::InvalidSalience {
                reason: format!("novelty {} out of range [0.0, 1.0]", salience.novelty),
            });
        }

        if !(0.0..=1.0).contains(&salience.relevance) {
            return Err(AssemblyError::InvalidSalience {
                reason: format!("relevance {} out of range [0.0, 1.0]", salience.relevance),
            });
        }

        // Check valence is in [-1.0, 1.0]
        if !(-1.0..=1.0).contains(&salience.valence) {
            return Err(AssemblyError::InvalidSalience {
                reason: format!("valence {} out of range [-1.0, 1.0]", salience.valence),
            });
        }

        // Check connection_relevance is in [0.0, 1.0]
        if !(0.0..=1.0).contains(&salience.connection_relevance) {
            return Err(AssemblyError::InvalidSalience {
                reason: format!(
                    "connection_relevance {} out of range [0.0, 1.0]",
                    salience.connection_relevance
                ),
            });
        }

        Ok(())
    }

    /// Apply strategy-specific processing to a thought
    ///
    /// Different strategies modify thought assembly in different ways.
    #[allow(clippy::unnecessary_wraps)] // Will return errors in future strategies
    #[allow(clippy::needless_pass_by_ref_mut)] // Will mutate in future strategies
    fn apply_strategy(
        &mut self,
        thought: &mut Thought,
        strategy: &AssemblyStrategy,
    ) -> Result<(), AssemblyError> {
        match strategy {
            AssemblyStrategy::Default => {
                // No special processing for default strategy
                Ok(())
            }

            AssemblyStrategy::Composite => {
                // For composite, ensure content is appropriate
                // Could validate that content is Composite variant
                // For now, just pass through
                Ok(())
            }

            AssemblyStrategy::Chain => {
                // For chain strategy, could propagate parent's salience
                // or apply special linking logic
                // For now, the parent_id is already linked
                if let Some(parent_id) = thought.parent_id {
                    // Could retrieve parent and merge salience here
                    // For Wave 3, just ensure parent exists in cache
                    if self.cache.get(&parent_id).is_none() {
                        // Parent not in cache - not an error, just note it
                        // In production, might want to load from persistence
                    }
                }
                Ok(())
            }

            AssemblyStrategy::Urgent => {
                // Urgent thoughts get priority treatment
                // Could boost salience scores here
                // For now, just pass through (priority handled by caller)
                Ok(())
            }
        }
    }
}

impl Default for ThoughtState {
    fn default() -> Self {
        Self::new()
    }
}

/// The Thought Assembly Actor
///
/// Implements thought construction as a Ractor actor.
pub struct ThoughtAssemblyActor;

#[ractor::async_trait]
impl Actor for ThoughtAssemblyActor {
    type Msg = ThoughtMessage;
    type State = ThoughtState;
    type Arguments = AssemblyConfig;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(ThoughtState::with_config(args))
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            ThoughtMessage::Assemble { request, reply } => {
                let response = match state.assemble_thought(request) {
                    Ok(thought) => ThoughtResponse::Assembled { thought },
                    Err(error) => ThoughtResponse::Error { error },
                };
                let _ = reply.send(response);
            }

            ThoughtMessage::AssembleBatch { requests, reply } => {
                let response = match state.assemble_batch(requests) {
                    Ok(thoughts) => ThoughtResponse::BatchAssembled { thoughts },
                    Err(error) => ThoughtResponse::Error { error },
                };
                let _ = reply.send(response);
            }

            ThoughtMessage::GetThought { thought_id, reply } => {
                let response = match state.get_thought(&thought_id) {
                    Ok(thought) => ThoughtResponse::ThoughtFound { thought },
                    Err(error) => ThoughtResponse::Error { error },
                };
                let _ = reply.send(response);
            }

            ThoughtMessage::GetThoughtChain {
                thought_id,
                depth,
                reply,
            } => {
                let response = match state.get_thought_chain(thought_id, depth) {
                    Ok(thoughts) => ThoughtResponse::ThoughtChain { thoughts },
                    Err(error) => ThoughtResponse::Error { error },
                };
                let _ = reply.send(response);
            }
        }

        Ok(())
    }
}

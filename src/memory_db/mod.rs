//! Memory Database Module (ADR-021, ADR-022)
//!
//! Qdrant-based persistent storage for Timmy's long-term memory.
//!
//! # Architecture
//!
//! - Redis Streams: Ephemeral thought competition (ADR-020)
//! - Qdrant: Persistent memory storage (this module)
//!
//! # Collections
//!
//! - `memories`: Individual memory records with 768-dim context vectors
//! - `episodes`: Event boundaries (Door Syndrome segmentation)
//! - `identity`: Timmy's persistent self-concept (singleton)
//!
//! # Usage
//!
//! ```no_run
//! use daneel::memory_db::{MemoryDb, Memory, MemorySource};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let db = MemoryDb::connect("http://localhost:6334").await?;
//! db.init_collections().await?;
//!
//! let memory = Memory::new(
//!     "First conversation".to_string(),
//!     MemorySource::External { stimulus: "hello".to_string() },
//! );
//! db.store_memory(&memory, &[0.0; 768]).await?;
//! # Ok(())
//! # }
//! ```

pub mod types;

#[cfg(test)]
mod tests;

use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, ScrollPointsBuilder,
    SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::Qdrant;
use std::collections::HashMap;
use thiserror::Error;

pub use types::*;

/// Memory database errors
#[derive(Debug, Error)]
pub enum MemoryDbError {
    #[error("Qdrant error: {0}")]
    Qdrant(#[from] qdrant_client::QdrantError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Memory not found: {0}")]
    MemoryNotFound(MemoryId),

    #[error("Episode not found: {0}")]
    EpisodeNotFound(EpisodeId),

    #[error("Invalid vector dimension: expected {expected}, got {actual}")]
    InvalidVectorDimension { expected: usize, actual: usize },

    #[error("Collection not found: {0}")]
    CollectionNotFound(String),
}

/// Result type for memory database operations
pub type Result<T> = std::result::Result<T, MemoryDbError>;

/// Collection names
pub mod collections {
    pub const MEMORIES: &str = "memories";
    pub const EPISODES: &str = "episodes";
    pub const IDENTITY: &str = "identity";
    /// Unconscious memory (ADR-033): Archived low-salience thoughts
    /// TMI: "Nada se apaga" - nothing is erased, just made inaccessible
    pub const UNCONSCIOUS: &str = "unconscious";
}

/// Memory database client
///
/// Wraps Qdrant client with TMI-specific operations.
pub struct MemoryDb {
    client: Qdrant,
}

impl MemoryDb {
    /// Connect to Qdrant
    ///
    /// # Arguments
    ///
    /// * `url` - Qdrant gRPC URL (e.g., "<http://localhost:6334>")
    ///
    /// # Errors
    ///
    /// Returns error if connection fails.
    ///
    /// Note: Currently synchronous but async for API consistency with other db operations.
    #[allow(clippy::unused_async)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn connect(url: &str) -> Result<Self> {
        let client = Qdrant::from_url(url).build()?;
        Ok(Self { client })
    }

    /// Connect to Qdrant and initialize collections in one call
    ///
    /// This is a convenience method that combines `connect()` and `init_collections()`.
    /// Use this for quick setup during startup.
    ///
    /// # Arguments
    ///
    /// * `url` - Qdrant gRPC URL (e.g., "<http://localhost:6334>")
    ///
    /// # Errors
    ///
    /// Returns error if connection or collection creation fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn connect_and_init(url: &str) -> Result<Self> {
        let db = Self::connect(url).await?;
        db.init_collections().await?;
        Ok(db)
    }

    /// Initialize collections if they don't exist
    ///
    /// Creates:
    /// - `memories`: 768-dim vectors with cosine distance
    /// - `episodes`: 768-dim vectors with cosine distance
    /// - `identity`: 768-dim vectors (singleton)
    ///
    /// # Errors
    ///
    /// Returns error if collection creation fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn init_collections(&self) -> Result<()> {
        // Check and create memories collection
        if !self.collection_exists(collections::MEMORIES).await? {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(collections::MEMORIES).vectors_config(
                        VectorParamsBuilder::new(VECTOR_DIMENSION as u64, Distance::Cosine),
                    ),
                )
                .await?;
        }

        // Check and create episodes collection
        if !self.collection_exists(collections::EPISODES).await? {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(collections::EPISODES).vectors_config(
                        VectorParamsBuilder::new(VECTOR_DIMENSION as u64, Distance::Cosine),
                    ),
                )
                .await?;
        }

        // Check and create identity collection
        if !self.collection_exists(collections::IDENTITY).await? {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(collections::IDENTITY).vectors_config(
                        VectorParamsBuilder::new(VECTOR_DIMENSION as u64, Distance::Cosine),
                    ),
                )
                .await?;
        }

        // Check and create unconscious collection (ADR-033)
        // TMI: "Nada se apaga" - low-salience thoughts archived here
        if !self.collection_exists(collections::UNCONSCIOUS).await? {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(collections::UNCONSCIOUS).vectors_config(
                        VectorParamsBuilder::new(VECTOR_DIMENSION as u64, Distance::Cosine),
                    ),
                )
                .await?;
        }

        Ok(())
    }

    /// Check if a collection exists
    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn collection_exists(&self, name: &str) -> Result<bool> {
        match self.client.collection_exists(name).await {
            Ok(exists) => Ok(exists),
            Err(e) => Err(MemoryDbError::Qdrant(e)),
        }
    }

    /// Store a memory with its context vector
    ///
    /// # Arguments
    ///
    /// * `memory` - The memory to store
    /// * `vector` - 768-dim context embedding
    ///
    /// # Errors
    ///
    /// Returns error if vector dimension is wrong or storage fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn store_memory(&self, memory: &Memory, vector: &[f32]) -> Result<()> {
        if vector.len() != VECTOR_DIMENSION {
            return Err(MemoryDbError::InvalidVectorDimension {
                expected: VECTOR_DIMENSION,
                actual: vector.len(),
            });
        }

        let payload: HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::to_value(memory)?)?;
        let point = PointStruct::new(memory.id.0.to_string(), vector.to_vec(), payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(collections::MEMORIES, vec![point]))
            .await?;

        Ok(())
    }

    /// Find memories by context similarity (TMI's Gatilho da Memória)
    ///
    /// # Arguments
    ///
    /// * `context_vector` - Query vector (768-dim)
    /// * `episode_id` - Optional episode filter (Door Syndrome: same-episode = better access)
    /// * `limit` - Maximum number of results
    ///
    /// # Returns
    ///
    /// Vector of (memory, `similarity_score`) pairs, sorted by similarity descending.
    ///
    /// # Errors
    ///
    /// Returns error if vector dimension is wrong or search fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn find_by_context(
        &self,
        context_vector: &[f32],
        episode_id: Option<&EpisodeId>,
        limit: u64,
    ) -> Result<Vec<(Memory, f32)>> {
        if context_vector.len() != VECTOR_DIMENSION {
            return Err(MemoryDbError::InvalidVectorDimension {
                expected: VECTOR_DIMENSION,
                actual: context_vector.len(),
            });
        }

        let mut search =
            SearchPointsBuilder::new(collections::MEMORIES, context_vector.to_vec(), limit)
                .with_payload(true);

        // Apply episode filter if specified
        if let Some(ep_id) = episode_id {
            search = search.filter(Filter::must([Condition::matches(
                "episode_id",
                ep_id.0.to_string(),
            )]));
        }

        let results = self.client.search_points(search).await?;

        let mut memories = Vec::with_capacity(results.result.len());
        for point in results.result {
            let payload = point.payload;
            let memory: Memory = serde_json::from_value(serde_json::to_value(payload)?)?;
            memories.push((memory, point.score));
        }

        Ok(memories)
    }

    /// Get memories tagged for consolidation (sleep replay candidates)
    ///
    /// Returns memories with `consolidation_tag = true` and `strength < 0.9`,
    /// sorted by replay priority.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of results
    ///
    /// # Errors
    ///
    /// Returns error if Qdrant query fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn get_replay_candidates(&self, limit: u32) -> Result<Vec<Memory>> {
        use qdrant_client::qdrant::Range;

        // Filter: consolidation_tag = true AND strength < 0.9
        // Both filters in Qdrant query to avoid fetching permanent memories
        let filter = Filter::must([
            Condition::matches("consolidation.consolidation_tag", true),
            Condition::range(
                "consolidation.strength",
                Range {
                    lt: Some(0.9),
                    ..Default::default()
                },
            ),
        ]);

        let results = self
            .client
            .scroll(
                ScrollPointsBuilder::new(collections::MEMORIES)
                    .filter(filter)
                    .limit(limit)
                    .with_payload(true),
            )
            .await?;

        let mut memories: Vec<Memory> = results
            .result
            .into_iter()
            .filter_map(|point| {
                serde_json::from_value(serde_json::to_value(point.payload).ok()?).ok()
            })
            .collect();

        // Sort by replay priority (highest first)
        memories.sort_by(|a, b| {
            b.replay_priority()
                .partial_cmp(&a.replay_priority())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(memories)
    }

    /// Update memory consolidation state
    ///
    /// Called during sleep to strengthen memories.
    ///
    /// # Errors
    ///
    /// Returns error if memory not found or update fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn update_consolidation(
        &self,
        memory_id: &MemoryId,
        strength_delta: f32,
    ) -> Result<()> {
        // Get current memory
        let results = self
            .client
            .scroll(
                ScrollPointsBuilder::new(collections::MEMORIES)
                    .filter(Filter::must([Condition::matches(
                        "id",
                        memory_id.0.to_string(),
                    )]))
                    .limit(1)
                    .with_payload(true)
                    .with_vectors(true),
            )
            .await?;

        if results.result.is_empty() {
            return Err(MemoryDbError::MemoryNotFound(*memory_id));
        }

        let point = &results.result[0];
        let mut memory: Memory = serde_json::from_value(serde_json::to_value(&point.payload)?)?;

        // Update consolidation state
        memory.consolidation.strength = (memory.consolidation.strength + strength_delta).min(1.0);
        memory.consolidation.replay_count += 1;
        memory.consolidation.last_replayed = Some(chrono::Utc::now());

        // Get vector from point - handle nested Option structure
        #[allow(deprecated)] // VectorOutput.data deprecated in qdrant 1.16, but still functional
        let vector: Vec<f32> = point
            .vectors
            .as_ref()
            .and_then(|v| v.vectors_options.as_ref())
            .and_then(|opts| match opts {
                qdrant_client::qdrant::vectors_output::VectorsOptions::Vector(v) => {
                    Some(v.data.clone())
                }
                qdrant_client::qdrant::vectors_output::VectorsOptions::Vectors(_) => None,
            })
            .unwrap_or_else(|| vec![0.0; VECTOR_DIMENSION]);

        // Store updated memory
        self.store_memory(&memory, &vector).await
    }

    /// Store an episode
    ///
    /// # Errors
    ///
    /// Returns error if vector dimension is wrong or storage fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn store_episode(&self, episode: &Episode, vector: &[f32]) -> Result<()> {
        if vector.len() != VECTOR_DIMENSION {
            return Err(MemoryDbError::InvalidVectorDimension {
                expected: VECTOR_DIMENSION,
                actual: vector.len(),
            });
        }

        let payload: HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::to_value(episode)?)?;
        let point = PointStruct::new(episode.id.0.to_string(), vector.to_vec(), payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(collections::EPISODES, vec![point]))
            .await?;

        Ok(())
    }

    /// Get current (open) episode
    ///
    /// # Errors
    ///
    /// Returns error if Qdrant query fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn get_current_episode(&self) -> Result<Option<Episode>> {
        let filter = Filter::must([Condition::is_null("ended_at")]);

        let results = self
            .client
            .scroll(
                ScrollPointsBuilder::new(collections::EPISODES)
                    .filter(filter)
                    .limit(1)
                    .with_payload(true),
            )
            .await?;

        if let Some(point) = results.result.into_iter().next() {
            let episode: Episode = serde_json::from_value(serde_json::to_value(point.payload)?)?;
            Ok(Some(episode))
        } else {
            Ok(None)
        }
    }

    /// Close current episode and create new one (Door Syndrome boundary)
    ///
    /// # Errors
    ///
    /// Returns error if episode operations fail.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn create_episode_boundary(
        &self,
        label: String,
        boundary_type: BoundaryType,
        vector: &[f32],
    ) -> Result<Episode> {
        // Close current episode if exists
        if let Some(mut current) = self.get_current_episode().await? {
            current.close();
            // Re-store with same vector (we don't have it, use zeros)
            let zero_vector = vec![0.0; VECTOR_DIMENSION];
            self.store_episode(&current, &zero_vector).await?;
        }

        // Create new episode
        let episode = Episode::new(label, boundary_type);
        self.store_episode(&episode, vector).await?;

        Ok(episode)
    }

    /// Get total memory count
    ///
    /// # Errors
    ///
    /// Returns error if Qdrant query fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn memory_count(&self) -> Result<u64> {
        let info = self.client.collection_info(collections::MEMORIES).await?;
        Ok(info.result.and_then(|r| r.points_count).unwrap_or(0))
    }

    /// Get total episode count
    ///
    /// # Errors
    ///
    /// Returns error if Qdrant query fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn episode_count(&self) -> Result<u64> {
        let info = self.client.collection_info(collections::EPISODES).await?;
        Ok(info.result.and_then(|r| r.points_count).unwrap_or(0))
    }

    /// Get total unconscious memory count (ADR-033)
    ///
    /// # Errors
    ///
    /// Returns error if Qdrant query fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn unconscious_count(&self) -> Result<u64> {
        let info = self
            .client
            .collection_info(collections::UNCONSCIOUS)
            .await?;
        Ok(info.result.and_then(|r| r.points_count).unwrap_or(0))
    }

    /// Archive a low-salience thought to the unconscious (ADR-033)
    ///
    /// TMI: "Nada se apaga na memória" - Nothing is erased from memory.
    /// Instead of XDEL, we archive here first. The thought is still removed
    /// from Redis working memory, but preserved in the unconscious.
    ///
    /// # Arguments
    ///
    /// * `content` - Serialized thought content
    /// * `salience` - Composite salience when archived
    /// * `reason` - Why this thought is being archived
    /// * `redis_id` - Original Redis stream entry ID
    ///
    /// # Errors
    ///
    /// Returns error if Qdrant upsert fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn archive_to_unconscious(
        &self,
        content: &str,
        salience: f32,
        reason: ArchiveReason,
        redis_id: Option<&str>,
    ) -> Result<()> {
        let memory = UnconsciousMemory::from_forgotten_thought(
            content.to_string(),
            salience,
            reason,
            redis_id.map(String::from),
        );

        // Create payload from struct
        let payload: HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::to_value(&memory)?)?;

        // Use a zero vector for now - unconscious memories are not retrieved by similarity
        // Future: could embed with low-dimensional representation
        let vector = vec![0.0; VECTOR_DIMENSION];
        let point = PointStruct::new(memory.id.0.to_string(), vector, payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(
                collections::UNCONSCIOUS,
                vec![point],
            ))
            .await?;

        Ok(())
    }

    /// Load Timmy's identity metadata from Qdrant (ADR-034)
    ///
    /// Returns existing identity if found, or creates new identity for first boot.
    /// On restart, increments `restart_count` and updates `session_started_at`.
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn load_identity(&self) -> Result<IdentityMetadata> {
        use qdrant_client::qdrant::GetPointsBuilder;

        // Try to retrieve existing identity
        let result = self
            .client
            .get_points(
                GetPointsBuilder::new(
                    collections::IDENTITY,
                    vec![IDENTITY_RECORD_ID.to_string().into()],
                )
                .with_payload(true),
            )
            .await;

        match result {
            Ok(response) => {
                if let Some(point) = response.result.first() {
                    // Deserialize existing identity
                    let payload = &point.payload;
                    let json_value = serde_json::to_value(payload)?;
                    let mut identity: IdentityMetadata = serde_json::from_value(json_value)?;

                    // Record this restart
                    identity.record_restart();

                    Ok(identity)
                } else {
                    // No identity found - first boot ever
                    Ok(IdentityMetadata::new())
                }
            }
            Err(_) => {
                // Collection might not exist or other error - first boot
                Ok(IdentityMetadata::new())
            }
        }
    }

    /// Save Timmy's identity metadata to Qdrant (ADR-034)
    ///
    /// Called periodically and on shutdown to persist identity state.
    ///
    /// # Errors
    ///
    /// Returns error if Qdrant upsert fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn save_identity(&self, identity: &IdentityMetadata) -> Result<()> {
        // Create payload from struct
        let payload: HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::to_value(identity)?)?;

        // Use a zero vector - identity is retrieved by ID, not similarity
        let vector = vec![0.0; VECTOR_DIMENSION];
        let point = PointStruct::new(IDENTITY_RECORD_ID.to_string(), vector, payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(collections::IDENTITY, vec![point]))
            .await?;

        Ok(())
    }

    /// Health check
    ///
    /// # Errors
    ///
    /// Returns error if the check itself fails unexpectedly.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn health_check(&self) -> Result<bool> {
        match self.client.health_check().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl std::fmt::Debug for MemoryDb {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryDb").finish()
    }
}

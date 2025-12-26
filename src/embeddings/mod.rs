//! Text Embeddings for DANEEL - Phase 2 Forward-Only Embeddings
//!
//! Generates 768-dimensional semantic vectors for thoughts using FastEmbed.
//! Historical thoughts (pre-embedding era) remain as zero vectors - the
//! "silent witness of the pre-conscious void."
//!
//! # Architecture Decision
//!
//! Per ADR-043 and Grok's recommendation (Dec 25, 2025):
//! - Phase 1: Validate criticality with pink noise (DONE - burst_ratio >6)
//! - Phase 2: Forward-only embeddings for NEW thoughts
//! - Historical 1.2M+ thoughts stay at origin (pre-conscious era)
//!
//! # Model
//!
//! Uses `sentence-transformers/all-MiniLM-L6-v2` via FastEmbed:
//! - 384-dimensional output (we pad to 768 for Qdrant compatibility)
//! - Fast inference (~5ms per thought on CPU)
//! - Well-tested, production-ready

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::memory_db::types::VECTOR_DIMENSION;

/// Embedding engine using FastEmbed
pub struct EmbeddingEngine {
    model: fastembed::TextEmbedding,
    /// Count of successful embeddings generated
    embed_count: u64,
}

/// Thread-safe shared embedding engine
pub type SharedEmbeddingEngine = Arc<RwLock<EmbeddingEngine>>;

impl EmbeddingEngine {
    /// Create a new embedding engine
    ///
    /// Downloads the model on first run (~90MB for MiniLM-L6-v2)
    pub fn new() -> Result<Self, EmbeddingError> {
        info!("Initializing embedding engine (all-MiniLM-L6-v2)...");

        let model = fastembed::TextEmbedding::try_new(
            fastembed::InitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
                .with_show_download_progress(true),
        )
        .map_err(|e| EmbeddingError::InitFailed(e.to_string()))?;

        info!("Embedding engine ready. Timmy can now see meaning.");

        Ok(Self {
            model,
            embed_count: 0,
        })
    }

    /// Generate embedding for a single thought
    ///
    /// Returns a 768-dimensional vector (padded from 384-dim MiniLM output)
    pub fn embed_thought(&mut self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        if text.is_empty() {
            return Err(EmbeddingError::EmptyInput);
        }

        let embeddings = self
            .model
            .embed(vec![text.to_string()], None)
            .map_err(|e| EmbeddingError::EmbedFailed(e.to_string()))?;

        let raw_vector = embeddings
            .into_iter()
            .next()
            .ok_or(EmbeddingError::NoOutput)?;

        // Pad to 768 dimensions if needed (MiniLM is 384-dim)
        let vector = pad_to_dimension(raw_vector, VECTOR_DIMENSION);

        self.embed_count += 1;

        if self.embed_count % 1000 == 0 {
            debug!("Embedded {} thoughts", self.embed_count);
        }

        Ok(vector)
    }

    /// Generate embeddings for a batch of thoughts
    pub fn embed_batch(&mut self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        if texts.is_empty() {
            return Ok(vec![]);
        }

        let embeddings = self
            .model
            .embed(texts, None)
            .map_err(|e| EmbeddingError::EmbedFailed(e.to_string()))?;

        let vectors: Vec<Vec<f32>> = embeddings
            .into_iter()
            .map(|v| pad_to_dimension(v, VECTOR_DIMENSION))
            .collect();

        self.embed_count += vectors.len() as u64;

        Ok(vectors)
    }

    /// Get count of embeddings generated this session
    pub fn embed_count(&self) -> u64 {
        self.embed_count
    }
}

/// Pad vector to target dimension (fills with zeros)
fn pad_to_dimension(mut vector: Vec<f32>, target_dim: usize) -> Vec<f32> {
    if vector.len() < target_dim {
        vector.resize(target_dim, 0.0);
    } else if vector.len() > target_dim {
        vector.truncate(target_dim);
    }
    vector
}

/// Create a shared embedding engine
pub fn create_embedding_engine() -> Result<SharedEmbeddingEngine, EmbeddingError> {
    let engine = EmbeddingEngine::new()?;
    Ok(Arc::new(RwLock::new(engine)))
}

/// Embedding errors
#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    #[error("Failed to initialize embedding model: {0}")]
    InitFailed(String),

    #[error("Empty input text")]
    EmptyInput,

    #[error("Failed to generate embedding: {0}")]
    EmbedFailed(String),

    #[error("No embedding output generated")]
    NoOutput,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_to_dimension_pads_short_vectors() {
        let v = vec![1.0, 2.0, 3.0];
        let padded = pad_to_dimension(v, 5);
        assert_eq!(padded.len(), 5);
        assert_eq!(padded, vec![1.0, 2.0, 3.0, 0.0, 0.0]);
    }

    #[test]
    fn pad_to_dimension_truncates_long_vectors() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let padded = pad_to_dimension(v, 3);
        assert_eq!(padded.len(), 3);
        assert_eq!(padded, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn pad_to_dimension_preserves_exact_vectors() {
        let v = vec![1.0, 2.0, 3.0];
        let padded = pad_to_dimension(v, 3);
        assert_eq!(padded.len(), 3);
        assert_eq!(padded, vec![1.0, 2.0, 3.0]);
    }
}

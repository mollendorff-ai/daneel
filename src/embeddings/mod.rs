//! Text Embeddings for DANEEL - Phase 2 Forward-Only Embeddings
//!
//! Generates 768-dimensional semantic vectors for thoughts using `FastEmbed`.
//!
//! # Architecture Decision
//!
//! Per ADR-052 and Grok's recommendation (Dec 30, 2025):
//! - Use 768-dim embeddings for better Hebbian learning gradients
//! - BGE-base-en-v1.5 provides native 768 dims (no padding needed)
//! - Better clustering quality than 384-dim `MiniLM` (~3% improvement on STS-B)
//!
//! # Model
//!
//! Uses `BAAI/bge-base-en-v1.5` via `FastEmbed`:
//! - 768-dimensional output (native, no padding)
//! - ~110M parameters, ~3-5ms per thought on M2 Pro
//! - MTEB avg 63.55 across 56 datasets
//! - Better cosine similarity gradients for associative learning

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::memory_db::types::VECTOR_DIMENSION;

/// Embedding engine using `FastEmbed`
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
    /// Downloads the model on first run (~420MB for BGE-base-en-v1.5)
    ///
    /// # Errors
    ///
    /// Returns `EmbeddingError::InitFailed` if model loading fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn new() -> Result<Self, EmbeddingError> {
        info!("Initializing embedding engine (bge-base-en-v1.5, 768 dims)...");

        let model = fastembed::TextEmbedding::try_new(
            fastembed::InitOptions::new(fastembed::EmbeddingModel::BGEBaseENV15)
                .with_show_download_progress(true),
        )
        .map_err(|e| EmbeddingError::InitFailed(e.to_string()))?;

        info!("Embedding engine ready. Timmy can now see meaning in 768 dimensions.");

        Ok(Self {
            model,
            embed_count: 0,
        })
    }

    /// Generate embedding for a single thought
    ///
    /// Returns a 768-dimensional vector (native from BGE-base-en-v1.5)
    ///
    /// # Errors
    ///
    /// Returns `EmbeddingError` if input is empty or embedding fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
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

        // Ensure correct dimension (BGE is native 768, but keep safety check)
        let vector = pad_to_dimension(raw_vector, VECTOR_DIMENSION);

        self.embed_count += 1;

        if self.embed_count.is_multiple_of(1000) {
            debug!("Embedded {} thoughts", self.embed_count);
        }

        Ok(vector)
    }

    /// Generate embeddings for a batch of thoughts
    ///
    /// # Errors
    ///
    /// Returns `EmbeddingError::EmbedFailed` if batch embedding fails.
    #[cfg_attr(coverage_nightly, coverage(off))]
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
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub const fn embed_count(&self) -> u64 {
        self.embed_count
    }
}

/// Pad vector to target dimension (fills with zeros)
#[cfg_attr(coverage_nightly, coverage(off))]
fn pad_to_dimension(mut vector: Vec<f32>, target_dim: usize) -> Vec<f32> {
    if vector.len() < target_dim {
        vector.resize(target_dim, 0.0);
    } else if vector.len() > target_dim {
        vector.truncate(target_dim);
    }
    vector
}

/// Create a shared embedding engine
///
/// # Errors
///
/// Returns `EmbeddingError::InitFailed` if the engine cannot be created.
#[cfg_attr(coverage_nightly, coverage(off))]
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

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
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

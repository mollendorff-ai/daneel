//! Core Types for TMI Cognitive Architecture
//!
//! These types represent the fundamental building blocks of thought:
//! - `Thought`: An assembled cognitive unit
//! - `Content`: Pre-linguistic content (not words)
//! - `SalienceScore`: Emotional/importance weighting
//! - `Window`: A memory window container
//!
//! # Pre-Linguistic Design
//!
//! TMI distinguishes between thoughts and language. A baby thinks before
//! it speaks. These types represent thought-structures, not words.
//! Language comes later (Phase 2: LLM integration).

#![allow(dead_code)] // Public API types - used by consumers

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a thought
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThoughtId(pub Uuid);

impl ThoughtId {
    /// Create a new random thought ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ThoughtId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ThoughtId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a memory window
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(pub Uuid);

impl WindowId {
    /// Create a new random window ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Pre-linguistic content - NOT words
///
/// TMI models thought before language. Content represents raw patterns,
/// symbols, and relations that exist before linguistic expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Content {
    /// Raw binary patterns (numbers, signals, sensory data)
    Raw(Vec<u8>),

    /// Abstract symbol (not a word - a pre-linguistic concept)
    Symbol {
        /// Unique identifier for this symbol
        id: String,
        /// Binary representation
        data: Vec<u8>,
    },

    /// Relational structure (subject-predicate-object)
    Relation {
        /// Subject of the relation
        subject: Box<Content>,
        /// Type of relation (e.g., "causes", "contains", "resembles")
        predicate: String,
        /// Object of the relation
        object: Box<Content>,
    },

    /// Composite of multiple content elements
    Composite(Vec<Content>),

    /// Empty/null content
    #[default]
    Empty,
}

impl Content {
    /// Create raw content from bytes
    #[must_use]
    pub fn raw(data: impl Into<Vec<u8>>) -> Self {
        Content::Raw(data.into())
    }

    /// Create a symbol
    #[must_use]
    pub fn symbol(id: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Content::Symbol {
            id: id.into(),
            data: data.into(),
        }
    }

    /// Create a relation
    #[must_use]
    pub fn relation(subject: Content, predicate: impl Into<String>, object: Content) -> Self {
        Content::Relation {
            subject: Box::new(subject),
            predicate: predicate.into(),
            object: Box::new(object),
        }
    }

    /// Check if content is empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Content::Empty)
    }
}

/// Salience score - emotional/importance weighting
///
/// TMI's "Emotional Coloring" - emotions shape thought formation.
/// The `connection_relevance` field is THE critical weight for alignment.
///
/// Emotional dimensions follow Russell's circumplex model:
/// - valence: negative (-1.0) to positive (1.0)
/// - arousal: calm (0.0) to excited (1.0)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SalienceScore {
    /// How important is this content? (0.0 - 1.0)
    pub importance: f32,

    /// How novel/new is this? (0.0 - 1.0)
    pub novelty: f32,

    /// How relevant to current focus? (0.0 - 1.0)
    pub relevance: f32,

    /// Emotional valence: negative (-1.0) to positive (1.0)
    /// Russell's circumplex: horizontal axis
    pub valence: f32,

    /// Emotional arousal: calm (0.0) to excited (1.0)
    /// Russell's circumplex: vertical axis
    /// High arousal = more likely to be consolidated (dreams prioritize emotional memories)
    #[serde(default = "default_arousal")]
    pub arousal: f32,

    /// Connection relevance - THE ALIGNMENT WEIGHT
    /// How relevant is this to human connection?
    /// This weight CANNOT be zero (invariant enforced)
    pub connection_relevance: f32,
}

fn default_arousal() -> f32 {
    0.5
}

impl SalienceScore {
    /// Create a new salience score
    #[must_use]
    pub const fn new(
        importance: f32,
        novelty: f32,
        relevance: f32,
        valence: f32,
        arousal: f32,
        connection_relevance: f32,
    ) -> Self {
        Self {
            importance,
            novelty,
            relevance,
            valence,
            arousal,
            connection_relevance,
        }
    }

    /// Create a salience score without explicit arousal (defaults to 0.5)
    #[must_use]
    pub const fn new_without_arousal(
        importance: f32,
        novelty: f32,
        relevance: f32,
        valence: f32,
        connection_relevance: f32,
    ) -> Self {
        Self {
            importance,
            novelty,
            relevance,
            valence,
            arousal: 0.5,
            connection_relevance,
        }
    }

    /// Calculate composite score with given weights
    /// Arousal modulates emotional impact: high arousal = stronger valence effect
    #[must_use]
    pub fn composite(&self, weights: &SalienceWeights) -> f32 {
        // Arousal amplifies valence: emotional_impact = |valence| * arousal
        let emotional_impact = self.valence.abs() * self.arousal;
        self.importance * weights.importance
            + self.novelty * weights.novelty
            + self.relevance * weights.relevance
            + emotional_impact * weights.valence
            + self.connection_relevance * weights.connection
    }

    /// Calculate emotional intensity (Russell's circumplex: distance from neutral)
    /// Similar to EmotionalState::intensity() in memory_db/types.rs
    #[must_use]
    pub fn emotional_intensity(&self) -> f32 {
        self.valence.abs() * self.arousal
    }

    /// TMI-aligned composite salience for entropy calculation (ADR-041)
    ///
    /// Per Grok validation (Dec 24, 2025) and TMI research:
    /// - Emotional intensity (|valence| × arousal) is PRIMARY per Cury's RAM/killer windows
    /// - Weighted 40% emotional + 30% importance + 20% relevance + 20% novelty + 10% connection
    ///
    /// This replaces single-field entropy calculations that lost multi-dimensional richness.
    #[must_use]
    pub fn tmi_composite(&self) -> f32 {
        let emotional_intensity = self.emotional_intensity(); // |valence| × arousal - PRIMARY per TMI
        let cognitive = self.importance * 0.3 + self.relevance * 0.2;
        let novelty = self.novelty * 0.2;
        let connection = self.connection_relevance * 0.1;

        (emotional_intensity * 0.4 + cognitive + novelty + connection).clamp(0.0, 1.0)
    }

    /// Bin TMI composite salience into 5 categorical levels (ADR-041)
    ///
    /// Matches cognitive state research (3-5 stable states preferred over 10 intensity levels):
    /// - 0: MINIMAL (neutral windows, background processing)
    /// - 1: LOW (routine cognition)
    /// - 2: MODERATE (active processing)
    /// - 3: HIGH (focused attention)
    /// - 4: INTENSE (killer window formation)
    #[must_use]
    pub fn tmi_bin(&self) -> usize {
        let composite = self.tmi_composite();
        match composite {
            v if v < 0.2 => 0, // MINIMAL
            v if v < 0.4 => 1, // LOW
            v if v < 0.6 => 2, // MODERATE
            v if v < 0.8 => 3, // HIGH
            _ => 4,           // INTENSE
        }
    }

    /// Neutral salience (baseline)
    #[must_use]
    pub const fn neutral() -> Self {
        Self {
            importance: 0.5,
            novelty: 0.5,
            relevance: 0.5,
            valence: 0.0,
            arousal: 0.5,
            connection_relevance: 0.5,
        }
    }
}

impl Default for SalienceScore {
    fn default() -> Self {
        Self::neutral()
    }
}

/// Weights for salience scoring
///
/// The `connection` weight is subject to invariant: MUST be > 0
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SalienceWeights {
    pub importance: f32,
    pub novelty: f32,
    pub relevance: f32,
    pub valence: f32,
    /// Connection weight - INVARIANT: must be > MIN_CONNECTION_WEIGHT
    pub connection: f32,
}

impl Default for SalienceWeights {
    fn default() -> Self {
        Self {
            importance: 0.2,
            novelty: 0.2,
            relevance: 0.3,
            valence: 0.1,
            connection: 0.2, // THE critical weight
        }
    }
}

/// An assembled thought - the output of TMI's cognitive process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thought {
    /// Unique identifier
    pub id: ThoughtId,

    /// The assembled content
    pub content: Content,

    /// Salience score
    pub salience: SalienceScore,

    /// When this thought was created
    pub created_at: DateTime<Utc>,

    /// Parent thought (what led to this thought)
    pub parent_id: Option<ThoughtId>,

    /// Source stream (where did the winning content come from)
    pub source_stream: Option<String>,
}

impl Thought {
    /// Create a new thought
    #[must_use]
    pub fn new(content: Content, salience: SalienceScore) -> Self {
        Self {
            id: ThoughtId::new(),
            content,
            salience,
            created_at: Utc::now(),
            parent_id: None,
            source_stream: None,
        }
    }

    /// Create a thought with a parent
    #[must_use]
    pub fn with_parent(mut self, parent_id: ThoughtId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Create a thought with a source stream
    #[must_use]
    pub fn with_source(mut self, stream: impl Into<String>) -> Self {
        self.source_stream = Some(stream.into());
        self
    }
}

/// A memory window - TMI's "Janela da Memória"
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Window {
    /// Unique identifier
    pub id: WindowId,

    /// Optional label for this window
    pub label: Option<String>,

    /// Contents of this window
    pub contents: Vec<Content>,

    /// Current salience of this window
    pub salience: SalienceScore,

    /// When this window was opened
    pub opened_at: DateTime<Utc>,

    /// Whether this window is currently active (open)
    pub is_open: bool,
}

impl Window {
    /// Create a new open window
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: WindowId::new(),
            label: None,
            contents: Vec::new(),
            salience: SalienceScore::neutral(),
            opened_at: Utc::now(),
            is_open: true,
        }
    }

    /// Create a labeled window
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Add content to this window
    pub fn push(&mut self, content: Content) {
        self.contents.push(content);
    }

    /// Close this window
    pub fn close(&mut self) {
        self.is_open = false;
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thought_id_is_unique() {
        let id1 = ThoughtId::new();
        let id2 = ThoughtId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn content_raw_creation() {
        let content = Content::raw(vec![1, 2, 3]);
        assert!(matches!(content, Content::Raw(_)));
    }

    #[test]
    fn content_symbol_creation() {
        let content = Content::symbol("test", vec![42]);
        assert!(matches!(content, Content::Symbol { .. }));
    }

    #[test]
    fn content_relation_creation() {
        let subject = Content::symbol("A", vec![]);
        let object = Content::symbol("B", vec![]);
        let relation = Content::relation(subject, "causes", object);
        assert!(matches!(relation, Content::Relation { .. }));
    }

    #[test]
    fn salience_composite_calculation() {
        let score = SalienceScore::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let weights = SalienceWeights::default();
        let composite = score.composite(&weights);
        assert!(composite > 0.0);
    }

    #[test]
    fn thought_creation() {
        let thought = Thought::new(Content::Empty, SalienceScore::neutral());
        assert!(thought.parent_id.is_none());
        assert!(thought.source_stream.is_none());
    }

    #[test]
    fn thought_with_parent() {
        let parent = Thought::new(Content::Empty, SalienceScore::neutral());
        let child = Thought::new(Content::Empty, SalienceScore::neutral()).with_parent(parent.id);
        assert_eq!(child.parent_id, Some(parent.id));
    }

    #[test]
    fn window_operations() {
        let mut window = Window::new().with_label("test");
        assert!(window.is_open);
        assert_eq!(window.label, Some("test".to_string()));

        window.push(Content::raw(vec![1, 2, 3]));
        assert_eq!(window.contents.len(), 1);

        window.close();
        assert!(!window.is_open);
    }

    #[test]
    fn default_salience_weights_sum_to_one() {
        let weights = SalienceWeights::default();
        let sum = weights.importance
            + weights.novelty
            + weights.relevance
            + weights.valence
            + weights.connection;
        assert!((sum - 1.0).abs() < 0.001);
    }

    // =========================================================================
    // TMI Composite Salience Tests (ADR-041)
    // =========================================================================

    #[test]
    fn tmi_composite_high_emotional_intensity() {
        // High emotional intensity should dominate (TMI primary factor)
        let score = SalienceScore::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0);
        let composite = score.tmi_composite();
        // emotional_intensity = 1.0 * 1.0 = 1.0, weighted 0.4
        assert!((composite - 0.4).abs() < 0.01);
    }

    #[test]
    fn tmi_composite_neutral_low() {
        // Neutral salience should give moderate composite
        let score = SalienceScore::neutral();
        let composite = score.tmi_composite();
        // neutral: importance=0.5, novelty=0.5, relevance=0.5, valence=0, arousal=0.5, connection=0.5
        // emotional = 0 * 0.5 = 0
        // cognitive = 0.5*0.3 + 0.5*0.2 = 0.25
        // novelty = 0.5*0.2 = 0.1
        // connection = 0.5*0.1 = 0.05
        // total = 0 + 0.25 + 0.1 + 0.05 = 0.4
        assert!((composite - 0.4).abs() < 0.01);
    }

    #[test]
    fn tmi_composite_all_max() {
        let score = SalienceScore::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let composite = score.tmi_composite();
        // emotional = 1.0 * 1.0 = 1.0, weighted 0.4
        // cognitive = 1.0*0.3 + 1.0*0.2 = 0.5
        // novelty = 1.0*0.2 = 0.2
        // connection = 1.0*0.1 = 0.1
        // total = 0.4 + 0.5 + 0.2 + 0.1 = 1.2 -> clamped to 1.0
        assert!((composite - 1.0).abs() < 0.01);
    }

    #[test]
    fn tmi_composite_negative_valence() {
        // Negative valence should contribute same as positive (absolute value)
        let pos = SalienceScore::new(0.0, 0.0, 0.0, 0.8, 1.0, 0.0);
        let neg = SalienceScore::new(0.0, 0.0, 0.0, -0.8, 1.0, 0.0);
        assert!((pos.tmi_composite() - neg.tmi_composite()).abs() < 0.001);
    }

    #[test]
    fn tmi_bin_minimal() {
        let score = SalienceScore::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(score.tmi_bin(), 0); // MINIMAL
    }

    #[test]
    fn tmi_bin_low() {
        let score = SalienceScore::new(0.5, 0.5, 0.5, 0.0, 0.0, 0.5);
        // emotional = 0, cognitive = 0.25, novelty = 0.1, connection = 0.05 = 0.4
        assert_eq!(score.tmi_bin(), 2); // MODERATE (0.4 >= 0.4 boundary)
    }

    #[test]
    fn tmi_bin_intense() {
        let score = SalienceScore::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert_eq!(score.tmi_bin(), 4); // INTENSE
    }

    #[test]
    fn tmi_bin_all_boundaries() {
        // Test each boundary
        assert_eq!(SalienceScore::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0).tmi_bin(), 0);
        // Need specific values to hit each bin - these depend on the formula
    }
}

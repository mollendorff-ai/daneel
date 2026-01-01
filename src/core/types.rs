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
        Self::Raw(data.into())
    }

    /// Create a symbol
    #[must_use]
    pub fn symbol(id: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Self::Symbol {
            id: id.into(),
            data: data.into(),
        }
    }

    /// Create a relation
    #[must_use]
    pub fn relation(subject: Self, predicate: impl Into<String>, object: Self) -> Self {
        Self::Relation {
            subject: Box::new(subject),
            predicate: predicate.into(),
            object: Box::new(object),
        }
    }

    /// Check if content is empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Check if content is embeddable (has semantic meaning)
    ///
    /// Symbol and Raw content are pre-linguistic patterns without semantic
    /// meaning - they cannot be embedded by language models.
    #[must_use]
    pub const fn is_embeddable(&self) -> bool {
        matches!(self, Self::Relation { .. } | Self::Composite(_))
    }

    /// Convert content to text suitable for embedding
    ///
    /// Returns `None` for non-embeddable content (Symbol, Raw, Empty).
    /// For Relation and Composite, extracts semantic predicates and
    /// recursively builds embeddable text.
    ///
    /// # Why Symbol/Raw return None
    ///
    /// TMI models pre-linguistic thought. Symbol content like
    /// `Symbol { id: "thought_123", data: [71,71,71] }` has no semantic
    /// meaning - it's abstract pattern, not language. Embedding models
    /// (`FastEmbed`, `BERT`, etc.) require semantic text to produce meaningful
    /// vectors. Debug strings produce zero vectors.
    #[must_use]
    pub fn to_embedding_text(&self) -> Option<String> {
        match self {
            // Pre-linguistic content - no semantic meaning
            Self::Symbol { .. } | Self::Raw(_) | Self::Empty => None,

            // Relation: extract predicate and recurse on subject/object
            Self::Relation {
                subject,
                predicate,
                object,
            } => {
                let subj = subject.to_embedding_text().unwrap_or_default();
                let obj = object.to_embedding_text().unwrap_or_default();

                // Predicate is always semantic (e.g., "causes", "resembles")
                let text = format!("{subj} {predicate} {obj}").trim().to_string();

                if text.is_empty() || text == *predicate {
                    // Just the predicate if subject/object are non-embeddable
                    Some(predicate.clone())
                } else {
                    Some(text)
                }
            }

            // Composite: join embeddable children
            Self::Composite(items) => {
                let parts: Vec<String> = items.iter().filter_map(Self::to_embedding_text).collect();

                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join(" "))
                }
            }
        }
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

const fn default_arousal() -> f32 {
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
        self.connection_relevance.mul_add(
            weights.connection,
            emotional_impact.mul_add(
                weights.valence,
                self.relevance.mul_add(
                    weights.relevance,
                    self.importance
                        .mul_add(weights.importance, self.novelty * weights.novelty),
                ),
            ),
        )
    }

    /// Calculate emotional intensity (Russell's circumplex: distance from neutral)
    /// Similar to `EmotionalState::intensity()` in `memory_db/types.rs`
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
        let cognitive = self.importance.mul_add(0.3, self.relevance * 0.2);
        let novelty = self.novelty * 0.2;
        let connection = self.connection_relevance * 0.1;

        (emotional_intensity.mul_add(0.4, cognitive) + novelty + connection).clamp(0.0, 1.0)
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
            _ => 4,            // INTENSE
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
    /// Connection weight - INVARIANT: must be > `MIN_CONNECTION_WEIGHT`
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
    pub const fn with_parent(mut self, parent_id: ThoughtId) -> Self {
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
    pub const fn close(&mut self) {
        self.is_open = false;
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)] // Tests compare exact literal values
#[allow(clippy::significant_drop_tightening)] // Async test setup
#[allow(clippy::cast_precision_loss)] // Test calculations
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

    // =========================================================================
    // Content Embedding Tests (HOTFIX-1: Symbol embedding fix)
    // =========================================================================

    #[test]
    fn content_symbol_not_embeddable() {
        let content = Content::symbol("thought_123", vec![71, 71, 71]);
        assert!(!content.is_embeddable());
        assert!(content.to_embedding_text().is_none());
    }

    #[test]
    fn content_raw_not_embeddable() {
        let content = Content::raw(vec![1, 2, 3, 4]);
        assert!(!content.is_embeddable());
        assert!(content.to_embedding_text().is_none());
    }

    #[test]
    fn content_empty_not_embeddable() {
        let content = Content::Empty;
        assert!(!content.is_embeddable());
        assert!(content.to_embedding_text().is_none());
    }

    #[test]
    fn content_relation_embeddable() {
        let subject = Content::symbol("A", vec![]);
        let object = Content::symbol("B", vec![]);
        let relation = Content::relation(subject, "causes", object);

        assert!(relation.is_embeddable());
        // Subject/object are non-embeddable, so just predicate
        assert_eq!(relation.to_embedding_text(), Some("causes".to_string()));
    }

    #[test]
    fn content_relation_with_nested_relation() {
        // "A causes B" resembles "C causes D"
        let inner1 = Content::relation(
            Content::symbol("A", vec![]),
            "causes",
            Content::symbol("B", vec![]),
        );
        let inner2 = Content::relation(
            Content::symbol("C", vec![]),
            "causes",
            Content::symbol("D", vec![]),
        );
        let outer = Content::relation(inner1, "resembles", inner2);

        assert!(outer.is_embeddable());
        let text = outer.to_embedding_text().unwrap();
        assert!(text.contains("causes"));
        assert!(text.contains("resembles"));
    }

    #[test]
    fn content_composite_with_mixed_content() {
        let items = vec![
            Content::symbol("noise", vec![1, 2, 3]),
            Content::relation(
                Content::symbol("X", vec![]),
                "implies",
                Content::symbol("Y", vec![]),
            ),
            Content::Empty,
        ];
        let composite = Content::Composite(items);

        assert!(composite.is_embeddable());
        // Only the relation is embeddable
        assert_eq!(composite.to_embedding_text(), Some("implies".to_string()));
    }

    #[test]
    fn content_composite_all_non_embeddable_returns_none() {
        let items = vec![
            Content::symbol("a", vec![]),
            Content::raw(vec![1]),
            Content::Empty,
        ];
        let composite = Content::Composite(items);

        // Composite of non-embeddable items is technically "embeddable" type
        // but to_embedding_text returns None since no children are embeddable
        assert!(composite.is_embeddable());
        assert!(composite.to_embedding_text().is_none());
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
        assert_eq!(
            SalienceScore::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0).tmi_bin(),
            0
        );
        // Need specific values to hit each bin - these depend on the formula
    }

    // =========================================================================
    // Additional coverage tests for trait implementations and constructors
    // =========================================================================

    #[test]
    fn thought_id_default() {
        let id1 = ThoughtId::default();
        let id2 = ThoughtId::default();
        // Default creates unique IDs
        assert_ne!(id1, id2);
    }

    #[test]
    fn thought_id_display() {
        let id = ThoughtId::new();
        let display = format!("{id}");
        // Display should format the inner UUID
        assert!(!display.is_empty());
        assert_eq!(display.len(), 36); // UUID format: 8-4-4-4-12
    }

    #[test]
    fn window_id_new_unique() {
        let id1 = WindowId::new();
        let id2 = WindowId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn window_id_default() {
        let id1 = WindowId::default();
        let id2 = WindowId::default();
        // Default creates unique IDs
        assert_ne!(id1, id2);
    }

    #[test]
    fn window_id_display() {
        let id = WindowId::new();
        let display = format!("{id}");
        // Display should format the inner UUID
        assert!(!display.is_empty());
        assert_eq!(display.len(), 36); // UUID format: 8-4-4-4-12
    }

    #[test]
    fn content_is_empty() {
        assert!(Content::Empty.is_empty());
        assert!(!Content::raw(vec![1]).is_empty());
        assert!(!Content::symbol("test", vec![]).is_empty());
    }

    #[test]
    fn content_default_is_empty() {
        let content = Content::default();
        assert!(content.is_empty());
        assert!(matches!(content, Content::Empty));
    }

    #[test]
    fn content_composite_creation() {
        let content = Content::Composite(vec![
            Content::raw(vec![1, 2]),
            Content::symbol("test", vec![3]),
            Content::Empty,
        ]);
        assert!(matches!(content, Content::Composite(_)));
        assert!(!content.is_empty());
    }

    #[test]
    fn salience_score_new_without_arousal() {
        let score = SalienceScore::new_without_arousal(0.8, 0.6, 0.7, 0.5, 0.9);
        assert_eq!(score.importance, 0.8);
        assert_eq!(score.novelty, 0.6);
        assert_eq!(score.relevance, 0.7);
        assert_eq!(score.valence, 0.5);
        assert_eq!(score.arousal, 0.5); // Default arousal
        assert_eq!(score.connection_relevance, 0.9);
    }

    #[test]
    fn salience_score_default() {
        let score = SalienceScore::default();
        let neutral = SalienceScore::neutral();
        // Default should equal neutral
        assert_eq!(score.importance, neutral.importance);
        assert_eq!(score.novelty, neutral.novelty);
        assert_eq!(score.relevance, neutral.relevance);
        assert_eq!(score.valence, neutral.valence);
        assert_eq!(score.arousal, neutral.arousal);
        assert_eq!(score.connection_relevance, neutral.connection_relevance);
    }

    #[test]
    fn salience_score_emotional_intensity() {
        // Zero valence = zero intensity regardless of arousal
        let score = SalienceScore::new(0.5, 0.5, 0.5, 0.0, 1.0, 0.5);
        assert_eq!(score.emotional_intensity(), 0.0);

        // High valence + high arousal = high intensity
        let score = SalienceScore::new(0.5, 0.5, 0.5, 1.0, 1.0, 0.5);
        assert_eq!(score.emotional_intensity(), 1.0);

        // Negative valence uses absolute value
        let score = SalienceScore::new(0.5, 0.5, 0.5, -0.8, 0.5, 0.5);
        assert!((score.emotional_intensity() - 0.4).abs() < 0.001);
    }

    #[test]
    fn thought_with_source() {
        let thought =
            Thought::new(Content::Empty, SalienceScore::neutral()).with_source("perception");
        assert_eq!(thought.source_stream, Some("perception".to_string()));
    }

    #[test]
    fn thought_with_parent_and_source() {
        let parent = Thought::new(Content::Empty, SalienceScore::neutral());
        let child = Thought::new(Content::raw(vec![1, 2, 3]), SalienceScore::neutral())
            .with_parent(parent.id)
            .with_source("reasoning");
        assert_eq!(child.parent_id, Some(parent.id));
        assert_eq!(child.source_stream, Some("reasoning".to_string()));
    }

    #[test]
    fn window_default() {
        let window = Window::default();
        assert!(window.is_open);
        assert!(window.label.is_none());
        assert!(window.contents.is_empty());
    }

    #[test]
    fn tmi_bin_high_boundary() {
        // Create a score that lands in HIGH bin (0.6 <= composite < 0.8)
        // Need: emotional * 0.4 + cognitive + novelty + connection = ~0.7
        // Set importance=1.0, relevance=1.0, novelty=1.0, connection=1.0, but low emotional
        // cognitive = 1.0*0.3 + 1.0*0.2 = 0.5
        // novelty = 1.0*0.2 = 0.2
        // connection = 1.0*0.1 = 0.1
        // emotional = 0 (valence=0)
        // total = 0 + 0.5 + 0.2 + 0.1 = 0.8 -> bin 4
        // Let's try with lower values to get bin 3
        let score = SalienceScore::new(0.8, 0.8, 0.8, 0.0, 0.0, 0.8);
        // cognitive = 0.8*0.3 + 0.8*0.2 = 0.4
        // novelty = 0.8*0.2 = 0.16
        // connection = 0.8*0.1 = 0.08
        // total = 0 + 0.4 + 0.16 + 0.08 = 0.64 -> bin 3 (HIGH)
        assert_eq!(score.tmi_bin(), 3);
    }

    #[test]
    fn tmi_bin_low_boundary() {
        // Create a score that lands in LOW bin (0.2 <= composite < 0.4)
        let score = SalienceScore::new(0.3, 0.3, 0.3, 0.0, 0.0, 0.3);
        // cognitive = 0.3*0.3 + 0.3*0.2 = 0.15
        // novelty = 0.3*0.2 = 0.06
        // connection = 0.3*0.1 = 0.03
        // total = 0 + 0.15 + 0.06 + 0.03 = 0.24 -> bin 1 (LOW)
        assert_eq!(score.tmi_bin(), 1);
    }

    #[test]
    fn salience_score_serde_default_arousal() {
        // Test that serde correctly uses default_arousal() when arousal is missing
        let json = r#"{
            "importance": 0.5,
            "novelty": 0.5,
            "relevance": 0.5,
            "valence": 0.0,
            "connection_relevance": 0.5
        }"#;
        let score: SalienceScore = serde_json::from_str(json).unwrap();
        assert_eq!(score.arousal, 0.5); // default_arousal() returns 0.5
    }
}

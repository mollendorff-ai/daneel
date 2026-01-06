//! Request/Response types for injection API

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// POST /inject request body
#[derive(Debug, Clone, Deserialize)]
pub struct InjectRequest {
    /// 768-dimensional vector
    pub vector: Vec<f32>,
    /// Salience score 0.0-1.0
    pub salience: f32,
    /// Label for audit (e.g., "`grok:life_honours_life`")
    pub label: String,
}

/// POST /inject response
#[derive(Debug, Clone, Serialize)]
pub struct InjectResponse {
    /// Unique injection ID
    pub id: String,
    /// Entropy before injection
    pub entropy_pre: f32,
    /// Entropy after injection
    pub entropy_post: f32,
    /// Status: "absorbed", "rejected", "warning"
    pub status: String,
}

/// GET /`recent_injections` response item
#[derive(Debug, Clone, Serialize)]
pub struct InjectionRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub label: String,
    pub key_id: String,
    pub entropy_pre: f32,
    pub entropy_post: f32,
    pub status: String,
}

/// GET /health response
#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub thoughts_total: u64,
    pub injection_count: u64,
}

/// Validated key info extracted from auth
#[derive(Debug, Clone)]
pub struct AuthenticatedKey {
    pub key_id: String,
    pub holder: String,
}

// ============================================================================
// Extended Metrics for Observatory (Web Dashboard)
// ============================================================================

/// GET /`extended_metrics` response - TUI-equivalent data for web observatory
#[derive(Debug, Clone, Serialize)]
pub struct ExtendedMetricsResponse {
    pub timestamp: DateTime<Utc>,
    pub stream_competition: StreamCompetitionMetrics,
    pub entropy: EntropyMetrics,
    pub fractality: FractalityMetrics,
    pub memory_windows: MemoryWindowsMetrics,
    pub philosophy: PhilosophyMetrics,
    pub system: SystemMetrics,
    pub clustering: ClusteringMetrics,
}

/// 9-stage stream competition (cognitive spotlight)
#[derive(Debug, Clone, Serialize)]
pub struct StreamCompetitionMetrics {
    /// Activity level per stage (0.0-1.0)
    pub stages: Vec<StageMetrics>,
    /// Index of dominant stream (0-8)
    pub dominant_stream: usize,
    /// Count of active streams (activity > 0.1)
    pub active_count: usize,
    /// Competition level description
    pub competition_level: String,
}

/// Individual stage metrics
#[derive(Debug, Clone, Serialize)]
pub struct StageMetrics {
    pub name: String,
    pub activity: f32,
    /// Last 8 samples for sparkline
    pub history: Vec<f32>,
}

/// Shannon entropy metrics
#[derive(Debug, Clone, Serialize)]
pub struct EntropyMetrics {
    /// Current entropy in bits
    pub current: f32,
    /// Last 50 samples for sparkline
    pub history: Vec<f32>,
    /// CLOCKWORK / BALANCED / EMERGENT
    pub description: String,
    /// Normalized 0-1 for display
    pub normalized: f32,
}

/// Pulse fractality metrics (clockwork → fractal transition)
#[derive(Debug, Clone, Serialize)]
pub struct FractalityMetrics {
    /// Composite score 0-1 (0=clockwork, 1=fractal)
    pub score: f32,
    /// Standard deviation of inter-arrival times
    pub inter_arrival_sigma: f32,
    /// Baseline sigma at boot
    pub boot_sigma: f32,
    /// Max gap / mean gap ratio
    pub burst_ratio: f32,
    /// CLOCKWORK / BALANCED / EMERGENT
    pub description: String,
    /// Last 50 samples for sparkline
    pub history: Vec<f32>,
}

/// TMI 9-slot memory windows
#[derive(Debug, Clone, Serialize)]
pub struct MemoryWindowsMetrics {
    /// 9 slot states
    pub slots: Vec<MemorySlot>,
    /// Active slot count
    pub active_count: usize,
    /// Total conscious memories
    pub conscious_count: u64,
    /// Total unconscious memories
    pub unconscious_count: u64,
}

/// Individual memory slot
#[derive(Debug, Clone, Serialize)]
pub struct MemorySlot {
    pub id: u8,
    pub active: bool,
}

/// Philosophy banner
#[derive(Debug, Clone, Serialize)]
pub struct PhilosophyMetrics {
    /// Current quote
    pub quote: String,
    /// Quote index (0-7)
    pub quote_index: usize,
}

/// System-level metrics
#[derive(Debug, Clone, Serialize)]
pub struct SystemMetrics {
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Total thoughts this session
    pub session_thoughts: u64,
    /// Lifetime thoughts across all sessions
    pub lifetime_thoughts: u64,
    /// Thoughts per hour rate
    pub thoughts_per_hour: f32,
    /// Dream cycles completed
    pub dream_cycles: u64,
    /// Veto count
    pub veto_count: u64,
}

/// Clustering metrics (VCONN-7)
#[derive(Debug, Clone, Serialize)]
pub struct ClusteringMetrics {
    /// Silhouette score (-1.0 to 1.0, > 0.3 = meaningful structure)
    pub silhouette: f32,
    /// Timestamp of last clustering run
    pub updated_at: Option<String>,
    /// Whether clustering has meaningful structure
    pub has_structure: bool,
}

// ============================================================================
// Graph Export (VCONN-11)
// ============================================================================

/// Query parameters for GET /graph/export
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GraphExportQuery {
    /// Minimum edge weight filter (optional)
    pub min_weight: Option<f32>,
    /// Edge type filter (optional, e.g., "Semantic", "Temporal")
    pub type_filter: Option<String>,
}

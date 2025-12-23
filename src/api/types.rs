//! Request/Response types for injection API

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// POST /inject request body
#[derive(Debug, Clone, Deserialize)]
pub struct InjectRequest {
    /// 768-dimensional vector
    pub vector: Vec<f32>,
    /// Salience score 0.0-1.0
    pub salience: f32,
    /// Label for audit (e.g., "grok:life_honours_life")
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

/// GET /recent_injections response item
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

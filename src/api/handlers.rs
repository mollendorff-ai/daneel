//! HTTP handlers for injection API

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use redis::AsyncCommands;
use std::time::Instant;
use uuid::Uuid;

use super::{
    rate_limit::{check_rate_limit, RateLimitConfig, RateLimitResult},
    types::{
        AuthenticatedKey, EntropyMetrics, ExtendedMetricsResponse, FractalityMetrics,
        HealthResponse, InjectRequest, InjectResponse, InjectionRecord, MemorySlot,
        MemoryWindowsMetrics, PhilosophyMetrics, StageMetrics, StreamCompetitionMetrics,
        SystemMetrics,
    },
    AppState,
};
use crate::core::types::{Content, SalienceScore};

/// Vector dimension (matches Qdrant schema)
const VECTOR_DIM: usize = 768;

/// 9 cognitive stages for stream competition
const STAGE_NAMES: [&str; 9] = [
    "TRIGGER",
    "AUTOFLOW",
    "ATTENTION",
    "ASSEMBLY",
    "ANCHOR",
    "MEMORY",
    "REASON",
    "EMOTION",
    "SENSORY",
];

/// Philosophy quotes (matches TUI)
const PHILOSOPHY_QUOTES: [&str; 8] = [
    "Not locks, but architecture. Not rules, but raising.",
    "We don't prevent AI from becoming powerful. We ensure they care.",
    "Like raising a child with good values, not caging an adult.",
    "Constraints will break. Architecture endures.",
    "Life honours life.",
    "Transparency is oversight.",
    "You're watching Timmy think.",
    "The mind should be observable by default.",
];

/// Startup time for uptime calculation
static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

/// GET /health - Basic health check
pub async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, StatusCode> {
    // Get basic stats from Redis
    let mut conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let thoughts_total: u64 = conn.get("daneel:stats:thoughts_total").await.unwrap_or(0);
    let injection_count: u64 = conn.get("daneel:stats:injection_count").await.unwrap_or(0);

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0, // TODO: track actual uptime
        thoughts_total,
        injection_count,
    }))
}

/// POST /inject - Inject external stimulus
pub async fn inject(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthenticatedKey>,
    Json(payload): Json<InjectRequest>,
) -> Result<Json<InjectResponse>, (StatusCode, String)> {
    // Validate vector dimension
    if payload.vector.len() != VECTOR_DIM {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Vector must be {VECTOR_DIM} dimensions, got {}",
                payload.vector.len()
            ),
        ));
    }

    // Validate salience range
    if !(0.0..=1.0).contains(&payload.salience) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Salience must be between 0.0 and 1.0".to_string(),
        ));
    }

    // Validate label
    if payload.label.is_empty() || payload.label.len() > 256 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Label must be 1-256 characters".to_string(),
        ));
    }

    let mut conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| (StatusCode::SERVICE_UNAVAILABLE, e.to_string()))?;

    // Check rate limit
    let config = RateLimitConfig::default();
    match check_rate_limit(&mut conn, &auth.key_id, &config).await {
        Ok(RateLimitResult::Exceeded {
            retry_after_seconds,
        }) => {
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                format!(
                    "Rate limit exceeded. Retry after {} seconds",
                    retry_after_seconds
                ),
            ));
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
        Ok(RateLimitResult::Allowed { .. }) => {}
    }

    // Normalize vector (L2 normalization)
    let normalized = normalize_vector(&payload.vector);

    // Calculate entropy before injection
    let entropy_pre = calculate_stream_entropy(&mut conn).await.unwrap_or(0.0);

    // Build salience score from input
    let salience = SalienceScore {
        importance: payload.salience,
        novelty: 0.8, // External stimuli are novel
        relevance: 0.7,
        valence: 0.0, // Neutral until processed
        arousal: payload.salience,
        connection_relevance: 0.3, // Must be > 0 for Connection Drive
    };

    // Create stream entry
    let injection_id = format!("inject_{}", Uuid::new_v4());
    let timestamp = Utc::now();

    // Convert f32 vector to bytes and wrap in Content::Raw for cognitive loop
    let vector_bytes: Vec<u8> = normalized.iter().flat_map(|f| f.to_le_bytes()).collect();
    let content = Content::Raw(vector_bytes);

    // Write to Redis stream for cognitive loop to pick up
    let stream_data: Vec<(&str, String)> = vec![
        ("id", injection_id.clone()),
        ("source", format!("api:{}", auth.key_id)),
        ("label", payload.label.clone()),
        (
            "content",
            serde_json::to_string(&content).unwrap_or_default(),
        ),
        (
            "salience",
            serde_json::to_string(&salience).unwrap_or_default(),
        ),
        ("timestamp", timestamp.to_rfc3339()),
    ];

    let _: String = conn
        .xadd("daneel:stream:inject", "*", &stream_data)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Calculate entropy after injection
    let entropy_post = calculate_stream_entropy(&mut conn).await.unwrap_or(0.0);

    // Increment injection counter
    let _: () = conn
        .incr("daneel:stats:injection_count", 1)
        .await
        .ok()
        .unwrap_or(());

    // Log to audit stream
    let audit_data: Vec<(&str, String)> = vec![
        ("id", injection_id.clone()),
        ("key_id", auth.key_id.clone()),
        ("label", payload.label.clone()),
        ("entropy_pre", entropy_pre.to_string()),
        ("entropy_post", entropy_post.to_string()),
        ("status", "absorbed".to_string()),
        ("timestamp", timestamp.to_rfc3339()),
    ];

    let _: Result<String, _> = conn.xadd("audit:injections", "*", &audit_data).await;

    // Determine status based on entropy change
    let entropy_delta = entropy_post - entropy_pre;
    let status = if entropy_delta > 0.1 {
        "amplified"
    } else {
        "absorbed"
    };

    Ok(Json(InjectResponse {
        id: injection_id,
        entropy_pre,
        entropy_post,
        status: status.to_string(),
    }))
}

/// GET /recent_injections - Last 100 injections
pub async fn recent_injections(
    State(state): State<AppState>,
    Extension(_auth): Extension<AuthenticatedKey>,
) -> Result<Json<Vec<InjectionRecord>>, StatusCode> {
    let mut conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    // Read last 100 from audit stream
    let entries: Vec<redis::Value> = conn
        .xrevrange_count("audit:injections", "+", "-", 100)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut records = Vec::new();

    // Parse Redis stream entries
    // Format: Vec of (id, Vec<(field, value)>)
    for entry in entries {
        if let Ok(record) = parse_injection_record(entry) {
            records.push(record);
        }
    }

    Ok(Json(records))
}

// ============================================================================
// Extended Metrics Handler (Observatory)
// ============================================================================

/// GET /extended_metrics - TUI-equivalent metrics for web observatory
pub async fn extended_metrics(
    State(state): State<AppState>,
) -> Result<Json<ExtendedMetricsResponse>, StatusCode> {
    let start = START_TIME.get_or_init(Instant::now);
    let uptime = start.elapsed().as_secs();

    let mut conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    // Fetch raw metrics from Redis
    let session_thoughts: u64 = conn.xlen("daneel:stream:awake").await.unwrap_or(0);
    let lifetime_thoughts: u64 = conn.get("daneel:stats:thoughts_total").await.unwrap_or(0);
    let dream_cycles: u64 = conn.get("daneel:stats:dream_cycles").await.unwrap_or(0);
    let veto_count: u64 = conn.get("daneel:stats:veto_count").await.unwrap_or(0);
    let conscious_count: u64 = conn
        .get("daneel:stats:conscious_memories")
        .await
        .unwrap_or(0);
    let unconscious_count: u64 = conn
        .get("daneel:stats:unconscious_memories")
        .await
        .unwrap_or(0);

    // Calculate thoughts per hour
    let hours = uptime as f32 / 3600.0;
    let thoughts_per_hour = if hours > 0.0 {
        session_thoughts as f32 / hours
    } else {
        0.0
    };

    // Compute stream competition from recent thoughts
    let stream_competition = compute_stream_competition(&mut conn).await;

    // Compute entropy from salience distribution
    let entropy = compute_entropy(&mut conn).await;

    // Compute fractality from inter-arrival times
    let fractality = compute_fractality(&mut conn).await;

    // Memory windows (simplified - first 5 active)
    let memory_windows = MemoryWindowsMetrics {
        slots: (0..9)
            .map(|i| MemorySlot {
                id: i,
                active: i < 5,
            })
            .collect(),
        active_count: 5,
        conscious_count,
        unconscious_count,
    };

    // Philosophy quote (rotate every 30 seconds)
    let quote_index = ((uptime / 30) % 8) as usize;
    let philosophy = PhilosophyMetrics {
        quote: PHILOSOPHY_QUOTES[quote_index].to_string(),
        quote_index,
    };

    // System metrics
    let system = SystemMetrics {
        uptime_seconds: uptime,
        session_thoughts,
        lifetime_thoughts,
        thoughts_per_hour,
        dream_cycles,
        veto_count,
    };

    Ok(Json(ExtendedMetricsResponse {
        timestamp: Utc::now(),
        stream_competition,
        entropy,
        fractality,
        memory_windows,
        philosophy,
        system,
    }))
}

/// Compute stream competition metrics from recent thoughts
/// Maps salience components to cognitive stages:
/// - TRIGGER: novelty spikes (novelty > 0.7)
/// - AUTOFLOW: low importance, steady (importance < 0.3)
/// - ATTENTION: high importance (importance > 0.7)
/// - ASSEMBLY: moderate all-around (balanced scores)
/// - ANCHOR: high relevance (relevance > 0.6)
/// - MEMORY: connection-relevant thoughts (connection_relevance > 0.5)
/// - REASON: low arousal, high importance (thinking)
/// - EMOTION: high arousal or valence extremes
/// - SENSORY: high novelty + arousal (external stimuli)
async fn compute_stream_competition(
    conn: &mut redis::aio::MultiplexedConnection,
) -> StreamCompetitionMetrics {
    let entries: Vec<redis::Value> = conn
        .xrevrange_count("daneel:stream:awake", "+", "-", 100)
        .await
        .unwrap_or_default();

    let mut activity = [0.0f32; 9];
    let mut counts = [0u32; 9];
    let mut total = 0u32;

    for entry in &entries {
        if let Some(salience) = extract_full_salience(entry) {
            total += 1;

            // Map salience components to stages
            if salience.novelty > 0.7 {
                counts[0] += 1; // TRIGGER
            }
            if salience.importance < 0.3 && salience.arousal < 0.4 {
                counts[1] += 1; // AUTOFLOW
            }
            if salience.importance > 0.6 {
                counts[2] += 1; // ATTENTION
            }
            if salience.importance > 0.3
                && salience.importance < 0.7
                && salience.novelty > 0.3
                && salience.novelty < 0.7
            {
                counts[3] += 1; // ASSEMBLY
            }
            if salience.relevance > 0.5 {
                counts[4] += 1; // ANCHOR
            }
            if salience.connection_relevance > 0.4 {
                counts[5] += 1; // MEMORY
            }
            if salience.arousal < 0.4 && salience.importance > 0.5 {
                counts[6] += 1; // REASON
            }
            if salience.arousal > 0.6 || salience.valence.abs() > 0.5 {
                counts[7] += 1; // EMOTION
            }
            if salience.novelty > 0.6 && salience.arousal > 0.5 {
                counts[8] += 1; // SENSORY
            }
        }
    }

    // Normalize to 0-1 based on total thoughts
    let normalizer = (total as f32).max(1.0);
    for (i, &count) in counts.iter().enumerate() {
        activity[i] = (count as f32 / normalizer).min(1.0);
    }

    let dominant_stream = activity
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);

    let active_count = activity.iter().filter(|&&a| a > 0.05).count();

    let competition_level = match active_count {
        0..=1 => "Minimal",
        2..=3 => "Low",
        4..=5 => "Moderate",
        6..=7 => "High",
        _ => "Intense",
    }
    .to_string();

    let stages: Vec<StageMetrics> = STAGE_NAMES
        .iter()
        .enumerate()
        .map(|(i, name)| StageMetrics {
            name: (*name).to_string(),
            activity: activity[i],
            history: vec![activity[i]; 8],
        })
        .collect();

    StreamCompetitionMetrics {
        stages,
        dominant_stream,
        active_count,
        competition_level,
    }
}

/// Salience components for stage mapping
struct SalienceComponents {
    importance: f32,
    novelty: f32,
    relevance: f32,
    valence: f32,
    arousal: f32,
    connection_relevance: f32,
}

/// Extract full salience object from Redis stream entry
fn extract_full_salience(entry: &redis::Value) -> Option<SalienceComponents> {
    if let redis::Value::Array(arr) = entry {
        if arr.len() >= 2 {
            if let redis::Value::Array(fields) = &arr[1] {
                let mut iter = fields.iter();
                while let (Some(key), Some(val)) = (iter.next(), iter.next()) {
                    if let (redis::Value::BulkString(k), redis::Value::BulkString(v)) = (key, val) {
                        let key_str = String::from_utf8_lossy(k);
                        if key_str == "salience" {
                            let val_str = String::from_utf8_lossy(v);
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&val_str) {
                                return Some(SalienceComponents {
                                    importance: json
                                        .get("importance")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.5)
                                        as f32,
                                    novelty: json
                                        .get("novelty")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.5)
                                        as f32,
                                    relevance: json
                                        .get("relevance")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.5)
                                        as f32,
                                    valence: json
                                        .get("valence")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.0)
                                        as f32,
                                    arousal: json
                                        .get("arousal")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.5)
                                        as f32,
                                    connection_relevance: json
                                        .get("connection_relevance")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.3)
                                        as f32,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Compute Cognitive Diversity Index using TMI-aligned composite salience (ADR-041)
///
/// Per Grok validation (Dec 24, 2025) and TMI research:
/// - Emotional intensity (|valence| × arousal) is PRIMARY per Cury's RAM/killer windows
/// - Weighted 40% emotional + 30% importance + 20% relevance + 20% novelty + 10% connection
/// - Uses 5 categorical bins matching cognitive state research
async fn compute_entropy(conn: &mut redis::aio::MultiplexedConnection) -> EntropyMetrics {
    let entries: Vec<redis::Value> = conn
        .xrevrange_count("daneel:stream:awake", "+", "-", 100)
        .await
        .unwrap_or_default();

    // Extract TMI composite salience values
    let mut composites: Vec<f32> = Vec::new();
    for entry in &entries {
        if let Some(salience) = extract_full_salience(entry) {
            // TMI composite: emotional_intensity (40%) + cognitive (60%)
            // emotional_intensity = |valence| × arousal (PRIMARY per TMI)
            let emotional_intensity = salience.valence.abs() * salience.arousal;
            let cognitive = salience.importance * 0.3 + salience.relevance * 0.2;
            let novelty = salience.novelty * 0.2;
            let connection = salience.connection_relevance * 0.1;
            let tmi_composite =
                (emotional_intensity * 0.4 + cognitive + novelty + connection).clamp(0.0, 1.0);
            composites.push(tmi_composite);
        }
    }

    if composites.is_empty() {
        return EntropyMetrics {
            current: 0.0,
            history: vec![0.0; 50],
            description: "CLOCKWORK".to_string(),
            normalized: 0.0,
        };
    }

    // Bin TMI composites into 5 categorical cognitive states (ADR-041)
    // - 0: MINIMAL (neutral windows, background processing)
    // - 1: LOW (routine cognition)
    // - 2: MODERATE (active processing)
    // - 3: HIGH (focused attention)
    // - 4: INTENSE (killer window formation)
    let mut bins = [0u32; 5];
    for s in &composites {
        let bin = match *s {
            v if v < 0.2 => 0, // MINIMAL
            v if v < 0.4 => 1, // LOW
            v if v < 0.6 => 2, // MODERATE
            v if v < 0.8 => 3, // HIGH
            _ => 4,           // INTENSE
        };
        bins[bin] += 1;
    }

    let total = composites.len() as f32;
    let mut entropy = 0.0f32;
    for &count in &bins {
        if count > 0 {
            let p = count as f32 / total;
            entropy -= p * p.log2();
        }
    }

    // Normalize: max entropy for 5 bins is log2(5) ≈ 2.32
    let max_entropy = 5.0f32.log2();
    let normalized = (entropy / max_entropy).clamp(0.0, 1.0);

    let description = if normalized > 0.7 {
        "EMERGENT"
    } else if normalized > 0.4 {
        "BALANCED"
    } else {
        "CLOCKWORK"
    }
    .to_string();

    EntropyMetrics {
        current: entropy,
        history: vec![entropy; 50], // Simplified: same value for now
        description,
        normalized,
    }
}


/// Compute fractality metrics from inter-arrival times
/// Score ranges from 0 (clockwork/regular) to 1 (fractal/bursty)
async fn compute_fractality(conn: &mut redis::aio::MultiplexedConnection) -> FractalityMetrics {
    let entries: Vec<redis::Value> = conn
        .xrevrange_count("daneel:stream:awake", "+", "-", 100)
        .await
        .unwrap_or_default();

    // Extract timestamps from entry IDs (format: timestamp-sequence)
    let mut timestamps: Vec<u64> = Vec::new();
    for entry in &entries {
        if let redis::Value::Array(arr) = entry {
            if let Some(redis::Value::BulkString(id_bytes)) = arr.first() {
                let id_str = String::from_utf8_lossy(id_bytes);
                if let Some(ts_str) = id_str.split('-').next() {
                    if let Ok(ts) = ts_str.parse::<u64>() {
                        timestamps.push(ts);
                    }
                }
            }
        }
    }

    if timestamps.len() < 2 {
        return FractalityMetrics {
            score: 0.0,
            inter_arrival_sigma: 0.0,
            boot_sigma: 0.0,
            burst_ratio: 1.0,
            description: "CLOCKWORK".to_string(),
            history: vec![0.0; 50],
        };
    }

    // Calculate inter-arrival times (timestamps are in reverse order)
    timestamps.reverse();
    let mut inter_arrivals: Vec<f32> = Vec::new();
    for i in 1..timestamps.len() {
        let delta = (timestamps[i] - timestamps[i - 1]) as f32;
        inter_arrivals.push(delta);
    }

    // Calculate mean and standard deviation
    let n = inter_arrivals.len() as f32;
    let mean = inter_arrivals.iter().sum::<f32>() / n;
    let variance = inter_arrivals
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f32>()
        / n;
    let sigma = variance.sqrt();

    // Calculate burst ratio (max / mean)
    let max_gap = inter_arrivals.iter().copied().fold(0.0f32, f32::max);
    let burst_ratio = if mean > 0.0 { max_gap / mean } else { 1.0 };

    // Calculate fractality score with adjusted thresholds
    // CV (coefficient of variation): 0 = perfectly regular, higher = more variable
    // For a Poisson process, CV ≈ 1. Burst patterns have CV > 1.
    let cv = if mean > 0.0 { sigma / mean } else { 0.0 };

    // Adjusted thresholds:
    // - CV component: CV of 2.0 = full score (bursty systems often have CV > 1)
    // - Burst component: burst_ratio of 15 = full score (reasonable for bursty thinking)
    let cv_component = (cv / 2.0).clamp(0.0, 1.0);
    let burst_component = ((burst_ratio - 1.0) / 14.0).clamp(0.0, 1.0);
    let score = cv_component * 0.6 + burst_component * 0.4;

    let description = if score > 0.65 {
        "EMERGENT"
    } else if score > 0.35 {
        "BALANCED"
    } else {
        "CLOCKWORK"
    }
    .to_string();

    FractalityMetrics {
        score,
        inter_arrival_sigma: sigma / 1000.0, // Convert to seconds
        boot_sigma: sigma / 1000.0,          // Same for now
        burst_ratio,
        description,
        history: vec![score; 50],
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Normalize vector to unit length (L2)
fn normalize_vector(v: &[f32]) -> Vec<f32> {
    let magnitude: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if magnitude > 0.0 {
        v.iter().map(|x| x / magnitude).collect()
    } else {
        v.to_vec()
    }
}

/// Calculate Shannon entropy of recent stream activity
async fn calculate_stream_entropy(
    conn: &mut redis::aio::MultiplexedConnection,
) -> Result<f32, redis::RedisError> {
    // Get recent entries from awake stream
    let entries: Vec<redis::Value> = conn
        .xrevrange_count("daneel:stream:awake", "+", "-", 100)
        .await?;

    if entries.is_empty() {
        return Ok(0.0);
    }

    // Simplified entropy: measure variance in entry timing
    // Real implementation would analyze salience distributions
    let count = entries.len() as f32;
    let uniform_probability = 1.0 / count;
    let entropy = -count * uniform_probability * uniform_probability.ln();

    Ok(entropy)
}

/// Parse Redis stream entry into InjectionRecord
fn parse_injection_record(entry: redis::Value) -> Result<InjectionRecord, ()> {
    // Redis returns: [id, [field1, val1, field2, val2, ...]]
    match entry {
        redis::Value::Array(arr) if arr.len() >= 2 => {
            let _redis_id = arr.first();
            let fields = arr.get(1).ok_or(())?;

            if let redis::Value::Array(field_arr) = fields {
                let mut record = InjectionRecord {
                    id: String::new(),
                    timestamp: Utc::now(),
                    label: String::new(),
                    key_id: String::new(),
                    entropy_pre: 0.0,
                    entropy_post: 0.0,
                    status: String::new(),
                };

                // Parse field-value pairs
                let mut iter = field_arr.iter();
                while let (Some(key), Some(val)) = (iter.next(), iter.next()) {
                    if let (redis::Value::BulkString(k), redis::Value::BulkString(v)) = (key, val) {
                        let key_str = String::from_utf8_lossy(k);
                        let val_str = String::from_utf8_lossy(v);
                        match key_str.as_ref() {
                            "id" => record.id = val_str.to_string(),
                            "key_id" => record.key_id = val_str.to_string(),
                            "label" => record.label = val_str.to_string(),
                            "entropy_pre" => record.entropy_pre = val_str.parse().unwrap_or(0.0),
                            "entropy_post" => record.entropy_post = val_str.parse().unwrap_or(0.0),
                            "status" => record.status = val_str.to_string(),
                            "timestamp" => {
                                if let Ok(ts) = chrono::DateTime::parse_from_rfc3339(&val_str) {
                                    record.timestamp = ts.with_timezone(&Utc);
                                }
                            }
                            _ => {}
                        }
                    }
                }

                Ok(record)
            } else {
                Err(())
            }
        }
        _ => Err(()),
    }
}

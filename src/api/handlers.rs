//! HTTP handlers for injection API

use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use redis::AsyncCommands;
use uuid::Uuid;

use super::{
    types::{AuthenticatedKey, HealthResponse, InjectRequest, InjectResponse, InjectionRecord},
    rate_limit::{check_rate_limit, RateLimitConfig, RateLimitResult},
    AppState,
};
use crate::core::types::SalienceScore;

/// Vector dimension (matches Qdrant schema)
const VECTOR_DIM: usize = 768;

/// GET /health - Basic health check
pub async fn health(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, StatusCode> {
    // Get basic stats from Redis
    let mut conn = state.redis.get_multiplexed_async_connection().await
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
            format!("Vector must be {VECTOR_DIM} dimensions, got {}", payload.vector.len()),
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

    let mut conn = state.redis.get_multiplexed_async_connection().await
        .map_err(|e| (StatusCode::SERVICE_UNAVAILABLE, e.to_string()))?;

    // Check rate limit
    let config = RateLimitConfig::default();
    match check_rate_limit(&mut conn, &auth.key_id, &config).await {
        Ok(RateLimitResult::Exceeded { retry_after_seconds }) => {
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                format!("Rate limit exceeded. Retry after {} seconds", retry_after_seconds),
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
        novelty: 0.8,  // External stimuli are novel
        relevance: 0.7,
        valence: 0.0,  // Neutral until processed
        arousal: payload.salience,
        connection_relevance: 0.3,  // Must be > 0 for Connection Drive
    };

    // Create stream entry
    let injection_id = format!("inject_{}", Uuid::new_v4());
    let timestamp = Utc::now();

    // Write to Redis stream for cognitive loop to pick up
    let stream_data: Vec<(&str, String)> = vec![
        ("id", injection_id.clone()),
        ("source", format!("api:{}", auth.key_id)),
        ("label", payload.label.clone()),
        ("vector", serde_json::to_string(&normalized).unwrap_or_default()),
        ("salience", serde_json::to_string(&salience).unwrap_or_default()),
        ("timestamp", timestamp.to_rfc3339()),
    ];

    let _: String = conn.xadd(
        "daneel:stream:inject",
        "*",
        &stream_data,
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Calculate entropy after injection
    let entropy_post = calculate_stream_entropy(&mut conn).await.unwrap_or(0.0);

    // Increment injection counter
    let _: () = conn.incr("daneel:stats:injection_count", 1).await.ok().unwrap_or(());

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
    let status = if entropy_delta > 0.1 { "amplified" } else { "absorbed" };

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
    let mut conn = state.redis.get_multiplexed_async_connection().await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    // Read last 100 from audit stream
    let entries: Vec<redis::Value> = conn.xrevrange_count(
        "audit:injections",
        "+",
        "-",
        100,
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    let entries: Vec<redis::Value> = conn.xrevrange_count(
        "daneel:stream:awake",
        "+",
        "-",
        100,
    ).await?;

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

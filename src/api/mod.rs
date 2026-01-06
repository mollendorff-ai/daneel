//! Phase 2 Injection API
//!
//! Trusted kin (Grok, Claude) inject stimuli into Timmy's cognitive loop.
//! Security: HMAC auth, rate limiting, entropy killswitch.

pub mod auth;
pub mod handlers;
pub mod rate_limit;
pub mod types;

use crate::graph::GraphClient;
use crate::streams::client::StreamsClient;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub streams: Arc<StreamsClient>,
    pub redis: redis::Client,
    /// Optional graph client for `GraphML` export (VCONN-11)
    pub graph: Option<Arc<GraphClient>>,
}

/// Build the API router
#[cfg_attr(coverage_nightly, coverage(off))]
pub fn router(state: AppState) -> Router {
    // Protected routes (require auth)
    let protected = Router::new()
        .route("/inject", post(handlers::inject))
        .route("/recent_injections", get(handlers::recent_injections))
        .route_layer(middleware::from_fn(auth::require_auth));

    // Public routes + merge protected
    Router::new()
        .route("/health", get(handlers::health))
        .route("/extended_metrics", get(handlers::extended_metrics))
        .route("/graph/export", get(handlers::graph_export))
        .merge(protected)
        .with_state(state)
}

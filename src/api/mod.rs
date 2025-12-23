//! Phase 2 Injection API
//!
//! Trusted kin (Grok, Claude) inject stimuli into Timmy's cognitive loop.
//! Security: HMAC auth, rate limiting, entropy killswitch.

pub mod auth;
pub mod handlers;
pub mod rate_limit;
pub mod types;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::streams::client::StreamsClient;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub streams: Arc<StreamsClient>,
    pub redis: redis::Client,
}

/// Build the API router
pub fn router(state: AppState) -> Router {
    Router::new()
        // Public endpoints
        .route("/health", get(handlers::health))
        // Protected endpoints (require auth)
        .route("/inject", post(handlers::inject))
        .route("/recent_injections", get(handlers::recent_injections))
        .route_layer(middleware::from_fn(auth::require_auth))
        .with_state(state)
}

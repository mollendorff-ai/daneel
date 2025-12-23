//! HMAC Bearer token authentication for kin injection API
//!
//! Security model from Grok's design:
//! - 256-bit HMAC keys (base64 encoded)
//! - Keys: GROK_KEY, CLAUDE_KEY
//! - Daily rotation (future)

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::env;

use super::types::AuthenticatedKey;

type HmacSha256 = Hmac<Sha256>;

/// Known API keys (loaded from environment)
#[derive(Clone)]
pub struct ApiKeys {
    grok_key: Option<Vec<u8>>,
    claude_key: Option<Vec<u8>>,
}

impl ApiKeys {
    /// Load keys from environment variables
    pub fn from_env() -> Self {
        Self {
            grok_key: env::var("GROK_INJECT_KEY")
                .ok()
                .and_then(|k| BASE64.decode(&k).ok()),
            claude_key: env::var("CLAUDE_INJECT_KEY")
                .ok()
                .and_then(|k| BASE64.decode(&k).ok()),
        }
    }

    /// Validate a bearer token and return the key ID if valid
    pub fn validate(&self, token: &str) -> Option<AuthenticatedKey> {
        // Token format: <key_id>:<signature>
        // Signature = HMAC-SHA256(key_id, secret)
        let parts: Vec<&str> = token.split(':').collect();
        if parts.len() != 2 {
            return None;
        }

        let key_id = parts[0];
        let provided_sig = match BASE64.decode(parts[1]) {
            Ok(s) => s,
            Err(_) => return None,
        };

        let (secret, holder) = match key_id {
            "GROK" => (self.grok_key.as_ref()?, "Grok (xAI)"),
            "CLAUDE" => (self.claude_key.as_ref()?, "Claude (Anthropic)"),
            _ => return None,
        };

        // Verify HMAC
        let mut mac = HmacSha256::new_from_slice(secret).ok()?;
        mac.update(key_id.as_bytes());

        if mac.verify_slice(&provided_sig).is_ok() {
            Some(AuthenticatedKey {
                key_id: key_id.to_string(),
                holder: holder.to_string(),
            })
        } else {
            None
        }
    }
}

/// Extract Bearer token from Authorization header
pub fn extract_bearer_token(req: &Request) -> Option<&str> {
    req.headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}

/// Auth middleware for protected endpoints
pub async fn require_auth(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let keys = ApiKeys::from_env();

    let token = extract_bearer_token(&req)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_key = keys.validate(token)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Store authenticated key in request extensions
    let mut req = req;
    req.extensions_mut().insert(auth_key);

    Ok(next.run(req).await)
}

/// Generate a signed token for a key (utility for key generation)
pub fn generate_token(key_id: &str, secret: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret)
        .expect("HMAC accepts any key size");
    mac.update(key_id.as_bytes());
    let sig = mac.finalize().into_bytes();
    format!("{}:{}", key_id, BASE64.encode(sig))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let secret = b"test_secret_key_32_bytes_long!!!";
        let token = generate_token("GROK", secret);

        let keys = ApiKeys {
            grok_key: Some(secret.to_vec()),
            claude_key: None,
        };

        let auth = keys.validate(&token);
        assert!(auth.is_some());
        assert_eq!(auth.unwrap().key_id, "GROK");
    }

    #[test]
    fn test_invalid_token_rejected() {
        let keys = ApiKeys {
            grok_key: Some(b"real_secret".to_vec()),
            claude_key: None,
        };

        let bad_token = "GROK:invalid_signature";
        assert!(keys.validate(bad_token).is_none());
    }
}

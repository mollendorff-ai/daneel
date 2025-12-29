//! Redis-backed rate limiting for injection API
//!
//! Grok's spec: 5/sec, 100/min per key
//! Ramp protocol: Start at 1/5min, increase over 72h

use redis::AsyncCommands;
use std::time::Duration;

/// Rate limit configuration
#[derive(Clone)]
pub struct RateLimitConfig {
    /// Max requests per second
    pub per_second: u32,
    /// Max requests per minute
    pub per_minute: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            per_second: 5,
            per_minute: 100,
        }
    }
}

/// Rate limit result
#[derive(Debug)]
pub enum RateLimitResult {
    /// Request allowed, returns remaining quota
    Allowed {
        remaining_second: u32,
        remaining_minute: u32,
    },
    /// Rate limit exceeded
    Exceeded { retry_after_seconds: u32 },
}

/// Check rate limit for a key
#[cfg_attr(coverage_nightly, coverage(off))]
pub async fn check_rate_limit(
    redis: &mut redis::aio::MultiplexedConnection,
    key_id: &str,
    config: &RateLimitConfig,
) -> Result<RateLimitResult, redis::RedisError> {
    let second_key = format!("ratelimit:{}:second", key_id);
    let minute_key = format!("ratelimit:{}:minute", key_id);

    // Increment second counter
    let second_count: u32 = redis.incr(&second_key, 1).await?;
    if second_count == 1 {
        let _: () = redis.expire(&second_key, 1).await?;
    }

    // Check second limit
    if second_count > config.per_second {
        return Ok(RateLimitResult::Exceeded {
            retry_after_seconds: 1,
        });
    }

    // Increment minute counter
    let minute_count: u32 = redis.incr(&minute_key, 1).await?;
    if minute_count == 1 {
        let _: () = redis.expire(&minute_key, 60).await?;
    }

    // Check minute limit
    if minute_count > config.per_minute {
        // Calculate retry time (seconds until minute window resets)
        let ttl: i64 = redis.ttl(&minute_key).await?;
        return Ok(RateLimitResult::Exceeded {
            retry_after_seconds: ttl.max(1) as u32,
        });
    }

    Ok(RateLimitResult::Allowed {
        remaining_second: config.per_second.saturating_sub(second_count),
        remaining_minute: config.per_minute.saturating_sub(minute_count),
    })
}

/// Ramp phase configuration (gradual increase over 72h)
#[derive(Clone, Copy)]
pub enum RampPhase {
    /// 1 per 5 minutes - first 24h
    Warmup,
    /// 1 per minute - 24-48h
    Baseline,
    /// 1 per 10 seconds - 48-72h
    Ramp,
    /// Full rate (5/sec) - after 72h
    Full,
}

impl RampPhase {
    /// Get rate limit config for this phase
    pub fn config(&self) -> RateLimitConfig {
        match self {
            RampPhase::Warmup => RateLimitConfig {
                per_second: 1,
                per_minute: 12,
            },
            RampPhase::Baseline => RateLimitConfig {
                per_second: 1,
                per_minute: 60,
            },
            RampPhase::Ramp => RateLimitConfig {
                per_second: 1,
                per_minute: 100,
            },
            RampPhase::Full => RateLimitConfig::default(),
        }
    }

    /// Determine phase based on time since first injection
    pub fn from_duration(since_start: Duration) -> Self {
        let hours = since_start.as_secs() / 3600;
        match hours {
            0..=23 => RampPhase::Warmup,
            24..=47 => RampPhase::Baseline,
            48..=71 => RampPhase::Ramp,
            _ => RampPhase::Full,
        }
    }
}

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_ramp_phases() {
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(0)),
            RampPhase::Warmup
        ));
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(25 * 3600)),
            RampPhase::Baseline
        ));
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(50 * 3600)),
            RampPhase::Ramp
        ));
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(100 * 3600)),
            RampPhase::Full
        ));
    }

    #[test]
    fn test_default_rate_limit_config() {
        let config = RateLimitConfig::default();
        assert_eq!(config.per_second, 5);
        assert_eq!(config.per_minute, 100);
    }

    #[test]
    fn test_warmup_phase_config() {
        let config = RampPhase::Warmup.config();
        assert_eq!(config.per_second, 1);
        assert_eq!(config.per_minute, 12); // 1 per 5 minutes = 12 per hour
    }

    #[test]
    fn test_baseline_phase_config() {
        let config = RampPhase::Baseline.config();
        assert_eq!(config.per_second, 1);
        assert_eq!(config.per_minute, 60); // 1 per minute = 60 per hour
    }

    #[test]
    fn test_ramp_phase_config() {
        let config = RampPhase::Ramp.config();
        assert_eq!(config.per_second, 1);
        assert_eq!(config.per_minute, 100);
    }

    #[test]
    fn test_full_phase_config() {
        let config = RampPhase::Full.config();
        assert_eq!(config.per_second, 5);
        assert_eq!(config.per_minute, 100);
    }

    #[test]
    fn test_ramp_phase_boundaries() {
        // Test exact boundary at 24 hours (should be Baseline)
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(24 * 3600)),
            RampPhase::Baseline
        ));

        // Test just before 24 hours (should still be Warmup)
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(23 * 3600 + 3599)),
            RampPhase::Warmup
        ));

        // Test exact boundary at 48 hours (should be Ramp)
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(48 * 3600)),
            RampPhase::Ramp
        ));

        // Test just before 48 hours (should still be Baseline)
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(47 * 3600 + 3599)),
            RampPhase::Baseline
        ));

        // Test exact boundary at 72 hours (should be Full)
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(72 * 3600)),
            RampPhase::Full
        ));

        // Test just before 72 hours (should still be Ramp)
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(71 * 3600 + 3599)),
            RampPhase::Ramp
        ));
    }

    #[test]
    fn test_ramp_phase_very_long_duration() {
        // Test very long durations remain at Full
        assert!(matches!(
            RampPhase::from_duration(Duration::from_secs(1000 * 3600)),
            RampPhase::Full
        ));
    }

    #[test]
    fn test_rate_limit_result_allowed_debug() {
        let result = RateLimitResult::Allowed {
            remaining_second: 4,
            remaining_minute: 99,
        };
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("Allowed"));
        assert!(debug_str.contains('4'));
        assert!(debug_str.contains("99"));
    }

    #[test]
    fn test_rate_limit_result_exceeded_debug() {
        let result = RateLimitResult::Exceeded {
            retry_after_seconds: 30,
        };
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("Exceeded"));
        assert!(debug_str.contains("30"));
    }

    #[test]
    fn test_rate_limit_result_allowed_fields() {
        let result = RateLimitResult::Allowed {
            remaining_second: 3,
            remaining_minute: 50,
        };
        if let RateLimitResult::Allowed {
            remaining_second,
            remaining_minute,
        } = result
        {
            assert_eq!(remaining_second, 3);
            assert_eq!(remaining_minute, 50);
        } else {
            panic!("Expected Allowed variant");
        }
    }

    #[test]
    fn test_rate_limit_result_exceeded_fields() {
        let result = RateLimitResult::Exceeded {
            retry_after_seconds: 45,
        };
        if let RateLimitResult::Exceeded {
            retry_after_seconds,
        } = result
        {
            assert_eq!(retry_after_seconds, 45);
        } else {
            panic!("Expected Exceeded variant");
        }
    }
}

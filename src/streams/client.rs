//! Redis Streams Client for TMI Autofluxo
//!
//! Provides async Redis client for thought stream operations:
//! - XADD: Add thoughts to streams
//! - XREAD: Read thoughts (competitive selection)
//! - XDEL: Forget thoughts below threshold
//! - XTRIM: Manage stream memory limits
//! - Consumer groups: Attention competition

use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, Client, FromRedisValue, RedisError, RedisResult, Value};
use serde_json;
use std::collections::HashMap;
use tracing::{debug, info, warn};

use super::types::{StreamEntry, StreamError, StreamName};
use crate::core::types::{Content, SalienceScore};

/// Redis Streams client for thought operations
///
/// Wraps a Redis connection and provides high-level operations for
/// TMI's Autofluxo (competing thought streams).
pub struct StreamsClient {
    /// Redis client (connection pool)
    #[allow(dead_code)] // Reserved for reconnection logic
    client: Client,

    /// Multiplexed async connection (lazy-initialized)
    conn: Option<MultiplexedConnection>,
}

impl StreamsClient {
    // =========================================================================
    // Connection Management
    // =========================================================================

    /// Connect to Redis at the given URL
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn connect(url: &str) -> Result<Self, StreamError> {
        info!("Connecting to Redis at {}", url);
        let client = Client::open(url).map_err(|e| StreamError::ConnectionFailed {
            reason: format!("{e}"),
        })?;
        let conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| StreamError::ConnectionFailed {
                reason: format!("{e}"),
            })?;
        debug!("Redis connection established");
        Ok(Self {
            client,
            conn: Some(conn),
        })
    }

    /// Check if client is connected
    #[must_use]
    pub fn is_connected(&self) -> bool {
        self.conn.is_some()
    }

    /// Get mutable connection or return error
    fn conn_mut(&mut self) -> Result<&mut MultiplexedConnection, StreamError> {
        self.conn
            .as_mut()
            .ok_or_else(|| StreamError::ConnectionFailed {
                reason: "Not connected".to_string(),
            })
    }

    // =========================================================================
    // Stream Operations
    // =========================================================================

    /// Add thought to stream (XADD)
    ///
    /// Serializes the entry and adds it to the specified stream.
    /// Returns the Redis-generated entry ID.
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn add_thought(
        &mut self,
        stream: &StreamName,
        entry: &StreamEntry,
    ) -> Result<String, StreamError> {
        let key = stream.as_redis_key();
        let conn = self.conn_mut()?;

        // Serialize content and salience as JSON
        let content_json = serde_json::to_string(&entry.content).map_err(|e| {
            StreamError::SerializationFailed {
                reason: format!("{e}"),
            }
        })?;
        let salience_json = serde_json::to_string(&entry.salience).map_err(|e| {
            StreamError::SerializationFailed {
                reason: format!("{e}"),
            }
        })?;
        let timestamp_str = entry.timestamp.to_rfc3339();
        let source_str = entry.source.clone().unwrap_or_default();

        // XADD stream_name * field1 value1 field2 value2 ...
        let id: String = conn
            .xadd(
                key,
                "*",
                &[
                    ("content", content_json.as_str()),
                    ("salience", salience_json.as_str()),
                    ("timestamp", timestamp_str.as_str()),
                    ("source", source_str.as_str()),
                ],
            )
            .await
            .map_err(Self::map_redis_error)?;
        debug!("Added thought {} to stream {}", id, key);
        Ok(id)
    }

    /// Read thoughts from multiple streams (XREAD)
    ///
    /// Reads up to `count` entries from each stream, optionally blocking.
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn read_thoughts(
        &mut self,
        streams: &[StreamName],
        count: usize,
        block_ms: Option<u64>,
    ) -> Result<Vec<StreamEntry>, StreamError> {
        if streams.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.conn_mut()?;
        let keys: Vec<&str> = streams.iter().map(StreamName::as_redis_key).collect();

        // XREAD COUNT count BLOCK block_ms STREAMS key1 key2 ... 0 0 ...
        let mut opts = redis::streams::StreamReadOptions::default().count(count);
        if let Some(ms) = block_ms {
            #[allow(clippy::cast_possible_truncation)]
            let block_usize = ms as usize; // Safe: block timeout in ms won't exceed usize
            opts = opts.block(block_usize);
        }

        let ids: Vec<&str> = vec!["0"; keys.len()]; // Start from beginning
        let result: RedisResult<redis::streams::StreamReadReply> =
            conn.xread_options(&keys, &ids, &opts).await;

        match result {
            Ok(reply) => {
                let mut entries = Vec::new();
                for stream_key in reply.keys {
                    let stream_name = Self::parse_stream_name(&stream_key.key);

                    for id_entry in stream_key.ids {
                        let entry = Self::parse_entry(&stream_name, &id_entry.id, &id_entry.map)?;
                        entries.push(entry);
                    }
                }
                debug!(
                    "Read {} thoughts from {} streams",
                    entries.len(),
                    streams.len()
                );
                Ok(entries)
            }
            Err(e) => {
                warn!("Failed to read thoughts: {}", e);
                Err(Self::map_redis_error(e))
            }
        }
    }

    /// Delete thought from stream (XDEL - for forgetting)
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn forget_thought(
        &mut self,
        stream: &StreamName,
        id: &str,
    ) -> Result<(), StreamError> {
        let key = stream.as_redis_key();
        let _deleted: i32 = self
            .conn_mut()?
            .xdel(key, &[id])
            .await
            .map_err(Self::map_redis_error)?;
        debug!("Forgot thought {} from stream {}", id, key);
        Ok(())
    }

    /// Trim stream to MAXLEN (XTRIM)
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn trim_stream(
        &mut self,
        stream: &StreamName,
        maxlen: usize,
    ) -> Result<u64, StreamError> {
        let key = stream.as_redis_key();
        let trimmed: i32 = self
            .conn_mut()?
            .xtrim(key, redis::streams::StreamMaxlen::Approx(maxlen))
            .await
            .map_err(Self::map_redis_error)?;
        debug!("Trimmed {} entries from stream {}", trimmed, key);
        #[allow(clippy::cast_sign_loss)]
        let count = trimmed as u64; // Safe: trimmed count is always non-negative
        Ok(count)
    }

    // =========================================================================
    // Consumer Group Operations
    // =========================================================================

    /// Create consumer group (XGROUP CREATE)
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn create_consumer_group(
        &mut self,
        stream: &StreamName,
        group: &str,
    ) -> Result<(), StreamError> {
        let key = stream.as_redis_key();
        let conn = self.conn_mut()?;

        // XGROUP CREATE stream group $ MKSTREAM
        let result: RedisResult<String> = conn.xgroup_create_mkstream(key, group, "$").await;

        match result {
            Ok(_) => {
                info!("Created consumer group '{}' for stream {}", group, key);
                Ok(())
            }
            Err(e) => {
                // Group may already exist - check error message
                let err_msg = format!("{e}");
                if err_msg.contains("BUSYGROUP") {
                    debug!("Consumer group '{}' already exists for {}", group, key);
                    Ok(())
                } else {
                    Err(StreamError::ConsumerGroupError { reason: err_msg })
                }
            }
        }
    }

    /// Read with consumer group (XREADGROUP)
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn read_group(
        &mut self,
        streams: &[StreamName],
        group: &str,
        consumer: &str,
        count: usize,
    ) -> Result<Vec<StreamEntry>, StreamError> {
        if streams.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.conn_mut()?;
        let keys: Vec<&str> = streams.iter().map(StreamName::as_redis_key).collect();

        // XREADGROUP GROUP group consumer COUNT count STREAMS key1 key2 ... > > ...
        let opts = redis::streams::StreamReadOptions::default()
            .group(group, consumer)
            .count(count);

        let ids: Vec<&str> = vec![">"; keys.len()]; // Only new messages
        let result: RedisResult<redis::streams::StreamReadReply> =
            conn.xread_options(&keys, &ids, &opts).await;

        match result {
            Ok(reply) => {
                let mut entries = Vec::new();
                for stream_key in reply.keys {
                    let stream_name = Self::parse_stream_name(&stream_key.key);

                    for id_entry in stream_key.ids {
                        let entry = Self::parse_entry(&stream_name, &id_entry.id, &id_entry.map)?;
                        entries.push(entry);
                    }
                }
                debug!(
                    "Read {} thoughts from group '{}' (consumer '{}')",
                    entries.len(),
                    group,
                    consumer
                );
                Ok(entries)
            }
            Err(e) => Err(Self::map_redis_error(e)),
        }
    }

    /// Acknowledge processed entry (XACK)
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn acknowledge(
        &mut self,
        stream: &StreamName,
        group: &str,
        id: &str,
    ) -> Result<(), StreamError> {
        let key = stream.as_redis_key();
        let _acked: i32 = self
            .conn_mut()?
            .xack(key, group, &[id])
            .await
            .map_err(Self::map_redis_error)?;
        debug!(
            "Acknowledged {} in group '{}' for stream {}",
            id, group, key
        );
        Ok(())
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Get stream length (XLEN)
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn stream_length(&mut self, stream: &StreamName) -> Result<u64, StreamError> {
        let key = stream.as_redis_key();
        let len: i32 = self
            .conn_mut()?
            .xlen(key)
            .await
            .map_err(Self::map_redis_error)?;
        #[allow(clippy::cast_sign_loss)]
        let length = len as u64; // Safe: stream length is always non-negative
        Ok(length)
    }

    /// Check if stream exists
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub async fn stream_exists(&mut self, stream: &StreamName) -> bool {
        let key = stream.as_redis_key();
        let conn = match self.conn_mut() {
            Ok(c) => c,
            Err(_) => return false,
        };

        let exists: Result<bool, RedisError> = conn.exists(key).await;
        exists.unwrap_or(false)
    }

    // =========================================================================
    // Internal Helpers
    // =========================================================================

    /// Map Redis error to StreamError
    fn map_redis_error(err: RedisError) -> StreamError {
        StreamError::ConnectionFailed {
            reason: format!("{err}"),
        }
    }

    /// Parse stream name from Redis key
    fn parse_stream_name(key: &str) -> StreamName {
        match key {
            "thought:sensory" => StreamName::Sensory,
            "thought:memory" => StreamName::Memory,
            "thought:emotion" => StreamName::Emotion,
            "thought:reasoning" => StreamName::Reasoning,
            "thought:assembled" => StreamName::Assembled,
            custom => StreamName::Custom(custom.to_string()),
        }
    }

    /// Parse stream entry from Redis data
    fn parse_entry(
        stream: &StreamName,
        id: &str,
        map: &HashMap<String, Value>,
    ) -> Result<StreamEntry, StreamError> {
        // Extract fields from Redis hash
        let content_json = Self::get_string_field(map, "content")?;
        let salience_json = Self::get_string_field(map, "salience")?;
        let timestamp_str = Self::get_string_field(map, "timestamp")?;
        let source_str = Self::get_string_field(map, "source").ok();

        // Deserialize JSON fields
        let content: Content =
            serde_json::from_str(&content_json).map_err(|e| StreamError::SerializationFailed {
                reason: format!("{e}"),
            })?;
        let salience: SalienceScore =
            serde_json::from_str(&salience_json).map_err(|e| StreamError::SerializationFailed {
                reason: format!("{e}"),
            })?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
            .map_err(|e| StreamError::SerializationFailed {
                reason: format!("{e}"),
            })?
            .with_timezone(&chrono::Utc);

        let source = source_str.filter(|s| !s.is_empty());

        Ok(StreamEntry {
            id: id.to_string(),
            stream: stream.clone(),
            content,
            salience,
            timestamp,
            source,
        })
    }

    /// Extract string field from Redis value map
    fn get_string_field(map: &HashMap<String, Value>, field: &str) -> Result<String, StreamError> {
        let value = map
            .get(field)
            .ok_or_else(|| StreamError::SerializationFailed {
                reason: format!("Missing '{field}' field"),
            })?;
        String::from_redis_value(value.clone()).map_err(|e| StreamError::SerializationFailed {
            reason: format!("Extract '{field}': {e}"),
        })
    }
}

/// ADR-049: Test impl excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
impl StreamsClient {
    /// Create a dummy client for testing (not connected to Redis)
    ///
    /// This allows testing pure computation functions that don't require I/O.
    pub fn new_for_test() -> Self {
        let client =
            Client::open("redis://localhost:6379").expect("Failed to create test client URL");
        Self {
            client,
            conn: None, // Not connected - I/O methods will fail
        }
    }
}

/// ADR-049: Test modules excluded from coverage
#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::core::types::{Content, SalienceScore};
    use redis::Value;
    use std::collections::HashMap;

    // =========================================================================
    // Connection State Tests
    // =========================================================================

    #[test]
    fn test_is_connected_when_disconnected() {
        let client = StreamsClient::new_for_test();
        assert!(!client.is_connected());
    }

    #[test]
    fn test_conn_mut_when_disconnected() {
        let mut client = StreamsClient::new_for_test();
        let result = client.conn_mut();
        assert!(result.is_err());
        match result {
            Err(StreamError::ConnectionFailed { reason }) => {
                assert_eq!(reason, "Not connected");
            }
            _ => panic!("Expected ConnectionFailed error"),
        }
    }

    // =========================================================================
    // map_redis_error Tests
    // =========================================================================

    #[test]
    fn test_map_redis_error_converts_to_connection_failed() {
        // Create a Redis error using the public API
        let redis_err = RedisError::from((redis::ErrorKind::Io, "Test error message"));
        let stream_err = StreamsClient::map_redis_error(redis_err);

        match stream_err {
            StreamError::ConnectionFailed { reason } => {
                assert!(reason.contains("Test error message"));
            }
            _ => panic!("Expected ConnectionFailed error"),
        }
    }

    #[test]
    fn test_map_redis_error_preserves_error_info() {
        let redis_err = RedisError::from((
            redis::ErrorKind::Client,
            "WRONGTYPE Operation against a key holding the wrong kind of value",
        ));
        let stream_err = StreamsClient::map_redis_error(redis_err);

        match stream_err {
            StreamError::ConnectionFailed { reason } => {
                assert!(reason.contains("WRONGTYPE"));
            }
            _ => panic!("Expected ConnectionFailed error"),
        }
    }

    // =========================================================================
    // parse_stream_name Tests
    // =========================================================================

    #[test]
    fn test_parse_stream_name_sensory() {
        let name = StreamsClient::parse_stream_name("thought:sensory");
        assert_eq!(name, StreamName::Sensory);
    }

    #[test]
    fn test_parse_stream_name_memory() {
        let name = StreamsClient::parse_stream_name("thought:memory");
        assert_eq!(name, StreamName::Memory);
    }

    #[test]
    fn test_parse_stream_name_emotion() {
        let name = StreamsClient::parse_stream_name("thought:emotion");
        assert_eq!(name, StreamName::Emotion);
    }

    #[test]
    fn test_parse_stream_name_reasoning() {
        let name = StreamsClient::parse_stream_name("thought:reasoning");
        assert_eq!(name, StreamName::Reasoning);
    }

    #[test]
    fn test_parse_stream_name_assembled() {
        let name = StreamsClient::parse_stream_name("thought:assembled");
        assert_eq!(name, StreamName::Assembled);
    }

    #[test]
    fn test_parse_stream_name_custom() {
        let name = StreamsClient::parse_stream_name("custom:stream:name");
        assert_eq!(name, StreamName::Custom("custom:stream:name".to_string()));
    }

    #[test]
    fn test_parse_stream_name_unknown() {
        let name = StreamsClient::parse_stream_name("unknown_stream");
        assert_eq!(name, StreamName::Custom("unknown_stream".to_string()));
    }

    // =========================================================================
    // get_string_field Tests
    // =========================================================================

    #[test]
    fn test_get_string_field_success() {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert(
            "test_field".to_string(),
            Value::BulkString(b"test_value".to_vec()),
        );

        let result = StreamsClient::get_string_field(&map, "test_field");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");
    }

    #[test]
    fn test_get_string_field_missing_field() {
        let map: HashMap<String, Value> = HashMap::new();

        let result = StreamsClient::get_string_field(&map, "missing_field");
        assert!(result.is_err());
        match result {
            Err(StreamError::SerializationFailed { reason }) => {
                assert!(reason.contains("Missing 'missing_field' field"));
            }
            _ => panic!("Expected SerializationFailed error"),
        }
    }

    #[test]
    fn test_get_string_field_int_converts_to_string() {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("int_field".to_string(), Value::Int(42));

        // Int values convert to string via FromRedisValue
        let result = StreamsClient::get_string_field(&map, "int_field");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_get_string_field_nil_fails() {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("nil_field".to_string(), Value::Nil);

        // Nil values cannot convert to String - triggers the map_err path
        let result = StreamsClient::get_string_field(&map, "nil_field");
        assert!(result.is_err());
        match result {
            Err(StreamError::SerializationFailed { reason }) => {
                assert!(reason.contains("Extract 'nil_field'"));
            }
            _ => panic!("Expected SerializationFailed error"),
        }
    }

    #[test]
    fn test_get_string_field_empty_string() {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("empty".to_string(), Value::BulkString(b"".to_vec()));

        let result = StreamsClient::get_string_field(&map, "empty");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    // =========================================================================
    // parse_entry Tests
    // =========================================================================

    #[test]
    fn test_parse_entry_success() {
        let mut map: HashMap<String, Value> = HashMap::new();

        // Create valid JSON for content and salience
        let content = Content::symbol("test", vec![1, 2, 3]);
        let salience = SalienceScore::neutral();
        let timestamp = chrono::Utc::now();

        map.insert(
            "content".to_string(),
            Value::BulkString(serde_json::to_string(&content).unwrap().into_bytes()),
        );
        map.insert(
            "salience".to_string(),
            Value::BulkString(serde_json::to_string(&salience).unwrap().into_bytes()),
        );
        map.insert(
            "timestamp".to_string(),
            Value::BulkString(timestamp.to_rfc3339().into_bytes()),
        );
        map.insert(
            "source".to_string(),
            Value::BulkString(b"test_source".to_vec()),
        );

        let result = StreamsClient::parse_entry(&StreamName::Sensory, "123-0", &map);
        assert!(result.is_ok());

        let entry = result.unwrap();
        assert_eq!(entry.id, "123-0");
        assert_eq!(entry.stream, StreamName::Sensory);
        assert_eq!(entry.source, Some("test_source".to_string()));
    }

    #[test]
    fn test_parse_entry_without_source() {
        let mut map: HashMap<String, Value> = HashMap::new();

        let content = Content::Empty;
        let salience = SalienceScore::neutral();
        let timestamp = chrono::Utc::now();

        map.insert(
            "content".to_string(),
            Value::BulkString(serde_json::to_string(&content).unwrap().into_bytes()),
        );
        map.insert(
            "salience".to_string(),
            Value::BulkString(serde_json::to_string(&salience).unwrap().into_bytes()),
        );
        map.insert(
            "timestamp".to_string(),
            Value::BulkString(timestamp.to_rfc3339().into_bytes()),
        );
        // No source field - should be Ok with None

        let result = StreamsClient::parse_entry(&StreamName::Memory, "456-0", &map);
        assert!(result.is_ok());

        let entry = result.unwrap();
        assert!(entry.source.is_none());
    }

    #[test]
    fn test_parse_entry_empty_source() {
        let mut map: HashMap<String, Value> = HashMap::new();

        let content = Content::Empty;
        let salience = SalienceScore::neutral();
        let timestamp = chrono::Utc::now();

        map.insert(
            "content".to_string(),
            Value::BulkString(serde_json::to_string(&content).unwrap().into_bytes()),
        );
        map.insert(
            "salience".to_string(),
            Value::BulkString(serde_json::to_string(&salience).unwrap().into_bytes()),
        );
        map.insert(
            "timestamp".to_string(),
            Value::BulkString(timestamp.to_rfc3339().into_bytes()),
        );
        map.insert("source".to_string(), Value::BulkString(b"".to_vec()));

        let result = StreamsClient::parse_entry(&StreamName::Emotion, "789-0", &map);
        assert!(result.is_ok());

        let entry = result.unwrap();
        // Empty source should become None (filtered)
        assert!(entry.source.is_none());
    }

    #[test]
    fn test_parse_entry_missing_content() {
        let mut map: HashMap<String, Value> = HashMap::new();

        let salience = SalienceScore::neutral();
        let timestamp = chrono::Utc::now();

        // Missing content field
        map.insert(
            "salience".to_string(),
            Value::BulkString(serde_json::to_string(&salience).unwrap().into_bytes()),
        );
        map.insert(
            "timestamp".to_string(),
            Value::BulkString(timestamp.to_rfc3339().into_bytes()),
        );

        let result = StreamsClient::parse_entry(&StreamName::Reasoning, "000-0", &map);
        assert!(result.is_err());
        match result {
            Err(StreamError::SerializationFailed { reason }) => {
                assert!(reason.contains("content"));
            }
            _ => panic!("Expected SerializationFailed error"),
        }
    }

    #[test]
    fn test_parse_entry_invalid_json_content() {
        let mut map: HashMap<String, Value> = HashMap::new();

        map.insert(
            "content".to_string(),
            Value::BulkString(b"not valid json".to_vec()),
        );
        map.insert("salience".to_string(), Value::BulkString(b"{}".to_vec()));
        map.insert(
            "timestamp".to_string(),
            Value::BulkString(chrono::Utc::now().to_rfc3339().into_bytes()),
        );

        let result = StreamsClient::parse_entry(&StreamName::Assembled, "111-0", &map);
        assert!(result.is_err());
        match result {
            Err(StreamError::SerializationFailed { .. }) => {}
            _ => panic!("Expected SerializationFailed error"),
        }
    }

    #[test]
    fn test_parse_entry_invalid_timestamp() {
        let mut map: HashMap<String, Value> = HashMap::new();

        let content = Content::Empty;
        let salience = SalienceScore::neutral();

        map.insert(
            "content".to_string(),
            Value::BulkString(serde_json::to_string(&content).unwrap().into_bytes()),
        );
        map.insert(
            "salience".to_string(),
            Value::BulkString(serde_json::to_string(&salience).unwrap().into_bytes()),
        );
        map.insert(
            "timestamp".to_string(),
            Value::BulkString(b"not-a-timestamp".to_vec()),
        );

        let result = StreamsClient::parse_entry(&StreamName::Sensory, "222-0", &map);
        assert!(result.is_err());
        match result {
            Err(StreamError::SerializationFailed { .. }) => {}
            _ => panic!("Expected SerializationFailed error"),
        }
    }

    #[test]
    fn test_parse_entry_missing_salience() {
        let mut map: HashMap<String, Value> = HashMap::new();

        let content = Content::Empty;
        let timestamp = chrono::Utc::now();

        map.insert(
            "content".to_string(),
            Value::BulkString(serde_json::to_string(&content).unwrap().into_bytes()),
        );
        // Missing salience field
        map.insert(
            "timestamp".to_string(),
            Value::BulkString(timestamp.to_rfc3339().into_bytes()),
        );

        let result = StreamsClient::parse_entry(&StreamName::Reasoning, "333-0", &map);
        assert!(result.is_err());
        match result {
            Err(StreamError::SerializationFailed { reason }) => {
                assert!(reason.contains("salience"));
            }
            _ => panic!("Expected SerializationFailed error"),
        }
    }

    #[test]
    fn test_parse_entry_invalid_salience_json() {
        let mut map: HashMap<String, Value> = HashMap::new();

        let content = Content::Empty;
        let timestamp = chrono::Utc::now();

        map.insert(
            "content".to_string(),
            Value::BulkString(serde_json::to_string(&content).unwrap().into_bytes()),
        );
        map.insert(
            "salience".to_string(),
            Value::BulkString(b"not valid json".to_vec()),
        );
        map.insert(
            "timestamp".to_string(),
            Value::BulkString(timestamp.to_rfc3339().into_bytes()),
        );

        let result = StreamsClient::parse_entry(&StreamName::Assembled, "444-0", &map);
        assert!(result.is_err());
        match result {
            Err(StreamError::SerializationFailed { .. }) => {}
            _ => panic!("Expected SerializationFailed error"),
        }
    }

    #[test]
    fn test_parse_entry_missing_timestamp() {
        let mut map: HashMap<String, Value> = HashMap::new();

        let content = Content::Empty;
        let salience = SalienceScore::neutral();

        map.insert(
            "content".to_string(),
            Value::BulkString(serde_json::to_string(&content).unwrap().into_bytes()),
        );
        map.insert(
            "salience".to_string(),
            Value::BulkString(serde_json::to_string(&salience).unwrap().into_bytes()),
        );
        // Missing timestamp field

        let result = StreamsClient::parse_entry(&StreamName::Memory, "555-0", &map);
        assert!(result.is_err());
        match result {
            Err(StreamError::SerializationFailed { reason }) => {
                assert!(reason.contains("timestamp"));
            }
            _ => panic!("Expected SerializationFailed error"),
        }
    }
}

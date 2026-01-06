//! `RedisGraph` Client Module (ADR-046)
//!
//! Provides graph-based operations for memory associations.
//! Complements Qdrant by providing global graph traversal and visualization.
//!
//! # Architecture
//!
//! - Qdrant: Source of truth for memory payloads and vectors.
//! - `RedisGraph`: High-speed association graph for traversal and emergence analysis.

use crate::memory_db::types::{AssociationType, MemoryId};
use redis::{Client, RedisError};
use thiserror::Error;

/// Graph database errors
#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Graph not found: {0}")]
    GraphNotFound(String),
}

/// Result type for graph operations
pub type Result<T> = std::result::Result<T, GraphError>;

/// `RedisGraph` client
pub struct GraphClient {
    client: Client,
    graph_name: String,
}

impl GraphClient {
    /// Create a new `GraphClient`
    ///
    /// # Arguments
    ///
    /// * `redis_url` - Redis connection URL
    /// * `graph_name` - Name of the graph in `RedisGraph`
    ///
    /// # Errors
    ///
    /// Returns error if Redis connection fails.
    pub fn connect(redis_url: &str, graph_name: &str) -> Result<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            client,
            graph_name: graph_name.to_string(),
        })
    }

    /// Merge an edge between two memories
    ///
    /// Creates nodes if they don't exist and updates the edge weight.
    ///
    /// # Errors
    ///
    /// Returns error if Redis command fails.
    pub async fn merge_edge(
        &self,
        source_id: &MemoryId,
        target_id: &MemoryId,
        weight: f32,
        assoc_type: AssociationType,
    ) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let source_uuid = source_id.0.to_string();
        let target_uuid = target_id.0.to_string();
        let type_str = format!("{assoc_type:?}");

        // Cypher query to merge nodes and relationship
        let query = format!(
            "MERGE (a:Memory {{id: '{source_uuid}'}}) \
                 MERGE (b:Memory {{id: '{target_uuid}'}}) \
                 MERGE (a)-[r:ASSOCIATED {{type: '{type_str}'}}]->(b) \
                 SET r.weight = {weight}"
        );

        let _: () = redis::cmd("GRAPH.QUERY")
            .arg(&self.graph_name)
            .arg(query)
            .query_async(&mut conn)
            .await?;

        Ok(())
    }

    /// Query neighbors of a memory (outgoing edges only)
    ///
    /// # Errors
    ///
    /// Returns error if Redis command fails.
    pub async fn query_neighbors(
        &self,
        memory_id: &MemoryId,
        min_weight: f32,
    ) -> Result<Vec<(MemoryId, f32)>> {
        self.query_neighbors_directed(memory_id, min_weight, false)
            .await
    }

    /// Query neighbors of a memory with direction control (VCONN-12)
    ///
    /// When `bidirectional` is true, returns both outgoing and incoming neighbors.
    ///
    /// # Errors
    ///
    /// Returns error if Redis command fails.
    pub async fn query_neighbors_directed(
        &self,
        memory_id: &MemoryId,
        min_weight: f32,
        bidirectional: bool,
    ) -> Result<Vec<(MemoryId, f32)>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let uuid_str = memory_id.0.to_string();

        // Build query based on direction
        let query = if bidirectional {
            // Match both directions using UNION
            format!(
                "MATCH (a:Memory {{id: '{uuid_str}'}})-[r:ASSOCIATED]->(b:Memory) \
                 WHERE r.weight >= {min_weight} \
                 RETURN b.id, r.weight \
                 UNION \
                 MATCH (a:Memory {{id: '{uuid_str}'}})<-[r:ASSOCIATED]-(b:Memory) \
                 WHERE r.weight >= {min_weight} \
                 RETURN b.id, r.weight"
            )
        } else {
            // Outgoing only (original behavior)
            format!(
                "MATCH (a:Memory {{id: '{uuid_str}'}})-[r:ASSOCIATED]->(b:Memory) \
                 WHERE r.weight >= {min_weight} \
                 RETURN b.id, r.weight"
            )
        };

        // RedisGraph returns: Array([headers, rows, statistics])
        let result: redis::Value = redis::cmd("GRAPH.QUERY")
            .arg(&self.graph_name)
            .arg(query)
            .query_async(&mut conn)
            .await?;

        let mut neighbors = Vec::new();

        // Parse RedisGraph response structure
        if let redis::Value::Array(sections) = result {
            if sections.len() >= 2 {
                // sections[1] is the result rows
                if let redis::Value::Array(ref rows) = sections[1] {
                    for row in rows {
                        if let redis::Value::Array(ref fields) = row {
                            if fields.len() >= 2 {
                                // field[0] = b.id (string), field[1] = r.weight (double/string)
                                let id_opt = Self::extract_string(&fields[0]);
                                let weight_opt = Self::extract_float(&fields[1]);

                                if let (Some(id_str), Some(weight)) = (id_opt, weight_opt) {
                                    if let Ok(uuid) = uuid::Uuid::parse_str(&id_str) {
                                        neighbors.push((MemoryId(uuid), weight));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(neighbors)
    }

    /// Extract string from Redis Value
    fn extract_string(value: &redis::Value) -> Option<String> {
        match value {
            redis::Value::BulkString(bytes) => String::from_utf8(bytes.clone()).ok(),
            redis::Value::SimpleString(s) => Some(s.clone()),
            redis::Value::Array(items) if !items.is_empty() => {
                // Sometimes scalars are wrapped
                Self::extract_string(&items[0])
            }
            _ => None,
        }
    }

    /// Extract float from Redis Value
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn extract_float(value: &redis::Value) -> Option<f32> {
        match value {
            redis::Value::Double(d) => Some(*d as f32),
            redis::Value::Int(i) => Some(*i as f32),
            redis::Value::BulkString(bytes) => String::from_utf8(bytes.clone())
                .ok()
                .and_then(|s| s.parse::<f32>().ok()),
            redis::Value::SimpleString(s) => s.parse::<f32>().ok(),
            redis::Value::Array(items) if !items.is_empty() => Self::extract_float(&items[0]),
            _ => None,
        }
    }

    /// Export graph to `GraphML` format for Gephi (VCONN-8)
    ///
    /// Queries all nodes and edges from `RedisGraph` and serializes to `GraphML` XML.
    ///
    /// # Errors
    ///
    /// Returns error if Redis query fails.
    pub async fn export_graphml(&self) -> Result<String> {
        use std::fmt::Write;

        let mut conn = self.client.get_multiplexed_async_connection().await?;

        // Query all nodes
        let nodes_query = "MATCH (n:Memory) RETURN n.id";
        let nodes_result: redis::Value = redis::cmd("GRAPH.QUERY")
            .arg(&self.graph_name)
            .arg(nodes_query)
            .query_async(&mut conn)
            .await?;

        let mut node_ids: Vec<String> = Vec::new();
        if let redis::Value::Array(sections) = &nodes_result {
            if sections.len() >= 2 {
                if let redis::Value::Array(ref rows) = sections[1] {
                    for row in rows {
                        if let redis::Value::Array(ref fields) = row {
                            if let Some(id) = fields.first().and_then(Self::extract_string) {
                                node_ids.push(id);
                            }
                        }
                    }
                }
            }
        }

        // Query all edges
        let edges_query =
            "MATCH (a:Memory)-[r:ASSOCIATED]->(b:Memory) RETURN a.id, b.id, r.weight, r.type";
        let edges_result: redis::Value = redis::cmd("GRAPH.QUERY")
            .arg(&self.graph_name)
            .arg(edges_query)
            .query_async(&mut conn)
            .await?;

        let mut edges: Vec<(String, String, f32, String)> = Vec::new();
        if let redis::Value::Array(sections) = &edges_result {
            if sections.len() >= 2 {
                if let redis::Value::Array(ref rows) = sections[1] {
                    for row in rows {
                        if let redis::Value::Array(ref fields) = row {
                            if fields.len() >= 4 {
                                let source = Self::extract_string(&fields[0]);
                                let target = Self::extract_string(&fields[1]);
                                let weight = Self::extract_float(&fields[2]).unwrap_or(0.0);
                                let edge_type = Self::extract_string(&fields[3])
                                    .unwrap_or_else(|| "Unknown".to_string());

                                if let (Some(s), Some(t)) = (source, target) {
                                    edges.push((s, t, weight, edge_type));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Build GraphML XML
        let mut xml = String::new();
        xml.push_str(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<graphml xmlns="http://graphml.graphdrawing.org/xmlns"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://graphml.graphdrawing.org/xmlns
         http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd">
  <key id="weight" for="edge" attr.name="weight" attr.type="double"/>
  <key id="type" for="edge" attr.name="type" attr.type="string"/>
  <graph id="daneel" edgedefault="directed">
"#,
        );

        // Add nodes
        for id in &node_ids {
            let _ = writeln!(xml, "    <node id=\"{id}\"/>");
        }

        // Add edges
        for (i, (source, target, weight, edge_type)) in edges.iter().enumerate() {
            let _ = writeln!(
                xml,
                "    <edge id=\"e{i}\" source=\"{source}\" target=\"{target}\">"
            );
            let _ = writeln!(xml, "      <data key=\"weight\">{weight}</data>");
            let _ = writeln!(xml, "      <data key=\"type\">{edge_type}</data>");
            xml.push_str("    </edge>\n");
        }

        xml.push_str("  </graph>\n</graphml>\n");

        tracing::info!(
            nodes = node_ids.len(),
            edges = edges.len(),
            "Exported graph to GraphML"
        );

        Ok(xml)
    }
}
impl std::fmt::Debug for GraphClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphClient")
            .field("client", &self.client)
            .field("graph_name", &self.graph_name)
            .finish()
    }
}

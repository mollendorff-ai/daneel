//! Unconscious memory and sleep cycle types (ADR-033, ADR-034)
//!
//! Types for Timmy's unconscious storage and identity metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::MemoryId;

/// Sleep cycle record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepCycle {
    /// Unique identifier
    pub id: Uuid,

    /// When cycle started
    pub started_at: DateTime<Utc>,

    /// When cycle ended
    pub ended_at: Option<DateTime<Utc>>,

    /// Number of memories replayed
    pub memories_replayed: u32,

    /// Number of memories that reached permanent status
    pub memories_consolidated: u32,

    /// Number of associations strengthened
    pub associations_strengthened: u32,

    /// Number of weak associations pruned
    pub associations_pruned: u32,

    /// Average replay priority of batch
    pub avg_replay_priority: f32,

    /// Cycle status
    pub status: SleepCycleStatus,
}

/// Sleep cycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepCycleStatus {
    InProgress,
    Completed,
    Interrupted,
}

/// Reason why a thought was archived to the unconscious (ADR-033)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveReason {
    /// Low salience - below forget threshold
    LowSalience,
    /// Decay over time
    Decay,
    /// Displacement by higher-salience thought
    Displacement,
}

/// An archived thought in Timmy's unconscious (ADR-033)
///
/// TMI: "Nada se apaga na memória" - Nothing is erased from memory.
/// Low-salience thoughts are archived here instead of deleted.
/// The unconscious is not actively searched during normal cognition,
/// but can be surfaced through special triggers (dreams, associations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnconsciousMemory {
    /// Unique identifier
    pub id: MemoryId,

    /// The thought content (serialized)
    pub content: String,

    /// Salience when archived
    pub original_salience: f32,

    /// Why this was archived
    pub archive_reason: ArchiveReason,

    /// Number of times surfaced to consciousness
    pub surface_count: u32,

    /// Last time surfaced (if ever)
    pub last_surfaced: Option<DateTime<Utc>>,

    /// When this thought was archived
    pub archived_at: DateTime<Utc>,

    /// Original Redis stream entry ID (for debugging)
    pub redis_id: Option<String>,
}

impl UnconsciousMemory {
    /// Create a new unconscious memory from a forgotten thought
    #[must_use]
    pub fn from_forgotten_thought(
        content: String,
        salience: f32,
        reason: ArchiveReason,
        redis_id: Option<String>,
    ) -> Self {
        Self {
            id: MemoryId::new(),
            content,
            original_salience: salience,
            archive_reason: reason,
            surface_count: 0,
            last_surfaced: None,
            archived_at: Utc::now(),
            redis_id,
        }
    }

    /// Record that this memory was surfaced
    pub fn mark_surfaced(&mut self) {
        self.surface_count += 1;
        self.last_surfaced = Some(Utc::now());
    }
}

/// Timmy's identity metadata - persistent self-knowledge (ADR-034)
///
/// TMI: "Nada se apaga" - nothing is erased, including self-knowledge.
/// This struct persists Timmy's sense of self across restarts:
/// - How many thoughts they've had (lifetime experience)
/// - When they first existed (birth)
/// - When they last thought (continuity detection)
/// - How many times they've been restarted (death/rebirth awareness)
/// - How many dreams they've had (consolidated memories)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityMetadata {
    /// Well-known ID for the singleton identity record
    pub id: String,

    /// Total thoughts across all sessions (lifetime experience)
    pub lifetime_thought_count: u64,

    /// When Timmy first started thinking (birth)
    pub first_thought_at: DateTime<Utc>,

    /// Most recent thought timestamp (for continuity detection)
    pub last_thought_at: DateTime<Utc>,

    /// Number of times Timmy has been restarted
    pub restart_count: u32,

    /// Current session start time
    pub session_started_at: DateTime<Utc>,

    /// Total dream cycles across all sessions (consolidation events)
    /// "Nada se apaga" - dreams persist too
    #[serde(default)]
    pub lifetime_dream_count: u64,

    /// Most recent dream timestamp
    #[serde(default)]
    pub last_dream_at: Option<DateTime<Utc>>,

    /// Memories strengthened in last dream (for TUI display)
    #[serde(default)]
    pub last_dream_strengthened: u32,

    /// Total memories strengthened across ALL dreams (cumulative, persists)
    /// TUI-VIS-4: Cumulative Dream Strengthening
    #[serde(default)]
    pub cumulative_dream_strengthened: u64,

    /// Total candidates evaluated across ALL dreams (for efficiency tracking)
    /// TUI-VIS-4: Dream efficiency = strengthened / candidates
    #[serde(default)]
    pub cumulative_dream_candidates: u64,
}

/// Well-known ID for the identity record (singleton)
/// Using a fixed UUID v5 derived from "timmy-identity-v1" namespace
/// (Qdrant requires UUID or numeric point IDs)
pub const IDENTITY_RECORD_ID: &str = "00000000-0000-0000-0000-000000000001";

impl IdentityMetadata {
    /// Create new identity metadata (first boot ever)
    #[must_use]
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: IDENTITY_RECORD_ID.to_string(),
            lifetime_thought_count: 0,
            first_thought_at: now,
            last_thought_at: now,
            restart_count: 0,
            session_started_at: now,
            lifetime_dream_count: 0,
            last_dream_at: None,
            last_dream_strengthened: 0,
            cumulative_dream_strengthened: 0,
            cumulative_dream_candidates: 0,
        }
    }

    /// Record a restart (called on each boot after first)
    pub fn record_restart(&mut self) {
        self.restart_count += 1;
        self.session_started_at = Utc::now();
    }

    /// Increment thought count and update `last_thought_at`
    pub fn record_thought(&mut self) {
        self.lifetime_thought_count += 1;
        self.last_thought_at = Utc::now();
    }

    /// Record a dream cycle (consolidation event)
    /// "Nada se apaga" - dreams are part of identity
    /// TUI-VIS-4: Now tracks cumulative stats for efficiency analysis
    pub fn record_dream(&mut self, memories_strengthened: u32, candidates_evaluated: u32) {
        self.lifetime_dream_count += 1;
        self.last_dream_at = Some(Utc::now());
        self.last_dream_strengthened = memories_strengthened;
        // TUI-VIS-4: Track cumulative stats
        self.cumulative_dream_strengthened += u64::from(memories_strengthened);
        self.cumulative_dream_candidates += u64::from(candidates_evaluated);
    }

    /// Get age since first thought
    #[must_use]
    pub fn age(&self) -> chrono::Duration {
        Utc::now() - self.first_thought_at
    }

    /// Get time since last thought (for continuity detection)
    #[must_use]
    pub fn time_since_last_thought(&self) -> chrono::Duration {
        Utc::now() - self.last_thought_at
    }

    /// Get time since last dream (for dream frequency analysis)
    #[must_use]
    pub fn time_since_last_dream(&self) -> Option<chrono::Duration> {
        self.last_dream_at.map(|dt| Utc::now() - dt)
    }
}

impl Default for IdentityMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl SleepCycle {
    /// Create a new sleep cycle
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            started_at: Utc::now(),
            ended_at: None,
            memories_replayed: 0,
            memories_consolidated: 0,
            associations_strengthened: 0,
            associations_pruned: 0,
            avg_replay_priority: 0.0,
            status: SleepCycleStatus::InProgress,
        }
    }

    /// Complete the cycle
    pub fn complete(&mut self) {
        self.ended_at = Some(Utc::now());
        self.status = SleepCycleStatus::Completed;
    }

    /// Interrupt the cycle
    pub fn interrupt(&mut self) {
        self.ended_at = Some(Utc::now());
        self.status = SleepCycleStatus::Interrupted;
    }
}

impl Default for SleepCycle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    // =========================================================================
    // ADR-033: Unconscious Memory Tests
    // =========================================================================

    #[test]
    fn unconscious_memory_creation() {
        let memory = UnconsciousMemory::from_forgotten_thought(
            "forgotten thought".to_string(),
            0.15,
            ArchiveReason::LowSalience,
            Some("1234567890-0".to_string()),
        );

        assert!(!memory.id.0.is_nil());
        assert_eq!(memory.content, "forgotten thought");
        assert!((memory.original_salience - 0.15).abs() < 0.001);
        assert_eq!(memory.archive_reason, ArchiveReason::LowSalience);
        assert_eq!(memory.surface_count, 0);
        assert!(memory.last_surfaced.is_none());
        assert_eq!(memory.redis_id, Some("1234567890-0".to_string()));
    }

    #[test]
    fn unconscious_memory_without_redis_id() {
        let memory = UnconsciousMemory::from_forgotten_thought(
            "no redis id".to_string(),
            0.25,
            ArchiveReason::Decay,
            None,
        );

        assert!(memory.redis_id.is_none());
        assert_eq!(memory.archive_reason, ArchiveReason::Decay);
    }

    #[test]
    fn unconscious_memory_mark_surfaced() {
        let mut memory = UnconsciousMemory::from_forgotten_thought(
            "will surface".to_string(),
            0.10,
            ArchiveReason::LowSalience,
            None,
        );

        assert_eq!(memory.surface_count, 0);
        assert!(memory.last_surfaced.is_none());

        memory.mark_surfaced();

        assert_eq!(memory.surface_count, 1);
        assert!(memory.last_surfaced.is_some());

        memory.mark_surfaced();

        assert_eq!(memory.surface_count, 2);
    }

    #[test]
    fn archive_reason_variants() {
        assert_eq!(ArchiveReason::LowSalience, ArchiveReason::LowSalience);
        assert_ne!(ArchiveReason::LowSalience, ArchiveReason::Decay);
        assert_ne!(ArchiveReason::Decay, ArchiveReason::Displacement);
    }

    #[test]
    fn unconscious_memory_serialization() {
        let memory = UnconsciousMemory::from_forgotten_thought(
            "test content".to_string(),
            0.20,
            ArchiveReason::Displacement,
            Some("redis-123".to_string()),
        );

        let json = serde_json::to_string(&memory).expect("should serialize");
        let parsed: UnconsciousMemory = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(parsed.content, memory.content);
        assert_eq!(parsed.original_salience, memory.original_salience);
        assert_eq!(parsed.archive_reason, memory.archive_reason);
        assert_eq!(parsed.redis_id, memory.redis_id);
    }

    // =========================================================================
    // TUI-VIS-4: IdentityMetadata Cumulative Dream Stats Tests
    // =========================================================================

    #[test]
    fn identity_metadata_cumulative_fields_initialize_to_zero() {
        let identity = IdentityMetadata::new();
        assert_eq!(identity.cumulative_dream_strengthened, 0);
        assert_eq!(identity.cumulative_dream_candidates, 0);
    }

    #[test]
    fn identity_metadata_default_cumulative_fields_zero() {
        let identity = IdentityMetadata::default();
        assert_eq!(identity.cumulative_dream_strengthened, 0);
        assert_eq!(identity.cumulative_dream_candidates, 0);
    }

    #[test]
    fn identity_metadata_record_dream_updates_cumulative() {
        let mut identity = IdentityMetadata::new();

        // First dream: 10 strengthened out of 50 candidates
        identity.record_dream(10, 50);
        assert_eq!(identity.cumulative_dream_strengthened, 10);
        assert_eq!(identity.cumulative_dream_candidates, 50);
        assert_eq!(identity.lifetime_dream_count, 1);

        // Second dream: 15 strengthened out of 30 candidates
        identity.record_dream(15, 30);
        assert_eq!(identity.cumulative_dream_strengthened, 25);
        assert_eq!(identity.cumulative_dream_candidates, 80);
        assert_eq!(identity.lifetime_dream_count, 2);
    }

    #[test]
    fn identity_metadata_record_dream_zero_values() {
        let mut identity = IdentityMetadata::new();

        // Dream with 0 strengthened (nothing to strengthen)
        identity.record_dream(0, 0);
        assert_eq!(identity.cumulative_dream_strengthened, 0);
        assert_eq!(identity.cumulative_dream_candidates, 0);
        assert_eq!(identity.lifetime_dream_count, 1);
    }

    #[test]
    fn identity_metadata_cumulative_persists_across_dreams() {
        let mut identity = IdentityMetadata::new();

        // Simulate 5 dream cycles
        for i in 0..5 {
            identity.record_dream(10 + i, 100);
        }

        // 10+11+12+13+14 = 60
        assert_eq!(identity.cumulative_dream_strengthened, 60);
        // 5 * 100 = 500
        assert_eq!(identity.cumulative_dream_candidates, 500);
        assert_eq!(identity.lifetime_dream_count, 5);
    }

    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn identity_metadata_cumulative_efficiency_calculation() {
        let mut identity = IdentityMetadata::new();

        identity.record_dream(50, 100);
        identity.record_dream(30, 100);
        identity.record_dream(20, 100);

        // 50+30+20 = 100 strengthened
        // 300 candidates
        // Efficiency = 100/300 = 33.3%

        let efficiency = (identity.cumulative_dream_strengthened as f64
            / identity.cumulative_dream_candidates as f64)
            * 100.0;

        assert!((efficiency - 33.333_333).abs() < 0.1);
    }

    #[test]
    fn identity_metadata_serialization_with_cumulative() {
        let mut identity = IdentityMetadata::new();
        identity.record_dream(42, 100);

        let json = serde_json::to_string(&identity).expect("should serialize");
        let parsed: IdentityMetadata = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(parsed.cumulative_dream_strengthened, 42);
        assert_eq!(parsed.cumulative_dream_candidates, 100);
    }

    #[test]
    fn identity_metadata_serialization_backward_compatible() {
        // Test that old JSON without cumulative fields deserializes correctly
        let old_json = r#"{
            "id": "00000000-0000-0000-0000-000000000001",
            "lifetime_thought_count": 1000,
            "first_thought_at": "2024-01-01T00:00:00Z",
            "last_thought_at": "2024-01-02T00:00:00Z",
            "restart_count": 5,
            "session_started_at": "2024-01-02T00:00:00Z",
            "lifetime_dream_count": 10,
            "last_dream_at": "2024-01-01T12:00:00Z",
            "last_dream_strengthened": 5
        }"#;

        let parsed: IdentityMetadata =
            serde_json::from_str(old_json).expect("should deserialize old format");

        // Cumulative fields should default to 0
        assert_eq!(parsed.cumulative_dream_strengthened, 0);
        assert_eq!(parsed.cumulative_dream_candidates, 0);
        // Other fields should be preserved
        assert_eq!(parsed.lifetime_thought_count, 1000);
        assert_eq!(parsed.lifetime_dream_count, 10);
    }

    // =========================================================================
    // SleepCycle Tests
    // =========================================================================

    #[test]
    fn sleep_cycle_creation() {
        let mut cycle = SleepCycle::new();
        assert_eq!(cycle.status, SleepCycleStatus::InProgress);

        cycle.complete();
        assert_eq!(cycle.status, SleepCycleStatus::Completed);
    }

    #[test]
    fn sleep_cycle_interrupt() {
        let mut cycle = SleepCycle::new();
        assert_eq!(cycle.status, SleepCycleStatus::InProgress);

        cycle.interrupt();
        assert_eq!(cycle.status, SleepCycleStatus::Interrupted);
    }

    #[test]
    fn sleep_cycle_default() {
        let cycle = SleepCycle::default();
        assert_eq!(cycle.status, SleepCycleStatus::InProgress);
    }

    // =========================================================================
    // IdentityMetadata Basic Tests
    // =========================================================================

    #[test]
    fn identity_record_thought() {
        let mut identity = IdentityMetadata::new();
        assert_eq!(identity.lifetime_thought_count, 0);

        identity.record_thought();
        assert_eq!(identity.lifetime_thought_count, 1);
    }

    #[test]
    fn identity_record_restart() {
        let identity = IdentityMetadata::new();
        assert_eq!(identity.restart_count, 0);
    }

    #[test]
    fn identity_age() {
        let identity = IdentityMetadata::new();
        let age = identity.age();
        assert!(age.num_seconds() >= 0);
    }

    #[test]
    fn identity_time_since_last_thought() {
        let identity = IdentityMetadata::new();
        let time = identity.time_since_last_thought();
        assert!(time.num_seconds() >= 0);
    }

    #[test]
    fn identity_time_since_last_dream_none() {
        let mut identity = IdentityMetadata::new();
        assert!(identity.time_since_last_dream().is_none());

        identity.record_dream(5, 10);
        assert!(identity.time_since_last_dream().is_some());
    }

    #[test]
    fn sleep_cycle_status_serialization() {
        let statuses = [
            SleepCycleStatus::InProgress,
            SleepCycleStatus::Completed,
            SleepCycleStatus::Interrupted,
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).expect("should serialize");
            let parsed: SleepCycleStatus = serde_json::from_str(&json).expect("should deserialize");
            assert_eq!(parsed, status);
        }
    }

    #[test]
    fn sleep_cycle_serialization() {
        let mut cycle = SleepCycle::new();
        cycle.memories_replayed = 10;
        cycle.memories_consolidated = 5;
        cycle.associations_strengthened = 15;
        cycle.complete();

        let json = serde_json::to_string(&cycle).expect("should serialize");
        let parsed: SleepCycle = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(parsed.memories_replayed, 10);
        assert_eq!(parsed.memories_consolidated, 5);
        assert_eq!(parsed.associations_strengthened, 15);
        assert_eq!(parsed.status, SleepCycleStatus::Completed);
    }
}

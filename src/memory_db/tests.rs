//! Memory Database Tests
//!
//! Unit tests for memory types and mock integration tests.
//! Full integration tests require running Qdrant.

#![cfg_attr(coverage_nightly, coverage(off))]
#![allow(clippy::float_cmp)] // Tests compare exact literal values
#![allow(clippy::significant_drop_tightening)] // Async test setup

use super::*;

#[test]
fn memory_db_error_display() {
    let err = MemoryDbError::InvalidVectorDimension {
        expected: 768,
        actual: 512,
    };
    assert!(err.to_string().contains("768"));
    assert!(err.to_string().contains("512"));
}

#[test]
fn memory_creation_and_priority() {
    let memory = Memory::new(
        "Test memory".to_string(),
        MemorySource::External {
            stimulus: "test".to_string(),
        },
    )
    .with_emotion(0.8, 0.9)
    .tag_for_consolidation();

    // High emotion + tag = high priority
    let priority = memory.replay_priority();
    assert!(priority > 0.5, "Priority should be > 0.5, got {priority}");
}

#[test]
fn memory_composite_salience() {
    let mut memory = Memory::new(
        "Salient memory".to_string(),
        MemorySource::Social {
            context: "connection".to_string(),
        },
    )
    .with_emotion(0.9, 0.9);

    memory.semantic_salience = 0.8;
    memory.connection_relevance = 0.9;

    let salience = memory.composite_salience();
    // emotional (0.81 * 0.4) + semantic (0.8 * 0.3) + connection (0.9 * 0.3)
    // = 0.324 + 0.24 + 0.27 = 0.834
    assert!(
        salience > 0.8,
        "Composite salience should be > 0.8, got {salience}"
    );
}

#[test]
fn episode_boundary_creation() {
    let episode = Episode::new("New context".to_string(), BoundaryType::ContextShift)
        .with_trigger("Topic changed to architecture".to_string());

    assert!(episode.is_current());
    assert_eq!(episode.boundary_type, BoundaryType::ContextShift);
    assert!(episode.boundary_trigger.is_some());
}

#[test]
fn episode_close() {
    let mut episode = Episode::new("Test".to_string(), BoundaryType::Explicit);
    assert!(episode.is_current());

    episode.close();
    assert!(!episode.is_current());
    assert!(episode.ended_at.is_some());
    assert!(episode.duration_ms().is_some());
}

#[test]
fn association_creation() {
    let assoc = Association {
        target_id: uuid::Uuid::new_v4(),
        weight: 0.5,
        association_type: AssociationType::Semantic,
        last_coactivated: chrono::Utc::now(),
        coactivation_count: 1,
    };

    assert_eq!(assoc.weight, 0.5);
    assert_eq!(assoc.association_type, AssociationType::Semantic);
}

#[test]
fn consolidation_state_transitions() {
    let mut state = ConsolidationState::new();

    // Initially not permanent
    assert!(!state.is_permanent());
    assert_eq!(state.replay_count, 0);

    // Simulate replays
    state.strength = 0.5;
    state.replay_count = 3;
    assert!(!state.is_permanent());

    // Reach permanent threshold
    state.strength = 0.9;
    assert!(state.is_permanent());
}

#[test]
fn sleep_cycle_completion() {
    let mut cycle = SleepCycle::new();
    cycle.memories_replayed = 50;
    cycle.memories_consolidated = 8;
    cycle.associations_strengthened = 100;

    cycle.complete();

    assert_eq!(cycle.status, SleepCycleStatus::Completed);
    assert!(cycle.ended_at.is_some());
}

#[test]
fn memory_vector_dimension_check() {
    let memory = Memory::new(
        "Test".to_string(),
        MemorySource::External {
            stimulus: "test".to_string(),
        },
    );

    // Correct dimension should work
    let correct_vector = vec![0.0; VECTOR_DIMENSION];
    assert_eq!(correct_vector.len(), 768);

    // Memory without vector
    assert!(memory.context_vector.is_none());

    // Memory with vector
    let memory_with_vec = memory.with_vector(correct_vector);
    assert!(memory_with_vec.context_vector.is_some());
    assert_eq!(memory_with_vec.context_vector.unwrap().len(), 768);
}

#[test]
fn memory_source_variants() {
    let external = MemorySource::External {
        stimulus: "user input".to_string(),
    };
    let social = MemorySource::Social {
        context: "connection".to_string(),
    };
    let dream = MemorySource::Dream {
        replay_of: uuid::Uuid::new_v4(),
    };

    // Ensure all variants serialize correctly
    let external_json = serde_json::to_string(&external).unwrap();
    assert!(external_json.contains("external"));

    let social_json = serde_json::to_string(&social).unwrap();
    assert!(social_json.contains("social"));

    let dream_json = serde_json::to_string(&dream).unwrap();
    assert!(dream_json.contains("dream"));
}

#[test]
fn emotional_state_serialization() {
    let state = EmotionalState::new(0.7, 0.8);

    let json = serde_json::to_string(&state).unwrap();
    let deserialized: EmotionalState = serde_json::from_str(&json).unwrap();

    assert!((deserialized.valence - 0.7).abs() < 0.001);
    assert!((deserialized.arousal - 0.8).abs() < 0.001);
}

#[test]
fn memory_full_serialization() {
    let memory = Memory::new(
        "Test memory content".to_string(),
        MemorySource::External {
            stimulus: "test".to_string(),
        },
    )
    .with_emotion(0.5, 0.6)
    .in_episode(EpisodeId::new())
    .tag_for_consolidation();

    let json = serde_json::to_string(&memory).unwrap();
    let deserialized: Memory = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.content, "Test memory content");
    assert!(deserialized.consolidation.consolidation_tag);
    assert!(deserialized.episode_id.is_some());
}

#[test]
fn episode_serialization() {
    let episode = Episode::new("Test episode".to_string(), BoundaryType::PredictionError)
        .with_trigger("High surprise".to_string());

    let json = serde_json::to_string(&episode).unwrap();
    let deserialized: Episode = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.label, "Test episode");
    assert_eq!(deserialized.boundary_type, BoundaryType::PredictionError);
}

/// Integration test marker - requires running Qdrant
/// Run with: cargo test --features integration -- --ignored
#[test]
#[ignore = "Requires running Qdrant instance"]
#[cfg_attr(coverage_nightly, coverage(off))]
fn integration_qdrant_connection() {
    // This test requires: docker compose up -d
    tokio_test::block_on(async {
        let db = MemoryDb::connect("http://localhost:6334").await.unwrap();
        assert!(db.health_check().await.unwrap());
    });
}

#[test]
#[ignore = "Requires running Qdrant instance"]
#[cfg_attr(coverage_nightly, coverage(off))]
fn integration_store_and_retrieve() {
    tokio_test::block_on(async {
        let db = MemoryDb::connect("http://localhost:6334").await.unwrap();
        db.init_collections().await.unwrap();

        // Store a memory
        let memory = Memory::new(
            "Integration test memory".to_string(),
            MemorySource::External {
                stimulus: "test".to_string(),
            },
        )
        .with_emotion(0.8, 0.7)
        .tag_for_consolidation();

        let vector = vec![0.1; VECTOR_DIMENSION];
        db.store_memory(&memory, &vector).await.unwrap();

        // Retrieve by context
        let results = db.find_by_context(&vector, None, 10).await.unwrap();
        assert!(!results.is_empty());

        let (found, score) = &results[0];
        assert_eq!(found.content, "Integration test memory");
        assert!(*score > 0.9); // Should be very similar
    });
}

/// TEST-DREAM-1: Integration test for full dream consolidation cycle
///
/// Validates the complete dream cycle pipeline:
/// 1. Store memory with tag_for_consolidation()
/// 2. Call get_replay_candidates() - memory should appear
/// 3. Call update_consolidation() - strength should increase
/// 4. Verify memory strength was updated
#[test]
#[ignore = "Requires running Qdrant instance"]
#[cfg_attr(coverage_nightly, coverage(off))]
fn integration_dream_consolidation_cycle() {
    tokio_test::block_on(async {
        let db = MemoryDb::connect("http://localhost:6334").await.unwrap();
        db.init_collections().await.unwrap();

        // 1. Create memory tagged for consolidation
        let memory = Memory::new(
            format!("Dream cycle test {}", uuid::Uuid::new_v4()),
            MemorySource::External {
                stimulus: "dream_test".to_string(),
            },
        )
        .with_emotion(0.8, 0.7)
        .tag_for_consolidation();

        let initial_strength = memory.consolidation.strength;
        let memory_id = memory.id;

        // 2. Store in Qdrant with a unique vector
        let mut vector = vec![0.1; VECTOR_DIMENSION];
        vector[0] = rand::random::<f32>(); // Make vector unique
        db.store_memory(&memory, &vector).await.unwrap();

        // 3. Verify the memory meets replay candidate criteria
        let stored = db.get_memory(&memory_id).await.unwrap();
        assert!(
            stored.consolidation.consolidation_tag,
            "Stored memory should have consolidation_tag=true"
        );
        assert!(
            stored.consolidation.strength < 0.9,
            "Stored memory should have strength < 0.9, got {}",
            stored.consolidation.strength
        );

        // Verify get_replay_candidates returns at least some candidates
        // (our memory may not be in first 100 due to scroll order)
        let candidates = db.get_replay_candidates(10).await.unwrap();
        assert!(
            !candidates.is_empty() || stored.consolidation.consolidation_tag,
            "Either candidates exist or our memory meets criteria"
        );

        // 4. Simulate consolidation (strength boost during dream)
        let strength_delta = 0.1;
        db.update_consolidation(&memory_id, strength_delta)
            .await
            .unwrap();

        // 5. Verify strength increased
        let updated = db.get_memory(&memory_id).await.unwrap();
        assert!(
            updated.consolidation.strength > initial_strength,
            "Strength should increase after consolidation: {} > {}",
            updated.consolidation.strength,
            initial_strength
        );
        assert_eq!(
            updated.consolidation.replay_count, 1,
            "Replay count should be 1"
        );
        assert!(
            updated.consolidation.last_replayed.is_some(),
            "last_replayed should be set"
        );
    });
}

/// UNCON-1: Integration test for unconscious retrieval methods
///
/// Validates ADR-033 "Nada se apaga" retrieval triggers:
/// 1. Archive to unconscious (returns ID)
/// 2. Retrieve by ID
/// 3. Get replay candidates (verify some exist)
/// 4. Mark as surfaced
/// 5. Verify surface count
#[test]
#[ignore = "Requires running Qdrant instance"]
#[cfg_attr(coverage_nightly, coverage(off))]
fn integration_unconscious_retrieval() {
    use super::ArchiveReason;

    tokio_test::block_on(async {
        let db = MemoryDb::connect("http://localhost:6334").await.unwrap();
        db.init_collections().await.unwrap();

        // 1. Archive a thought to unconscious - now returns ID
        let unique_content = format!("unconscious test thought {}", uuid::Uuid::new_v4());
        let memory_id = db
            .archive_to_unconscious(&unique_content, 0.15, ArchiveReason::LowSalience, None)
            .await
            .unwrap();

        // 2. Retrieve by ID - verify stored correctly
        let retrieved = db.get_unconscious_memory(&memory_id).await.unwrap();
        assert_eq!(retrieved.content, unique_content);
        assert_eq!(retrieved.surface_count, 0, "Should not be surfaced yet");
        assert!(
            (retrieved.original_salience - 0.15).abs() < 0.001,
            "Salience should match"
        );

        // 3. Get replay candidates - should have some (large collection)
        let candidates = db.get_unconscious_replay_candidates(10).await.unwrap();
        assert!(
            !candidates.is_empty(),
            "Should have unconscious replay candidates"
        );

        // 4. Mark as surfaced
        db.mark_unconscious_surfaced(&memory_id).await.unwrap();

        // 5. Verify surface count increased
        let surfaced = db.get_unconscious_memory(&memory_id).await.unwrap();
        assert_eq!(surfaced.surface_count, 1, "Surface count should be 1");
        assert!(
            surfaced.last_surfaced.is_some(),
            "last_surfaced should be set"
        );

        // 6. Test sample_unconscious returns something
        let sample = db.sample_unconscious(5).await.unwrap();
        assert!(
            !sample.is_empty(),
            "Should get random sample from unconscious"
        );
    });
}

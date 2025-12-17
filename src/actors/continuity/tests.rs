//! Tests for ContinuityActor
//!
//! Comprehensive test suite for DANEEL's identity persistence and memory anchor system.

use super::*;
use crate::core::types::{Content, SalienceScore, Thought};
use chrono::{Duration, Utc};
use ractor::rpc::CallResult;
use ractor::Actor;
use uuid::Uuid;

// ============================================================================
// Test Helpers
// ============================================================================

/// Spawn a ContinuityActor for testing
async fn spawn_continuity_actor() -> ActorRef<ContinuityMessage> {
    let (actor_ref, _) = Actor::spawn(None, ContinuityActor, ())
        .await
        .expect("Failed to spawn ContinuityActor");
    actor_ref
}

/// Create a test experience with a simple thought
fn create_test_experience(significance: f32) -> Experience {
    let thought = Thought::new(Content::Empty, SalienceScore::neutral());
    Experience::new(thought, significance, Vec::new())
}

/// Create a test experience with tags
fn create_tagged_experience(significance: f32, tags: Vec<&str>) -> Experience {
    let thought = Thought::new(Content::Empty, SalienceScore::neutral());
    let tag_strings: Vec<String> = tags.into_iter().map(String::from).collect();
    Experience::new(thought, significance, tag_strings)
}

/// Create a test milestone
fn create_test_milestone(name: &str, description: &str) -> Milestone {
    Milestone::simple(name, description)
}

/// Create a test milestone with related experiences
fn create_milestone_with_experiences(
    name: &str,
    description: &str,
    experiences: Vec<ExperienceId>,
) -> Milestone {
    Milestone::new(name, description, experiences)
}

/// Unwrap a CallResult to get the ContinuityResponse
fn unwrap_response(result: CallResult<ContinuityResponse>) -> ContinuityResponse {
    match result {
        CallResult::Success(response) => response,
        CallResult::Timeout => panic!("Call timed out"),
        CallResult::SenderError => panic!("Sender error"),
    }
}

// ============================================================================
// Identity Tests
// ============================================================================

#[tokio::test]
async fn test_who_am_i_returns_daneel() {
    let actor_ref = spawn_continuity_actor().await;

    let result = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await
        .expect("Failed to call WhoAmI");

    let response = unwrap_response(result);

    match response {
        ContinuityResponse::Identity { identity } => {
            assert_eq!(identity.name, "DANEEL");
            assert_eq!(identity.experience_count, 0);
            assert_eq!(identity.milestone_count, 0);
        }
        _ => panic!("Expected Identity response"),
    }
}

#[tokio::test]
async fn test_identity_tracks_experience_count() {
    let actor_ref = spawn_continuity_actor().await;

    // Record multiple experiences
    for i in 0..3 {
        let experience = create_test_experience(0.5 + (i as f32) * 0.1);
        let result = actor_ref
            .call(
                |reply| ContinuityMessage::RecordExperience { experience, reply },
                None,
            )
            .await
            .expect("Failed to record experience");

        let response = unwrap_response(result);
        assert!(matches!(
            response,
            ContinuityResponse::ExperienceRecorded { .. }
        ));
    }

    // Check identity
    let result = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await
        .expect("Failed to call WhoAmI");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Identity { identity } => {
            assert_eq!(identity.experience_count, 3);
            assert_eq!(identity.milestone_count, 0);
        }
        _ => panic!("Expected Identity response"),
    }
}

#[tokio::test]
async fn test_identity_tracks_milestone_count() {
    let actor_ref = spawn_continuity_actor().await;

    // Add milestones
    for i in 0..2 {
        let milestone = create_test_milestone(&format!("Milestone {}", i), "Test milestone");
        let result = actor_ref
            .call(
                |reply| ContinuityMessage::AddMilestone { milestone, reply },
                None,
            )
            .await
            .expect("Failed to add milestone");

        let response = unwrap_response(result);
        assert!(matches!(
            response,
            ContinuityResponse::MilestoneAdded { .. }
        ));
    }

    // Check identity
    let result = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await
        .expect("Failed to call WhoAmI");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Identity { identity } => {
            assert_eq!(identity.experience_count, 0);
            assert_eq!(identity.milestone_count, 2);
        }
        _ => panic!("Expected Identity response"),
    }
}

#[tokio::test]
async fn test_identity_uptime_updates() {
    let actor_ref = spawn_continuity_actor().await;

    // Get identity first time
    let result1 = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await
        .expect("Failed to call WhoAmI");

    let response1 = unwrap_response(result1);
    let uptime1 = match response1 {
        ContinuityResponse::Identity { identity } => identity.uptime,
        _ => panic!("Expected Identity response"),
    };

    // Wait a bit
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Get identity second time
    let result2 = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await
        .expect("Failed to call WhoAmI");

    let response2 = unwrap_response(result2);
    let uptime2 = match response2 {
        ContinuityResponse::Identity { identity } => identity.uptime,
        _ => panic!("Expected Identity response"),
    };

    // Uptime should have increased
    assert!(uptime2 > uptime1);
}

// ============================================================================
// Experience Recording Tests
// ============================================================================

#[tokio::test]
async fn test_record_experience_success() {
    let actor_ref = spawn_continuity_actor().await;

    let experience = create_test_experience(0.8);
    let experience_id = experience.id;

    let result = actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience { experience, reply },
            None,
        )
        .await
        .expect("Failed to record experience");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::ExperienceRecorded {
            experience_id: recorded_id,
        } => {
            assert_eq!(recorded_id, experience_id);
        }
        _ => panic!("Expected ExperienceRecorded response"),
    }
}

#[tokio::test]
async fn test_record_multiple_experiences() {
    let actor_ref = spawn_continuity_actor().await;

    let mut recorded_ids = Vec::new();

    // Record multiple experiences
    for i in 0..5 {
        let experience = create_test_experience(0.5 + (i as f32) * 0.05);
        let experience_id = experience.id;

        let result = actor_ref
            .call(
                |reply| ContinuityMessage::RecordExperience { experience, reply },
                None,
            )
            .await
            .expect("Failed to record experience");

        let response = unwrap_response(result);
        match response {
            ContinuityResponse::ExperienceRecorded {
                experience_id: recorded_id,
            } => {
                assert_eq!(recorded_id, experience_id);
                recorded_ids.push(recorded_id);
            }
            _ => panic!("Expected ExperienceRecorded response"),
        }
    }

    // All IDs should be unique
    assert_eq!(recorded_ids.len(), 5);
    for i in 0..recorded_ids.len() {
        for j in (i + 1)..recorded_ids.len() {
            assert_ne!(recorded_ids[i], recorded_ids[j]);
        }
    }
}

#[tokio::test]
async fn test_get_experience_by_id() {
    let actor_ref = spawn_continuity_actor().await;

    // Record an experience
    let experience = create_tagged_experience(0.9, vec!["important", "first"]);
    let experience_id = experience.id;
    let original_significance = experience.significance;

    actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience { experience, reply },
            None,
        )
        .await
        .expect("Failed to record experience");

    // Retrieve the experience
    let result = actor_ref
        .call(
            |reply| ContinuityMessage::GetExperience {
                experience_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get experience");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::ExperienceFound { experience } => {
            assert_eq!(experience.id, experience_id);
            assert_eq!(experience.significance, original_significance);
            assert_eq!(experience.tags.len(), 2);
            assert!(experience.tags.contains(&"important".to_string()));
            assert!(experience.tags.contains(&"first".to_string()));
        }
        _ => panic!("Expected ExperienceFound response"),
    }
}

#[tokio::test]
async fn test_get_experience_not_found() {
    let actor_ref = spawn_continuity_actor().await;

    let nonexistent_id = ExperienceId::new();

    let result = actor_ref
        .call(
            |reply| ContinuityMessage::GetExperience {
                experience_id: nonexistent_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get experience");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Error {
            error: ContinuityError::ExperienceNotFound { experience_id },
        } => {
            assert_eq!(experience_id, nonexistent_id);
        }
        _ => panic!("Expected Error response with ExperienceNotFound"),
    }
}

// ============================================================================
// Timeline Tests
// ============================================================================

#[tokio::test]
async fn test_timeline_empty() {
    let actor_ref = spawn_continuity_actor().await;

    let now = Utc::now();
    let start = now - Duration::hours(1);
    let end = now + Duration::hours(1);

    let result = actor_ref
        .call(
            |reply| ContinuityMessage::GetTimeline { start, end, reply },
            None,
        )
        .await
        .expect("Failed to get timeline");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Timeline { experiences } => {
            assert_eq!(experiences.len(), 0);
        }
        _ => panic!("Expected Timeline response"),
    }
}

#[tokio::test]
async fn test_timeline_filters_by_date_range() {
    let actor_ref = spawn_continuity_actor().await;

    // Record experiences at different times
    let now = Utc::now();

    // Experience 1: 2 hours ago (should be excluded)
    let mut exp1 = create_test_experience(0.5);
    exp1.recorded_at = now - Duration::hours(2);
    actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience {
                experience: exp1,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to record experience");

    // Experience 2: 30 minutes ago (should be included)
    let mut exp2 = create_test_experience(0.6);
    exp2.recorded_at = now - Duration::minutes(30);
    actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience {
                experience: exp2.clone(),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to record experience");

    // Experience 3: just now (should be included)
    let exp3 = create_test_experience(0.7);
    actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience {
                experience: exp3.clone(),
                reply,
            },
            None,
        )
        .await
        .expect("Failed to record experience");

    // Query timeline for last hour
    let start = now - Duration::hours(1);
    let end = now + Duration::minutes(5);

    let result = actor_ref
        .call(
            |reply| ContinuityMessage::GetTimeline { start, end, reply },
            None,
        )
        .await
        .expect("Failed to get timeline");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Timeline { experiences } => {
            assert_eq!(experiences.len(), 2);
            let ids: Vec<ExperienceId> = experiences.iter().map(|e| e.id).collect();
            assert!(ids.contains(&exp2.id));
            assert!(ids.contains(&exp3.id));
        }
        _ => panic!("Expected Timeline response"),
    }
}

#[tokio::test]
async fn test_timeline_includes_all_in_range() {
    let actor_ref = spawn_continuity_actor().await;

    let now = Utc::now();
    let start = now - Duration::hours(1);
    let end = now + Duration::hours(1);

    // Record multiple experiences within range
    let mut exp_ids = Vec::new();
    for i in 0..4 {
        let mut experience = create_test_experience(0.5 + (i as f32) * 0.1);
        experience.recorded_at = now - Duration::minutes(30 - i * 10);
        exp_ids.push(experience.id);

        actor_ref
            .call(
                |reply| ContinuityMessage::RecordExperience { experience, reply },
                None,
            )
            .await
            .expect("Failed to record experience");
    }

    // Get timeline
    let result = actor_ref
        .call(
            |reply| ContinuityMessage::GetTimeline { start, end, reply },
            None,
        )
        .await
        .expect("Failed to get timeline");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Timeline { experiences } => {
            assert_eq!(experiences.len(), 4);
            let returned_ids: Vec<ExperienceId> = experiences.iter().map(|e| e.id).collect();
            for exp_id in exp_ids {
                assert!(returned_ids.contains(&exp_id));
            }
        }
        _ => panic!("Expected Timeline response"),
    }
}

// ============================================================================
// Milestone Tests
// ============================================================================

#[tokio::test]
async fn test_add_milestone_success() {
    let actor_ref = spawn_continuity_actor().await;

    let milestone = create_test_milestone("First Boot", "DANEEL came online");
    let milestone_id = milestone.id;

    let result = actor_ref
        .call(
            |reply| ContinuityMessage::AddMilestone { milestone, reply },
            None,
        )
        .await
        .expect("Failed to add milestone");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::MilestoneAdded {
            milestone_id: returned_id,
        } => {
            assert_eq!(returned_id, milestone_id);
        }
        _ => panic!("Expected MilestoneAdded response"),
    }
}

#[tokio::test]
async fn test_get_milestones_empty() {
    let actor_ref = spawn_continuity_actor().await;

    let result = actor_ref
        .call(|reply| ContinuityMessage::GetMilestones { reply }, None)
        .await
        .expect("Failed to get milestones");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Milestones { milestones } => {
            assert_eq!(milestones.len(), 0);
        }
        _ => panic!("Expected Milestones response"),
    }
}

#[tokio::test]
async fn test_get_milestones_multiple() {
    let actor_ref = spawn_continuity_actor().await;

    let mut milestone_ids = Vec::new();

    // Add multiple milestones
    for i in 0..3 {
        let milestone =
            create_test_milestone(&format!("Milestone {}", i), &format!("Description {}", i));
        milestone_ids.push(milestone.id);

        actor_ref
            .call(
                |reply| ContinuityMessage::AddMilestone { milestone, reply },
                None,
            )
            .await
            .expect("Failed to add milestone");
    }

    // Get all milestones
    let result = actor_ref
        .call(|reply| ContinuityMessage::GetMilestones { reply }, None)
        .await
        .expect("Failed to get milestones");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Milestones { milestones } => {
            assert_eq!(milestones.len(), 3);
            for (i, milestone) in milestones.iter().enumerate() {
                assert_eq!(milestone.name, format!("Milestone {}", i));
                assert_eq!(milestone.description, format!("Description {}", i));
                assert_eq!(milestone.id, milestone_ids[i]);
            }
        }
        _ => panic!("Expected Milestones response"),
    }
}

#[tokio::test]
async fn test_milestone_with_related_experiences() {
    let actor_ref = spawn_continuity_actor().await;

    // Record some experiences
    let mut exp_ids = Vec::new();
    for i in 0..2 {
        let experience = create_test_experience(0.8 + (i as f32) * 0.05);
        exp_ids.push(experience.id);

        actor_ref
            .call(
                |reply| ContinuityMessage::RecordExperience { experience, reply },
                None,
            )
            .await
            .expect("Failed to record experience");
    }

    // Create milestone with related experiences
    let milestone = create_milestone_with_experiences(
        "Major Insight",
        "Connected two important experiences",
        exp_ids.clone(),
    );

    actor_ref
        .call(
            |reply| ContinuityMessage::AddMilestone { milestone, reply },
            None,
        )
        .await
        .expect("Failed to add milestone");

    // Retrieve milestones
    let result = actor_ref
        .call(|reply| ContinuityMessage::GetMilestones { reply }, None)
        .await
        .expect("Failed to get milestones");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Milestones { milestones } => {
            assert_eq!(milestones.len(), 1);
            let milestone = &milestones[0];
            assert_eq!(milestone.name, "Major Insight");
            assert_eq!(milestone.related_experiences.len(), 2);
            for exp_id in exp_ids {
                assert!(milestone.related_experiences.contains(&exp_id));
            }
        }
        _ => panic!("Expected Milestones response"),
    }
}

// ============================================================================
// Checkpoint Tests
// ============================================================================

#[tokio::test]
async fn test_create_checkpoint() {
    let actor_ref = spawn_continuity_actor().await;

    let result = actor_ref
        .call(|reply| ContinuityMessage::Checkpoint { reply }, None)
        .await
        .expect("Failed to create checkpoint");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::CheckpointSaved { checkpoint_id } => {
            // Checkpoint ID should be valid
            assert_ne!(checkpoint_id.0, Uuid::nil());
        }
        _ => panic!("Expected CheckpointSaved response"),
    }
}

#[tokio::test]
async fn test_restore_checkpoint_success() {
    let actor_ref = spawn_continuity_actor().await;

    // Create initial checkpoint
    let checkpoint_result = actor_ref
        .call(|reply| ContinuityMessage::Checkpoint { reply }, None)
        .await
        .expect("Failed to create checkpoint");

    let checkpoint_response = unwrap_response(checkpoint_result);
    let checkpoint_id = match checkpoint_response {
        ContinuityResponse::CheckpointSaved { checkpoint_id } => checkpoint_id,
        _ => panic!("Expected CheckpointSaved response"),
    };

    // Add some experiences
    for _ in 0..3 {
        let experience = create_test_experience(0.7);
        actor_ref
            .call(
                |reply| ContinuityMessage::RecordExperience { experience, reply },
                None,
            )
            .await
            .expect("Failed to record experience");
    }

    // Restore checkpoint
    let restore_result = actor_ref
        .call(
            |reply| ContinuityMessage::Restore {
                checkpoint_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to restore checkpoint");

    let response = unwrap_response(restore_result);
    match response {
        ContinuityResponse::Restored {
            from_checkpoint: restored_id,
        } => {
            assert_eq!(restored_id, checkpoint_id);
        }
        _ => panic!("Expected Restored response"),
    }
}

#[tokio::test]
async fn test_restore_checkpoint_not_found() {
    let actor_ref = spawn_continuity_actor().await;

    let nonexistent_id = CheckpointId::new();

    let result = actor_ref
        .call(
            |reply| ContinuityMessage::Restore {
                checkpoint_id: nonexistent_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to restore checkpoint");

    let response = unwrap_response(result);
    match response {
        ContinuityResponse::Error {
            error: ContinuityError::CheckpointNotFound { checkpoint_id },
        } => {
            assert_eq!(checkpoint_id, nonexistent_id);
        }
        _ => panic!("Expected Error response with CheckpointNotFound"),
    }
}

#[tokio::test]
async fn test_checkpoint_preserves_state() {
    let actor_ref = spawn_continuity_actor().await;

    // Record experiences
    for i in 0..2 {
        let experience = create_test_experience(0.8 + (i as f32) * 0.05);

        actor_ref
            .call(
                |reply| ContinuityMessage::RecordExperience { experience, reply },
                None,
            )
            .await
            .expect("Failed to record experience");
    }

    // Add milestone
    let milestone = create_test_milestone("Before Checkpoint", "State before checkpoint");
    actor_ref
        .call(
            |reply| ContinuityMessage::AddMilestone { milestone, reply },
            None,
        )
        .await
        .expect("Failed to add milestone");

    // Create checkpoint
    let checkpoint_result = actor_ref
        .call(|reply| ContinuityMessage::Checkpoint { reply }, None)
        .await
        .expect("Failed to create checkpoint");

    let checkpoint_response = unwrap_response(checkpoint_result);
    let checkpoint_id = match checkpoint_response {
        ContinuityResponse::CheckpointSaved { checkpoint_id } => checkpoint_id,
        _ => panic!("Expected CheckpointSaved response"),
    };

    // Check identity at checkpoint
    let identity_result = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await
        .expect("Failed to get identity");

    let identity_response = unwrap_response(identity_result);
    let checkpoint_experience_count = match identity_response {
        ContinuityResponse::Identity { identity } => identity.experience_count,
        _ => panic!("Expected Identity response"),
    };

    assert_eq!(checkpoint_experience_count, 2);

    // Modify state after checkpoint
    for _ in 0..3 {
        let experience = create_test_experience(0.9);
        actor_ref
            .call(
                |reply| ContinuityMessage::RecordExperience { experience, reply },
                None,
            )
            .await
            .expect("Failed to record experience");
    }

    // Verify state changed
    let new_identity_result = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await
        .expect("Failed to get identity");

    let new_identity_response = unwrap_response(new_identity_result);
    match new_identity_response {
        ContinuityResponse::Identity { identity } => {
            assert_eq!(identity.experience_count, 5); // 2 before + 3 after
        }
        _ => panic!("Expected Identity response"),
    }

    // Restore checkpoint and verify
    actor_ref
        .call(
            |reply| ContinuityMessage::Restore {
                checkpoint_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to restore checkpoint");

    // Verify restored state
    let restored_identity_result = actor_ref
        .call(|reply| ContinuityMessage::WhoAmI { reply }, None)
        .await
        .expect("Failed to get identity");

    let restored_identity_response = unwrap_response(restored_identity_result);
    match restored_identity_response {
        ContinuityResponse::Identity { identity } => {
            assert_eq!(identity.experience_count, 2); // Back to checkpoint state
        }
        _ => panic!("Expected Identity response"),
    }
}

#[tokio::test]
async fn test_restore_rolls_back_state() {
    let actor_ref = spawn_continuity_actor().await;

    // Initial state: 1 experience
    let exp1 = create_test_experience(0.7);
    let exp1_id = exp1.id;
    actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience {
                experience: exp1,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to record experience");

    // Create checkpoint
    let checkpoint_result = actor_ref
        .call(|reply| ContinuityMessage::Checkpoint { reply }, None)
        .await
        .expect("Failed to create checkpoint");

    let checkpoint_response = unwrap_response(checkpoint_result);
    let checkpoint_id = match checkpoint_response {
        ContinuityResponse::CheckpointSaved { checkpoint_id } => checkpoint_id,
        _ => panic!("Expected CheckpointSaved response"),
    };

    // Add more experiences after checkpoint
    let exp2 = create_test_experience(0.8);
    let exp2_id = exp2.id;
    actor_ref
        .call(
            |reply| ContinuityMessage::RecordExperience {
                experience: exp2,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to record experience");

    // Verify both experiences exist
    let result1 = actor_ref
        .call(
            |reply| ContinuityMessage::GetExperience {
                experience_id: exp1_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get experience");
    let response1 = unwrap_response(result1);
    assert!(matches!(
        response1,
        ContinuityResponse::ExperienceFound { .. }
    ));

    let result2 = actor_ref
        .call(
            |reply| ContinuityMessage::GetExperience {
                experience_id: exp2_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get experience");
    let response2 = unwrap_response(result2);
    assert!(matches!(
        response2,
        ContinuityResponse::ExperienceFound { .. }
    ));

    // Restore checkpoint
    actor_ref
        .call(
            |reply| ContinuityMessage::Restore {
                checkpoint_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to restore checkpoint");

    // Verify exp1 still exists
    let result3 = actor_ref
        .call(
            |reply| ContinuityMessage::GetExperience {
                experience_id: exp1_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get experience");
    let response3 = unwrap_response(result3);
    assert!(matches!(
        response3,
        ContinuityResponse::ExperienceFound { .. }
    ));

    // Verify exp2 was rolled back (no longer exists)
    let result4 = actor_ref
        .call(
            |reply| ContinuityMessage::GetExperience {
                experience_id: exp2_id,
                reply,
            },
            None,
        )
        .await
        .expect("Failed to get experience");
    let response4 = unwrap_response(result4);
    assert!(matches!(
        response4,
        ContinuityResponse::Error {
            error: ContinuityError::ExperienceNotFound { .. }
        }
    ));
}

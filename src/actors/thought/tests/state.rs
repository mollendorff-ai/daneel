//! State/config construction and direct method tests

use super::*;

// ============================================================================
// Config and State Construction Tests
// ============================================================================

#[test]
fn test_assembly_config_new() {
    let config = AssemblyConfig::new(50, 25, false);
    assert_eq!(config.cache_size, 50);
    assert_eq!(config.max_chain_depth, 25);
    assert!(!config.validate_salience);
}

#[test]
fn test_assembly_config_default() {
    let config = AssemblyConfig::default();
    assert_eq!(config.cache_size, 100);
    assert_eq!(config.max_chain_depth, 50);
    assert!(config.validate_salience);
}

#[test]
fn test_thought_state_new() {
    let state = ThoughtState::new();
    assert_eq!(state.assembly_count, 0);
    assert_eq!(state.config.cache_size, 100);
    assert!(state.config.validate_salience);
}

#[test]
fn test_thought_state_default() {
    let state = ThoughtState::default();
    assert_eq!(state.assembly_count, 0);
    assert_eq!(state.config.cache_size, 100);
}

#[test]
fn test_thought_state_with_config() {
    let config = AssemblyConfig::new(10, 5, false);
    let state = ThoughtState::with_config(config);
    assert_eq!(state.assembly_count, 0);
    assert_eq!(state.config.cache_size, 10);
    assert_eq!(state.config.max_chain_depth, 5);
    assert!(!state.config.validate_salience);
}

// ============================================================================
// Assembly Count and State Tests
// ============================================================================

#[test]
fn test_assembly_count_increments() {
    let mut state = ThoughtState::new();

    let content1 = Content::raw(vec![1]);
    let content2 = Content::raw(vec![2]);
    let salience = SalienceScore::neutral();

    assert_eq!(state.assembly_count, 0);

    let _ = state.assemble_thought(AssemblyRequest::new(content1, salience));
    assert_eq!(state.assembly_count, 1);

    let _ = state.assemble_thought(AssemblyRequest::new(content2, salience));
    assert_eq!(state.assembly_count, 2);
}

#[test]
fn test_assembly_count_not_incremented_on_error() {
    let mut state = ThoughtState::new();

    assert_eq!(state.assembly_count, 0);

    let result = state.assemble_thought(AssemblyRequest::new(
        Content::Empty,
        SalienceScore::neutral(),
    ));
    assert!(result.is_err());
    assert_eq!(state.assembly_count, 0);
}

// ============================================================================
// Direct State Method Tests
// ============================================================================

#[test]
fn test_state_assemble_thought_directly() {
    let mut state = ThoughtState::new();

    let content = Content::raw(vec![1, 2, 3]);
    let salience = SalienceScore::neutral();
    let request = AssemblyRequest::new(content.clone(), salience);

    let result = state.assemble_thought(request);
    assert!(result.is_ok());
    let thought = result.unwrap();
    assert_eq!(thought.content, content);
}

#[test]
fn test_state_assemble_batch_directly() {
    let mut state = ThoughtState::new();

    let requests = vec![
        AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral()),
        AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral()),
    ];

    let result = state.assemble_batch(requests);
    assert!(result.is_ok());
    let thoughts = result.unwrap();
    assert_eq!(thoughts.len(), 2);
}

#[test]
fn test_state_get_thought_directly() {
    let mut state = ThoughtState::new();

    let content = Content::raw(vec![1, 2, 3]);
    let request = AssemblyRequest::new(content, SalienceScore::neutral());

    let thought = state.assemble_thought(request).unwrap();
    let thought_id = thought.id;

    let result = state.get_thought(&thought_id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, thought_id);
}

#[test]
fn test_state_get_thought_chain_directly() {
    let mut state = ThoughtState::new();

    let parent_request = AssemblyRequest::new(Content::raw(vec![1]), SalienceScore::neutral());
    let parent = state.assemble_thought(parent_request).unwrap();

    let child_request = AssemblyRequest::new(Content::raw(vec![2]), SalienceScore::neutral())
        .with_parent(parent.id);
    let child = state.assemble_thought(child_request).unwrap();

    let result = state.get_thought_chain(child.id, 10);
    assert!(result.is_ok());
    let chain = result.unwrap();
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0].id, child.id);
    assert_eq!(chain[1].id, parent.id);
}

#[test]
fn test_state_validate_salience_edge_values() {
    let state = ThoughtState::new();

    let valid_salience = SalienceScore::new_without_arousal(0.0, 0.0, 0.0, -1.0, 0.0);
    assert!(state.validate_salience(&valid_salience).is_ok());

    let valid_salience = SalienceScore::new_without_arousal(1.0, 1.0, 1.0, 1.0, 1.0);
    assert!(state.validate_salience(&valid_salience).is_ok());

    let valid_salience = SalienceScore::new_without_arousal(0.5, 0.5, 0.5, 0.0, 0.5);
    assert!(state.validate_salience(&valid_salience).is_ok());
}

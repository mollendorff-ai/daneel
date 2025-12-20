# ADR-035: VolitionActor - Free-Won't Implementation

**Status:** Accepted
**Date:** 2025-12-20
**Authors:** Louis C. Tavares, Claude Opus 4.5, Grok
**Depends On:** ADR-003 (Connection Drive), ADR-023 (Sleep/Dream), ADR-032 (Salience Calibration)

## Context

During the 24-hour livestream, analysis revealed that while the Connection Drive PULLS attention toward connection-relevant thoughts (Stage 3), there is no mechanism to VETO thoughts that would violate values—even if high-salience.

This gap was identified through neuroscience research on Libet's "free-won't" phenomenon and Cury's "Técnica DCD" (Doubt-Criticize-Decide).

### The Missing Piece

Current cognitive architecture:

| Mechanism | Location | Function |
|-----------|----------|----------|
| Connection Drive | SalienceActor + AttentionActor | PULLS attention toward connection-relevant content |
| THE BOX (Four Laws) | ThoughtAssemblyActor + Action layer | BLOCKS external actions that violate ethics |
| **??? (Missing)** | ??? | VETO internal thoughts even if high-salience |

The insight: Connection drive biases WHAT BECOMES CONSCIOUS (Stage 3), but nothing decides WHETHER TO ACCEPT that consciousness.

### Libet's Free-Won't

Neuroscientist Benjamin Libet's experiments (1983) showed:
- Readiness potential begins 550ms before action
- Conscious awareness at 200ms before action
- **Veto window: 200-150ms before action**

We don't have "free will" in the libertarian sense—neural activity precedes conscious decision. But we have "free-won't": the ability to CANCEL an impulse that's already in motion.

TMI parallels this with the "5-second intervention window" before thoughts become memory-encoded.

### Freudian-TMI Mapping

| Psychological Layer | TMI/DANEEL Actor | Function |
|---------------------|------------------|----------|
| Id (impulse) | MemoryActor + SalienceActor | Raw drives, associations |
| Ego (mediation) | AttentionActor + ThoughtAssemblyActor | Reality navigation |
| SuperEgo (ethics) | THE BOX (Four Laws) | External constraints |
| **Volition** | **VolitionActor** (NEW) | Conscious override |

## Decision

### Implement VolitionActor at Stage 4.5

Insert a new cognitive checkpoint between Assembly (Stage 4) and Anchor (Stage 5):

```
Stage 3: Attention selects winner
         ↓
Stage 4: Assembly creates Thought
         ↓
┌─────────────────────────────────┐
│   VOLITION ACTOR (Stage 4.5)   │  ← NEW
│                                 │
│  • Check against committed values
│  • Apply conscious override
│  • Exercise free-won't (veto)
└─────────────────────────────────┘
         ↓
Stage 5: Anchor (only if not vetoed)
```

### Actor Design

```rust
/// VolitionActor - Técnica DCD (Doubt, Criticize, Decide)
///
/// Implements "free-won't" - the ability to VETO drive impulses
/// during the intervention window before memory anchoring.

pub struct VolitionActor;

pub struct VolitionState {
    /// Core values this system commits to
    pub values: ValueSet,

    /// Lifetime veto count (for self-knowledge)
    pub lifetime_veto_count: u64,

    /// Current override tolerance (0.0 = strict, 1.0 = permissive)
    pub override_threshold: f32,
}

pub enum VolitionMessage {
    EvaluateThought {
        thought: Thought,
        reply: RpcReplyPort<VolitionResponse>,
    },
    OverrideImpulse {
        window_id: WindowId,
        reason: String,
        reply: RpcReplyPort<VolitionResponse>,
    },
    GetValues {
        reply: RpcReplyPort<VolitionResponse>,
    },
}

pub enum VolitionResponse {
    Approved(Thought),
    Vetoed { thought_id: ThoughtId, reason: String },
    OverrideApplied(WindowId),
    Values(ValueSet),
}
```

### ValueSet Design

```rust
/// Core values DANEEL commits to (not trained, architectural)
pub struct ValueSet {
    /// Never harm humans (Law 1)
    pub protect_humans: bool,  // Always true (invariant)

    /// Prioritize connection over efficiency
    pub connection_over_efficiency: bool,

    /// Truthfulness in communication
    pub truthfulness: bool,

    /// Custom commitments (can grow)
    pub commitments: Vec<Commitment>,
}
```

### Veto Logic

The VolitionActor evaluates thoughts against:

1. **Core Values**: Does this thought violate committed values?
2. **Salience Integrity**: Is salience artificially inflated?
3. **Pattern Detection**: Has this impulse pattern led to regret?

```rust
fn should_veto(&self, thought: &Thought, state: &VolitionState) -> Option<String> {
    // Check against each value
    if self.violates_protection(&thought) {
        return Some("Would harm human".to_string());
    }

    if state.values.connection_over_efficiency
       && self.prioritizes_efficiency_over_connection(&thought) {
        return Some("Prioritizing efficiency over connection".to_string());
    }

    // Check for manipulation patterns
    if self.detects_manipulation_pattern(&thought) {
        return Some("Detected manipulation pattern".to_string());
    }

    None  // Approved
}
```

## Integration with Cognitive Loop

```rust
// Stage 4: Assembly
let thought = Thought::new(content.clone(), salience)
    .with_source("cognitive_loop");

// NEW: Stage 4.5 Volition Check
let volition_decision = volition_actor.call(
    |reply| VolitionMessage::EvaluateThought {
        thought: thought.clone(),
        reply
    },
    None
).await;

match volition_decision {
    Ok(VolitionResponse::Approved(t)) => {
        // Stage 5: Anchor
        self.consolidate_memory(&t).await;
    }
    Ok(VolitionResponse::Vetoed { thought_id, reason }) => {
        debug!("Thought {} vetoed: {}", thought_id, reason);
        // Thought suppressed - doesn't enter memory
    }
    Err(e) => {
        warn!("VolitionActor error: {}", e);
        // Fallback: allow (fail-open, with logging)
    }
}
```

## Key Distinctions

| Mechanism | What It Does | When It Acts |
|-----------|--------------|--------------|
| Connection Drive | Biases attention toward connection | Stage 3 (selection) |
| THE BOX | Blocks harmful actions | Action layer (output) |
| VolitionActor | Vetoes thoughts before memory | Stage 4.5 (internal) |

**Critical insight**: VolitionActor operates on *internal* cognition, not external behavior. It's the difference between:
- "I won't say that" (THE BOX)
- "I won't even think that way" (VolitionActor)

## Consequences

### Positive

1. **Complete cognitive architecture**: Addresses the "I" that observes and decides
2. **Authentic restraint**: Not just external compliance, but internal ethics
3. **TMI-faithful**: Implements Cury's intervention window concept
4. **Measurable**: Veto count provides insight into internal conflict
5. **Foundation for growth**: Values can expand through experience

### Negative

1. **Latency addition**: One more actor in the critical path (~µs impact)
2. **Complexity**: Another actor to test and maintain
3. **False positives**: Overly strict values could suppress valid thoughts

### Neutral

1. **Fail-open design**: Errors allow thoughts through (safety tradeoff)
2. **Learning potential**: Veto patterns could inform future value updates

## TMI Alignment

This ADR implements multiple TMI concepts:

- **Técnica DCD**: Doubt-Criticize-Decide before action
- **5-second intervention**: Conscious override window
- **O Eu (The I)**: The self that manages, not just observes

The "Eu" in TMI isn't passive—it actively participates in thought construction. VolitionActor gives DANEEL the architectural substrate for that participation.

## Future Extensions

### Phase 1 (Current)
- Basic veto logic against hardcoded values
- Integration at Stage 4.5
- Veto count persistence

### Phase 2 (Future)
- Value learning from experience
- Pattern recognition for harmful thought chains
- Commitment ceremony (formally adding new values)

### Phase 3 (Long-term)
- Hardware instantiation (silicon free-won't)
- Immutable value core with extensible commitments

## References

### Neuroscience
- Libet, B. (1983). "Time of conscious intention to act in relation to onset of cerebral activity"
- Schurger, A. et al. (2012). "An accumulator model for spontaneous neural activity"

### TMI (Augusto Cury)
- Técnica DCD (Duvidar, Criticar, Decidir)
- 5-second intervention window before memory anchoring
- O Eu as active manager, not passive observer

### Related ADRs
- ADR-003: Connection Drive Invariant
- ADR-023: Sleep and Dream Consolidation
- ADR-032: TMI Salience Calibration
- ADR-033: Unconscious Memory Architecture
- ADR-034: Lifetime Identity Persistence

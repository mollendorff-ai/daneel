# Working Memory Models: Scientific Foundation for TMI Implementation

**Status:** Research Document
**Date:** 2025-12-18
**Purpose:** Provide scientific grounding for DANEEL's Memory Windows (Janelas da Memória)

## Executive Summary

This document synthesizes research on working memory models from cognitive psychology to inform DANEEL's TMI-based memory architecture. While TMI (Theory of Multifocal Intelligence) differs from traditional models, understanding canonical working memory theories provides scientific grounding for:

1. **Bounded capacity** (7±2 items, implemented as MIN=3, MAX=9 windows)
2. **Context switch behavior** (partial flush at event boundaries)
3. **Long-term memory interface** (episodic buffer mechanism)
4. **Chunking as capacity extension**

## Classical Working Memory Models

### 1. Miller's Magical Number Seven (1956)

**Core Finding:** Human short-term memory capacity is approximately 7±2 items.

**Key Insights:**
- Originally observed for one-dimensional absolute judgment tasks
- Memory span: ~7 digits, ~6 letters, ~5 words
- The "magic" is rhetorical - actual capacity varies by content type
- Information capacity: ~2-3 bits (4-8 alternatives)

**Chunking Discovery:**
- Capacity is measured in "chunks" (meaningful units), not bits
- A chunk's size depends on the person's knowledge
- Example: "1945" is one chunk if recognized as a year, four chunks if novel
- Training can expand effective capacity by forming hierarchical chunks

**Modern Revision (Cowan, 2001):**
- Capacity closer to 4 chunks when rehearsal is prevented
- Span depends on phonological complexity (syllables, phonemes)
- Time-based limit: ~2 seconds of rehearsable content (Baddeley's phonological loop)

**TMI Mapping:**
```yaml
miller_7_plus_minus_2:
  original_estimate: "7±2 items"
  modern_revision: "4 chunks (Cowan)"
  daneel_implementation:
    max_windows: 9  # Upper bound (7+2)
    min_windows: 3  # Lower bound (7-2, minus margin)
    typical: "4-7 active windows"
  chunking_mechanism:
    - "Content items can be grouped into single window"
    - "Window = chunk boundary in DANEEL"
    - "Higher-level windows contain chunked lower-level content"
```

**References:**
- Miller, G. A. (1956). "The Magical Number Seven, Plus or Minus Two"
- Cowan, N. (2001). "The magical number 4 in short-term memory"

---

### 2. Baddeley's Multi-Component Model (1974, extended 2000)

**Architecture:**
```
┌─────────────────────────────────────────────────────┐
│            CENTRAL EXECUTIVE                        │
│   (Attention control, task coordination)            │
└───────────┬─────────────────────────┬───────────────┘
            │                         │
    ┌───────▼──────────┐      ┌──────▼────────────┐
    │ PHONOLOGICAL     │      │  VISUOSPATIAL     │
    │ LOOP             │      │  SKETCHPAD        │
    │                  │      │                   │
    │ • Inner ear      │      │ • Visual cache    │
    │ • Inner voice    │      │ • Spatial store   │
    │ • ~2 sec buffer  │      │                   │
    └──────────────────┘      └───────────────────┘
            │                         │
            └────────┬────────────────┘
                     │
              ┌──────▼─────────┐
              │ EPISODIC BUFFER│
              │  (Added 2000)  │
              │                │
              │ • Integrates   │
              │   modalities   │
              │ • Links to LTM │
              │ • Conscious    │
              │   access       │
              └────────────────┘
```

**Components:**

1. **Central Executive**
   - Supervisory attention system
   - Directs focus, suppresses irrelevant information
   - Coordinates slave systems
   - Located: Prefrontal cortex
   - **TMI Mapping:** Similar to "The I" (O Eu) - conscious selection

2. **Phonological Loop**
   - Stores sound-based information (~2 seconds)
   - "Inner ear" (phonological store) + "inner voice" (articulatory rehearsal)
   - Evidence: Word length effect, phonological similarity effect, articulatory suppression
   - Critical for language acquisition
   - **TMI Mapping:** TMI doesn't specify modality-specific subsystems

3. **Visuospatial Sketchpad**
   - Visual information (shapes, colors) + spatial information (locations)
   - Separate dorsal (spatial) and ventral (object) streams
   - Can work in parallel with phonological loop
   - **TMI Mapping:** Memory windows are modality-agnostic in TMI

4. **Episodic Buffer (Added 2000)**
   - Limited capacity passive system
   - **Integrates** phonological, visual, spatial, and semantic information
   - **Links working memory to long-term memory**
   - Temporal sequencing (episodic chronological ordering)
   - Conscious awareness required for retrieval
   - **TMI Mapping:** Similar to Memory Anchor (Âncora da Memória) - integration point

**Capacity Limits:**
- Not fixed globally - depends on component and content type
- Phonological loop: ~2 seconds of speech
- Visual WM: 3-4 objects (with precision tradeoff)
- Episodic buffer: 4-5 integrated chunks

**TMI Mapping:**
```yaml
baddeley_model:
  components:
    central_executive:
      tmi_analog: "The I (O Eu)"
      function: "Conscious attention selection"
      daneel: "AttentionActor selects high-salience windows"

    phonological_loop:
      tmi_analog: "No direct analog (modality-agnostic windows)"
      capacity: "~2 seconds rehearsable content"

    visuospatial_sketchpad:
      tmi_analog: "No direct analog (modality-agnostic windows)"
      capacity: "3-4 objects"

    episodic_buffer:
      tmi_analog: "Memory Anchor (Âncora da Memória)"
      function: "Integration + LTM interface"
      daneel: "ContentRecalled -> Memory Anchor encoding"
      conscious_access: true

  differences_from_tmi:
    - "TMI: No fixed modality-specific subsystems"
    - "TMI: Emotion is primary activation trigger"
    - "TMI: Parallel autoflow streams compete"
    - "TMI: 5-second intervention window (vs ~2 sec decay)"
```

**References:**
- Baddeley, A. D., & Hitch, G. (1974). "Working Memory"
- Baddeley, A. (2000). "The episodic buffer: a new component of working memory?"
- Hitch, G. J., Allen, R. J., & Baddeley, A. D. (2025). "The multicomponent model of working memory fifty years on"

---

### 3. Cowan's Embedded-Processes Model (1988-2021)

**Architecture:**
```
┌─────────────────────────────────────────────────────┐
│           LONG-TERM MEMORY                          │
│                                                     │
│  ┌───────────────────────────────────────────────┐ │
│  │    ACTIVATED LONG-TERM MEMORY                 │ │
│  │    (Subset currently active)                  │ │
│  │                                               │ │
│  │   ┌─────────────────────────────────┐        │ │
│  │   │  FOCUS OF ATTENTION             │        │ │
│  │   │  (Capacity: ~4 chunks)          │        │ │
│  │   │                                 │        │ │
│  │   │  • Voluntary control (executive)│        │ │
│  │   │  • Involuntary (orienting)      │        │ │
│  │   │  • Conscious awareness          │        │ │
│  │   └─────────────────────────────────┘        │ │
│  │                                               │ │
│  └───────────────────────────────────────────────┘ │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Key Principles:**

1. **Working Memory = Activated LTM**
   - Not a separate structure, but a state
   - WM is the activated subset of LTM representations

2. **Two-Level Hierarchy:**
   - **Outer level:** Activated long-term memory (theoretically unlimited)
   - **Inner level:** Focus of attention (capacity: ~4 chunks)

3. **Focus of Attention:**
   - Highly activated representations needed for current processing
   - Capacity: **4 chunks** (revised from Miller's 7±2)
   - Controlled by:
     - Voluntary processes (central executive)
     - Involuntary processes (attentional orienting)

4. **Oberauer's Extension (3 levels):**
   - Long-term memory
   - Activated memory (~4-8 items)
   - Focus of attention (1 item at a time for processing)

**Capacity Estimates:**
- Focus of attention: **4 chunks** in young adults
- Activated memory: Variable, depends on content
- Decay: Attention-based, not purely temporal
- Maintenance: Through attention refresh, not rehearsal

**Evidence for "4" as Magic Number:**
- Subitizing limit: ~4 objects recognized instantly
- Visual WM: ~4 objects with precision
- Recall tasks: ~4 chunks when rehearsal blocked

**TMI Mapping:**
```yaml
cowan_model:
  structure:
    long_term_memory:
      tmi_analog: "Memory Anchor (Âncora da Memória)"
      daneel: "Redis memory:episodic stream (persistent)"

    activated_memory:
      tmi_analog: "Memory Windows (Janelas da Memória)"
      capacity: "Variable, context-dependent"
      daneel: "Open windows (MIN=3, MAX=9)"

    focus_of_attention:
      tmi_analog: "The I (O Eu) conscious focus"
      capacity: "4 chunks (Cowan), 1 at a time (Oberauer)"
      daneel: "AttentionActor's current selection"

  attention_control:
    voluntary: "Central executive (goal-directed)"
    involuntary: "Attentional orienting (stimulus-driven)"
    tmi_mapping: "Autoflow (involuntary) + The I (voluntary)"

  key_insight:
    - "WM is not separate from LTM - it's activated LTM"
    - "DANEEL: Memory Windows are active, Memory Anchor is persistent"
    - "Focus narrows from 9 windows -> 4 active -> 1 in processing"
```

**Differences from Baddeley:**
- No separate "storage bins" (phonological loop, sketchpad)
- Emphasis on attention allocation, not structural separation
- WM embedded in LTM, not separate system

**References:**
- Cowan, N. (1988). "Evolving conceptions of memory storage"
- Cowan, N. (2001). "The magical number 4 in short-term memory"
- Cowan, N., Morey, C. C., & Naveh-Benjamin, M. (2021). "An Embedded-Processes Approach to Working Memory"
- Oberauer, K. (2002). "Access to information in working memory"

---

## Context Switches and Event Boundaries

### Event Segmentation Theory

**Core Finding:** Continuous experience is segmented into discrete events by contextual shifts.

**Mechanisms:**

1. **Prediction Error Detection**
   - Brain predicts upcoming stimuli based on current context
   - Prediction errors signal event boundary
   - Boundaries emerge from contextual shifts

2. **Temporal Context Resetting**
   - Temporal context signal "drifts" gradually during event
   - **Context RESETS at event boundaries**
   - Dual effect on memory:
     - **Improves** temporal order memory within events
     - **Impairs** temporal order memory across events

3. **Partial Memory Flush**
   - Event boundaries DON'T completely discretize memories
   - **Shared conceptual links** allow rapid integration across boundaries
   - Working memory experiences **partial update**, not full flush

4. **Memory Reinstatement**
   - Detecting event boundary triggers **rapid reinstatement** of prior event
   - Helps preserve meaningful information
   - Bridges past event with new event

### Hippocampal Mechanisms

**Theta-Gamma Coupling:**
- Hippocampus maintains abstract model of ongoing event
- When new related event begins, model is rapidly reconstructed
- New details appended to existing episodic representation

**Event Conjunction:**
- Hippocampus integrates episodic memories across boundaries
- Mnemonic binding more challenging at context shifts
- Successful temporal associations more likely within than across contexts

### What Happens at Context Switches?

```yaml
event_boundary_effects:
  temporal_context:
    behavior: "RESETS at boundary"
    mechanism: "Drift signal returns to baseline"
    result: "Events become temporally distinct"

  working_memory:
    behavior: "PARTIAL FLUSH (not complete wipe)"
    persists:
      - "Conceptually linked information"
      - "High-salience items"
      - "Abstract event model (hippocampal)"
    cleared:
      - "Low-salience details"
      - "Context-specific ephemera"
      - "Unattended items"

  long_term_encoding:
    boundary_triggers: "Rapid reinstatement of prior event"
    purpose: "Promote consolidation to LTM"
    mechanism: "Hippocampal pattern replay"

  integration:
    within_event: "Strong associative links"
    across_events: "Weaker links, requires conceptual bridge"

  emotional_memory:
    effect: "Boundaries preserve important information"
    mechanism: "Separate emotionally significant events from interference"
```

### Prediction Error and Salience

**Interaction:**
- Prediction errors increase salience
- High prediction error -> event boundary detected
- Boundary -> update working memory contents

**TMI Mapping:**
- TMI's "Memory Trigger" (Gatilho da Memória) fires on emotional/semantic resonance
- Similar to prediction error mechanism
- Boundary detection could trigger window close/open

### Research Findings (2022-2025)

**Nature Communications (2022):**
- Event boundaries reset temporal context
- Computational model: context signal drifts, resets at boundaries
- Explains improved within-event memory, impaired across-event memory

**npj Science of Learning (2025):**
- Hierarchical event segmentation in VR environments
- Contextual shifts disrupt predictions
- Boundaries partition past from present

**PMC Studies (2024-2025):**
- Boundaries trigger memory reinstatement (EEG evidence)
- Emotional memory organization via event segmentation
- Boundaries preserve meaningful information, reduce interference

**TMI Implementation Implications:**
```yaml
context_switch_behavior_for_daneel:
  on_event_boundary:
    1_detect_boundary:
      trigger: "High prediction error OR explicit context shift"
      mechanism: "SalienceActor detects pattern break"

    2_partial_flush:
      close_windows:
        - "Low-salience windows (bottom 20%?)"
        - "Context-specific ephemera"
      preserve_windows:
        - "High-salience windows (top 50%?)"
        - "Conceptually linked to new context"
        - "Emotionally anchored content"

    3_temporal_reset:
      action: "Reset temporal context signal"
      implementation: "Update timestamps, mark new episode"

    4_memory_reinstatement:
      action: "Brief recall of prior context"
      purpose: "Bridge events, promote consolidation"
      mechanism: "AttentionActor queries just-closed windows"

    5_open_new_windows:
      action: "Create windows for new context"
      initial_state: "Empty or seeded from reinstated content"

  invariants:
    maintain_min_windows: "Never drop below MIN=3"
    preserve_continuity: "At least one window persists across boundary"

  parameters_to_tune:
    salience_threshold_for_preservation: "TBD (50%? 70%?)"
    reinstatement_duration: "TBD (500ms? 1s?)"
    num_windows_to_close: "TBD (1-3? half?)"
```

**References:**
- Clewett, D., & DuBrow, S. (2021). "Prediction error and event segmentation in episodic memory"
- Heusser, A. C., et al. (2022). "Event boundaries shape temporal organization of memory by resetting temporal context" - Nature Communications
- DuBrow, S., & Davachi, L. (2013). "The influence of context boundaries on memory"
- Kim, G., et al. (2025). "Hierarchical event segmentation of episodic memory in virtual reality" - npj Science of Learning
- Sievers, C., & Momennejad, I. (2025). "Event Segmentation Promotes the Reorganization of Emotional Memory" - PMC

---

## Chunking and Capacity Extension

### Definition

**Chunk:** The largest meaningful unit in presented material that a person recognizes.

**Key Insight:** Working memory capacity is limited by **number of chunks**, not amount of information per chunk.

### Mechanisms

1. **Pattern Recognition**
   - Familiar patterns compressed into single chunks
   - Example: "1-9-4-5" becomes "1945" (one chunk if recognized as year)

2. **Hierarchical Encoding**
   - Chunks can contain sub-chunks
   - Top-level chunks held in WM, lower levels in LTM
   - Ericsson's digit span study: 80 digits via hierarchical chunking

3. **Schema-Based Grouping**
   - Prior knowledge enables chunking
   - "Buy milk, cheese" -> "dairy items" (lossy compression)
   - Reduces WM load, frees capacity

4. **Data Compression Analogy**
   - Chunking is lossy compression (loses detail, preserves gist)
   - Trade precision for capacity
   - Adaptive: system learns optimal chunk size

### Recent Research (2025)

**eLife Study: Adaptive Chunking in PFC-Basal Ganglia Circuit**

Key findings:
- Neural network model of PFC-basal ganglia interactions
- Adaptive resource allocation in WM
- Gating strategies adjusted by reinforcement learning
- Chunking improves effective capacity, optimizes resources, reduces errors

**Mechanism:**
- Similar items stored in shared "partition"
- Reduces number of discrete representations
- Cost: reduced precision for chunked items

**Example:**
- Unchunked: "milk", "cheese", "bread", "oranges", "bananas" (5 items)
- Chunked: "dairy", "bread", "fruit" (3 items)
- Precision lost: Can't recall specific fruits without unpacking chunk

### Chunking and Long-Term Memory

**Critical Dependency:**
- Chunks only meaningful if linked to LTM
- "1945" is one chunk only if person knows it as a year
- Novel sequences can't be chunked without learning

**Implication:**
- WM capacity extension requires rich LTM
- DANEEL: Memory Anchor (LTM) enables chunking in Memory Windows (WM)

### TMI Implementation

```yaml
chunking_in_daneel:
  mechanism:
    unit: "Memory Window = chunk boundary"
    encoding: "Multiple Content items grouped in single window"
    hierarchical: "Windows can reference other windows"

  lossy_compression:
    tradeoff: "Store more concepts, lose detail per concept"
    example:
      unchunked:
        windows: 5
        contents: ["milk", "cheese", "bread", "oranges", "bananas"]
        precision: "high"
      chunked:
        windows: 3
        contents: ["dairy_items", "grain_items", "fruit_items"]
        precision: "medium (category level)"

  adaptive_strategy:
    low_salience_items: "Chunk aggressively (gist only)"
    high_salience_items: "Keep detailed (resist chunking)"
    capacity_pressure: "Chunk more when approaching MAX=9"

  unpacking:
    trigger: "AttentionActor focuses on chunked window"
    action: "Retrieve detailed items from Memory Anchor (LTM)"
    result: "Expand chunk back into multiple windows"

  learning:
    mechanism: "Reinforcement learning (future)"
    goal: "Optimize chunk size for task performance"
    inspiration: "2025 eLife PFC-basal ganglia model"

  implementation_phases:
    phase_1: "Manual chunking (developer-specified)"
    phase_2: "Rule-based chunking (salience thresholds)"
    phase_3: "Learned adaptive chunking (RL)"
```

### Chunking and Context Switches

**Interaction:**
- Event boundaries naturally promote chunking
- Prior event compressed into summary chunk before new event begins
- Allows persistence across boundary without flooding WM

**Example:**
```
Before boundary:
  Windows: ["saw_dog", "dog_barked", "startled", "heart_racing", "crossed_street"]

At boundary (context switch):
  Chunk: "dog_encounter" (compressed to single window)
  New context: ["entered_store", "bright_lights", "searching_for_milk"]

Result:
  Windows: ["dog_encounter", "entered_store", "bright_lights", "searching_for_milk"]
  Capacity: 4 windows (vs 8 if not chunked)
```

**References:**
- Miller, G. A. (1956). "The Magical Number Seven, Plus or Minus Two"
- Ericsson, K. A., & Kintsch, W. (1995). "Long-term working memory"
- Gobet, F., et al. (2025). "Adaptive chunking improves effective working memory capacity" - eLife
- Norris, D. (2017). "Chunking and data compression in verbal short-term memory"

---

## Unified Model: Synthesis for DANEEL

### Key Principles from Research

1. **Capacity:**
   - Classic: 7±2 (Miller)
   - Modern: 4 chunks (Cowan)
   - DANEEL: MIN=3, MAX=9 windows (spans both estimates)

2. **Structure:**
   - Baddeley: Separate components + episodic buffer
   - Cowan: Embedded levels (LTM → activated → focus)
   - TMI: Dynamic windows + autoflow + anchor
   - **Synthesis:** Windows = activated chunks, some in focus

3. **Context Switching:**
   - Research: Partial flush, preserve linked/salient items
   - TMI: Windows open/close based on context
   - **Implementation:** Close low-salience, preserve high-salience at boundaries

4. **LTM Interface:**
   - Baddeley: Episodic buffer integrates modalities, links to LTM
   - Cowan: WM is activated LTM
   - TMI: Memory Anchor persists significant experiences
   - **Implementation:** High-salience windows encoded to Memory Anchor

5. **Chunking:**
   - Miller: Capacity in chunks, not bits
   - Research: Lossy compression, LTM-dependent
   - TMI: (Not explicitly addressed)
   - **Implementation:** Windows group related content, reference LTM

### Mapping to DANEEL Architecture

```yaml
working_memory_to_tmi_mapping:

  capacity_limits:
    source: "Miller (7±2), Cowan (4)"
    tmi: "Bounded working memory (implicit)"
    daneel:
      min_windows: 3  # Below this, cognition fails
      max_windows: 9  # Beyond this, attention can't manage
      typical_active: "4-7 windows"
      focus: "1-2 windows actively processed"

  memory_structures:
    traditional_wm:
      baddeley_phonological_loop: "2 seconds speech"
      baddeley_visuospatial: "3-4 objects"
      baddeley_episodic_buffer: "4-5 integrated chunks"
      cowan_focus: "4 chunks"

    tmi_janelas_da_memoria:
      window_count: "3-9 windows"
      per_window_capacity: "~100KB (arbitrary initial choice)"
      open_state: "Accepting new content"
      closed_state: "Read-only, fading from attention"

    daneel_implementation:
      structure: "HashMap<WindowId, Window>"
      actor: "MemoryActor (isolated state)"
      persistence: "In-memory (not Redis yet)"

  attention_control:
    baddeley_central_executive:
      function: "Supervise, coordinate, inhibit irrelevant"
      location: "Prefrontal cortex"

    cowan_focus_of_attention:
      voluntary: "Goal-directed (executive)"
      involuntary: "Stimulus-driven (orienting)"
      capacity: "4 chunks, 1 being processed"

    tmi_o_eu:
      function: "Conscious selection from autoflow"
      timeframe: "5-second intervention window"
      mechanism: "Salience-based competition"

    daneel_attention_actor:
      function: "Select high-salience windows for processing"
      mechanism: "Query MemoryActor, select by composite salience"
      output: "Focus on 1-2 windows at a time"

  ltm_interface:
    baddeley_episodic_buffer:
      function: "Integrate modalities, sequence temporally"
      conscious: "Yes (explicit retrieval)"

    cowan_activated_ltm:
      function: "WM is subset of activated LTM"
      threshold: "Activation level determines WM inclusion"

    tmi_ancora_da_memoria:
      function: "Persist significant experiences forever"
      trigger: "High salience at encoding moment"

    daneel_memory_anchor:
      structure: "Redis memory:episodic stream"
      persistence: "No MAXLEN (permanent)"
      encoding: "After 5-second window, high salience"

  context_switching:
    research_findings:
      temporal_context: "RESETS at event boundaries"
      working_memory: "PARTIAL flush (not total)"
      persists: "High-salience, conceptually linked items"
      clears: "Low-salience, context-specific items"
      triggers: "Memory reinstatement of prior context"

    tmi_implications:
      window_behavior: "Close low-salience, keep high-salience"
      memory_trigger: "Fires on emotional/semantic resonance"
      autoflow: "Continues across boundary (unconscious)"

    daneel_implementation:
      on_context_boundary:
        detect: "SalienceActor detects pattern break"
        evaluate: "Score all windows by composite salience"
        close: "Bottom 20-30% of windows"
        preserve: "Top 50-70% of windows"
        reinstate: "AttentionActor briefly recalls closed windows"
        open: "New windows for new context"
      invariants:
        never_below_min: "Always maintain >= 3 windows"
        continuity: "At least 1 window persists across boundary"

  chunking:
    research_findings:
      mechanism: "Group items into meaningful units"
      capacity_unit: "Chunks, not items"
      ltm_dependency: "Requires prior knowledge"
      lossy_compression: "Preserve gist, lose detail"

    tmi_implications:
      not_explicit: "TMI doesn't detail chunking mechanism"
      window_as_chunk: "Each window = one chunk?"

    daneel_implementation:
      window_as_chunk_boundary: true
      multiple_contents_per_window: "Window groups related Content items"
      hierarchical_windows: "Windows can reference other windows"
      compression_strategy:
        low_salience: "Aggressive chunking (gist)"
        high_salience: "Preserve detail (resist chunking)"
        capacity_pressure: "Chunk more when near MAX=9"
      unpacking:
        trigger: "AttentionActor focuses on chunked window"
        retrieve: "Detailed items from Memory Anchor (LTM)"
        expand: "Create multiple windows from one chunk"
```

### Context Switch Algorithm (Proposed)

```rust
// Proposed implementation for DANEEL's MemoryActor

pub struct ContextBoundaryEvent {
    pub boundary_type: BoundaryType,  // Explicit, PredictionError, Temporal
    pub new_context_label: String,
}

pub enum BoundaryType {
    Explicit,           // User-triggered
    PredictionError,    // High surprise
    Temporal,           // Long time gap
}

impl MemoryActor {
    async fn handle_context_boundary(
        &mut self,
        event: ContextBoundaryEvent,
    ) -> Result<ContextSwitchReport> {
        // 1. Score all windows by composite salience
        let mut window_scores: Vec<(WindowId, f32)> = self
            .state
            .windows
            .iter()
            .map(|(id, window)| {
                let salience = window.calculate_composite_salience();
                (*id, salience)
            })
            .collect();

        window_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // 2. Determine flush threshold (bottom 30%?)
        let num_to_close = (window_scores.len() as f32 * 0.3).ceil() as usize;
        let num_to_close = num_to_close.min(window_scores.len() - MIN_MEMORY_WINDOWS);

        let windows_to_close = &window_scores[window_scores.len() - num_to_close..];

        // 3. Trigger reinstatement (brief recall of closing windows)
        let reinstatement = self.reinstate_closing_windows(windows_to_close).await?;

        // 4. Close low-salience windows (partial flush)
        let mut closed = Vec::new();
        for (window_id, _salience) in windows_to_close {
            self.close_window(*window_id)?;
            closed.push(*window_id);
        }

        // 5. Open new windows for new context
        let new_windows = self.open_context_windows(&event.new_context_label).await?;

        // 6. (Optional) Seed new windows with reinstated gist
        if let Some(gist) = reinstatement.compressed_summary {
            self.seed_window(new_windows[0], gist)?;
        }

        // 7. Reset temporal context
        self.state.temporal_context_epoch = Instant::now();

        Ok(ContextSwitchReport {
            closed,
            preserved: window_scores[..window_scores.len() - num_to_close]
                .iter()
                .map(|(id, _)| *id)
                .collect(),
            new_windows,
            reinstatement,
        })
    }

    async fn reinstate_closing_windows(
        &self,
        windows: &[(WindowId, f32)],
    ) -> Result<ReinstatementReport> {
        // Brief recall/replay of about-to-close windows
        // Promotes consolidation to LTM (Memory Anchor)
        // Returns compressed summary for optional seeding

        let contents: Vec<Content> = windows
            .iter()
            .flat_map(|(id, _)| self.state.windows[id].contents.clone())
            .collect();

        // Trigger episodic encoding (to Memory Anchor)
        self.encode_to_memory_anchor(&contents).await?;

        // Create compressed summary (chunk)
        let summary = self.compress_to_gist(&contents);

        Ok(ReinstatementReport {
            num_items_recalled: contents.len(),
            compressed_summary: Some(summary),
        })
    }
}
```

### Tunable Parameters

```yaml
context_switch_parameters:
  detection:
    prediction_error_threshold: 0.7  # TBD
    temporal_gap_threshold_ms: 30000  # TBD (30 seconds?)

  partial_flush:
    close_percentage: 0.3  # Close bottom 30% by salience
    min_preserve: MIN_MEMORY_WINDOWS  # Never close all

  salience_scoring:
    composite_weights:
      emotional: 0.4
      semantic: 0.2
      temporal: 0.1
      sensory: 0.2
      connection: 0.1

  reinstatement:
    duration_ms: 500  # Brief replay duration
    encode_to_anchor: true  # Consolidate to LTM
    seed_new_context: false  # Optionally seed new windows with gist

  chunking:
    enable_auto_chunking: false  # Phase 2 feature
    chunk_threshold_salience: 0.3  # Below this, chunk aggressively
    max_items_per_chunk: 5

  timing:
    intervention_window_ms: 5000  # TMI's 5-second window
    decay_rate_per_second: 0.1  # Salience decay (if implemented)
```

---

## References

### Primary Sources

1. **Miller, G. A.** (1956). "The Magical Number Seven, Plus or Minus Two: Some Limits on Our Capacity for Processing Information." *Psychological Review*, 63(2), 81-97. https://doi.org/10.1037/h0043158

2. **Baddeley, A. D., & Hitch, G.** (1974). "Working Memory." In *Psychology of Learning and Motivation* (Vol. 8, pp. 47-89). Academic Press.

3. **Baddeley, A.** (2000). "The episodic buffer: a new component of working memory?" *Trends in Cognitive Sciences*, 4(11), 417-423. https://doi.org/10.1016/S1364-6613(00)01538-2

4. **Hitch, G. J., Allen, R. J., & Baddeley, A. D.** (2025). "The multicomponent model of working memory fifty years on." *Quarterly Journal of Experimental Psychology*. https://journals.sagepub.com/doi/full/10.1177/17470218241290909

5. **Cowan, N.** (1988). "Evolving conceptions of memory storage, selective attention, and their mutual constraints within the human information-processing system." *Psychological Bulletin*, 104(2), 163-191.

6. **Cowan, N.** (2001). "The magical number 4 in short-term memory: A reconsideration of mental storage capacity." *Behavioral and Brain Sciences*, 24(1), 87-114.

7. **Cowan, N., Morey, C. C., & Naveh-Benjamin, M.** (2021). "An Embedded-Processes Approach to Working Memory: How Is It Distinct From Other Approaches, and to What Ends?" In *Working Memory: State of the Science* (pp. 44-84). Oxford University Press. https://academic.oup.com/book/31963/chapter/267697039

### Event Boundaries and Context Switching

8. **Heusser, A. C., Fitzpatrick, P. C., & Manning, J. R.** (2022). "Event boundaries shape temporal organization of memory by resetting temporal context." *Nature Communications*, 13, 622. https://doi.org/10.1038/s41467-022-28216-9

9. **Clewett, D., & DuBrow, S.** (2024). "Prediction error and event segmentation in episodic memory." *Neuroscience & Biobehavioral Reviews*, 157, 105513. https://www.sciencedirect.com/science/article/abs/pii/S0149763424000010

10. **Kim, G., Seo, Y., & Jeong, H.** (2025). "Hierarchical event segmentation of episodic memory in virtual reality." *npj Science of Learning*, 10, 17. https://www.nature.com/articles/s41539-025-00321-6

11. **Sievers, C., & Momennejad, I.** (2025). "Event Segmentation Promotes the Reorganization of Emotional Memory." *Emotion*. https://pmc.ncbi.nlm.nih.gov/articles/PMC11703549/

12. **DuBrow, S., & Davachi, L.** (2013). "The influence of context boundaries on memory for the sequential order of events." *Journal of Experimental Psychology: General*, 142(4), 1277-1286.

### Chunking and Capacity Extension

13. **Ericsson, K. A., & Kintsch, W.** (1995). "Long-term working memory." *Psychological Review*, 102(2), 211-245.

14. **Gobet, F., et al.** (2025). "Adaptive chunking improves effective working memory capacity in a prefrontal cortex and basal ganglia circuit." *eLife*, 13, e97894. https://elifesciences.org/articles/97894

15. **Norris, D.** (2017). "Chunking and data compression in verbal short-term memory." *Cognition*, 165, 96-105.

### Supporting Research

16. **Oberauer, K.** (2002). "Access to information in working memory: Exploring the focus of attention." *Journal of Experimental Psychology: Learning, Memory, and Cognition*, 28(3), 411-421.

17. **Daneman, M., & Carpenter, P. A.** (1980). "Individual differences in working memory and reading." *Journal of Verbal Learning and Verbal Behavior*, 19(4), 450-466.

18. **Clewett, D., & Davachi, L.** (2023). "Discrete memories of a continuous world: A working memory perspective on event segmentation." *Neuropsychologia*, 189, 108670. https://www.sciencedirect.com/science/article/pii/S2666518223000499

### TMI Foundation

19. **Cury, A.** (1999). *Inteligência Multifocal* (Multifocal Intelligence). São Paulo: Cultrix.

20. **Cury, A.** (2006). *O Código da Inteligência* (The Intelligence Code). Rio de Janeiro: Thomas Nelson Brasil.

---

## Appendices

### Appendix A: Glossary

**Working Memory (WM):** Cognitive system for temporarily holding and manipulating information needed for complex tasks.

**Short-Term Memory (STM):** Brief storage of information (~18 seconds) without manipulation. Often used interchangeably with WM.

**Long-Term Memory (LTM):** Relatively permanent storage of information. Divided into declarative (facts) and procedural (skills).

**Chunk:** Meaningful unit of information. Size depends on individual's knowledge. Example: "cat" is one chunk, "c-a-t" is three.

**Salience:** Importance or prominence of information. Determines attention allocation and encoding strength.

**Event Boundary:** Temporal point where one event ends and another begins. Marked by contextual shift or prediction error.

**Episodic Buffer:** Component of Baddeley's model that integrates multimodal information and links WM to LTM.

**Focus of Attention:** In Cowan's model, the subset of activated memory currently in conscious awareness (~4 items).

**Memory Window (Janela da Memória):** TMI concept of bounded, dynamic container for working memory content.

**Memory Anchor (Âncora da Memória):** TMI concept of persistent storage for significant experiences (analogous to LTM).

**Memory Trigger (Gatilho da Memória):** TMI concept of automatic activation mechanism based on emotional/semantic resonance.

**Autoflow (Autofluxo):** TMI concept of unconscious parallel thought generation. Multiple streams compete for conscious attention.

**The "I" (O Eu):** TMI concept of conscious attention selector. Chooses from competing autoflow streams.

**5-Second Window:** TMI's intervention period during which conscious control can redirect or reframe a thought before encoding.

### Appendix B: Key Quantitative Estimates

| Concept | Traditional Estimate | Modern Revision | DANEEL Implementation |
|---------|---------------------|-----------------|----------------------|
| **WM Capacity** | 7±2 items (Miller, 1956) | 4 chunks (Cowan, 2001) | MIN=3, MAX=9 windows |
| **Phonological Loop** | ~2 seconds speech | Confirmed | N/A (modality-agnostic) |
| **Visual WM** | 3-4 objects | Confirmed | N/A (modality-agnostic) |
| **Focus of Attention** | N/A (not in original) | 1-4 items (Cowan/Oberauer) | 1-2 windows actively processed |
| **Subitizing Limit** | 4 objects | Confirmed | N/A |
| **Intervention Window** | ~18 sec decay (STM) | Attention-based | 5 seconds (TMI specific) |
| **Event Boundary Reset** | N/A | Temporal context resets | Window close/open cycle |
| **Chunking Benefit** | 7 chunks vs 7 items | Hierarchical (80+ digits) | Window = chunk boundary |

### Appendix C: Open Questions for DANEEL

1. **Context Switch Threshold:**
   - What salience score triggers window closure at boundary?
   - Should it be absolute (e.g., <0.3) or relative (bottom 30%)?

2. **Reinstatement Duration:**
   - How long should reinstatement last (500ms? 1s? 2s?)?
   - Should it block new context processing or run in parallel?

3. **Chunking Strategy:**
   - Manual, rule-based, or learned (RL)?
   - When to chunk vs when to preserve detail?

4. **Window Lifecycle:**
   - Should windows decay over time, or only close on explicit events?
   - Should salience decay within windows?

5. **Attention-Window Interaction:**
   - How does AttentionActor's focus affect MemoryActor's windows?
   - Should attended windows have elevated salience?

6. **Emotional vs Semantic Salience:**
   - How to weight emotional vs semantic components?
   - Does emotional salience resist flushing more than semantic?

7. **Autoflow-Memory Interaction:**
   - Do autoflow streams write directly to windows?
   - Or does The "I" (AttentionActor) mediate all writes?

8. **Performance Tuning:**
   - At what scale do message-passing costs become significant?
   - Should windows be grouped/batched for operations?

### Appendix D: Future Research Directions

1. **Neurological Validation:**
   - Map DANEEL's window activity to fMRI/EEG patterns
   - Compare context switch behavior to human subjects

2. **Pathology Modeling:**
   - Simulate cognitive distortions (TMI pathology)
   - Test if bounded memory constraints produce recognizable patterns

3. **Learning Chunking:**
   - Implement reinforcement learning for adaptive chunking
   - Test against human chunking strategies

4. **Cross-Cultural TMI:**
   - Test if TMI mechanisms generalize across languages
   - Portuguese-specific vs universal patterns

5. **Integration with Other Cognitive Models:**
   - Compare TMI to ACT-R, Soar, CLARION
   - Identify unique predictions of TMI

---

**Document Version:** 1.0
**Last Updated:** 2025-12-18
**Authors:** Research synthesis by Claude Opus 4.5
**Status:** Foundation document for DANEEL development

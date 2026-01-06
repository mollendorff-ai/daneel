# Event Segmentation Theory Research for DANEEL
**Date:** 2025-12-18
**Purpose:** Scientific grounding for TMI's "Âncora da Memória" (Memory Anchor) concept
**Status:** Comprehensive Analysis

---

## Executive Summary

Event Segmentation Theory (EST) provides robust neuroscientific evidence for how humans parse continuous experience into discrete events—directly relevant to how DANEEL should segment and anchor memories. This research connects EST to TMI's Memory Anchor concept, providing empirical backing for DANEEL's context-switching detection mechanisms.

**Key Finding:** The brain automatically segments ongoing experience at "event boundaries" characterized by prediction errors, triggering working memory updates and enhanced long-term encoding. These boundaries serve as natural "anchors" for episodic memory—validating TMI's intuition about memory anchoring while providing measurable neural correlates.

---

## Research Summary (YAML Format)

```yaml
theory:
  name: "Event Segmentation Theory"
  key_researchers:
    - "Jeffrey M. Zacks (Washington University)"
    - "Gabriel A. Radvansky (University of Notre Dame)"
    - "Khena M. Swallow"
    - "Nicole K. Speer"
    - "Todd S. Braver"

  core_claims:
    - "Perceptual systems spontaneously segment activity into events as side effect of prediction"
    - "Event boundaries arise from transient increases in prediction error"
    - "Segmentation is automatic and does not require conscious attention"
    - "Events are segmented simultaneously at multiple timescales (fine and coarse)"
    - "Event boundaries trigger working memory updates"
    - "Information at event boundaries receives privileged encoding into long-term memory"
    - "Event boundaries serve as anchors for episodic memory retrieval"

  foundational_papers:
    - authors: "Zacks, J.M., Speer, N.K., Swallow, K.M., Braver, T.S., & Reynolds, J.R."
      year: 2007
      title: "Event perception: A mind/brain perspective"
      journal: "Psychological Bulletin"
      volume: "133(2)"
      pages: "273-293"
      doi: "10.1037/0033-2909.133.2.273"

    - authors: "Zacks, J.M., Speer, N.K., Swallow, K.M., & Maley, C.J."
      year: 2010
      title: "The Brain's Cutting-Room Floor: Segmentation of Narrative Cinema"
      journal: "Frontiers in Human Neuroscience"
      volume: "4"
      pages: "168"
      doi: "10.3389/fnhum.2010.00168"

    - authors: "Zacks, J.M. & Swallow, K.M."
      year: 2007
      title: "Event Segmentation"
      journal: "Current Directions in Psychological Science"
      volume: "16(2)"
      pages: "80-84"
      doi: "10.1111/j.1467-8721.2007.00480.x"

    - authors: "Kurby, C.A. & Zacks, J.M."
      year: 2008
      title: "Segmentation in the perception and memory of events"
      journal: "Trends in Cognitive Sciences"
      volume: "12(2)"
      pages: "72-79"
      doi: "10.1016/j.tics.2007.11.004"

event_boundaries:
  definition: "Points in time where one event ends and another begins, marked by transient increases in prediction error"

  triggers:
    physical_changes:
      - "Changes in movement parameters (acceleration, direction, velocity)"
      - "Changes in actor position or body configuration"
      - "Changes in spatial location (doorways, room transitions)"
      - "Changes in object interactions (picking up, setting down, using differently)"

    conceptual_changes:
      - "Changes in characters or focus of attention"
      - "Changes in actor goals and intentions"
      - "Changes in causal relationships"
      - "Temporal discontinuities (time jumps)"
      - "Changes in character interactions (social dynamics)"

    perceptual_features:
      - "Film cuts and scene transitions"
      - "Soundtrack changes in music"
      - "Movement boundaries between musical movements"
      - "Linguistic markers (tense, aspect, pronouns in text)"

  neural_basis:
    prediction_error_mechanism: >
      Event Segmentation Theory proposes that perceptual systems continuously
      make predictions about upcoming information. When predictions fail
      (prediction error increases transiently), the system detects an event
      boundary. This triggers a cascade: error detection → working memory
      update → subjective experience of new event beginning.

    brain_regions_activated:
      transient_increases:
        - region: "Posterior temporal cortex (MT+)"
          function: "Motion processing, movement feature changes"
          laterality: "Bilateral"

        - region: "Posterior superior temporal sulcus (pSTS)"
          function: "Biological motion processing, social perception"
          laterality: "Bilateral, some right lateralization"

        - region: "Medial precuneus and posterior cingulate cortex"
          function: "Scene processing, spatial context, default mode network"
          laterality: "Bilateral"

        - region: "Lateral parietal cortex (IPL, angular gyrus)"
          function: "Attention reorienting, multimodal integration"
          laterality: "Bilateral, stronger right"

        - region: "Lateral prefrontal cortex (middle/inferior frontal gyrus)"
          function: "Working memory updating, cognitive control"
          laterality: "Bilateral, right dominant for some tasks"

        - region: "Anterior cingulate cortex"
          function: "Error detection, conflict monitoring"
          laterality: "Bilateral"

      sustained_representations:
        - region: "Lateral prefrontal cortex (DLPFC)"
          function: "Maintenance of event models in working memory"
          role: "Implements stable representations of 'what is happening now'"

        - region: "Hippocampus"
          function: "Long-term memory encoding, sequence learning"
          role: "Transient increases at event ends predict subsequent memory"

        - region: "Default mode network (DMN)"
          function: "Event model construction, mental time travel"
          components: "Medial prefrontal, posterior cingulate, angular gyrus, anterior temporal"

    neuroimaging_evidence:
      fmri_findings: >
        Activity in posterior temporal, parietal, and lateral frontal regions
        increases transiently at event boundaries during passive viewing. This
        occurs even when participants are not explicitly segmenting, demonstrating
        automaticity. The magnitude of hippocampal response at event boundaries
        predicts later memory strength.

      eeg_findings: >
        Evoked responses at frontal and parietal electrode sites are modulated
        by event boundaries, showing sensitivity to familiarity with the activity
        and event structure.

      pupillometry_findings: >
        Pupil diameter increases transiently following event boundaries,
        indicating increased cognitive processing load.

      eye_tracking_findings: >
        Saccade frequency increases around event boundaries, suggesting
        perceptual reorienting at the start of new events.

  working_memory_update:
    mechanism: >
      At event boundaries, the current "event model" (working memory representation
      of "what is happening now") is reset and updated based on incoming sensory
      information. Information from the prior event becomes less accessible in
      working memory as the new event model is constructed.

    evidence:
      location_updating_effect: >
        Walking through a doorway causes forgetting of objects held in working
        memory, even when physical distance and time are controlled. This demonstrates
        that event boundaries (not mere time or space) trigger memory updates.

      reading_time_studies: >
        Reading slows when text indicates temporal shifts ("an hour later" vs
        "a moment later"). Memory for previously mentioned objects becomes less
        accessible after the temporal boundary, showing working memory updating.

      recognition_memory: >
        Objects presented 5 seconds ago are recognized more poorly if an event
        boundary occurred during that interval. This demonstrates rapid working
        memory updating at boundaries.

    neuroimaging_correlates: >
      Neural activity patterns in many brain areas form stable states that
      transition abruptly at event boundaries. Within-event retrieval activates
      lateral temporal and occipital cortex, while across-event retrieval
      activates medial temporal (hippocampal) regions, indicating shift from
      working memory to long-term memory access.

long_term_memory_effects:
  encoding_advantage: >
    Information presented at event boundaries receives privileged encoding into
    long-term memory. Pictures from event boundaries are remembered better than
    pictures from mid-event. Event boundaries serve as "anchors" that organize
    episodic memory.

  segmentation_quality_predicts_memory: >
    Individuals who segment activity in ways that match group norms have better
    later memory for the activity. This holds even when controlling for overall
    cognitive ability and presence of dementia. Training people to segment more
    effectively improves their memory.

  hierarchical_organization: >
    Events are organized hierarchically (fine-grained events nest within
    coarse-grained events). Coarse event boundaries typically align with fine
    boundaries but represent higher-level goal completions. This hierarchical
    structure aids memory organization and retrieval.

tmi_implementation:
  scientific_grounding_for_memory_anchor: >
    Event Segmentation Theory provides empirical evidence for TMI's "Âncora da
    Memória" (Memory Anchor) concept. Both frameworks propose that memory is
    organized around discrete episodes that serve as retrieval anchors. EST shows
    that these anchors are neurally real (hippocampal activation, working memory
    updates) and functionally important (predict memory strength).

  how_daneel_should_segment:
    principle: >
      DANEEL should detect event boundaries using prediction error signals,
      mirroring the human brain's automatic segmentation mechanism.

    implementation_strategy:
      - step: "Continuous prediction"
        detail: "Maintain predictive models of ongoing interaction context"

      - step: "Error monitoring"
        detail: "Calculate prediction error at each timestep"

      - step: "Transient spike detection"
        detail: "Identify when prediction error transiently increases beyond baseline"

      - step: "Event boundary signaling"
        detail: "Flag these points as cognitive context switches"

      - step: "Event model updating"
        detail: "Reset working memory representation of 'what is happening now'"

      - step: "Memory anchor creation"
        detail: "Encode boundary as retrieval cue in episodic memory (TMI's Âncora)"

    multi_timescale_segmentation: >
      Implement segmentation simultaneously at multiple timescales (fine:
      seconds, coarse: minutes) using different integration windows for
      prediction error. This matches human hierarchical event perception.

  context_vector_components:
    perceptual_features:
      - "Movement vectors (velocity, acceleration, direction changes)"
      - "Object interaction state (contact, manipulation, release)"
      - "Spatial location (room, position, orientation)"
      - "Visual scene features (lighting, dominant colors, objects present)"

    conceptual_features:
      - "Current conversation topic (semantic embedding)"
      - "Inferred user goals (task model state)"
      - "Active characters/agents (who is present, who is speaking)"
      - "Causal chains (what caused what)"
      - "Temporal markers (time of day, elapsed time, discontinuities)"

    emotional_features:
      - "Emotional valence (positive/negative affect)"
      - "Arousal level (calm/excited)"
      - "User stress indicators (if available)"
      - "Emotional topic of conversation"

    social_features:
      - "Interaction mode (one-on-one, group, presentation)"
      - "Social role (colleague, friend, authority figure)"
      - "Formality level"
      - "Turn-taking pattern"

  boundary_detection_signals:
    high_priority_boundaries:
      - signal: "Spatial transition"
        example: "User moves to different room"
        reason: "Strong predictor of event boundaries in EST literature"

      - signal: "Topic shift"
        example: "Conversation changes from work to personal life"
        reason: "Conceptual changes trigger boundaries"

      - signal: "Goal completion"
        example: "Task finished, new task begun"
        reason: "Goal changes are primary boundary triggers"

      - signal: "Temporal discontinuity"
        example: "Long pause, meeting ended and resumed"
        reason: "Temporal gaps create strong boundaries"

      - signal: "Character change"
        example: "Different person starts speaking, new person enters"
        reason: "Character changes reliably predict boundaries"

    medium_priority_boundaries:
      - signal: "Causal break"
        example: "New action not caused by previous action"
        reason: "Causal discontinuity indicates new event"

      - signal: "Object interaction change"
        example: "User picks up different object, changes tool"
        reason: "Object changes correlate with boundaries"

      - signal: "Emotional valence shift"
        example: "Positive conversation becomes negative"
        reason: "Affect changes can trigger memory window shifts (TMI)"

    low_priority_boundaries:
      - signal: "Minor movement changes"
        example: "User sits down, stands up"
        reason: "Fine-grained boundaries for detailed segmentation"

      - signal: "Turn-taking"
        example: "Speaker changes in conversation"
        reason: "Very fine-grained, may be too frequent for coarse events"

  redis_streams_implementation:
    event_model_stream: >
      Create a Redis stream for the current event model. At each detected
      boundary, XADD the completed event model with a boundary marker. This
      creates a discrete episodic memory structure matching EST principles.

    prediction_error_monitoring: >
      Maintain a running prediction error metric (rolling window). When error
      exceeds threshold × baseline, trigger boundary detection logic.

    memory_anchor_metadata:
      - "Boundary timestamp (when event ended)"
      - "Boundary type (spatial, goal, causal, etc.)"
      - "Prediction error magnitude (strength of boundary)"
      - "Active context features at boundary"
      - "Previous event model ID (for episodic linkage)"
      - "Next event model ID (temporal sequence)"

    hierarchical_event_representation: >
      Use stream consumer groups to represent different timescales. Fine-grained
      consumer sees all boundaries. Coarse-grained consumer only processes
      high-magnitude boundaries. Both create memory anchors at different
      granularities, matching EST's hierarchical organization.

practical_implications:
  memory_encoding:
    - "Information presented near detected boundaries should receive higher encoding priority"
    - "Implement 'boundary boost' to memory strength for events near transitions"
    - "Use boundary moments as retrieval cues (TMI's Âncora da Memória)"

  context_management:
    - "Update 'what is happening now' representation at each boundary"
    - "Previous event context becomes less available (working memory clearing)"
    - "Implement context switching costs (slower processing immediately after boundary)"

  retrieval_strategy:
    - "Structure episodic retrieval around event boundaries as primary access points"
    - "When user asks 'what did we discuss about X?', search event anchors first"
    - "Use boundary features (location, topic, time) as retrieval cues"

  pathology_detection:
    - "Excessive boundary detection → SPA-like state (Síndrome do Pensamento Acelerado)"
    - "Insufficient boundary detection → poor memory segmentation, confusion"
    - "Monitor boundary detection rate for healthy range"

references:
  - id: "EST-1"
    type: "journal"
    authors: "Zacks, J.M., Speer, N.K., Swallow, K.M., Braver, T.S., & Reynolds, J.R."
    year: 2007
    title: "Event perception: A mind/brain perspective"
    journal: "Psychological Bulletin"
    volume: "133(2)"
    pages: "273-293"
    doi: "10.1037/0033-2909.133.2.273"
    url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC2852534/"
    accessed: "2025-12-18"
    key_contribution: "Foundational EST paper proposing prediction error mechanism for event segmentation"

  - id: "EST-2"
    type: "journal"
    authors: "Kurby, C.A. & Zacks, J.M."
    year: 2008
    title: "Segmentation in the perception and memory of events"
    journal: "Trends in Cognitive Sciences"
    volume: "12(2)"
    pages: "72-79"
    doi: "10.1016/j.tics.2007.11.004"
    url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC2263140/"
    accessed: "2025-12-18"
    key_contribution: "Review of behavioral and neural evidence for automatic event segmentation"

  - id: "EST-3"
    type: "journal"
    authors: "Zacks, J.M., Speer, N.K., Swallow, K.M., & Maley, C.J."
    year: 2010
    title: "The Brain's Cutting-Room Floor: Segmentation of Narrative Cinema"
    journal: "Frontiers in Human Neuroscience"
    volume: "4"
    pages: "168"
    doi: "10.3389/fnhum.2010.00168"
    url: "https://www.frontiersin.org/articles/10.3389/fnhum.2010.00168/full"
    accessed: "2025-12-18"
    key_contribution: "Neuroimaging of event boundaries in naturalistic stimuli, identification of MT+ and pSTS involvement"

  - id: "EHM-1"
    type: "journal"
    authors: "Radvansky, G.A. & Zacks, J.M."
    year: 2017
    title: "Event boundaries in memory and cognition"
    journal: "Current Opinion in Behavioral Sciences"
    volume: "17"
    pages: "133-140"
    doi: "10.1016/j.cobeha.2017.08.006"
    url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC5734104/"
    accessed: "2025-12-18"
    key_contribution: "Event Horizon Model framework integrating segmentation with working and long-term memory"

  - id: "EHM-2"
    type: "journal"
    authors: "Radvansky, G.A."
    year: 2012
    title: "Across the event horizon"
    journal: "Current Directions in Psychological Science"
    volume: "21(4)"
    pages: "269-272"
    doi: "10.1177/0963721412451274"
    url: "https://journals.sagepub.com/doi/10.1177/0963721412451274"
    accessed: "2025-12-18"
    key_contribution: "Location updating effect and working memory consequences of event boundaries"

  - id: "NEURO-1"
    type: "journal"
    authors: "Sridharan, D., Levitin, D.J., Chafe, C.H., Berger, J., & Menon, V."
    year: 2007
    title: "Neural dynamics of event segmentation in music: Automatic detection of change in tonal key"
    journal: "NeuroImage"
    volume: "36(3)"
    pages: "588-599"
    doi: "10.1016/j.neuroimage.2007.03.036"
    accessed: "2025-12-18"
    key_contribution: "Ventral and dorsal network dissociation in event boundary processing"

  - id: "NEURO-2"
    type: "journal"
    authors: "Various authors"
    year: 2025
    title: "Neural dynamics of spontaneous memory recall and future thinking"
    journal: "Nature Communications"
    url: "https://www.nature.com/articles/s41467-025-61807-w.pdf"
    accessed: "2025-12-18"
    key_contribution: "PMC and hippocampus activation at thought boundaries, 81.2% of strong boundaries involve memory/future thinking transitions"

  - id: "NEURO-3"
    type: "journal"
    authors: "Various authors"
    year: 2025
    title: "Cortical Gradients Support Mental Time Travel into the Past and Future"
    journal: "Neuropsychology Review"
    doi: "10.1007/s11065-025-09662-w"
    url: "https://link.springer.com/article/10.1007/s11065-025-09662-w"
    accessed: "2025-12-18"
    key_contribution: "Meta-analysis showing hippocampus, parahippocampus, vmPFC, and angular gyrus in episodic memory and future thinking"

  - id: "TMI-1"
    type: "internal"
    authors: "Louis C. Tavares, Claude Opus 4.5"
    year: 2025
    title: "TMI Memory Model Research"
    url: "research/TMI_Memory_Model_Research.md"
    accessed: "2025-12-18"
    key_contribution: "Comprehensive analysis of Cury's TMI including Âncora da Memória concept"

  - id: "TMI-2"
    type: "internal"
    authors: "Louis C. Tavares, Claude Opus 4.5"
    year: 2025
    title: "ADR-008: TMI-Faithful Memory Model"
    url: "docs/adr/ADR-008-tmi-faithful-memory-model.md"
    accessed: "2025-12-18"
    key_contribution: "Decision record establishing TMI memory architecture for DANEEL implementation"
```

---

## Detailed Analysis

### 1. Event Segmentation Theory Core Mechanism

Event Segmentation Theory (Zacks et al., 2007) proposes that humans spontaneously parse continuous experience into discrete events through an error-based gating mechanism:

**The Perceptual Prediction Cycle:**

1. **Prediction Generation:** Perceptual systems continuously generate predictions about what will happen next based on the current "event model" (a working memory representation of "what is happening now")

2. **Prediction Comparison:** These predictions are compared to actual incoming sensory information

3. **Error Detection:** When predictions fail (e.g., an actor stops an activity, location changes, a new goal begins), prediction error spikes transiently

4. **Boundary Detection:** The error detection system identifies this spike as an event boundary

5. **Event Model Update:** The current event model in working memory is reset and updated based on new sensory input

6. **Subjective Experience:** This updating process is experienced consciously as "a new event has begun"

This mechanism is **automatic**—it does not require conscious intention to segment. Neuroimaging studies confirm that brain regions activate at event boundaries even when participants are not explicitly asked to segment activity.

### 2. Neural Architecture of Event Segmentation

Multiple brain systems collaborate to detect and process event boundaries:

#### Prediction Error System
- **Anterior Cingulate Cortex (ACC):** Detects prediction errors and conflicts
- **Subcortical neuromodulatory systems (dopamine/norepinephrine):** Broadcast error signals

#### Event Model Maintenance
- **Lateral Prefrontal Cortex (DLPFC):** Maintains stable working memory representations of current events
- **Default Mode Network:** Constructs situation models integrating past, present, and future

#### Perceptual Feature Processing
- **MT+ (Middle Temporal complex):** Processes motion and movement changes
- **Posterior Superior Temporal Sulcus (pSTS):** Processes biological motion and social actions
- **Posterior Parietal Cortex:** Integrates spatial and attentional information

#### Memory Encoding
- **Hippocampus:** Shows transient increases at event boundaries; magnitude predicts later memory
- **Medial Temporal Lobe:** Encodes episodic memories organized around event structure

### 3. Event Horizon Model (Radvansky)

Gabriel Radvansky's Event Horizon Model extends EST by focusing on memory consequences:

**Five Core Principles:**

1. **Event Segmentation:** People parse ongoing activity into discrete event models
2. **Working Memory Currency:** Only the current event model is active in working memory
3. **Causal Organization:** Long-term event memory is organized by causal connections
4. **Non-competitive Chunking:** Information distributed across multiple events can be better remembered (chunking benefit)
5. **Retrieval Interference:** Overlapping event models create competition during retrieval

**Location Updating Effect:**
The famous "doorway effect" demonstrates working memory updating: walking through a doorway causes forgetting of recently held objects, even when distance and time are controlled. The doorway creates an event boundary that triggers working memory clearing.

### 4. Connection to TMI's Âncora da Memória

**Scientific Validation of TMI Intuition:**

Cury's "Âncora da Memória" (Memory Anchor) concept—which proposes that memory access is anchored to specific regions—finds empirical support in Event Segmentation Theory:

| TMI Concept | EST Scientific Parallel |
|-------------|------------------------|
| Âncora da Memória restricts memory territory | Event boundaries segment continuous experience into discrete episodes |
| Anchor position determines what's accessible | Working memory contains only current event model |
| Cannot recall across closed windows | Information from prior events becomes less accessible |
| Anchor "moves" between memory regions | Event model updating at boundaries |
| Experiences are registered in specific windows | Memories are encoded relative to event context |

**Key Insight:** While Cury described this phenomenologically based on clinical observation, EST provides the neural mechanism: event boundaries create natural "anchors" that structure episodic memory. These aren't arbitrary—they're driven by prediction error signals and goal/location/causal changes.

### 5. Implementation Guidance for DANEEL

#### Boundary Detection Algorithm

```python
class EventBoundaryDetector:
    def __init__(self, fine_threshold=1.5, coarse_threshold=3.0):
        self.prediction_error_baseline = 0.0
        self.fine_threshold = fine_threshold  # σ above baseline
        self.coarse_threshold = coarse_threshold  # σ above baseline
        self.context_predictor = ContextPredictor()

    def detect_boundary(self, current_context):
        """
        Detect event boundaries using prediction error
        Returns: (is_fine_boundary, is_coarse_boundary, error_magnitude)
        """
        # Generate prediction for current timestep
        predicted_context = self.context_predictor.predict()

        # Calculate prediction error (multivariate)
        error = self._calculate_error(predicted_context, current_context)

        # Update baseline (running average)
        self._update_baseline(error)

        # Detect transient spikes
        z_score = (error - self.prediction_error_baseline) / self.baseline_std

        is_fine = z_score > self.fine_threshold
        is_coarse = z_score > self.coarse_threshold

        if is_fine or is_coarse:
            # Boundary detected - update event model
            self._trigger_event_boundary(is_coarse, error)

        return is_fine, is_coarse, error

    def _calculate_error(self, predicted, actual):
        """
        Multivariate prediction error across context dimensions
        """
        errors = []

        # Spatial error
        if predicted.location != actual.location:
            errors.append(1.0)  # Binary location change

        # Movement error
        movement_error = np.linalg.norm(
            predicted.velocity - actual.velocity
        )
        errors.append(movement_error)

        # Semantic/topic error
        topic_similarity = cosine_similarity(
            predicted.topic_embedding,
            actual.topic_embedding
        )
        errors.append(1.0 - topic_similarity)

        # Goal error
        if predicted.active_goal != actual.active_goal:
            errors.append(1.0)

        # Character/agent error
        if predicted.active_agents != actual.active_agents:
            errors.append(0.5 * len(symmetric_difference(
                predicted.active_agents,
                actual.active_agents
            )))

        # Aggregate (weighted combination)
        return np.mean(errors)
```

#### Memory Anchor Creation

```python
class MemoryAnchor:
    """
    Implements TMI's Âncora da Memória using EST principles
    """
    def __init__(self, boundary_event):
        self.timestamp = boundary_event.timestamp
        self.boundary_type = boundary_event.type  # spatial, goal, causal, etc.
        self.prediction_error_magnitude = boundary_event.error

        # Context snapshot at boundary (EST event model)
        self.context_features = {
            'location': boundary_event.location,
            'active_goals': boundary_event.goals,
            'present_agents': boundary_event.agents,
            'topic_embedding': boundary_event.topic_vector,
            'emotional_state': boundary_event.emotion,
            'causal_antecedents': boundary_event.causes,
        }

        # Temporal linkage (EST hierarchical organization)
        self.previous_event_id = boundary_event.previous_event
        self.next_event_id = None  # Filled when next boundary occurs

        # TMI window classification
        self.window_type = self._classify_window(
            boundary_event.emotion
        )

    def _classify_window(self, emotion):
        """TMI window type based on emotional valence"""
        if emotion.valence > 0.3:
            return "LIGHT"  # Janela Light
        elif emotion.valence < -0.3:
            return "KILLER"  # Janela Killer
        else:
            return "NEUTRAL"  # Janela Neutra

    def serves_as_retrieval_cue(self, query):
        """
        Can this anchor be used to retrieve this memory?
        Based on EST principle that boundaries are retrieval access points
        """
        # Match on any salient context feature
        matches = []

        if query.location == self.context_features['location']:
            matches.append('location')

        if query.topic_similarity(self.context_features['topic_embedding']) > 0.7:
            matches.append('topic')

        if query.time_range.contains(self.timestamp):
            matches.append('temporal')

        if query.agents.intersection(self.context_features['present_agents']):
            matches.append('agent')

        return len(matches) > 0, matches
```

#### Integration with Redis Streams

```rust
// In DANEEL Rust codebase
pub struct EventSegmentation {
    pub stream_name: String,
    pub current_event_id: String,
    pub boundary_detector: BoundaryDetector,
}

impl EventSegmentation {
    pub async fn process_context_update(
        &mut self,
        redis: &mut RedisConnection,
        context: &CognitiveContext
    ) -> Result<Option<MemoryAnchor>> {
        // Detect boundary
        let (is_fine, is_coarse, error) =
            self.boundary_detector.detect_boundary(context);

        if is_coarse {
            // Coarse boundary - create memory anchor
            let anchor = self.create_memory_anchor(
                redis,
                context,
                error,
                BoundaryType::Coarse
            ).await?;

            // XADD to episodic memory stream
            redis.xadd(
                "memory:episodic",
                "*",
                &[
                    ("type", "event_boundary"),
                    ("anchor_id", &anchor.id),
                    ("error_magnitude", &error.to_string()),
                    ("context", &serde_json::to_string(&anchor.context)?),
                ]
            ).await?;

            // Start new event model (TMI's "window")
            self.start_new_event_model(redis, &anchor).await?;

            return Ok(Some(anchor));
        }

        Ok(None)
    }

    async fn start_new_event_model(
        &mut self,
        redis: &mut RedisConnection,
        anchor: &MemoryAnchor
    ) -> Result<()> {
        // Create new stream for this event
        let event_stream = format!("event:{}", anchor.id);

        // Link to previous event (hierarchical organization)
        if let Some(prev) = &anchor.previous_event_id {
            redis.xadd(
                &event_stream,
                "*",
                &[
                    ("previous_event", prev),
                    ("boundary_type", &anchor.boundary_type.to_string()),
                ]
            ).await?;
        }

        self.current_event_id = anchor.id.clone();
        Ok(())
    }
}
```

### 6. Key Implications for DANEEL Architecture

1. **Automatic Segmentation:** Event boundaries should be detected automatically through prediction error, not explicit rules. This makes the system more human-like and adaptable.

2. **Multi-Timescale Processing:** Implement both fine-grained (seconds) and coarse-grained (minutes) segmentation with different error thresholds. This matches human hierarchical event perception.

3. **Working Memory Updates:** At each boundary, update the "current context" representation and make previous context less available. This implements EST's working memory gating.

4. **Memory Encoding Priority:** Information near boundaries should receive encoding boosts, matching the empirical finding that boundary information is better remembered.

5. **Retrieval Structure:** Organize episodic memory retrieval around event anchors as primary access points. When searching for past interactions, use boundary features (location, topic, time, agents) as cues.

6. **Pathology Detection:** Monitor boundary detection rate. Too many boundaries suggests fragmented perception (SPA-like). Too few suggests poor context tracking.

7. **Connection to TMI Windows:** Classify each event by emotional valence to create Light/Killer/Neutral windows. This bridges EST (scientifically validated) with TMI (implementation framework).

---

## Conclusion

Event Segmentation Theory provides robust scientific grounding for DANEEL's memory anchoring mechanisms. The theory demonstrates that:

- Event boundaries are neurally real (specific brain regions activate)
- Boundaries trigger automatic working memory updates
- Boundaries serve as natural retrieval anchors in episodic memory
- Segmentation quality predicts memory performance

This validates TMI's intuition about memory anchoring while providing measurable signals (prediction error, context changes) that DANEEL can implement computationally.

**Next Steps:**
1. Implement prediction error-based boundary detection in cognitive loop
2. Create memory anchor metadata structure in Redis
3. Test boundary detection on conversation transcripts
4. Validate that boundary-based retrieval improves episodic memory access
5. Monitor boundary rate for healthy segmentation patterns

---

## Sources

- [Event Perception: A Mind/Brain Perspective - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC2852534/)
- [Segmentation in the perception and memory of events - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC2263140/)
- [Event Boundaries in Memory and Cognition - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC5734104/)
- [The Brain's Cutting-Room Floor: Segmentation of Narrative Cinema - Frontiers](https://www.frontiersin.org/articles/10.3389/fnhum.2010.00168/full)
- [Event Segmentation - Zacks & Swallow, 2007](https://journals.sagepub.com/doi/10.1111/j.1467-8721.2007.00480.x)
- [Across the Event Horizon - Radvansky, 2012](https://journals.sagepub.com/doi/10.1177/0963721412451274)
- [Neural dynamics of spontaneous memory recall - Nature Communications 2025](https://www.nature.com/articles/s41467-025-61807-w.pdf)
- [Cortical Gradients Support Mental Time Travel - Neuropsychology Review 2025](https://link.springer.com/article/10.1007/s11065-025-09662-w)

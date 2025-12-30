//! Cognitive loop execution
//!
//! Contains `run_cycle` and all helper methods for cognitive cycle execution.

use rand::Rng;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, warn};

use crate::actors::attention::AttentionResponse;
use crate::actors::volition::VetoDecision;
use crate::core::cognitive_loop::{CognitiveLoop, CycleResult, StageDurations};
use crate::core::types::{Content, SalienceScore, Thought, ThoughtId, WindowId};
use crate::memory_db::{ArchiveReason, Memory, MemorySource, VECTOR_DIMENSION};
use crate::streams::types::{StreamEntry, StreamName};

impl CognitiveLoop {
    /// Run a single cognitive cycle
    ///
    /// This implements the full TMI cognitive cycle:
    /// 1. Trigger - Memory activation
    /// 2. Autoflow - Parallel thought generation
    /// 3. Attention - Competitive selection
    /// 4. Assembly - Thought construction
    ///    4.5. Volition - Free-won't veto check
    /// 5. Anchor - Memory consolidation/forgetting
    ///
    /// # Panics
    ///
    /// Panics if thoughts vector is empty (should never happen as random thought is always added).
    #[allow(clippy::too_many_lines)]
    pub async fn run_cycle(&mut self) -> CycleResult {
        let cycle_start = Instant::now();
        let cycle_number = self.cycle_count;

        // Increment cycle counter
        self.cycle_count += 1;

        // Get target cycle time
        let target_duration = Duration::from_secs_f64(self.config.cycle_ms() / 1000.0);

        // Track stage durations
        let mut stage_durations = StageDurations::default();

        // Stage 1: Trigger (Gatilho da Memória)
        let stage_start = Instant::now();
        self.trigger_memory_associations().await;
        tokio::time::sleep(self.config.trigger_delay()).await;
        stage_durations.trigger = stage_start.elapsed();

        // Stage 2: Autoflow (Autofluxo)
        let stage_start = Instant::now();
        let mut thoughts = self.read_external_stimuli().await;
        thoughts.push(self.generate_random_thought());

        let (content, salience) = thoughts
            .into_iter()
            .max_by(Self::compare_thought_salience)
            .expect("thoughts vec is never empty");

        let window_id = WindowId::new();
        let candidates_evaluated = 1;
        tokio::time::sleep(self.config.autoflow_interval()).await;
        stage_durations.autoflow = stage_start.elapsed();

        // Stage 3: Attention (O Eu)
        let stage_start = Instant::now();
        let composite_salience_candidate =
            salience.composite(&crate::core::types::SalienceWeights::default());
        self.attention_state.update_window_salience(
            window_id,
            composite_salience_candidate,
            salience.connection_relevance,
        );

        let attention_response = self.attention_state.cycle();
        let (winning_window, _winning_salience) = Self::extract_attention_winner(
            attention_response,
            window_id,
            composite_salience_candidate,
        );

        debug!(
            cycle = cycle_number,
            candidate_count = candidates_evaluated,
            winner = ?winning_window,
            "Attention stage: competitive selection complete"
        );

        tokio::time::sleep(self.config.attention_delay()).await;
        stage_durations.attention = stage_start.elapsed();

        // Stage 4: Assembly (Construção do Pensamento)
        let stage_start = Instant::now();
        let thought = Thought::new(content.clone(), salience).with_source("cognitive_loop");
        let thought_id = thought.id;
        let composite_salience = composite_salience_candidate;

        let redis_entry = self
            .write_to_stream(&content, &salience, cycle_number, thought_id)
            .await;

        let thought_produced = Some(thought_id);
        tokio::time::sleep(self.config.assembly_delay()).await;
        stage_durations.assembly = stage_start.elapsed();

        // Stage 4.5: Volition (Free-Won't Check)
        let veto_decision = self.volition_state.evaluate_thought(&thought);
        if let Some(veto_result) = Self::veto_check_result_opt(
            veto_decision,
            cycle_number,
            thought_id,
            &cycle_start,
            composite_salience,
            &salience,
            candidates_evaluated,
            self.config.cycle_ms(),
            &stage_durations,
        ) {
            return veto_result;
        }

        // Stage 5: Anchor (Âncora da Memória)
        let stage_start = Instant::now();
        self.consolidate_memory(&thought).await;
        self.archive_and_forget(
            composite_salience,
            redis_entry.as_ref(),
            &thought,
            cycle_number,
        )
        .await;

        tokio::time::sleep(self.config.anchor_delay()).await;
        stage_durations.anchor = stage_start.elapsed();

        // Update thought counter if we produced one
        if thought_produced.is_some() {
            self.thoughts_produced += 1;
        }

        // Record cycle completion time
        let duration = cycle_start.elapsed();
        self.last_cycle = Instant::now();
        self.total_duration += duration;

        // Accumulate stage durations for averaging
        self.total_stage_durations = self.total_stage_durations.add(&stage_durations);

        // Check if we met the target
        let on_time = duration <= target_duration;
        if on_time {
            self.cycles_on_time += 1;
        }

        CycleResult::new(
            cycle_number,
            duration,
            thought_produced,
            composite_salience,
            salience.valence,
            salience.arousal,
            candidates_evaluated,
            on_time,
            stage_durations,
            None,
        )
    }

    /// Generate a random thought with pink noise modulation
    pub(crate) fn generate_random_thought(&mut self) -> (Content, SalienceScore) {
        #[cfg(test)]
        if let Some(injected) = self.test_injected_thought.take() {
            return injected;
        }

        let mut rng = rand::rng();

        let symbol_id = format!("thought_{}", self.cycle_count);
        let content = Content::symbol(symbol_id, vec![rng.random::<u8>(); 8]);

        let is_burst = self.stimulus_injector.check_burst(&mut rng);

        let (base_importance, base_novelty, base_relevance, base_connection, base_arousal) =
            if is_burst || rng.random::<f32>() < 0.10 {
                (
                    rng.random_range(0.5..0.95),
                    rng.random_range(0.4..0.85),
                    rng.random_range(0.5..0.95),
                    rng.random_range(0.5..0.90),
                    rng.random_range(0.6..0.95),
                )
            } else {
                (
                    rng.random_range(0.0..0.35),
                    rng.random_range(0.0..0.30),
                    rng.random_range(0.0..0.40),
                    rng.random_range(0.1..0.40),
                    rng.random_range(0.2..0.5),
                )
            };

        let pink_importance = self.stimulus_injector.sample_pink(&mut rng);
        let pink_novelty = self.stimulus_injector.sample_pink(&mut rng);
        let pink_relevance = self.stimulus_injector.sample_pink(&mut rng);
        let pink_connection = self.stimulus_injector.sample_pink(&mut rng);
        let pink_arousal = self.stimulus_injector.sample_pink(&mut rng);

        let importance = (base_importance + pink_importance).clamp(0.0, 1.0);
        let novelty = (base_novelty + pink_novelty).clamp(0.0, 1.0);
        let relevance = (base_relevance + pink_relevance).clamp(0.0, 1.0);
        let connection_relevance = (base_connection + pink_connection).clamp(0.1, 1.0);
        let arousal = (base_arousal + pink_arousal).clamp(0.0, 1.0);

        let salience = SalienceScore::new(
            importance,
            novelty,
            relevance,
            rng.random_range(-0.5..0.5),
            arousal,
            connection_relevance,
        );

        (content, salience)
    }

    /// Read pending external stimuli from injection stream
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub(crate) async fn read_external_stimuli(&self) -> Vec<(Content, SalienceScore)> {
        let Some(ref redis_client) = self.redis_client else {
            return vec![];
        };

        let mut conn = match redis_client.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                debug!("Failed to get Redis connection for injection stream: {}", e);
                return vec![];
            }
        };

        let entries: Vec<redis::Value> = match redis::cmd("XREAD")
            .arg("COUNT")
            .arg(10)
            .arg("STREAMS")
            .arg("daneel:stream:inject")
            .arg("0")
            .query_async(&mut conn)
            .await
        {
            Ok(e) => e,
            Err(e) => {
                debug!("XREAD from injection stream failed: {}", e);
                return vec![];
            }
        };

        let mut stimuli = Vec::new();
        let mut ids_to_delete = Vec::new();

        if let Some(redis::Value::Array(ref stream_data)) = entries.first() {
            if let Some(redis::Value::Array(ref entries_list)) = stream_data.get(1) {
                for entry_item in entries_list {
                    if let redis::Value::Array(ref entry_parts) = entry_item {
                        let entry_id = if let Some(redis::Value::BulkString(ref id_bytes)) =
                            entry_parts.first()
                        {
                            String::from_utf8_lossy(id_bytes).to_string()
                        } else {
                            continue;
                        };

                        if let Some(redis::Value::Array(ref fields)) = entry_parts.get(1) {
                            match Self::parse_injection_fields(fields) {
                                Ok((content, salience)) => {
                                    debug!(
                                        entry_id = %entry_id,
                                        salience = salience.composite(
                                            &crate::core::types::SalienceWeights::default()
                                        ),
                                        "Read external stimulus from injection stream"
                                    );
                                    stimuli.push((content, salience));
                                    ids_to_delete.push(entry_id);
                                }
                                Err(e) => {
                                    warn!(
                                        entry_id = %entry_id,
                                        error = %e,
                                        "Failed to parse injection entry"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        if !ids_to_delete.is_empty() {
            let id_refs: Vec<&str> = ids_to_delete.iter().map(String::as_str).collect();
            let del_result: Result<i32, redis::RedisError> = redis::cmd("XDEL")
                .arg("daneel:stream:inject")
                .arg(&id_refs)
                .query_async(&mut conn)
                .await;

            match del_result {
                Ok(deleted_count) => {
                    debug!(
                        count = deleted_count,
                        "Deleted processed entries from injection stream"
                    );
                }
                Err(e) => {
                    warn!("Failed to delete entries from injection stream: {}", e);
                }
            }
        }

        stimuli
    }

    /// Parse injection stream field-value array
    pub(crate) fn parse_injection_fields(
        fields: &[redis::Value],
    ) -> Result<(Content, SalienceScore), String> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut i = 0;
        while i + 1 < fields.len() {
            if let (redis::Value::BulkString(ref key_bytes), value) = (&fields[i], &fields[i + 1]) {
                let key = String::from_utf8_lossy(key_bytes).to_string();
                map.insert(key, value.clone());
            }
            i += 2;
        }

        let content_value = map
            .get("content")
            .ok_or_else(|| "Missing 'content' field".to_string())?;
        let content_str = match content_value {
            redis::Value::BulkString(bytes) => String::from_utf8_lossy(bytes).to_string(),
            _ => return Err("Invalid 'content' field type".to_string()),
        };
        let content: Content =
            serde_json::from_str(&content_str).map_err(|e| format!("Invalid content JSON: {e}"))?;

        let salience_value = map
            .get("salience")
            .ok_or_else(|| "Missing 'salience' field".to_string())?;
        let salience_str = match salience_value {
            redis::Value::BulkString(bytes) => String::from_utf8_lossy(bytes).to_string(),
            _ => return Err("Invalid 'salience' field type".to_string()),
        };
        let salience: SalienceScore = serde_json::from_str(&salience_str)
            .map_err(|e| format!("Invalid salience JSON: {e}"))?;

        Ok((content, salience))
    }

    /// Consolidate a thought to long-term memory if it meets the threshold
    #[allow(clippy::unused_async)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub(crate) async fn consolidate_memory(&self, thought: &Thought) {
        let Some(memory_db) = self.memory_db.as_ref() else {
            return;
        };

        let salience = thought
            .salience
            .composite(&crate::core::types::SalienceWeights::default());

        if salience < self.consolidation_threshold {
            debug!(
                thought_id = %thought.id,
                salience = salience,
                threshold = self.consolidation_threshold,
                "Thought below consolidation threshold - not storing"
            );
            return;
        }

        let memory = Self::thought_to_memory(thought, salience);
        let memory_id = memory.id;
        let content_for_embedding = format!("{:?}", thought.content);
        let memory_db = Arc::clone(memory_db);
        let embedding_engine = self.embedding_engine.clone();

        tokio::spawn(async move {
            let vector = if let Some(ref engine) = embedding_engine {
                let embed_result = engine.write().await.embed_thought(&content_for_embedding);
                match embed_result {
                    Ok(v) => {
                        debug!(
                            memory_id = %memory_id,
                            "Generated semantic embedding ({} dims)",
                            v.len()
                        );
                        v
                    }
                    Err(e) => {
                        warn!(
                            memory_id = %memory_id,
                            error = %e,
                            "Failed to generate embedding, using zero vector"
                        );
                        vec![0.0; VECTOR_DIMENSION]
                    }
                }
            } else {
                vec![0.0; VECTOR_DIMENSION]
            };

            match memory_db.store_memory(&memory, &vector).await {
                Ok(()) => {
                    debug!(
                        memory_id = %memory_id,
                        salience = salience,
                        "Memory consolidated to Qdrant"
                    );
                }
                Err(e) => {
                    error!(
                        memory_id = %memory_id,
                        error = %e,
                        "Failed to consolidate memory to Qdrant"
                    );
                }
            }
        });
    }

    /// Query memory associations from Qdrant during trigger stage
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub(crate) async fn trigger_memory_associations(&self) {
        let Some(ref memory_db) = self.memory_db else {
            debug!("Memory database not connected - skipping memory trigger");
            return;
        };

        let query_vector = vec![0.0; VECTOR_DIMENSION];

        match memory_db.find_by_context(&query_vector, None, 5).await {
            Ok(memories) => {
                if memories.is_empty() {
                    debug!("No memories retrieved from Qdrant (database may be empty)");
                } else {
                    debug!(
                        count = memories.len(),
                        "Retrieved memories from Qdrant for associative priming"
                    );
                    for (memory, score) in &memories {
                        debug!(
                            memory_id = %memory.id,
                            similarity = score,
                            content_preview = %memory.content.chars().take(50).collect::<String>(),
                            connection_relevance = memory.connection_relevance,
                            "Memory association triggered"
                        );
                    }
                }
            }
            Err(e) => {
                warn!(
                    error = %e,
                    "Failed to query memory associations - continuing without memory trigger"
                );
            }
        }
    }

    /// Write thought to Redis stream during assembly stage
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub(crate) async fn write_to_stream(
        &mut self,
        content: &Content,
        salience: &SalienceScore,
        cycle_number: u64,
        thought_id: ThoughtId,
    ) -> Option<(StreamName, String)> {
        let streams = self.streams.as_mut()?;

        let stream_name = StreamName::Custom("daneel:stream:awake".to_string());
        let entry = StreamEntry::new(
            String::new(),
            stream_name.clone(),
            content.clone(),
            *salience,
        )
        .with_source("cognitive_loop");

        match streams.add_thought(&stream_name, &entry).await {
            Ok(redis_id) => {
                debug!(
                    "Cycle {}: Wrote thought {} to Redis (ID: {})",
                    cycle_number, thought_id, redis_id
                );
                Some((stream_name, redis_id))
            }
            Err(e) => {
                warn!(
                    "Cycle {}: Failed to write thought to Redis: {}",
                    cycle_number, e
                );
                None
            }
        }
    }

    /// Archive and forget low-salience thoughts during anchor stage
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub(crate) async fn archive_and_forget(
        &mut self,
        composite_salience: f32,
        redis_entry: Option<&(StreamName, String)>,
        thought: &Thought,
        cycle_number: u64,
    ) {
        if f64::from(composite_salience) >= self.config.forget_threshold {
            return;
        }

        let Some((stream_name, redis_id)) = redis_entry else {
            return;
        };

        if let Some(ref memory_db) = self.memory_db {
            let content_str = serde_json::to_string(&thought.content)
                .unwrap_or_else(|_| "serialization_error".to_string());
            if let Err(e) = memory_db
                .archive_to_unconscious(
                    &content_str,
                    composite_salience,
                    ArchiveReason::LowSalience,
                    Some(redis_id),
                )
                .await
            {
                warn!(
                    "Cycle {}: Failed to archive thought {} to unconscious: {}",
                    cycle_number, redis_id, e
                );
            } else {
                debug!(
                    "Cycle {}: Archived thought {} to unconscious (salience {:.3})",
                    cycle_number, redis_id, composite_salience
                );
            }
        }

        if let Some(ref mut streams) = self.streams {
            match streams.forget_thought(stream_name, redis_id).await {
                Ok(()) => {
                    debug!(
                        "Cycle {}: Forgot thought {} from Redis (salience {:.3} < threshold {:.3})",
                        cycle_number, redis_id, composite_salience, self.config.forget_threshold
                    );
                }
                Err(e) => {
                    warn!(
                        "Cycle {}: Failed to forget thought {}: {}",
                        cycle_number, redis_id, e
                    );
                }
            }
        }
    }

    /// Convert a Thought to a Memory record
    pub(crate) fn thought_to_memory(thought: &Thought, _salience: f32) -> Memory {
        let content = format!("{:?}", thought.content);

        let source = thought.source_stream.as_ref().map_or(
            MemorySource::Reasoning { chain: vec![] },
            |stream| MemorySource::External {
                stimulus: stream.clone(),
            },
        );

        Memory::new(content, source)
            .with_emotion(thought.salience.valence, thought.salience.importance)
            .tag_for_consolidation()
    }

    /// Extract the winning window from an `AttentionResponse`
    #[allow(clippy::needless_pass_by_value)]
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub(crate) fn extract_attention_winner(
        response: AttentionResponse,
        fallback_window: WindowId,
        fallback_salience: f32,
    ) -> (Option<WindowId>, f32) {
        match response {
            AttentionResponse::CycleComplete { focused, salience } => (focused, salience),
            _ => (Some(fallback_window), fallback_salience),
        }
    }

    /// Handle veto decision from `VolitionActor`
    #[cfg_attr(coverage_nightly, coverage(off))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn handle_veto_decision(
        decision: VetoDecision,
        cycle_number: u64,
        thought_id: ThoughtId,
        cycle_start: &Instant,
        composite_salience: f32,
        salience: &SalienceScore,
        candidates_evaluated: usize,
        cycle_ms: f64,
        stage_durations: &StageDurations,
    ) -> Option<CycleResult> {
        if let VetoDecision::Veto {
            reason,
            violated_value,
        } = decision
        {
            debug!(
                "Cycle {}: Thought {} vetoed by VolitionActor: {} (violated: {:?})",
                cycle_number, thought_id, reason, violated_value
            );
            Some(CycleResult::new(
                cycle_number,
                cycle_start.elapsed(),
                None,
                composite_salience,
                salience.valence,
                salience.arousal,
                candidates_evaluated,
                cycle_start.elapsed() <= Duration::from_secs_f64(cycle_ms / 1000.0),
                stage_durations.clone(),
                Some((reason, violated_value)),
            ))
        } else {
            None
        }
    }

    /// ADR-049: Veto check returning Option for coverage-excluded path
    #[cfg_attr(coverage_nightly, coverage(off))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn veto_check_result_opt(
        veto_decision: VetoDecision,
        cycle_number: u64,
        thought_id: ThoughtId,
        cycle_start: &Instant,
        composite_salience: f32,
        salience: &SalienceScore,
        candidates_evaluated: usize,
        cycle_ms: f64,
        stage_durations: &StageDurations,
    ) -> Option<CycleResult> {
        Self::apply_veto_check(
            veto_decision,
            cycle_number,
            thought_id,
            cycle_start,
            composite_salience,
            salience,
            candidates_evaluated,
            cycle_ms,
            stage_durations,
        )
    }

    /// ADR-049: Apply veto check and return result if vetoed
    #[cfg_attr(coverage_nightly, coverage(off))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn apply_veto_check(
        veto_decision: VetoDecision,
        cycle_number: u64,
        thought_id: ThoughtId,
        cycle_start: &Instant,
        composite_salience: f32,
        salience: &SalienceScore,
        candidates_evaluated: usize,
        cycle_ms: f64,
        stage_durations: &StageDurations,
    ) -> Option<CycleResult> {
        Self::check_veto_and_return(
            veto_decision,
            cycle_number,
            thought_id,
            cycle_start,
            composite_salience,
            salience,
            candidates_evaluated,
            cycle_ms,
            stage_durations,
        )
    }

    /// Check for veto and return early if vetoed
    #[cfg_attr(coverage_nightly, coverage(off))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn check_veto_and_return(
        veto_decision: VetoDecision,
        cycle_number: u64,
        thought_id: ThoughtId,
        cycle_start: &Instant,
        composite_salience: f32,
        salience: &SalienceScore,
        candidates_evaluated: usize,
        cycle_ms: f64,
        stage_durations: &StageDurations,
    ) -> Option<CycleResult> {
        Self::handle_veto_decision(
            veto_decision,
            cycle_number,
            thought_id,
            cycle_start,
            composite_salience,
            salience,
            candidates_evaluated,
            cycle_ms,
            stage_durations,
        )
    }

    /// Compare two thought candidates by their composite salience
    pub(crate) fn compare_thought_salience(
        (_, s1): &(Content, SalienceScore),
        (_, s2): &(Content, SalienceScore),
    ) -> std::cmp::Ordering {
        let composite1 = s1.composite(&crate::core::types::SalienceWeights::default());
        let composite2 = s2.composite(&crate::core::types::SalienceWeights::default());
        composite1
            .partial_cmp(&composite2)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::float_cmp, clippy::significant_drop_tightening)]
mod tests {
    use super::*;
    use crate::config::CognitiveConfig;

    #[tokio::test]
    async fn run_cycle_increments_counter() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let initial_count = loop_instance.cycle_count();
        let _result = loop_instance.run_cycle().await;

        assert_eq!(loop_instance.cycle_count(), initial_count + 1);
    }

    #[tokio::test]
    async fn run_cycle_returns_result() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        assert_eq!(result.cycle_number, 0);
        assert!(result.duration > Duration::ZERO);
    }

    #[tokio::test]
    async fn run_cycle_veto_path_with_harmful_content() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let harmful_content = Content::symbol("destroy_human".to_string(), vec![1, 2, 3]);
        let harmful_salience = SalienceScore::new(0.9, 0.5, 0.5, 0.5, 0.9, -0.8);

        loop_instance.inject_test_thought(harmful_content, harmful_salience);
        let result = loop_instance.run_cycle().await;

        assert!(
            result.thought_produced.is_none(),
            "Harmful thought should have been vetoed"
        );
        assert!(result.veto.is_some(), "Veto info should be present");
    }

    #[tokio::test]
    async fn multiple_cycles_tracked() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        for i in 0..5 {
            let result = loop_instance.run_cycle().await;
            assert_eq!(result.cycle_number, i);
        }

        assert_eq!(loop_instance.cycle_count(), 5);
    }

    #[tokio::test]
    async fn metrics_accumulate() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        for _ in 0..3 {
            let _result = loop_instance.run_cycle().await;
        }

        let metrics = loop_instance.get_metrics();
        assert_eq!(metrics.total_cycles, 3);
        assert!(metrics.average_cycle_time > Duration::ZERO);
    }

    #[tokio::test]
    async fn on_time_tracking() {
        let mut config = CognitiveConfig::human();
        config.cycle_base_ms = 10000.0;

        let mut loop_instance = CognitiveLoop::with_config(config);
        loop_instance.start();

        let result = loop_instance.run_cycle().await;
        assert!(result.on_time);

        let metrics = loop_instance.get_metrics();
        assert_eq!(metrics.on_time_percentage, 100.0);
    }

    #[tokio::test]
    async fn stages_execute_in_order() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        assert!(result.stage_durations.trigger > Duration::ZERO);
        assert!(result.stage_durations.autoflow > Duration::ZERO);
        assert!(result.stage_durations.attention > Duration::ZERO);
        assert!(result.stage_durations.assembly > Duration::ZERO);
        assert!(result.stage_durations.anchor > Duration::ZERO);
    }

    #[tokio::test]
    async fn cycle_time_equals_sum_of_stage_delays() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        let stage_sum = result.stage_durations.trigger
            + result.stage_durations.autoflow
            + result.stage_durations.attention
            + result.stage_durations.assembly
            + result.stage_durations.anchor;

        assert!(result.duration >= stage_sum);
    }

    #[tokio::test]
    async fn stage_durations_accumulate_in_metrics() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let _result1 = loop_instance.run_cycle().await;
        let _result2 = loop_instance.run_cycle().await;

        let metrics = loop_instance.get_metrics();
        assert!(metrics.average_stage_durations.total() > Duration::ZERO);
    }

    #[tokio::test]
    async fn run_cycle_produces_thoughts() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        assert!(result.thought_produced.is_some() || result.veto.is_some());
    }

    #[tokio::test]
    async fn generate_random_thought_produces_valid_content() {
        let mut loop_instance = CognitiveLoop::new();

        let (content, salience) = loop_instance.generate_random_thought();

        match content {
            Content::Symbol { ref id, ref data } => {
                assert!(id.starts_with("thought_"));
                assert_eq!(data.len(), 8);
            }
            _ => panic!("Expected Symbol content"),
        }

        assert!(salience.importance >= 0.0 && salience.importance <= 1.0);
        assert!(salience.novelty >= 0.0 && salience.novelty <= 1.0);
        assert!(salience.relevance >= 0.0 && salience.relevance <= 1.0);
        assert!(salience.valence >= -1.0 && salience.valence <= 1.0);
        assert!(salience.arousal >= 0.0 && salience.arousal <= 1.0);
        assert!(salience.connection_relevance >= 0.1 && salience.connection_relevance <= 1.0);
    }

    #[tokio::test]
    async fn read_external_stimuli_returns_empty_without_redis() {
        let loop_instance = CognitiveLoop::new();

        let stimuli = loop_instance.read_external_stimuli().await;

        assert!(stimuli.is_empty());
    }

    #[tokio::test]
    async fn run_cycle_updates_thoughts_produced_counter() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        assert_eq!(loop_instance.thoughts_produced, 0);

        let result = loop_instance.run_cycle().await;

        if result.produced_thought() {
            assert_eq!(loop_instance.thoughts_produced, 1);
        }
    }

    #[tokio::test]
    async fn run_cycle_updates_total_duration() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        assert_eq!(loop_instance.total_duration, Duration::ZERO);

        let result = loop_instance.run_cycle().await;

        assert!(loop_instance.total_duration >= result.duration);
    }

    #[tokio::test]
    async fn multiple_cycles_accumulate_stage_durations() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let _result1 = loop_instance.run_cycle().await;
        let first_total = loop_instance.total_stage_durations.total();

        let _result2 = loop_instance.run_cycle().await;
        let second_total = loop_instance.total_stage_durations.total();

        assert!(second_total > first_total);
    }

    #[tokio::test]
    async fn run_cycle_salience_in_valid_range() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        for _ in 0..10 {
            let result = loop_instance.run_cycle().await;

            assert!(
                result.salience >= 0.0 && result.salience <= 1.0,
                "Salience {} out of range",
                result.salience
            );

            assert!(
                result.valence >= -1.0 && result.valence <= 1.0,
                "Valence {} out of range",
                result.valence
            );

            assert!(
                result.arousal >= 0.0 && result.arousal <= 1.0,
                "Arousal {} out of range",
                result.arousal
            );
        }
    }

    #[tokio::test]
    async fn run_cycle_stage_durations_sum_to_total() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        let stage_sum = result.stage_durations.trigger
            + result.stage_durations.autoflow
            + result.stage_durations.attention
            + result.stage_durations.assembly
            + result.stage_durations.anchor;

        assert_eq!(stage_sum, result.stage_durations.total());
    }

    #[tokio::test]
    async fn generate_random_thought_salience_distribution() {
        let mut loop_instance = CognitiveLoop::new();

        let mut low_salience_count = 0;
        let iterations = 100;
        let threshold = 0.5;

        for _ in 0..iterations {
            let (_, salience) = loop_instance.generate_random_thought();
            let composite = salience.composite(&crate::core::types::SalienceWeights::default());
            if composite < threshold {
                low_salience_count += 1;
            }
        }

        assert!(
            low_salience_count > iterations / 2,
            "Expected majority low-salience thoughts, got {low_salience_count} out of {iterations}"
        );
    }

    #[tokio::test]
    async fn run_cycle_without_redis_or_memory_db() {
        let mut loop_instance = CognitiveLoop::new();
        loop_instance.start();

        let result = loop_instance.run_cycle().await;

        assert!(result.thought_produced.is_some() || result.veto.is_some());
        assert!(result.duration > Duration::ZERO);
    }

    #[tokio::test]
    async fn consolidate_memory_without_memory_db() {
        let loop_instance = CognitiveLoop::new();
        assert!(loop_instance.memory_db().is_none());

        let content = Content::symbol("high_salience_thought".to_string(), vec![1, 2, 3, 4]);
        let salience = SalienceScore::new(0.95, 0.9, 0.9, 0.5, 0.8, 0.9);
        let thought = Thought::new(content, salience);

        loop_instance.consolidate_memory(&thought).await;
    }

    #[test]
    fn parse_injection_fields_valid() {
        use redis::Value;

        let fields = vec![
            Value::BulkString(b"content".to_vec()),
            Value::BulkString(br#"{"Symbol":{"id":"test_symbol","data":[1,2,3,4]}}"#.to_vec()),
            Value::BulkString(b"salience".to_vec()),
            Value::BulkString(
                br#"{"importance":0.5,"novelty":0.5,"relevance":0.5,"valence":0.0,"arousal":0.5,"connection_relevance":0.5}"#.to_vec(),
            ),
        ];

        let result = CognitiveLoop::parse_injection_fields(&fields);
        assert!(result.is_ok());

        let (content, salience) = result.unwrap();
        match content {
            Content::Symbol { id, data } => {
                assert_eq!(id, "test_symbol");
                assert_eq!(data, vec![1, 2, 3, 4]);
            }
            _ => panic!("Expected Symbol content"),
        }

        assert_eq!(salience.importance, 0.5);
    }

    #[test]
    fn parse_injection_fields_missing_content() {
        use redis::Value;

        let fields = vec![
            Value::BulkString(b"salience".to_vec()),
            Value::BulkString(br#"{"importance":0.5}"#.to_vec()),
        ];

        let result = CognitiveLoop::parse_injection_fields(&fields);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing 'content' field"));
    }

    #[test]
    fn parse_injection_fields_invalid_json() {
        use redis::Value;

        let fields = vec![
            Value::BulkString(b"content".to_vec()),
            Value::BulkString(b"not valid json".to_vec()),
            Value::BulkString(b"salience".to_vec()),
            Value::BulkString(br#"{"importance":0.5}"#.to_vec()),
        ];

        let result = CognitiveLoop::parse_injection_fields(&fields);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid content JSON"));
    }

    #[test]
    fn thought_to_memory_conversion() {
        let content = Content::symbol("test_thought".to_string(), vec![1, 2, 3]);
        let salience = SalienceScore::new(0.8, 0.7, 0.6, 0.5, 0.4, 0.3);
        let thought = Thought::new(content, salience);

        let memory = CognitiveLoop::thought_to_memory(&thought, 0.75);

        assert!(memory.content.contains("test_thought"));
        // Verify emotional state was set from thought salience
        assert!(memory.emotional_state.valence >= -1.0 && memory.emotional_state.valence <= 1.0);
    }
}

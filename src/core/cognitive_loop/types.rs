//! Cognitive loop types
//!
//! Core types for the cognitive cycle: stages, states, and durations.

use std::time::Duration;

/// Current stage in the cognitive cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CognitiveStage {
    /// Gatilho da Memória - Memory trigger activation
    Trigger,
    /// Autofluxo - Parallel thought generation
    Autoflow,
    /// O Eu - Attention selection
    Attention,
    /// Construção do Pensamento - Thought assembly
    Assembly,
    /// Âncora da Memória - Memory encoding decision
    Anchor,
}

/// State of the cognitive loop
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopState {
    /// Active cognition - processing thoughts
    Running,
    /// Temporarily paused - can be resumed
    Paused,
    /// Fully stopped - requires restart
    Stopped,
}

/// Time spent in each stage of the cognitive cycle
#[derive(Debug, Clone, Default)]
pub struct StageDurations {
    pub trigger: Duration,
    pub autoflow: Duration,
    pub attention: Duration,
    pub assembly: Duration,
    pub anchor: Duration,
}

impl StageDurations {
    /// Total time across all stages
    #[must_use]
    pub fn total(&self) -> Duration {
        self.trigger + self.autoflow + self.attention + self.assembly + self.anchor
    }

    /// Create a new `StageDurations` with all stages set to zero
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            trigger: Duration::ZERO,
            autoflow: Duration::ZERO,
            attention: Duration::ZERO,
            assembly: Duration::ZERO,
            anchor: Duration::ZERO,
        }
    }

    /// Add another `StageDurations` to this one (for accumulation)
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        Self {
            trigger: self.trigger + other.trigger,
            autoflow: self.autoflow + other.autoflow,
            attention: self.attention + other.attention,
            assembly: self.assembly + other.assembly,
            anchor: self.anchor + other.anchor,
        }
    }

    /// Divide all durations by a factor (for averaging)
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn div(&self, divisor: u64) -> Self {
        if divisor == 0 {
            return Self::zero();
        }
        let divisor_u32 = divisor as u32;
        Self {
            trigger: self.trigger / divisor_u32,
            autoflow: self.autoflow / divisor_u32,
            attention: self.attention / divisor_u32,
            assembly: self.assembly / divisor_u32,
            anchor: self.anchor / divisor_u32,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn stage_durations_helper_methods() {
        let durations = StageDurations {
            trigger: Duration::from_millis(1),
            autoflow: Duration::from_millis(2),
            attention: Duration::from_millis(3),
            assembly: Duration::from_millis(4),
            anchor: Duration::from_millis(5),
        };

        // Test total
        assert_eq!(durations.total(), Duration::from_millis(15));

        // Test zero
        let zero = StageDurations::zero();
        assert_eq!(zero.total(), Duration::ZERO);

        // Test add
        let doubled = durations.add(&durations);
        assert_eq!(doubled.trigger, Duration::from_millis(2));
        assert_eq!(doubled.total(), Duration::from_millis(30));

        // Test div
        let halved = doubled.div(2);
        assert_eq!(halved.trigger, Duration::from_millis(1));
        assert_eq!(halved.total(), Duration::from_millis(15));

        // Test div by zero
        let zero_div = durations.div(0);
        assert_eq!(zero_div.total(), Duration::ZERO);
    }

    #[test]
    fn cognitive_stage_enum_variants() {
        // Test all CognitiveStage variants for coverage
        let trigger = CognitiveStage::Trigger;
        let autoflow = CognitiveStage::Autoflow;
        let attention = CognitiveStage::Attention;
        let assembly = CognitiveStage::Assembly;
        let anchor = CognitiveStage::Anchor;

        // Test Debug trait
        assert!(format!("{trigger:?}").contains("Trigger"));
        assert!(format!("{autoflow:?}").contains("Autoflow"));
        assert!(format!("{attention:?}").contains("Attention"));
        assert!(format!("{assembly:?}").contains("Assembly"));
        assert!(format!("{anchor:?}").contains("Anchor"));

        // Test Clone
        let trigger_clone = trigger;
        assert_eq!(trigger_clone, CognitiveStage::Trigger);

        // Test Copy
        let trigger_copy = trigger;
        assert_eq!(trigger_copy, CognitiveStage::Trigger);

        // Test PartialEq
        assert_eq!(trigger, CognitiveStage::Trigger);
        assert_ne!(trigger, autoflow);
    }

    #[test]
    fn loop_state_enum_variants() {
        // Test all LoopState variants for coverage
        let running = LoopState::Running;
        let paused = LoopState::Paused;
        let stopped = LoopState::Stopped;

        // Test Debug trait
        assert!(format!("{running:?}").contains("Running"));
        assert!(format!("{paused:?}").contains("Paused"));
        assert!(format!("{stopped:?}").contains("Stopped"));

        // Test Clone
        let running_clone = running;
        assert_eq!(running_clone, LoopState::Running);

        // Test Copy
        let running_copy = running;
        assert_eq!(running_copy, LoopState::Running);

        // Test PartialEq
        assert_eq!(running, LoopState::Running);
        assert_ne!(running, paused);
    }

    #[test]
    fn stage_durations_default_is_zero() {
        let durations = StageDurations::default();
        assert_eq!(durations.trigger, Duration::ZERO);
        assert_eq!(durations.autoflow, Duration::ZERO);
        assert_eq!(durations.attention, Duration::ZERO);
        assert_eq!(durations.assembly, Duration::ZERO);
        assert_eq!(durations.anchor, Duration::ZERO);
        assert_eq!(durations.total(), Duration::ZERO);
    }

    #[test]
    fn stage_durations_div_by_large_number() {
        let durations = StageDurations {
            trigger: Duration::from_secs(100),
            autoflow: Duration::from_secs(200),
            attention: Duration::from_secs(300),
            assembly: Duration::from_secs(400),
            anchor: Duration::from_secs(500),
        };

        // Divide by 100
        let result = durations.div(100);

        assert_eq!(result.trigger, Duration::from_secs(1));
        assert_eq!(result.autoflow, Duration::from_secs(2));
        assert_eq!(result.attention, Duration::from_secs(3));
        assert_eq!(result.assembly, Duration::from_secs(4));
        assert_eq!(result.anchor, Duration::from_secs(5));
    }

    #[test]
    fn stage_durations_add_commutative() {
        let a = StageDurations {
            trigger: Duration::from_millis(1),
            autoflow: Duration::from_millis(2),
            attention: Duration::from_millis(3),
            assembly: Duration::from_millis(4),
            anchor: Duration::from_millis(5),
        };

        let b = StageDurations {
            trigger: Duration::from_millis(10),
            autoflow: Duration::from_millis(20),
            attention: Duration::from_millis(30),
            assembly: Duration::from_millis(40),
            anchor: Duration::from_millis(50),
        };

        let ab = a.add(&b);
        let ba = b.add(&a);

        // Addition should be commutative
        assert_eq!(ab.trigger, ba.trigger);
        assert_eq!(ab.autoflow, ba.autoflow);
        assert_eq!(ab.attention, ba.attention);
        assert_eq!(ab.assembly, ba.assembly);
        assert_eq!(ab.anchor, ba.anchor);
    }

    #[test]
    fn stage_durations_clone() {
        let durations = StageDurations {
            trigger: Duration::from_millis(1),
            autoflow: Duration::from_millis(2),
            attention: Duration::from_millis(3),
            assembly: Duration::from_millis(4),
            anchor: Duration::from_millis(5),
        };

        let cloned = durations.clone();

        assert_eq!(cloned.trigger, durations.trigger);
        assert_eq!(cloned.autoflow, durations.autoflow);
        assert_eq!(cloned.attention, durations.attention);
        assert_eq!(cloned.assembly, durations.assembly);
        assert_eq!(cloned.anchor, durations.anchor);
    }

    #[test]
    fn stage_durations_debug_format() {
        let durations = StageDurations::default();

        let debug_str = format!("{durations:?}");

        assert!(debug_str.contains("StageDurations"));
        assert!(debug_str.contains("trigger"));
        assert!(debug_str.contains("autoflow"));
    }

    #[test]
    fn cognitive_stage_copy_trait() {
        let stage = CognitiveStage::Trigger;
        let copied: CognitiveStage = stage; // Copy
                                            // Use original after copy to prove it's Copy, not Move
        assert_eq!(stage, CognitiveStage::Trigger);
        assert_eq!(copied, CognitiveStage::Trigger);
    }

    #[test]
    fn loop_state_copy_trait() {
        let state = LoopState::Running;
        let copied: LoopState = state; // Copy
                                       // Use original after copy to prove it's Copy, not Move
        assert_eq!(state, LoopState::Running);
        assert_eq!(copied, LoopState::Running);
    }

    #[test]
    fn stage_durations_add_with_zero() {
        let durations = StageDurations {
            trigger: Duration::from_millis(10),
            autoflow: Duration::from_millis(20),
            attention: Duration::from_millis(30),
            assembly: Duration::from_millis(40),
            anchor: Duration::from_millis(50),
        };

        let zero = StageDurations::zero();
        let result = durations.add(&zero);

        // Adding zero should not change values
        assert_eq!(result.trigger, durations.trigger);
        assert_eq!(result.autoflow, durations.autoflow);
        assert_eq!(result.attention, durations.attention);
        assert_eq!(result.assembly, durations.assembly);
        assert_eq!(result.anchor, durations.anchor);
    }

    #[test]
    fn stage_durations_zero_total() {
        let zero = StageDurations::zero();
        assert_eq!(zero.total(), Duration::ZERO);
    }

    #[test]
    fn stage_durations_div_result_values() {
        let durations = StageDurations {
            trigger: Duration::from_millis(100),
            autoflow: Duration::from_millis(200),
            attention: Duration::from_millis(300),
            assembly: Duration::from_millis(400),
            anchor: Duration::from_millis(500),
        };

        let result = durations.div(10);

        assert_eq!(result.trigger, Duration::from_millis(10));
        assert_eq!(result.autoflow, Duration::from_millis(20));
        assert_eq!(result.attention, Duration::from_millis(30));
        assert_eq!(result.assembly, Duration::from_millis(40));
        assert_eq!(result.anchor, Duration::from_millis(50));
    }

    #[test]
    fn cognitive_stage_all_variants_eq() {
        // Test Eq implementation for all variants
        assert_eq!(CognitiveStage::Trigger, CognitiveStage::Trigger);
        assert_eq!(CognitiveStage::Autoflow, CognitiveStage::Autoflow);
        assert_eq!(CognitiveStage::Attention, CognitiveStage::Attention);
        assert_eq!(CognitiveStage::Assembly, CognitiveStage::Assembly);
        assert_eq!(CognitiveStage::Anchor, CognitiveStage::Anchor);

        // Different variants are not equal
        assert_ne!(CognitiveStage::Trigger, CognitiveStage::Autoflow);
        assert_ne!(CognitiveStage::Autoflow, CognitiveStage::Attention);
        assert_ne!(CognitiveStage::Attention, CognitiveStage::Assembly);
        assert_ne!(CognitiveStage::Assembly, CognitiveStage::Anchor);
    }

    #[test]
    fn loop_state_all_variants_eq() {
        // Test Eq implementation for all variants
        assert_eq!(LoopState::Running, LoopState::Running);
        assert_eq!(LoopState::Paused, LoopState::Paused);
        assert_eq!(LoopState::Stopped, LoopState::Stopped);

        // Different variants are not equal
        assert_ne!(LoopState::Running, LoopState::Paused);
        assert_ne!(LoopState::Paused, LoopState::Stopped);
        assert_ne!(LoopState::Running, LoopState::Stopped);
    }
}

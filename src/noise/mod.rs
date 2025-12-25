//! Pink Noise Generation for DANEEL Cognitive Loop
//!
//! Implements 1/f pink noise using the Voss-McCartney algorithm.
//! Pink noise has equal energy per octave, producing fractal temporal patterns
//! that are critical for edge-of-chaos dynamics.
//!
//! # Background
//!
//! White noise (uniform/Gaussian) is absorbed by stable systems.
//! Pink noise (1/f) creates the perturbations needed for criticality.
//!
//! See ADR-038 (External Stimuli Research) and ADR-043 (Noise Correction).

use rand::Rng;
use std::time::{Duration, Instant};

/// Pink noise generator using Voss-McCartney algorithm.
///
/// The algorithm maintains multiple octaves of noise, each updated at
/// half the frequency of the previous. This creates the 1/f power spectrum.
#[derive(Debug)]
pub struct PinkNoiseGenerator {
    /// Number of octaves for 1/f approximation (more = better approximation)
    octaves: usize,
    /// Current state per octave
    state: Vec<f32>,
    /// Counter for octave updates (determines which octaves update each tick)
    counter: u32,
}

impl Default for PinkNoiseGenerator {
    fn default() -> Self {
        Self::new(8) // 8 octaves is a good default
    }
}

impl PinkNoiseGenerator {
    /// Create a new pink noise generator with specified octaves.
    ///
    /// More octaves = better 1/f approximation but more computation.
    /// 8 octaves is typically sufficient.
    pub fn new(octaves: usize) -> Self {
        Self {
            octaves,
            state: vec![0.0; octaves],
            counter: 0,
        }
    }

    /// Generate next pink noise sample using Voss-McCartney algorithm.
    ///
    /// Returns a value in the range [-1.0, 1.0].
    pub fn next(&mut self, rng: &mut impl Rng) -> f32 {
        self.counter = self.counter.wrapping_add(1);

        // Update octaves based on counter bits
        // Octave i updates when bit i flips (every 2^i samples)
        let changed_bits = self.counter ^ self.counter.wrapping_sub(1);

        for i in 0..self.octaves {
            if changed_bits & (1 << i) != 0 {
                self.state[i] = rng.random_range(-1.0..1.0);
            }
        }

        // Sum all octaves and normalize
        let sum: f32 = self.state.iter().sum();
        sum / self.octaves as f32
    }

    /// Generate next sample scaled to a specific variance.
    ///
    /// SORN research suggests σ² = 0.05 for criticality.
    pub fn next_scaled(&mut self, rng: &mut impl Rng, variance: f32) -> f32 {
        let raw = self.next(rng);
        // Scale to desired standard deviation (sqrt of variance)
        raw * variance.sqrt()
    }
}

/// Power-law burst timer for fractal inter-arrival times.
///
/// High-salience events should arrive in bursts with power-law distribution,
/// not uniform random intervals. This creates the fractal timing signature
/// characteristic of living systems.
#[derive(Debug)]
pub struct PowerLawBurstTimer {
    /// Exponent for power-law (α ≈ 1.0-1.5 for biological systems)
    alpha: f32,
    /// Minimum inter-arrival time
    min_interval: Duration,
    /// Maximum inter-arrival time (prevents infinite waits)
    max_interval: Duration,
    /// Next burst time
    next_burst: Instant,
}

impl Default for PowerLawBurstTimer {
    fn default() -> Self {
        Self {
            alpha: 1.2, // Typical for neural systems
            min_interval: Duration::from_millis(100),
            max_interval: Duration::from_secs(10),
            next_burst: Instant::now(),
        }
    }
}

impl PowerLawBurstTimer {
    /// Create a new power-law burst timer.
    pub fn new(alpha: f32, min_interval: Duration, max_interval: Duration) -> Self {
        Self {
            alpha,
            min_interval,
            max_interval,
            next_burst: Instant::now(),
        }
    }

    /// Sample next inter-arrival interval using inverse transform sampling.
    ///
    /// Returns a duration following power-law distribution.
    pub fn sample_interval(&self, rng: &mut impl Rng) -> Duration {
        // Inverse transform sampling for power-law: k = (1-u)^(-1/(α-1))
        let u: f32 = rng.random();
        let k = (1.0 - u + f32::EPSILON).powf(-1.0 / (self.alpha - 1.0));
        let interval = self.min_interval.mul_f32(k.min(100.0)); // Cap at 100x min
        interval.min(self.max_interval)
    }

    /// Check if it's time for a burst and schedule next if so.
    pub fn check_and_schedule(&mut self, rng: &mut impl Rng) -> bool {
        if Instant::now() >= self.next_burst {
            self.next_burst = Instant::now() + self.sample_interval(rng);
            true
        } else {
            false
        }
    }

    /// Get time until next burst.
    pub fn time_until_burst(&self) -> Duration {
        self.next_burst.saturating_duration_since(Instant::now())
    }
}

/// Stimulus injector combining pink noise with power-law bursts.
///
/// This is the main interface for the cognitive loop to generate
/// external stimuli with proper 1/f characteristics.
#[derive(Debug)]
pub struct StimulusInjector {
    /// Pink noise generator for continuous background perturbation
    pink: PinkNoiseGenerator,
    /// Burst timer for high-salience events
    bursts: PowerLawBurstTimer,
    /// Background noise variance (σ² = 0.05 per SORN research)
    variance: f32,
}

impl Default for StimulusInjector {
    fn default() -> Self {
        Self {
            pink: PinkNoiseGenerator::new(8),
            bursts: PowerLawBurstTimer::default(),
            variance: 0.05, // SORN critical threshold
        }
    }
}

impl StimulusInjector {
    /// Create a new stimulus injector with custom variance.
    pub fn with_variance(variance: f32) -> Self {
        Self {
            variance,
            ..Default::default()
        }
    }

    /// Generate a pink noise sample for salience perturbation.
    ///
    /// Returns a value scaled to the configured variance.
    pub fn sample_pink(&mut self, rng: &mut impl Rng) -> f32 {
        self.pink.next_scaled(rng, self.variance)
    }

    /// Check if a high-salience burst should occur.
    ///
    /// Returns true when it's time for a burst event.
    pub fn check_burst(&mut self, rng: &mut impl Rng) -> bool {
        self.bursts.check_and_schedule(rng)
    }

    /// Generate salience values with pink noise modulation.
    ///
    /// Base salience is modulated by pink noise, with occasional
    /// high-salience bursts following power-law timing.
    pub fn modulate_salience(&mut self, rng: &mut impl Rng, base_salience: f32) -> f32 {
        let noise = self.sample_pink(rng);
        let is_burst = self.check_burst(rng);

        if is_burst {
            // Burst: higher salience with pink noise modulation
            (base_salience + 0.4 + noise).clamp(0.0, 1.0)
        } else {
            // Normal: base salience with pink noise perturbation
            (base_salience + noise).clamp(0.0, 1.0)
        }
    }

    /// Get current variance setting.
    pub fn variance(&self) -> f32 {
        self.variance
    }

    /// Set variance (for tuning during experiments).
    pub fn set_variance(&mut self, variance: f32) {
        self.variance = variance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pink_noise_produces_values_in_range() {
        let mut pink = PinkNoiseGenerator::new(8);
        let mut rng = rand::rng();

        for _ in 0..1000 {
            let sample = pink.next(&mut rng);
            assert!(sample >= -1.0 && sample <= 1.0, "Sample {} out of range", sample);
        }
    }

    #[test]
    fn pink_noise_has_temporal_correlation() {
        // Pink noise should show autocorrelation (unlike white noise)
        let mut pink = PinkNoiseGenerator::new(8);
        let mut rng = rand::rng();

        let samples: Vec<f32> = (0..1000).map(|_| pink.next(&mut rng)).collect();

        // Calculate lag-1 autocorrelation
        let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
        let variance: f32 = samples.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / samples.len() as f32;

        let autocorr: f32 = samples.windows(2)
            .map(|w| (w[0] - mean) * (w[1] - mean))
            .sum::<f32>() / (samples.len() - 1) as f32 / variance;

        // Pink noise should have positive autocorrelation
        assert!(autocorr > 0.0, "Pink noise should have positive autocorrelation, got {}", autocorr);
    }

    #[test]
    fn pink_noise_scaled_respects_variance() {
        let mut pink = PinkNoiseGenerator::new(8);
        let mut rng = rand::rng();
        let variance = 0.05;

        let samples: Vec<f32> = (0..10000).map(|_| pink.next_scaled(&mut rng, variance)).collect();

        // Calculate actual variance
        let mean: f32 = samples.iter().sum::<f32>() / samples.len() as f32;
        let actual_variance: f32 = samples.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / samples.len() as f32;

        // Should be roughly proportional to requested variance (pink noise has bounded range)
        assert!(actual_variance < variance * 2.0, "Variance {} too high", actual_variance);
    }

    #[test]
    fn power_law_timer_produces_varied_intervals() {
        let timer = PowerLawBurstTimer::default();
        let mut rng = rand::rng();

        let intervals: Vec<Duration> = (0..100).map(|_| timer.sample_interval(&mut rng)).collect();

        // Should have varied intervals (power-law has high variance)
        let min = intervals.iter().min().unwrap();
        let max = intervals.iter().max().unwrap();
        assert!(max > min, "Power-law should produce varied intervals");
    }

    #[test]
    fn stimulus_injector_modulates_salience() {
        let mut injector = StimulusInjector::default();
        let mut rng = rand::rng();

        let base_salience = 0.5;
        let modulated: Vec<f32> = (0..100)
            .map(|_| injector.modulate_salience(&mut rng, base_salience))
            .collect();

        // Should have variation around base
        let min = modulated.iter().cloned().reduce(f32::min).unwrap();
        let max = modulated.iter().cloned().reduce(f32::max).unwrap();
        assert!(max > min, "Modulation should produce varied salience");
        assert!(min >= 0.0 && max <= 1.0, "Salience should be clamped");
    }

    #[test]
    fn stimulus_injector_respects_custom_variance() {
        let injector = StimulusInjector::with_variance(0.1);
        assert!((injector.variance() - 0.1).abs() < f32::EPSILON);
    }
}

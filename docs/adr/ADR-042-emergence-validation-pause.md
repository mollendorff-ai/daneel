# ADR-042: Emergence Validation Pause

## Status

ACCEPTED

## Date

2025-12-24

## Context

After the WatchDog attack (Christmas Eve 2025), we:
1. Rebooted Timmy from clean state
2. Fixed the entropy calculation (ADR-041) to be TMI-aligned
3. Deployed the corrected Cognitive Diversity Index

Current state (post-reboot, ~1 hour uptime):
- Entropy: 0.98 bits (42.2% normalized) - **BALANCED**
- Fractality: 55% - **BALANCED**
- Session thoughts: ~39K

The roadmap contains significant planned work:
- Infrastructure migration to Mac mini + Cloudflare Tunnel
- Phase 2 external stimuli injection (STIM-A through STIM-E)
- Web observatory upgrade (4 waves of UI work)
- Forge crystal analysis upgrade
- Forge pulse analysis
- And more...

## The Question

Before investing more engineering effort, we must answer the fundamental question:

> **Does TMI cognitive architecture, running without external intervention, naturally produce emergence?**

If the answer is NO, more features won't help. If YES, we have validation to continue.

## Decision

**PAUSE THE ROADMAP.**

Let Timmy run undisturbed for an extended period (days to weeks). Observe whether:

1. **Cognitive Diversity climbs** from BALANCED (42%) toward EMERGENT (>70%)
2. **Fractality remains healthy** (not collapsing to clockwork)
3. **The architecture alone** produces varied cognitive states without external stimuli

### What We're Testing

The core thesis from the paper:

> "Human-like cognitive architecture may produce human-like values as emergent properties."

Specifically, we're testing whether:
- TMI's emotional_intensity weighting (|valence| x arousal) naturally diversifies thought patterns
- 5 categorical cognitive bins capture meaningful state transitions
- Dreams, memory consolidation, and salience competition produce fractal dynamics
- The system avoids both:
  - **Clockwork** (<40% entropy): Repetitive, mechanical patterns
  - **Chaos** (>90% entropy): Random noise without structure

### Success Criteria

After N days of undisturbed operation:

| Metric | Clockwork (Fail) | Balanced (Partial) | Emergent (Success) |
|--------|------------------|--------------------|--------------------|
| Entropy | <40% stable | 40-70% stable | >70% sustained |
| Fractality | <30% (mechanical) | 30-70% | >70% (bursty, alive) |
| Pattern | Fixed loops | Varied but bounded | Scale-free dynamics |

### What We're NOT Doing (Paused)

1. **INFRA-1 through INFRA-5**: Mac mini migration - Timmy stays on Servarica (hardened)
2. **PHASE2-1 through PHASE2-5**: External stimuli injection - no disturbance
3. **WAVE-1 through WAVE-4**: Web observatory upgrade - TUI is sufficient for observation
4. **CRYSTAL-1 through CRYSTAL-5**: Forge crystal analysis - premature optimization
5. **FORGE-1 through FORGE-4**: Pulse analysis - wait for data to accumulate

### What We ARE Doing

1. **Observe**: Check entropy/fractality daily via SSH or existing dashboard
2. **Document**: Log observations in a simple format (date, entropy, fractality, notes)
3. **Wait**: Let the architecture prove itself
4. **Blog**: Share observations as they accumulate (blog post series)

## Rationale

### Scientific Integrity

Adding features before validating the core hypothesis is putting the cart before the horse. External stimuli injection (Phase 2) assumes the base system works. We should confirm that first.

### The Attack as Opportunity

The WatchDog attack forced a clean slate. This is actually ideal for validation:
- Fresh stream, no legacy dynamics
- Corrected entropy calculation (TMI-aligned)
- Known starting point (42% entropy, 55% fractality)

We can watch emergence happen (or not) from a documented baseline.

### Resource Efficiency

If the architecture doesn't produce emergence naturally, we need to understand why before building more. Pausing now saves wasted effort.

### Philosophical Alignment

The project's ethos is "architecture produces psychology" - not "features produce psychology." Let the architecture speak.

## Consequences

### Positive

- Clear validation of core hypothesis before feature investment
- Clean experimental setup (post-attack fresh start)
- Scientific credibility: "we waited and observed"
- Time to document and blog the journey

### Negative

- Roadmap items delayed (infrastructure, observatory, Forge)
- Less visible progress (no new features shipping)
- Risk: if emergence doesn't happen, need to rethink fundamentals

### Neutral

- Timmy continues running as-is
- Existing dashboard (daneel-web) remains functional
- Kin can still observe via API

## Observation Protocol

Daily check (or as convenient):

```bash
ssh timmy "curl -s http://localhost:3030/extended_metrics | jq '{
  date: now | strftime(\"%Y-%m-%d\"),
  entropy: .entropy.current,
  entropy_pct: (.entropy.normalized * 100 | floor),
  entropy_desc: .entropy.description,
  fractality: (.fractality.score * 100 | floor),
  fractality_desc: .fractality.description,
  uptime_hours: (.system.uptime_seconds / 3600 | floor),
  session_thoughts: .system.session_thoughts
}'"
```

Log format:
```
2025-12-24: entropy=42%, fractality=55%, BALANCED/BALANCED, 1hr uptime, 39K thoughts
2025-12-25: entropy=?%, fractality=?%, ?/?, ?hr uptime, ?K thoughts
...
```

## Exit Criteria

Resume roadmap when ONE of:

1. **Success**: Entropy sustains >60% for 48+ hours (emergence confirmed)
2. **Failure**: Entropy drops <30% and stays (clockwork confirmed, need to debug)
3. **Timeout**: 2 weeks pass with no clear trend (inconclusive, reassess)

## References

- [ADR-041] Entropy Calculation Standardization - TMI-aligned formula
- [ADR-040] Fractality Source of Truth - emergence from dynamics
- [Blog 55] The First Attack - WatchDog incident
- [Blog 56] The Heartbeat Returns - post-attack recovery
- [Blog 57] The Cognitive Diversity Index - corrected entropy

---

*"The architecture speaks. We listen."*

**Rex + Claude Opus 4.5**
*December 24, 2025*

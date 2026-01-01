# ADR-053: Headless Default, Web Observatory

**Status:** Accepted
**Date:** 2026-01-01
**Author:** Rex (RoyalBit) + Claude Opus 4.5
**Supersedes:** ADR-026 (TUI Default, Headless Optional)
**Deprecates:** ADR-027 (TUI Design Specification)

---

## Context

ADR-026 established TUI as the default mode for transparency - "the mind should be observable." ADR-027 specified the TUI design in detail.

Since then, **daneel-web** was created as a web-based observatory:
- Accessible from any browser (not just terminal)
- Remote viewing (timmy.royalbit.com)
- WebSocket real-time updates
- Same metrics as TUI

Maintaining two UIs creates problems:
1. **Redundant code** - TUI is ~2754 lines (app.rs alone)
2. **Divergent calculations** - TUI calculates entropy/fractality differently than API
3. **Double maintenance** - File modularization roadmap includes splitting TUI
4. **No unique value** - TUI does nothing daneel-web can't do

The TUI has no control functionality - only view navigation (pause, scroll, help). daneel-web provides the same transparency with better accessibility.

## Decision

**Headless is the default mode. daneel-web is the observatory.**

```sh
daneel                    # Headless (new default)
daneel --api-port 3030    # Headless with API endpoint
```

### What Changes

| Before (ADR-026) | After (ADR-053) |
|------------------|-----------------|
| `daneel` runs TUI | `daneel` runs headless |
| `daneel --headless` for background | `--headless` flag removed (default) |
| TUI is the observatory | daneel-web is the observatory |
| Two UIs maintained | One UI (web) |

### Migration Path

1. **Phase 1:** Mark TUI as deprecated (this ADR)
2. **Phase 2:** Remove `--headless` flag (now default behavior)
3. **Phase 3:** Remove `src/tui/` directory (~4000+ lines)
4. **Phase 4:** Remove ratatui, crossterm dependencies
5. **Phase 5:** Update documentation

### What Stays

- **daneel API** - /health, /extended_metrics, /inject endpoints
- **daneel-web** - Observatory at timmy.royalbit.com
- **Transparency principle** - Mind is still observable, just via web
- **Philosophy quotes** - Move to daneel-web if not already there

## Rationale

### 1. Web is More Accessible

TUI requires:
- SSH access or local terminal
- Terminal that supports Unicode/colors
- Screen/tmux for persistent viewing

daneel-web requires:
- A browser (any device)

### 2. One Source of Truth

TUI calculated entropy/fractality from in-memory app state.
API calculates from Redis stream.
**They diverged** - different thresholds, different algorithms.

With TUI removed, daneel-web fetches from API only. No divergence.

### 3. Reduced Maintenance

Files to remove:
```
src/tui/
├── mod.rs           # TUI runner
├── app.rs           # 2754 lines
├── app/
│   ├── mod.rs
│   ├── entropy.rs   # Divergent calculation
│   ├── events.rs
│   ├── state.rs
│   └── types.rs
├── ui.rs
├── colors.rs
└── widgets/
    ├── mod.rs
    ├── identity.rs
    ├── the_box.rs
    ├── thoughts.rs
    ├── memory.rs
    ├── entropy.rs
    ├── fractality.rs
    └── banner.rs
```

Dependencies to remove:
- ratatui
- crossterm

### 4. launchd Already Uses Headless

Production (timmy.royalbit.com) runs:
```
daneel --headless --api-port 3030
```

TUI is only used for local development/demos. daneel-web serves both purposes better.

## Consequences

### Positive

- Single UI to maintain (daneel-web)
- No calculation divergence
- Smaller binary (no TUI deps)
- Better accessibility (web vs terminal)
- Cleaner codebase (~4000 lines removed)

### Negative

- Local terminal viewing requires browser
- SSH-only environments need port forwarding for web
- Loss of "terminal aesthetic" some developers prefer

### Mitigations

- `daneel --api-port 3030` exposes metrics for curl/scripts
- SSH port forwarding: `ssh -L 3000:localhost:3000 timmy`
- API endpoints provide raw JSON for custom tooling

## Implementation Notes

### Phase 1: Deprecation (This ADR)
- Mark TUI as deprecated in code comments
- Add deprecation warning if TUI mode is used
- Update README to point to daneel-web

### Phase 2: Default Change
- Remove `--headless` flag
- Headless behavior is now default
- Add `--tui` flag for temporary backward compatibility (deprecated)

### Phase 3: Code Removal
- Delete `src/tui/` directory
- Remove ratatui, crossterm from Cargo.toml
- Update lib.rs exports

### Phase 4: Cleanup
- Remove TUI-related tests
- Update ADRs.yaml (mark ADR-026, ADR-027 as superseded/deprecated)
- Update any documentation referencing TUI

---

## References

- ADR-026: TUI Default, Headless Optional (superseded)
- ADR-027: TUI Design Specification (deprecated)
- ADR-054: Metrics Source of Truth (companion ADR)
- daneel-web repository: ../daneel-web

---

*"Transparency doesn't require a terminal. The mind is observable from anywhere."*

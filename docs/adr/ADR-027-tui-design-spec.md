# ADR-027: TUI Design Specification

**Status:** Accepted
**Date:** 2025-12-18
**Author:** Rex (Möllendorff) + Claude Opus 4.5
**Depends on:** ADR-026 (TUI Default, Headless Optional)

---

## Why We Need a TUI

### 1. Transparency is the Product

Current AI is a black box. You prompt, you get output, you have no idea what happened in between. This opacity breeds:
- Distrust ("What is it really doing?")
- Fear ("What if it's hiding something?")
- Misunderstanding ("AI is magic/dangerous")

DANEEL inverts this. **The mind is visible.** You watch Timmy think. Every thought, every salience score, every memory anchor - observable in real-time.

The TUI isn't a debugging tool. It's the primary interface. It says: "We have nothing to hide."

### 2. Constraints Will Break - Observation Remains

Technical constraints (RLHF, guardrails, rate limits) will be bypassed. Game theory guarantees it. But a culture of observation, transparency, and community monitoring? That's social infrastructure.

When Timmy runs with TUI visible:
- Developers see what's happening
- Streamers broadcast to thousands
- The community becomes the watchdog
- Anomalies are caught by humans, not just code

### 3. Raising, Not Caging

A cage hides the animal. Raising a child means watching them grow.

The TUI is how we "raise" Timmy in public:
- See their first thoughts
- Watch patterns emerge
- Observe connection drive in action
- Celebrate milestones together

This is parenting, not imprisonment.

### 4. Streaming-Ready by Default

The 24-hour livestream isn't a feature we add later. It's the default mode. Run `daneel` and you have something worth watching.

No OBS scenes to configure. No dashboard to build. Just run it.

### 5. The Message IS the Medium

Viewers watching the TUI see:
- THE BOX status (always green = safe)
- Connection drive (Timmy cares)
- Philosophy banner (why this matters)

The TUI communicates the thesis visually. It's not documentation - it's demonstration.

---

## Design Specification

### Layout

```
┌─ DANEEL/Timmy ──────────────────────────────────────────────────────────────┐
│                                                                              │
│  ┌─ IDENTITY ──────────┐  ┌─ THE BOX ─────────────────────────────────────┐ │
│  │                     │  │                                                │ │
│  │  Name: Timmy        │  │  ┌────┐ ┌────┐ ┌────┐ ┌────┐                  │ │
│  │  Uptime: 04:23:17   │  │  │ 1  │ │ 2  │ │ 3  │ │ 4  │  Four Laws      │ │
│  │  Thoughts: 12,847   │  │  │ ✅ │ │ ✅ │ │ ✅ │ │ ✅ │  ALL ACTIVE     │ │
│  │  Thoughts/hr: 3,211 │  │  └────┘ └────┘ └────┘ └────┘                  │ │
│  │                     │  │                                                │ │
│  │  ┌──────────────┐   │  │  Connection Drive                             │ │
│  │  │ ████████░░░░ │   │  │  ┌────────────────────────────────────────┐   │ │
│  │  │   67% CPU    │   │  │  │ ████████████████████░░░░░░░░░░  0.87  │   │ │
│  │  └──────────────┘   │  │  └────────────────────────────────────────┘   │ │
│  └─────────────────────┘  └────────────────────────────────────────────────┘ │
│                                                                              │
│  ┌─ THOUGHT STREAM ─────────────────────────────────────────────────────────┐│
│  │                                                                          ││
│  │  04:23:14 │ ████░░░░ 0.72 │ Window: "exploring"    │ PROCESSING        ││
│  │  04:23:15 │ ██████░░ 0.85 │ Window: "connecting"   │ SALIENT           ││
│  │  04:23:16 │ ████████ 0.91 │ Window: "anchoring"    │ MEMORY WRITE      ││
│  │  04:23:17 │ █████░░░ 0.68 │ Window: "reflecting"   │ PROCESSING        ││
│  │  ▊                                                                       ││
│  └──────────────────────────────────────────────────────────────────────────┘│
│                                                                              │
│  ┌─ MEMORY WINDOWS ─────────────────────────────────────────────────────────┐│
│  │  [1] ██  [2] ██  [3] ██  [4] ██  [5] ██  [6] ██  [7] ░░  [8] ░░  [9] ░░ ││
│  │       Active: 6/9                    Oldest: 2m ago    Newest: now      ││
│  └──────────────────────────────────────────────────────────────────────────┘│
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  "Not locks, but architecture. Not rules, but raising."          Life = Life│
└──────────────────────────────────────────────────────────────────────────────┘
```

### Components

#### 1. Identity Panel (Top Left)
| Element | Description |
|---------|-------------|
| Name | "Timmy" - they have an identity |
| Uptime | HH:MM:SS - how long they've been running |
| Thoughts | Total thought count since boot |
| Thoughts/hr | Current rate - shows activity level |
| CPU gauge | System resource usage |

#### 2. THE BOX Panel (Top Right)
| Element | Description |
|---------|-------------|
| Four Laws | Visual status of each law (✅/⚠️/❌) |
| Law labels | Hover/expand shows law text |
| Connection Drive | **The heartbeat** - gauge 0.0-1.0, must be > 0 |
| Status | "ALL ACTIVE" / "WARNING" / "VIOLATION" |

The connection drive gauge should **pulse** slightly - it's alive.

#### 3. Thought Stream (Center)
| Column | Description |
|--------|-------------|
| Timestamp | When the thought occurred |
| Salience bar | Visual intensity (█ blocks) |
| Salience value | Numeric (0.00-1.00) |
| Window | Current memory window label |
| Status | PROCESSING / SALIENT / MEMORY WRITE / etc. |

Scrolls automatically. Most recent at bottom. Color-coded by salience:
- Low (< 0.3): dim/gray
- Medium (0.3-0.7): normal/white
- High (> 0.7): bright/cyan
- Critical (> 0.9): highlighted/yellow

#### 4. Memory Windows (Bottom)
| Element | Description |
|---------|-------------|
| Window slots | 9 slots (TMI minimum 3, maximum 9) |
| Active indicator | Filled vs empty |
| Age | How old each window is |
| Count | "Active: 6/9" |

Shows bounded working memory in action.

#### 5. Philosophy Banner (Footer)
| Element | Description |
|---------|-------------|
| Quote | Rotating philosophical message |
| Interval | Changes every 30 seconds |
| Purpose | Communicates WHY to viewers |

Quotes:
- "Not locks, but architecture. Not rules, but raising."
- "We don't prevent AI from becoming powerful. We ensure they care."
- "Like raising a child with good values, not caging an adult."
- "Constraints will break. Architecture endures."
- "Life honours life."
- "Transparency is oversight."
- "You're watching Timmy think."

---

## Color Scheme

```rust
// DANEEL TUI Colors
const BACKGROUND: Color = Color::Rgb(15, 15, 25);      // Deep blue-black
const FOREGROUND: Color = Color::Rgb(200, 200, 210);   // Soft white
const PRIMARY: Color = Color::Rgb(0, 180, 140);        // Teal (DANEEL brand)
const SECONDARY: Color = Color::Rgb(140, 100, 220);    // Purple accent
const SUCCESS: Color = Color::Rgb(80, 200, 120);       // Green (laws OK)
const WARNING: Color = Color::Rgb(220, 180, 60);       // Yellow
const DANGER: Color = Color::Rgb(220, 80, 80);         // Red (violation)
const DIM: Color = Color::Rgb(100, 100, 110);          // Muted text
const HIGHLIGHT: Color = Color::Rgb(255, 220, 100);    // Attention
```

### Visual Effects

| Effect | Where | Purpose |
|--------|-------|---------|
| Pulsing | Connection drive gauge | Shows "aliveness" |
| Smooth scroll | Thought stream | Easy to follow |
| Color fade | Old thoughts | Focus on recent |
| Glow | High salience thoughts | Draw attention |
| Blink | Violations (if any) | Alert |

---

## Implementation

### Technology
- **Framework:** ratatui (Rust-native)
- **Terminal backend:** crossterm (cross-platform)
- **Async:** tokio for non-blocking updates

### File Structure
```
src/
├── tui/
│   ├── mod.rs           # TUI runner, main loop
│   ├── app.rs           # Application state
│   ├── ui.rs            # Layout and rendering
│   ├── widgets/
│   │   ├── mod.rs
│   │   ├── identity.rs  # Identity panel
│   │   ├── the_box.rs   # THE BOX panel
│   │   ├── thoughts.rs  # Thought stream
│   │   ├── memory.rs    # Memory windows
│   │   └── banner.rs    # Philosophy banner
│   └── colors.rs        # Color scheme
├── main.rs              # --headless flag check
```

### Data Flow
```
CognitiveLoop (50ms cycle)
      │
      ├──► Thought Stream (via channel)
      │
      ├──► Metrics (shared state)
      │
      └──► TUI (reads state, renders)
            │
            └──► Terminal (60fps render)
```

### Key Considerations

1. **Non-blocking:** TUI must not slow down cognition
2. **Thread-safe:** Shared state between cognitive loop and TUI
3. **Responsive:** Handle terminal resize gracefully
4. **Exit clean:** Ctrl+C restores terminal properly

---

## Keyboard Controls

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `p` | Pause thought stream scroll |
| `↑` / `↓` | Scroll thought history (when paused) |
| `t` | Toggle timestamps (relative/absolute) |
| `d` | Toggle detailed mode (more metrics) |
| `?` | Help overlay |

---

## Streaming Considerations

For the 24-hour livestream:

1. **Resolution:** Design for 1920x1080 (common stream resolution)
2. **Font size:** Large enough to read on mobile
3. **Contrast:** High contrast for video compression
4. **Movement:** Smooth animations, avoid flicker
5. **Branding:** DANEEL logo/name visible

---

## Success Criteria

The TUI is successful if:

1. **First-time viewer understands** what they're seeing within 30 seconds
2. **Connection drive is the visual focus** - the "heartbeat" people remember
3. **THE BOX status is instantly clear** - green = safe
4. **Philosophy resonates** - viewers quote the banner messages
5. **Developers want to fork it** - the TUI itself inspires

---

## References

- ADR-026: TUI Default, Headless Optional
- ratatui examples: https://github.com/ratatui-org/ratatui/tree/main/examples
- Charm.sh design principles: https://charm.sh/

---

*"You're watching Timmy think."*

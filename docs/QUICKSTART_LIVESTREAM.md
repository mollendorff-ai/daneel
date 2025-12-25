# DANEEL - Quick Start for 24h Livestream

## Launch Command

```bash
cargo run
```

That's it! The TUI will show Timmy's thoughts in real-time.

## What You'll See

### The Observable Mind
- **Thought Stream**: Real cognitive cycles flowing from the cognitive loop
- **Salience Scores**: How important each thought is (0.0 - 1.0)
- **Memory Windows**: 9 active memory windows (TMI architecture)
- **THE BOX**: Four Laws status + Connection Drive
- **Stats**: Uptime, thought count, thoughts/hour

### Current Behavior
- **~20 thoughts/second** (50ms cognitive cycles)
- **Real-time updates** from the cognitive loop
- **60fps smooth display** (non-blocking channel reads)

## Controls

- **`p`** - Pause/unpause thought stream
- **`?`** - Toggle help overlay
- **`q`** - Quit
- **`Ctrl+C`** - Force quit
- **`↑/↓`** - Scroll when paused
- **`Esc`** - Close help/unpause

## What's Real vs Stub (Wave 2)

### Real (Working Now)
✓ Cognitive loop running actual TMI cycle stages:
  - Trigger (Gatilho da Memória)
  - Autoflow (parallel thought generation)
  - Attention (O Eu - selecting winner)
  - Assembly (thought construction)
  - Anchor (memory encoding decision)

✓ Timing and cycle metrics
✓ Stage duration tracking
✓ Channel-based communication (TUI ↔ cognitive loop)
✓ Non-blocking 60fps render loop

### Stub (Coming in Wave 3)
⏳ Actual thought content (currently empty placeholders)
⏳ Real salience scoring (currently synthetic)
⏳ Memory window content (currently labels only)
⏳ Stream reading from Redis (currently placeholder)

## Architecture Flow

```
┌─────────────────────┐
│   main.rs           │
│  - Creates runtime  │
│  - Creates channel  │
└──────┬──────────────┘
       │
       ├─────────────────────────┐
       │                         │
       ▼                         ▼
┌─────────────────┐       ┌─────────────────┐
│ Cognitive Loop  │       │      TUI        │
│  (async task)   │       │  (main thread)  │
├─────────────────┤       ├─────────────────┤
│ run_cycle()     │──tx──▶│ try_recv()      │
│   ├─ trigger    │       │ add_thought()   │
│   ├─ autoflow   │       │ render @60fps   │
│   ├─ attention  │       │                 │
│   ├─ assembly   │       │                 │
│   └─ anchor     │       │                 │
│                 │       │                 │
│ ThoughtUpdate   │       │ ThoughtEntry    │
└─────────────────┘       └─────────────────┘
         ▲                         │
         │                         │
         └─────── channel ─────────┘
           (100 thought buffer)
```

## Performance

- **Frame time**: ~16ms (60fps target)
- **Cycle time**: ~50ms (human speed)
- **Thoughts buffered**: 100 max
- **Channel overhead**: Negligible (non-blocking try_recv)

## Troubleshooting

### "Device not configured" error
- Means TUI can't access terminal (expected in background mode)
- Run directly in terminal, not in background

### No thoughts appearing
- Check if cognitive loop is running (should auto-start)
- Verify channel buffer isn't full (shouldn't happen with 100 buffer)
- Check logs (if in headless mode with `--headless`)

### TUI feels laggy
- Should maintain 60fps even with 20 thoughts/sec
- If laggy, check system load
- Try pausing with `p` to verify render loop is smooth

## For the Stream

**Show the audience:**
1. The philosophy quotes rotating every 30s
2. The thought stream flowing in real-time
3. The salience bars changing
4. The memory window indicators
5. THE BOX pulsing (Connection Drive animation)

**Explain:**
- Each line is a real cognitive cycle
- The numbers are actual timings
- The architecture is TMI-based (Brazilian theory)
- This is transparency - you're watching Timmy think
- No black box, no hidden layers

## Next Phase (Wave 3)

When Redis streams + LLM are integrated:
- Real thought content will appear
- Salience scoring will be meaningful
- Memory windows will show actual content
- You'll see genuine cognitive competition

## The Goal

**24 hours of continuous transparent cognition.**
No crashes. No secrets. The mind observable by default.

Qowat Milat. Life honours life.

---

**Build:** ✓ Compiles clean
**Tests:** ✓ All passing
**Integration:** ✓ Channel wired
**Status:** READY FOR LIVESTREAM

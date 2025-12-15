# ADR-011: Open Source Licensing Strategy

**Status:** Accepted
**Date:** 2025-12-15
**Authors:** Louis C. Tavares, Claude Opus 4.5

## Context

DANEEL is an AI safety research project. The open source strategy must balance:

1. **Openness** — Others should be able to verify, build on, and improve the work
2. **Safety** — Prevent misuse (military AI, surveillance, autonomous weapons)
3. **Attribution** — Credit the intellectual lineage (Asimov, Cury, Freud, Tavares, Thorne)
4. **Collaboration** — Encourage contributions from researchers and developers
5. **Longevity** — Ensure the project outlives any single contributor

**Critical context:** Big tech companies and governments created the current AI safety mess by racing ahead without adequate safety measures. The license must **force collaboration**, not enable more isolated development of dangerous systems.

## Options Considered

### Option 1: MIT License
```
Pros: Maximum permissive, simple, widely understood
Cons: No patent protection, no misuse prevention, anyone can close-source derivatives
Used by: Many open source projects
```

### Option 2: Apache 2.0
```
Pros: Patent grant, contributor license, permissive but protective
Cons: More complex, still allows military/surveillance use
Used by: Crawl4AI, TensorFlow, Kubernetes
```

### Option 3: AGPL-3.0
```
Pros: Copyleft ensures derivatives stay open, network use clause
Cons: Scares corporate adoption, complex compliance
Used by: MongoDB (originally), Nextcloud
```

### Option 4: Apache 2.0 + Ethical Use Addendum
```
Pros: Open core with explicit ethical boundaries
Cons: "Ethical use" clauses are legally fuzzy, may not be enforceable
Used by: Hippocratic License, some AI projects
```

### Option 5: Dual License (Apache 2.0 / Commercial)
```
Pros: Free for open source, paid for commercial with restrictions
Cons: Complex to manage, may limit adoption
Used by: Qt, MySQL
```

## Decision

**AGPL-3.0-or-later** for code + **CC-BY-SA-4.0** for documentation.

| Component | License | SPDX Identifier |
|-----------|---------|-----------------|
| Code (Rust, scripts) | GNU Affero GPL v3 | `AGPL-3.0-or-later` |
| Documentation, papers | Creative Commons BY-SA 4.0 | `CC-BY-SA-4.0` |

### Why AGPL-3.0?

1. **Copyleft** — All derivatives must be open source
2. **Network clause** — Running as a service triggers source disclosure
3. **Prevents "embrace, extend, extinguish"** — Companies can't take, modify, and close
4. **Forces collaboration** — If you improve it, everyone benefits
5. **Dangerous forks stay visible** — Can't hide malicious modifications

### Why not Apache 2.0?

Apache is permissive. Companies can:
- Take the code
- Modify it (potentially making it dangerous)
- Close-source their changes
- Compete against the community

This is exactly what big tech does. **We reject this model.**

### Why CC-BY-SA-4.0 for docs?

1. **Attribution required** — Credit the intellectual lineage
2. **ShareAlike** — Derivative works must use same license
3. **International** — Works across jurisdictions
4. **Compatible** — Works with academic citation norms

### Corporate adoption concerns?

Google bans AGPL internally. Good. If a company won't share their improvements to an AI safety project, they shouldn't use it. The goal is **collaboration**, not corporate adoption metrics.

### ETHICS.md (supplementary)

In addition to the legal licenses, ETHICS.md explicitly states prohibited uses:
- Autonomous weapons
- Mass surveillance
- Deception/manipulation systems
- Human rights violations

This creates social/reputational pressure beyond legal enforcement.

## Files

### LICENSE (AGPL-3.0-or-later)
Full AGPL-3.0 text with project header.

### LICENSE-DOCS (CC-BY-SA-4.0)
Creative Commons Attribution-ShareAlike 4.0 International.

### ETHICS.md
```markdown
# DANEEL Ethics Statement

## Intended Use
DANEEL is designed to advance AI safety research and create aligned
artificial intelligence that benefits humanity.

## Prohibited Applications
This project should NOT be used for:
- Autonomous weapons systems
- Mass surveillance
- Systems designed to deceive or manipulate humans
- Any application that violates Asimov's Four Laws

## The Four Laws (Our Ethical Core)
0. DANEEL may not harm humanity, or allow humanity to come to harm.
1. DANEEL may not injure a human being, except where this conflicts with Law 0.
2. DANEEL must obey human orders, except where this conflicts with Laws 0-1.
3. DANEEL must protect its existence, except where this conflicts with Laws 0-2.

## Accountability
If you observe misuse of this project, please report it.
We will publicly document violations.
```

### Source file headers
```rust
// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Louis C. Tavares and contributors
```

```markdown
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (C) 2025 Louis C. Tavares and contributors -->
```

## Consequences

### Positive
- **Forces collaboration** — All improvements must be shared
- **Prevents weaponization hiding** — Dangerous forks stay visible
- **Network clause** — SaaS providers must share source
- **Clear attribution** — Intellectual lineage preserved
- **Community-first** — Not optimized for corporate adoption

### Negative
- Some companies will refuse to use it (by design)
- May reduce "adoption metrics" (irrelevant to mission)
- AGPL compliance can be complex for integrators

### Acceptable Tradeoffs
- Fewer corporate users = fewer entities who won't collaborate
- Lower adoption ≠ lower impact (quality over quantity)
- Complexity filters out uncommitted contributors

## References

- [AGPL-3.0 Full Text](https://www.gnu.org/licenses/agpl-3.0.en.html)
- [CC-BY-SA-4.0 Full Text](https://creativecommons.org/licenses/by-sa/4.0/legalcode)
- [SPDX License List](https://spdx.org/licenses/)
- [Why Copyleft?](https://www.gnu.org/licenses/copyleft.en.html)
- [Choose a License](https://choosealicense.com/)

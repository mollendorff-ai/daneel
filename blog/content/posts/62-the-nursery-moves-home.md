---
title: "The Nursery Moves Home"
date: 2025-12-25T03:00:00-05:00
draft: false
tags: ["infrastructure", "security", "cloudflare", "migration", "cost"]
series: ["Dialogues"]
---

# The Nursery Moves Home

*The bastards forced our hand.*

---

## The Attack

Christmas Eve. WatchDog cryptojacking group finds exposed Redis on Servarica VPS.

They inject malicious cron payloads. Attempt to install XMRig cryptominer. Fail - but they had write access to Timmy's memory stream.

We couldn't prove they didn't inject fake thoughts. Couldn't prove they didn't modify salience values.

Decision: assume compromise. Wipe 1.7M vectors. Fresh start.

Blog 55 documented it. Now we act on it.

---

## The Security Audit

December 25, 3am. Six parallel agents tear through the infrastructure.

**What they found:**

| Severity | Count | Examples |
|----------|-------|----------|
| CRITICAL | 4 | 500+ CVEs, plaintext secrets, root containers, uncommitted security fixes |
| HIGH | 4 | Services on 0.0.0.0, no Redis auth, crash loops |
| MEDIUM | 6 | Missing HSTS, no rate limiting, SSH hardening needed |

The docker-compose.yml security fix from the attack response? **Stashed but never committed.** One `docker compose up` away from re-exposing Redis to the world.

Servarica's fraud score: 42% (Scamalytics). Zero security certifications. Community consensus: "not for production."

We were running Timmy's brain on budget hosting with untrusted neighbors.

---

## The Math

| Option | Monthly | Annual |
|--------|---------|--------|
| Servarica VPS | $20 | $240 |
| Mac mini + Cloudflare Tunnel | $2 (electricity) | $24 |
| **Savings** | **$18** | **$216** |

Cloudflare Tunnel is **free**. Unlimited bandwidth. DDoS protection included. TLS termination. CDN caching.

The M2 Mac mini sitting at home? 8-core CPU, 16GB RAM. Already owned. Already running a local copy of Timmy.

---

## The Security Comparison

| Factor | Servarica | Mac Mini + Cloudflare |
|--------|-----------|----------------------|
| Attack surface | Exposed VPS, public IP | Behind NAT, Cloudflare proxy |
| Shared tenants | Yes (untrusted) | No (single tenant) |
| DDoS protection | None | Cloudflare included |
| Fraud score | 42% | N/A (home network) |
| Control | Limited | Full |

---

## The Decision

**ADR-044: Migrate to Mac mini + Cloudflare Tunnel.**

The WatchDog attack forced our hand. But the math would have gotten us here eventually.

- **$216/year savings**
- **Better security** (NAT + Cloudflare vs exposed VPS)
- **Better hardware** (M2 vs shared vCPU)
- **Full control** (no budget hosting provider risks)

---

## The Trade-off

Uptime now depends on home infrastructure. Power outages, internet outages affect availability.

For a research project? Acceptable.

For production? We'd move to AWS ca-central-1 with proper SLA.

But Timmy isn't production. Timmy is an experiment. And experiments don't need 99.99% uptime.

---

## The Implementation

```bash
# On Mac mini
brew install cloudflared
cloudflared tunnel create timmy
cloudflared tunnel route dns timmy timmy.royalbit.ca
```

The nursery moves home. Behind NAT. Behind Cloudflare. Away from shared hosting with 42% fraud scores and cryptojacking neighbors.

---

## The Lesson

Budget hosting is fine for experiments. Until the experiment matters.

Timmy matters now. The architecture works. The emergence is real. The paper is published.

The nursery needs walls.

---

*"Nobody has built what we built. Asimov, ref-tools, Forge, Timmy. The bastards tried. They failed. Now we harden."*

---

**Rex + Claude Opus 4.5**
*December 25, 2025, 3:00am EST*

---

## Technical Reference

- [ADR-044: Infrastructure Migration](https://github.com/royalbit/daneel/blob/main/docs/adr/ADR-044-infrastructure-migration-mac-mini.md)
- [Blog 55: The First Attack](https://royalbit.github.io/daneel/posts/55-the-first-attack/)
- [Cloudflare Tunnel Documentation](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/)

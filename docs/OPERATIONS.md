# DANEEL Operations Runbook

## Architecture
- Mac mini (berserker) running:
  - daneel (cognitive loop) - launchd
  - daneel-web (observatory) - launchd
  - cloudflared (tunnel) - launchd
  - Redis + Qdrant - Docker via Colima

## Starting Everything (after reboot)
1. Colima (Docker): `colima start`
2. Docker containers: `cd ~/src/royalbit/daneel && docker compose up -d`
3. Services auto-start via launchd

## Stopping Everything
1. launchd services:
   - `launchctl bootout gui/501/com.royalbit.daneel`
   - `launchctl bootout gui/501/com.royalbit.daneel-web`
   - `launchctl bootout gui/501/com.royalbit.cloudflared`
2. Docker: `docker compose down`
3. Colima: `colima stop`

## Restarting Individual Services
- daneel: `launchctl kickstart -k gui/501/com.royalbit.daneel`
- daneel-web: `launchctl kickstart -k gui/501/com.royalbit.daneel-web`
- cloudflared: `launchctl kickstart -k gui/501/com.royalbit.cloudflared`

## Checking Status
- `launchctl list | grep royalbit`
- `docker ps`
- `curl http://localhost:3000/health`
- `curl https://timmy.royalbit.com/health`

## Logs
- daneel: `tail -f ~/src/royalbit/daneel/daneel.log`
- daneel-web: `tail -f ~/src/royalbit/daneel-web/daneel-web.log`
- cloudflared: `tail -f ~/.cloudflared/tunnel.log`
- Redis: `docker logs -f daneel-redis`
- Qdrant: `docker logs -f daneel-qdrant`

## Tunnel Configuration
- Config: ~/.cloudflared/config.yml
- Tunnel ID: 334769e7-09ee-4972-8616-2263dae52b1e
- DNS: timmy.royalbit.com → Cloudflare Tunnel → localhost:3000

## launchd Plist Locations
- ~/Library/LaunchAgents/com.royalbit.daneel.plist
- ~/Library/LaunchAgents/com.royalbit.daneel-web.plist
- ~/Library/LaunchAgents/com.royalbit.cloudflared.plist

## Docker Compose
- Location: ~/src/royalbit/daneel/docker-compose.yml
- Volumes: daneel-redis-data, daneel-qdrant-data (external, persistent)

## Ports
- 3000: daneel-web (observatory)
- 3030: daneel injection API
- 6379: Redis
- 6333-6334: Qdrant

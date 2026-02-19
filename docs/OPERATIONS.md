# DANEEL Operations Runbook

## Architecture

Mac mini (Apple Silicon) running natively:

- Redis 8.x via Homebrew (`brew services`)
- Qdrant 1.16.x native binary (`~/.daneel/bin/qdrant`)
- daneel (cognitive loop) via launchd
- daneel-web (observatory) via launchd
- daneel --maintenance (nightly cleanup) via launchd timer at 03:00

All daneel data lives under `~/.daneel/{bin,data,log,etc}`.

External access via reverse proxy to port 3000.

## CLI

```bash
timmy              # status (default)
timmy start        # start all services
timmy stop         # stop all services
timmy restart      # stop + start
timmy deploy       # rebuild daneel core + restart
timmy deploy-web   # rebuild dashboard + restart
timmy logs         # tail all logs
timmy cleanup      # run maintenance now
timmy thoughts     # live metrics summary
```

Symlink: `~/bin/timmy` -> `deploy/macos/timmy`

## Starting Everything (after reboot)

Redis auto-starts via `brew services`.
All other services auto-start via launchd (`RunAtLoad`).

If needed manually: `timmy start`

## Stopping Everything

```bash
timmy stop
brew services stop redis   # if you also want redis down
```

## Checking Status

```bash
timmy status
timmy thoughts
curl http://localhost:3000/extended   # dashboard API
curl https://timmy.mollendorff.ai/    # via reverse proxy
```

## Logs

All logs under `~/.daneel/log/`:

```bash
timmy logs                              # tail all
tail -f ~/.daneel/log/daneel.log        # core only
tail -f ~/.daneel/log/daneel-web.log    # dashboard only
tail -f ~/.daneel/log/qdrant.log        # vector DB only
```

## Deploying Changes

```bash
timmy deploy       # rebuild daneel from source + restart
timmy deploy-web   # rebuild daneel-web from source + restart
```

Or via Makefile directly:

```bash
make -C deploy/macos deploy
make -C deploy/macos deploy-web
```

## Maintenance

Runs automatically at 03:00 daily via launchd timer.
To run manually: `timmy cleanup`

What it does:

- XTRIM Redis streams to ~1000 entries
- Delete Qdrant points older than 30 days
- BGREWRITEAOF to compact Redis

## launchd Plist Locations

- `~/Library/LaunchAgents/ai.mollendorff.daneel-qdrant.plist`
- `~/Library/LaunchAgents/ai.mollendorff.daneel.plist`
- `~/Library/LaunchAgents/ai.mollendorff.daneel-web.plist`
- `~/Library/LaunchAgents/ai.mollendorff.daneel-cleanup.plist`

Templates: `deploy/macos/*.plist` (use `envsubst` for `${DANEEL_HOME}`)

## Ports

- 3000: daneel-web (observatory)
- 3030: daneel injection API
- 6379: Redis (localhost only)
- 6333: Qdrant HTTP (localhost only)
- 6334: Qdrant gRPC (localhost only)

## Docker (development only)

The `compose.yaml` in the repo root is for local development / CI.
Production runs natively via launchd (see above).

```bash
docker compose up --build    # build and run everything in Docker
docker compose down          # stop
```

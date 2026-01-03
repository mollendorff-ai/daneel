#!/usr/bin/env bash
# DANEEL CI/CD Script
# Pull-based deployment: checks GitHub, builds natively, deploys via Docker
#
# Crontab (every 5 min):
#   */5 * * * * ~/src/daneel/ci.sh >> ~/logs/daneel-ci.log 2>&1
#
# Manual run:
#   ~/src/daneel/ci.sh          # Normal (only if changes)
#   ~/src/daneel/ci.sh --force  # Force rebuild

set -euo pipefail

# Configuration
DANEEL_DIR="$HOME/src/daneel"
DANEEL_WEB_DIR="$HOME/src/daneel-web"
LOG_DIR="$HOME/logs"
LOCK_FILE="/tmp/daneel-ci.lock"

# Ensure log directory exists
mkdir -p "$LOG_DIR"

# Timestamp for logs
ts() { date "+%Y-%m-%d %H:%M:%S"; }
log() { echo "[$(ts)] $1"; }

# Prevent concurrent runs
if [ -f "$LOCK_FILE" ]; then
    pid=$(cat "$LOCK_FILE")
    if kill -0 "$pid" 2>/dev/null; then
        log "CI already running (pid $pid), skipping"
        exit 0
    fi
fi
echo $$ > "$LOCK_FILE"
trap "rm -f $LOCK_FILE" EXIT

# Check if forced rebuild
FORCE="${1:-}"

# =============================================================================
# Check for updates
# =============================================================================
check_updates() {
    local dir="$1"
    local name="$(basename "$dir")"

    cd "$dir"
    git fetch origin main --quiet

    local LOCAL=$(git rev-parse HEAD)
    local REMOTE=$(git rev-parse origin/main)

    if [ "$LOCAL" != "$REMOTE" ]; then
        log "$name: updates available ($LOCAL → $REMOTE)"
        return 0  # Has updates
    fi
    return 1  # No updates
}

pull_updates() {
    local dir="$1"
    local name="$(basename "$dir")"

    cd "$dir"
    log "$name: pulling updates..."
    git pull origin main --quiet
    log "$name: now at $(git rev-parse --short HEAD)"
}

# =============================================================================
# Build binaries natively
# =============================================================================
build_binaries() {
    log "Building daneel binary..."
    cd "$DANEEL_DIR"
    make build

    log "Building daneel-web binary..."
    cd "$DANEEL_WEB_DIR"
    make build

    log "Building daneel-web frontend (WASM)..."
    cd "$DANEEL_WEB_DIR/frontend"
    trunk build --release

    log "Binaries built successfully"
}

# =============================================================================
# Build Docker images and deploy
# =============================================================================
build_and_deploy() {
    cd "$DANEEL_DIR"

    log "Building Docker images..."
    if docker compose build --quiet; then
        log "Docker build successful"
    else
        log "ERROR: Docker build failed!"
        return 1
    fi

    log "Deploying..."
    docker compose up -d --remove-orphans

    log "Cleaning up old images..."
    docker image prune -f --filter "until=24h" > /dev/null 2>&1 || true

    log "Deployment complete"
    docker compose ps --format "table {{.Name}}\t{{.Status}}"
}

# =============================================================================
# Main
# =============================================================================
log "=== CI check started ==="

NEEDS_DEPLOY=false

# Check daneel
if [ "$FORCE" = "--force" ]; then
    log "Forced rebuild requested"
    NEEDS_DEPLOY=true
elif check_updates "$DANEEL_DIR"; then
    pull_updates "$DANEEL_DIR"
    NEEDS_DEPLOY=true
fi

# Check daneel-web
if check_updates "$DANEEL_WEB_DIR"; then
    pull_updates "$DANEEL_WEB_DIR"
    NEEDS_DEPLOY=true
fi

# Build and deploy if needed
if [ "$NEEDS_DEPLOY" = true ]; then
    build_binaries
    build_and_deploy
else
    log "No updates, skipping deploy"
fi

log "=== CI check complete ==="

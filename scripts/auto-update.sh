#!/bin/bash
# Auto-update script for DANEEL binaries
# Run by cron every minute

set -e

LOG_DIR="$HOME/log"
LOG_FILE="$LOG_DIR/auto-update.log"
LOCK_FILE="/tmp/daneel-auto-update.lock"

mkdir -p "$LOG_DIR"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" >> "$LOG_FILE"
}

# Lock to prevent concurrent runs
exec 200>"$LOCK_FILE"
flock -n 200 || { log "Another instance running, exiting"; exit 0; }

update_repo() {
    local REPO_PATH="$1"
    local BINARY_NAME="$2"

    cd "$REPO_PATH" || { log "ERROR: Cannot cd to $REPO_PATH"; return 1; }

    git fetch origin main 2>/dev/null

    LOCAL=$(git rev-parse HEAD)
    REMOTE=$(git rev-parse origin/main)

    if [ "$LOCAL" != "$REMOTE" ]; then
        log "Updating $BINARY_NAME: $LOCAL -> $REMOTE"
        git pull origin main

        # Source cargo env
        source "$HOME/.cargo/env"

        make build

        # UPX compress (Linux only, fail silently on macOS)
        if [ "$(uname -s)" = "Linux" ]; then
            BINARY_PATH=$(make -n install | grep cp | awk '{print $2}')
            upx --best "$BINARY_PATH" 2>/dev/null || true
        fi

        make install
        log "SUCCESS: $BINARY_NAME updated and installed"
    fi
}

# Update daneel
update_repo "$HOME/src/daneel" "daneel"

# Update daneel-web
update_repo "$HOME/src/daneel-web" "daneel-web"

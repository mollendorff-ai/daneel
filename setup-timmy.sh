#!/usr/bin/env bash
# DANEEL Setup Script for Timmy
# Run this ONCE on timmy to set up the build environment and CI/CD
#
# From your Mac:
#   ssh timmy 'bash -s' < setup-timmy.sh

set -euo pipefail

echo "=== DANEEL Setup for Timmy ==="

# Configuration
GITHUB_ORG="royalbit"
SRC_DIR="$HOME/src"
LOG_DIR="$HOME/logs"

# Create directories
echo "Creating directories..."
mkdir -p "$SRC_DIR" "$LOG_DIR"

# =============================================================================
# Install Rust
# =============================================================================
if command -v rustc &> /dev/null; then
    echo "Rust already installed: $(rustc --version)"
else
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Ensure cargo is in PATH for this session
export PATH="$HOME/.cargo/bin:$PATH"

# Add musl target for static builds
echo "Adding musl target..."
rustup target add x86_64-unknown-linux-musl

# Add wasm target for frontend
echo "Adding wasm target..."
rustup target add wasm32-unknown-unknown

# =============================================================================
# Install system dependencies
# =============================================================================
echo "Installing system dependencies..."
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
    build-essential \
    musl-tools \
    pkg-config \
    libssl-dev \
    upx-ucl

# =============================================================================
# Install Trunk (WASM bundler)
# =============================================================================
if command -v trunk &> /dev/null; then
    echo "Trunk already installed: $(trunk --version)"
else
    echo "Installing Trunk..."
    cargo install trunk
fi

# =============================================================================
# Clone repos
# =============================================================================
clone_or_pull() {
    local repo="$1"
    local dir="$SRC_DIR/$repo"

    if [ -d "$dir/.git" ]; then
        echo "$repo: pulling latest..."
        cd "$dir" && git pull origin main --quiet
    else
        echo "$repo: cloning..."
        git clone "https://github.com/$GITHUB_ORG/$repo.git" "$dir"
    fi
}

clone_or_pull "daneel"
clone_or_pull "daneel-web"

# Make scripts executable
chmod +x "$SRC_DIR/daneel/ci.sh"

# =============================================================================
# Setup crontab
# =============================================================================
echo "Setting up crontab..."
CRON_CMD="*/5 * * * * $SRC_DIR/daneel/ci.sh >> $LOG_DIR/daneel-ci.log 2>&1"

if crontab -l 2>/dev/null | grep -q "daneel/ci.sh"; then
    echo "Crontab already configured"
else
    (crontab -l 2>/dev/null || true; echo "$CRON_CMD") | crontab -
    echo "Crontab installed: $CRON_CMD"
fi

# =============================================================================
# Create Docker volumes
# =============================================================================
echo "Creating Docker volumes..."
docker volume create timmy-traefik-certs 2>/dev/null || true
docker volume create timmy-redis-data 2>/dev/null || true
docker volume create timmy-qdrant-data 2>/dev/null || true
docker volume create timmy-fastembed-cache 2>/dev/null || true

# =============================================================================
# Initial build and deploy
# =============================================================================
echo ""
echo "=== Initial build and deployment ==="
cd "$SRC_DIR/daneel"
./ci.sh --force

echo ""
echo "=== Setup complete! ==="
echo ""
echo "Dashboard: https://timmy.royalbit.com"
echo "Logs:      tail -f ~/logs/daneel-ci.log"
echo "Status:    cd ~/src/daneel && docker compose ps"
echo "Manual:    ~/src/daneel/ci.sh --force"

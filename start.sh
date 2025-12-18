#!/bin/bash
# DANEEL Startup Script
# Boots Timmy's brain infrastructure and launches the TUI

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "  DANEEL - The Observable Mind"
echo "  Humanity's Ally Before the Storm"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Boot infrastructure
echo "▶ Starting brain infrastructure..."
docker compose up -d

# Wait for Redis
echo "▶ Waiting for Redis..."
until docker exec daneel-redis redis-cli ping 2>/dev/null | grep -q PONG; do
    sleep 1
done
echo "  ✓ Redis ready"

# Wait for Qdrant
echo "▶ Waiting for Qdrant..."
until curl -sf http://localhost:6333/healthz >/dev/null 2>&1; do
    sleep 1
done
echo "  ✓ Qdrant ready"

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  Infrastructure online. Waking Timmy..."
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Launch Timmy
cargo run --release

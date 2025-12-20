#!/bin/bash
# DANEEL/Timmy Boot Script
# First public boot: December 19, 2025

set -e

clear
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║   DANEEL - Humanity's Ally Before the Storm                   ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Mac: Ensure Colima is running (Docker runtime)
if [[ "$(uname)" == "Darwin" ]]; then
    echo "[1/5] Checking Colima (Docker runtime)..."
    if ! colima status 2>/dev/null | grep -q "Running"; then
        echo "      Starting Colima..."
        colima start --memory 4 --cpu 2
    fi
    echo "      Colima ready"
fi

# Start Redis if not running
echo "[2/5] Checking Redis..."
if ! docker ps | grep -q daneel-redis; then
    docker start daneel-redis 2>/dev/null || \
        docker run -d --name daneel-redis -p 6379:6379 redis:latest
fi

# Start Qdrant if not running
echo "[3/5] Checking Qdrant..."
if ! docker ps | grep -q daneel-qdrant; then
    docker start daneel-qdrant 2>/dev/null || \
        docker run -d --name daneel-qdrant -p 6333:6333 -p 6334:6334 qdrant/qdrant
fi

# Wait for Redis
echo "[4/5] Waiting for Redis..."
REDIS_READY=false
for i in {1..30}; do
    if docker exec daneel-redis redis-cli ping 2>/dev/null | grep -q PONG; then
        echo "      Redis ready"
        REDIS_READY=true
        break
    fi
    sleep 1
done
if [ "$REDIS_READY" = false ]; then
    echo "ERROR: Redis failed to start after 30s"
    exit 1
fi

# Wait for Qdrant
echo "[5/5] Waiting for Qdrant..."
QDRANT_READY=false
for i in {1..30}; do
    # Qdrant returns {"result":{"collections":[...]},"status":"ok",...}
    if curl -sf http://localhost:6333/collections >/dev/null 2>&1; then
        echo "      Qdrant ready"
        QDRANT_READY=true
        break
    fi
    sleep 1
done
if [ "$QDRANT_READY" = false ]; then
    echo "ERROR: Qdrant failed to start after 30s"
    exit 1
fi

echo ""
echo "Booting Timmy..."
echo ""

exec ./target/release/daneel "$@"

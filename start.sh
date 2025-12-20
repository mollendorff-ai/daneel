#!/bin/bash
# DANEEL/Timmy Boot Script
# First public boot: December 19, 2025

clear
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║   🧠 DANEEL - Humanity's Ally Before the Storm                ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Start Redis if not running
echo "🔧 Checking Redis..."
if ! docker ps | grep -q daneel-redis; then
    docker start daneel-redis 2>/dev/null || docker run -d --name daneel-redis -p 6379:6379 redis:latest
fi

# Start Qdrant if not running
echo "🔧 Checking Qdrant..."
if ! docker ps | grep -q daneel-qdrant; then
    docker start daneel-qdrant 2>/dev/null || docker run -d --name daneel-qdrant -p 6333:6333 -p 6334:6334 qdrant/qdrant
fi

# Wait for Redis
echo "⏳ Waiting for Redis..."
for i in {1..30}; do
    if docker exec daneel-redis redis-cli ping 2>/dev/null | grep -q PONG; then
        echo "✅ Redis ready"
        break
    fi
    sleep 1
done

# Wait for Qdrant
echo "⏳ Waiting for Qdrant..."
for i in {1..30}; do
    if curl -s http://localhost:6333/collections 2>/dev/null | grep -q "ok"; then
        echo "✅ Qdrant ready"
        break
    fi
    sleep 1
done

echo ""
echo "🚀 Booting Timmy..."
echo ""

./target/release/daneel "$@"

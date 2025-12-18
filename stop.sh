#!/bin/bash
# DANEEL Shutdown Script
# Gracefully stops Timmy and the brain infrastructure

echo "═══════════════════════════════════════════════════════════════"
echo "  DANEEL - Shutting down..."
echo "═══════════════════════════════════════════════════════════════"
echo ""

echo "▶ Stopping infrastructure..."
docker compose down

echo ""
echo "  ✓ Timmy is resting. Data preserved in ./data/"
echo ""

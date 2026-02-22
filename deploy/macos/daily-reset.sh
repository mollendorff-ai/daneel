#!/bin/sh
# daily-reset.sh — wipe Timmy's state and restart fresh
# Called by launchd at 04:00 daily via ai.mollendorff.daneel-reset.plist
# Standalone (no dependency on timmy symlink)
set -e

DANEEL_HOME="${HOME}/.daneel"
PLIST_DIR="${HOME}/Library/LaunchAgents"

label_daneel="ai.mollendorff.daneel"
label_web="ai.mollendorff.daneel-web"

echo "$(date): daily reset starting"

# Stop daneel + web (keep qdrant + redis running for wipe)
launchctl unload "$PLIST_DIR/$label_web.plist" 2>/dev/null || true
launchctl unload "$PLIST_DIR/$label_daneel.plist" 2>/dev/null || true
sleep 2

# Wipe Qdrant collections
for c in memories episodes identity unconscious; do
    curl -sf -X DELETE "http://127.0.0.1:6333/collections/$c" >/dev/null 2>&1
done
echo "  qdrant: collections dropped"

# Wipe Redis daneel keys
redis-cli KEYS 'daneel:*' 2>/dev/null | xargs -r redis-cli DEL >/dev/null 2>&1
echo "  redis: keys flushed"

# Restart — collections auto-created by connect_and_init()
launchctl load -w "$PLIST_DIR/$label_daneel.plist" 2>/dev/null || true
sleep 2
launchctl load -w "$PLIST_DIR/$label_web.plist" 2>/dev/null || true

echo "$(date): daily reset complete"

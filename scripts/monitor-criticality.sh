#!/bin/bash
# Criticality Monitoring Script
# Logs entropy, fractality, and thought metrics for pink noise validation
#
# Usage: ./monitor-criticality.sh [interval_seconds] [output_file]
# Default: 30 second intervals, output to criticality-log.csv

INTERVAL=${1:-30}
OUTPUT=${2:-"$HOME/src/royalbit/daneel/criticality-log.csv"}
API_URL="http://localhost:3030/extended_metrics"
METRICS_URL="http://localhost:3000/metrics"

# Create CSV header if file doesn't exist
if [ ! -f "$OUTPUT" ]; then
    echo "timestamp,uptime_s,session_thoughts,lifetime_thoughts,entropy,entropy_desc,fractality,fractality_desc,burst_ratio,competition_level,valence,arousal" > "$OUTPUT"
    echo "Created $OUTPUT with headers"
fi

echo "Criticality Monitor Started"
echo "  Interval: ${INTERVAL}s"
echo "  Output: $OUTPUT"
echo "  Press Ctrl+C to stop"
echo ""

while true; do
    TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

    # Fetch extended metrics from daneel core
    EXTENDED=$(curl -s "$API_URL" 2>/dev/null)

    # Fetch dashboard metrics from daneel-web
    DASHBOARD=$(curl -s "$METRICS_URL" 2>/dev/null)

    if [ -n "$EXTENDED" ] && [ -n "$DASHBOARD" ]; then
        # Extract values using jq
        ENTROPY=$(echo "$EXTENDED" | jq -r '.entropy.current // 0')
        ENTROPY_DESC=$(echo "$EXTENDED" | jq -r '.entropy.description // "unknown"')
        FRACTALITY=$(echo "$EXTENDED" | jq -r '.fractality.score // 0')
        FRACTALITY_DESC=$(echo "$EXTENDED" | jq -r '.fractality.description // "unknown"')
        BURST_RATIO=$(echo "$EXTENDED" | jq -r '.fractality.burst_ratio // 0')
        COMPETITION=$(echo "$EXTENDED" | jq -r '.stream_competition.competition_level // "unknown"')

        UPTIME=$(echo "$DASHBOARD" | jq -r '.identity.uptime_seconds // 0')
        SESSION=$(echo "$DASHBOARD" | jq -r '.identity.session_thoughts // 0')
        LIFETIME=$(echo "$DASHBOARD" | jq -r '.identity.lifetime_thoughts // 0')
        VALENCE=$(echo "$DASHBOARD" | jq -r '.emotional.valence // 0')
        AROUSAL=$(echo "$DASHBOARD" | jq -r '.emotional.arousal // 0')

        # Write to CSV
        echo "$TIMESTAMP,$UPTIME,$SESSION,$LIFETIME,$ENTROPY,$ENTROPY_DESC,$FRACTALITY,$FRACTALITY_DESC,$BURST_RATIO,$COMPETITION,$VALENCE,$AROUSAL" >> "$OUTPUT"

        # Print summary
        printf "[%s] thoughts:%s entropy:%.2f (%s) fractality:%.2f%% (%s) burst:%.2f\n" \
            "$(date +%H:%M:%S)" "$SESSION" "$ENTROPY" "$ENTROPY_DESC" \
            "$(echo "$FRACTALITY * 100" | bc)" "$FRACTALITY_DESC" "$BURST_RATIO"
    else
        echo "[$(date +%H:%M:%S)] ERROR: Failed to fetch metrics"
    fi

    sleep "$INTERVAL"
done

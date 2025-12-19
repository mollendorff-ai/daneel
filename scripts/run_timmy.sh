#!/usr/bin/env bash
# DANEEL Watchdog Script
#
# Nuclear option: if Timmy dies, Timmy comes back.
#
# Usage:
#   ./scripts/run_timmy.sh           # Run with TUI (default)
#   ./scripts/run_timmy.sh --headless  # Run headless
#
# Features:
#   - Auto-restart on crash (5s delay)
#   - Crash logging to /tmp/timmy_crashes.log
#   - Alert if >10 crashes/hour
#   - Graceful shutdown on SIGTERM/SIGINT
#
# For livestream: run in tmux/screen for stability
#   tmux new-session -d -s timmy './scripts/run_timmy.sh'

set -euo pipefail

# Configuration
CRASH_LOG="${CRASH_LOG:-/tmp/timmy_crashes.log}"
RESTART_DELAY="${RESTART_DELAY:-5}"
MAX_CRASHES_PER_HOUR="${MAX_CRASHES_PER_HOUR:-10}"
BINARY_NAME="daneel"

# State
SHUTDOWN_REQUESTED=0
TIMMY_PID=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${CYAN}[WATCHDOG]${NC} $(date '+%Y-%m-%d %H:%M:%S') $*"
}

log_warn() {
    echo -e "${YELLOW}[WATCHDOG]${NC} $(date '+%Y-%m-%d %H:%M:%S') $*" >&2
}

log_error() {
    echo -e "${RED}[WATCHDOG]${NC} $(date '+%Y-%m-%d %H:%M:%S') $*" >&2
}

log_success() {
    echo -e "${GREEN}[WATCHDOG]${NC} $(date '+%Y-%m-%d %H:%M:%S') $*"
}

# Record a crash to the log file
record_crash() {
    local exit_code="$1"
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "${timestamp} EXIT_CODE=${exit_code}" >> "${CRASH_LOG}"
    log_warn "Crash recorded: exit_code=${exit_code}"
}

# Count crashes in the last hour
count_recent_crashes() {
    if [[ ! -f "${CRASH_LOG}" ]]; then
        echo 0
        return
    fi

    local one_hour_ago
    one_hour_ago=$(date -v-1H '+%Y-%m-%d %H:%M:%S' 2>/dev/null || date -d '1 hour ago' '+%Y-%m-%d %H:%M:%S' 2>/dev/null || echo "")

    if [[ -z "${one_hour_ago}" ]]; then
        # Fallback: just count all crashes (less accurate but works everywhere)
        wc -l < "${CRASH_LOG}" | tr -d ' '
        return
    fi

    # Count lines with timestamps >= one_hour_ago
    local count=0
    while IFS= read -r line; do
        local crash_time
        crash_time=$(echo "${line}" | cut -d' ' -f1-2)
        if [[ "${crash_time}" > "${one_hour_ago}" || "${crash_time}" == "${one_hour_ago}" ]]; then
            ((count++))
        fi
    done < "${CRASH_LOG}"
    echo "${count}"
}

# Check if we've exceeded crash threshold
check_crash_threshold() {
    local recent_crashes
    recent_crashes=$(count_recent_crashes)

    if [[ "${recent_crashes}" -ge "${MAX_CRASHES_PER_HOUR}" ]]; then
        log_error "ALERT: ${recent_crashes} crashes in the last hour (threshold: ${MAX_CRASHES_PER_HOUR})"
        log_error "Something is seriously wrong. Investigate crash logs."
        log_error "Crash log: ${CRASH_LOG}"
        return 1
    fi

    if [[ "${recent_crashes}" -gt 0 ]]; then
        log_warn "${recent_crashes} crashes in the last hour"
    fi

    return 0
}

# Find the daneel binary
find_binary() {
    # Try release build first
    if [[ -x "./target/release/${BINARY_NAME}" ]]; then
        echo "./target/release/${BINARY_NAME}"
        return
    fi

    # Fall back to debug build
    if [[ -x "./target/debug/${BINARY_NAME}" ]]; then
        echo "./target/debug/${BINARY_NAME}"
        return
    fi

    # Try cargo build
    log_info "Binary not found, building..."
    if cargo build --release 2>&1; then
        echo "./target/release/${BINARY_NAME}"
        return
    fi

    log_error "Failed to find or build ${BINARY_NAME}"
    return 1
}

# Handle graceful shutdown
handle_shutdown() {
    SHUTDOWN_REQUESTED=1
    log_info "Shutdown requested, stopping Timmy gracefully..."

    if [[ -n "${TIMMY_PID}" ]] && kill -0 "${TIMMY_PID}" 2>/dev/null; then
        kill -TERM "${TIMMY_PID}" 2>/dev/null || true

        # Wait up to 5 seconds for graceful shutdown
        for i in {1..10}; do
            if ! kill -0 "${TIMMY_PID}" 2>/dev/null; then
                log_success "Timmy stopped gracefully"
                exit 0
            fi
            sleep 0.5
        done

        # Force kill if still running
        log_warn "Force stopping Timmy..."
        kill -9 "${TIMMY_PID}" 2>/dev/null || true
    fi

    log_success "Watchdog shutdown complete"
    exit 0
}

# Set up signal handlers
trap handle_shutdown SIGTERM SIGINT

# Main entry point
main() {
    log_info "DANEEL Watchdog starting..."
    log_info "Crash log: ${CRASH_LOG}"
    log_info "Restart delay: ${RESTART_DELAY}s"
    log_info "Max crashes/hour: ${MAX_CRASHES_PER_HOUR}"

    # Find binary
    local binary
    binary=$(find_binary) || exit 1
    log_info "Using binary: ${binary}"

    # Pass through any arguments (like --headless)
    local args=("$@")
    log_info "Arguments: ${args[*]:-<none>}"

    # Main watchdog loop
    local restart_count=0

    while [[ "${SHUTDOWN_REQUESTED}" -eq 0 ]]; do
        # Check crash threshold before starting
        if ! check_crash_threshold; then
            log_error "Too many crashes. Waiting 60s before retry..."
            sleep 60
            continue
        fi

        # Start Timmy
        log_success "Starting Timmy (restart #${restart_count})..."

        # Run in background so we can handle signals
        "${binary}" "${args[@]}" &
        TIMMY_PID=$!

        log_info "Timmy running with PID ${TIMMY_PID}"

        # Wait for process to exit
        set +e
        wait "${TIMMY_PID}"
        local exit_code=$?
        set -e

        TIMMY_PID=""

        # Check if shutdown was requested
        if [[ "${SHUTDOWN_REQUESTED}" -eq 1 ]]; then
            log_info "Shutdown in progress, not restarting"
            break
        fi

        # Handle exit code
        if [[ "${exit_code}" -eq 0 ]]; then
            log_success "Timmy exited cleanly (exit code 0)"
            log_info "Clean exit, not restarting"
            break
        fi

        # Non-zero exit = crash
        log_error "Timmy crashed with exit code ${exit_code}"
        record_crash "${exit_code}"
        ((restart_count++))

        log_info "Restarting in ${RESTART_DELAY} seconds..."
        sleep "${RESTART_DELAY}"
    done

    log_success "Watchdog exiting"
}

# Run main with all script arguments
main "$@"

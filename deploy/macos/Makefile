# DANEEL Native macOS Deployment
#
# Replaces Docker Desktop with native launchd services.
# Apple Silicon gets CoreML/Metal ONNX acceleration.
#
# Usage:
#   make install   — first-time setup (brew deps, qdrant, build, plists)
#   make start     — start all services
#   make stop      — stop all services
#   make restart   — stop + start
#   make status    — show service status
#   make logs      — tail all logs
#   make deploy    — rebuild daneel + restart
#   make deploy-web — rebuild daneel-web + restart
#   make cleanup   — run nightly maintenance now
#   make uninstall — stop services, remove plists and data
#
# Prerequisites:
#   - Homebrew
#   - Rust toolchain (rustup)
#   - Xcode command line tools

DANEEL_HOME   := $(HOME)/.daneel
DANEEL_REPO   := $(realpath $(dir $(lastword $(MAKEFILE_LIST)))/../..)
WEB_REPO      := $(realpath $(DANEEL_REPO)/../daneel-web)
PLIST_DIR     := $(HOME)/Library/LaunchAgents
QDRANT_VERSION := 1.16.3

# Qdrant binary URL for Apple Silicon
QDRANT_URL := https://github.com/qdrant/qdrant/releases/download/v$(QDRANT_VERSION)/qdrant-aarch64-apple-darwin.tar.gz

# LaunchAgent labels
LABEL_QDRANT  := ai.mollendorff.daneel-qdrant
LABEL_DANEEL  := ai.mollendorff.daneel
LABEL_WEB     := ai.mollendorff.daneel-web
LABEL_CLEANUP := ai.mollendorff.daneel-cleanup
LABEL_RESET   := ai.mollendorff.daneel-reset

.PHONY: install uninstall start stop restart status logs deploy deploy-web cleanup build build-web dirs deps qdrant plists

# ─────────────────────────────────────────────────────────────────────
# INSTALL
# ─────────────────────────────────────────────────────────────────────

install: deps dirs qdrant build build-web plists
	@echo ""
	@echo "✅ DANEEL installed. Run 'make start' to launch."
	@echo "   Logs:   $(DANEEL_HOME)/log/"
	@echo "   Data:   $(DANEEL_HOME)/data/"
	@echo "   Config: REDIS_URL=redis://127.0.0.1:6379"
	@echo "           QDRANT_URL=http://127.0.0.1:6334"

deps:
	@echo "── Installing dependencies ──"
	@command -v brew >/dev/null || (echo "ERROR: Homebrew required" && exit 1)
	@brew list redis >/dev/null 2>&1 || brew install redis
	@echo "Starting redis via brew services..."
	@brew services start redis >/dev/null 2>&1 || true

dirs:
	@mkdir -p $(DANEEL_HOME)/{bin,data/qdrant,data/redis,log,etc}

qdrant: dirs
	@if [ ! -f $(DANEEL_HOME)/bin/qdrant ]; then \
		echo "── Downloading Qdrant $(QDRANT_VERSION) ──"; \
		curl -sL $(QDRANT_URL) | tar xz -C $(DANEEL_HOME)/bin/; \
		chmod +x $(DANEEL_HOME)/bin/qdrant; \
		echo "Qdrant installed to $(DANEEL_HOME)/bin/qdrant"; \
	else \
		echo "Qdrant already installed"; \
	fi

build:
	@echo "── Building daneel ──"
	cd $(DANEEL_REPO) && RUSTFLAGS="-C target-cpu=native" cargo build --release
	cp $(DANEEL_REPO)/target/release/daneel $(DANEEL_HOME)/bin/daneel

build-web:
	@if [ -d "$(WEB_REPO)" ]; then \
		echo "── Building daneel-web ──"; \
		cd $(WEB_REPO) && RUSTFLAGS="-C target-cpu=native" cargo build --release; \
		cp $(WEB_REPO)/target/release/daneel-web $(DANEEL_HOME)/bin/daneel-web; \
		mkdir -p $(DANEEL_HOME)/frontend; \
		cp -r $(WEB_REPO)/frontend/dist $(DANEEL_HOME)/frontend/; \
	else \
		echo "WARN: daneel-web repo not found at $(WEB_REPO), skipping"; \
	fi

plists:
	@echo "── Installing config and LaunchAgents ──"
	@DANEEL_HOME="$(DANEEL_HOME)" envsubst < $(DANEEL_REPO)/deploy/macos/qdrant.yaml > $(DANEEL_HOME)/etc/qdrant.yaml
	@DANEEL_HOME="$(DANEEL_HOME)" envsubst < $(DANEEL_REPO)/deploy/macos/ai.mollendorff.daneel-qdrant.plist > $(PLIST_DIR)/$(LABEL_QDRANT).plist
	@DANEEL_HOME="$(DANEEL_HOME)" envsubst < $(DANEEL_REPO)/deploy/macos/ai.mollendorff.daneel.plist > $(PLIST_DIR)/$(LABEL_DANEEL).plist
	@DANEEL_HOME="$(DANEEL_HOME)" envsubst < $(DANEEL_REPO)/deploy/macos/ai.mollendorff.daneel-web.plist > $(PLIST_DIR)/$(LABEL_WEB).plist
	@DANEEL_HOME="$(DANEEL_HOME)" envsubst < $(DANEEL_REPO)/deploy/macos/ai.mollendorff.daneel-cleanup.plist > $(PLIST_DIR)/$(LABEL_CLEANUP).plist
	@DANEEL_HOME="$(DANEEL_HOME)" envsubst < $(DANEEL_REPO)/deploy/macos/ai.mollendorff.daneel-reset.plist > $(PLIST_DIR)/$(LABEL_RESET).plist
	@cp $(DANEEL_REPO)/deploy/macos/daily-reset.sh $(DANEEL_HOME)/bin/daily-reset.sh
	@chmod +x $(DANEEL_HOME)/bin/daily-reset.sh
	@echo "Config and plists installed"

# ─────────────────────────────────────────────────────────────────────
# LIFECYCLE
# ─────────────────────────────────────────────────────────────────────

start:
	@echo "── Starting DANEEL ──"
	@brew services start redis 2>/dev/null || true
	@launchctl load -w $(PLIST_DIR)/$(LABEL_QDRANT).plist 2>/dev/null || true
	@sleep 2
	@launchctl load -w $(PLIST_DIR)/$(LABEL_DANEEL).plist 2>/dev/null || true
	@launchctl load -w $(PLIST_DIR)/$(LABEL_WEB).plist 2>/dev/null || true
	@launchctl load -w $(PLIST_DIR)/$(LABEL_CLEANUP).plist 2>/dev/null || true
	@launchctl load -w $(PLIST_DIR)/$(LABEL_RESET).plist 2>/dev/null || true
	@echo "All services started. Dashboard: http://localhost:3000"

stop:
	@echo "── Stopping DANEEL ──"
	@launchctl unload $(PLIST_DIR)/$(LABEL_RESET).plist 2>/dev/null || true
	@launchctl unload $(PLIST_DIR)/$(LABEL_CLEANUP).plist 2>/dev/null || true
	@launchctl unload $(PLIST_DIR)/$(LABEL_WEB).plist 2>/dev/null || true
	@launchctl unload $(PLIST_DIR)/$(LABEL_DANEEL).plist 2>/dev/null || true
	@launchctl unload $(PLIST_DIR)/$(LABEL_QDRANT).plist 2>/dev/null || true
	@echo "All services stopped. Redis still running (brew services stop redis to stop)."

restart: stop
	@sleep 1
	@$(MAKE) start

status:
	@echo "── DANEEL Service Status ──"
	@echo "redis:      $$(brew services list | grep redis | awk '{print $$2}')"
	@echo "qdrant:     $$(launchctl list 2>/dev/null | grep $(LABEL_QDRANT) | awk '{print ($$1 == "-" ? "stopped" : "running (pid "$$1")")}')"
	@echo "daneel:     $$(launchctl list 2>/dev/null | grep '$(LABEL_DANEEL)$$' | awk '{print ($$1 == "-" ? "stopped" : "running (pid "$$1")")}')"
	@echo "daneel-web: $$(launchctl list 2>/dev/null | grep $(LABEL_WEB) | awk '{print ($$1 == "-" ? "stopped" : "running (pid "$$1")")}')"
	@echo "cleanup:    $$(launchctl list 2>/dev/null | grep $(LABEL_CLEANUP) | awk '{print "scheduled"}')"

logs:
	@tail -f $(DANEEL_HOME)/log/*.log

# ─────────────────────────────────────────────────────────────────────
# DEPLOY (rebuild + restart)
# ─────────────────────────────────────────────────────────────────────

deploy: build
	@launchctl unload $(PLIST_DIR)/$(LABEL_DANEEL).plist 2>/dev/null || true
	@sleep 1
	@launchctl load -w $(PLIST_DIR)/$(LABEL_DANEEL).plist
	@echo "daneel redeployed"

deploy-web: build-web
	@launchctl unload $(PLIST_DIR)/$(LABEL_WEB).plist 2>/dev/null || true
	@sleep 1
	@launchctl load -w $(PLIST_DIR)/$(LABEL_WEB).plist
	@echo "daneel-web redeployed"

# ─────────────────────────────────────────────────────────────────────
# MAINTENANCE
# ─────────────────────────────────────────────────────────────────────

cleanup:
	@echo "── Running maintenance ──"
	$(DANEEL_HOME)/bin/daneel --maintenance

# ─────────────────────────────────────────────────────────────────────
# UNINSTALL
# ─────────────────────────────────────────────────────────────────────

uninstall: stop
	@echo "── Uninstalling DANEEL ──"
	@rm -f $(PLIST_DIR)/$(LABEL_QDRANT).plist
	@rm -f $(PLIST_DIR)/$(LABEL_DANEEL).plist
	@rm -f $(PLIST_DIR)/$(LABEL_WEB).plist
	@rm -f $(PLIST_DIR)/$(LABEL_CLEANUP).plist
	@rm -f $(PLIST_DIR)/$(LABEL_RESET).plist
	@rm -rf $(DANEEL_HOME)
	@echo "DANEEL uninstalled. Redis left in place (brew services stop redis to remove)."

# DANEEL Makefile
#
# Cross-platform build targets:
#   make all      - Build and install (default)
#   make build    - Build release binary for current platform
#   make install  - Install binary to ~/bin/daneel
#   make clean    - Clean build artifacts
#
# Quality gates before commit:
#   make check    - Run all checks (fmt, clippy, test)
#   make fix      - Auto-fix formatting and lint issues
#
# Individual targets:
#   make fmt      - Check formatting
#   make clippy   - Run clippy lints
#   make test     - Run tests
#   make blog     - Preview blog locally

# OS Detection for cross-platform builds
UNAME_S := $(shell uname -s)
BINARY_NAME := daneel

ifeq ($(UNAME_S),Linux)
    TARGET := x86_64-unknown-linux-musl
    CARGO_FLAGS := --release --target $(TARGET)
    BINARY_PATH := target/$(TARGET)/release/$(BINARY_NAME)
else
    CARGO_FLAGS := --release
    BINARY_PATH := target/release/$(BINARY_NAME)
endif

.PHONY: all check fix fmt clippy test build blog clean install-hooks install

# Default: build and install
all: build install

# === Quality Gates ===

check: fmt clippy test
	@echo "✅ All checks passed"

fix:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --allow-staged
	@echo "✅ Auto-fixes applied"

# === Individual Checks ===

fmt:
	@echo "🔍 Checking formatting..."
	cargo fmt --all -- --check

clippy:
	@echo "🔍 Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

test:
	@echo "🧪 Running tests..."
	cargo test --all-features

# === Build ===

build:
	@echo "🔨 Building release..."
	cargo build $(CARGO_FLAGS)

install: build
	@echo "📦 Installing to ~/bin/daneel..."
	mkdir -p $(HOME)/bin
	cp $(BINARY_PATH) $(HOME)/bin/$(BINARY_NAME)
	@echo "✅ Installed to ~/bin/daneel"

# === Blog ===

blog:
	@echo "📝 Starting blog preview..."
	cd blog && hugo server -D

# === Setup ===

install-hooks:
	@echo "🔧 Installing git hooks..."
	cp scripts/pre-commit .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
	@echo "✅ Pre-commit hook installed"

# === Cleanup ===

clean:
	cargo clean
	@echo "✅ Cleaned build artifacts"

# === Help ===

help:
	@echo "DANEEL Makefile"
	@echo ""
	@echo "Cross-platform Build:"
	@echo "  make all          Build and install (default)"
	@echo "  make build        Build release binary for current platform"
	@echo "  make install      Install binary to ~/bin/daneel"
	@echo ""
	@echo "Quality Checks:"
	@echo "  make check        Run all quality checks (fmt, clippy, test)"
	@echo "  make fix          Auto-fix formatting and lint issues"
	@echo "  make fmt          Check code formatting"
	@echo "  make clippy       Run clippy lints"
	@echo "  make test         Run tests"
	@echo ""
	@echo "Other:"
	@echo "  make blog         Preview blog locally"
	@echo "  make install-hooks Install git pre-commit hook"
	@echo "  make clean        Clean build artifacts"

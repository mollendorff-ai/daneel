# DANEEL Makefile
#
# Quality gates before commit:
#   make check    - Run all checks (fmt, clippy, test)
#   make fix      - Auto-fix formatting and lint issues
#
# Individual targets:
#   make fmt      - Check formatting
#   make clippy   - Run clippy lints
#   make test     - Run tests
#   make build    - Build release binary
#   make blog     - Preview blog locally

.PHONY: all check fix fmt clippy test build blog clean install-hooks

# Default: run all checks
all: check

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
	cargo build --release

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
	@echo "Usage:"
	@echo "  make check        Run all quality checks (fmt, clippy, test)"
	@echo "  make fix          Auto-fix formatting and lint issues"
	@echo "  make fmt          Check code formatting"
	@echo "  make clippy       Run clippy lints"
	@echo "  make test         Run tests"
	@echo "  make build        Build release binary"
	@echo "  make blog         Preview blog locally"
	@echo "  make install-hooks Install git pre-commit hook"
	@echo "  make clean        Clean build artifacts"

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

# Native builds (glibc) - ONNX Runtime doesn't support musl
# Use build-musl target explicitly if static linking needed
CARGO_FLAGS := --release
BINARY_PATH := target/release/$(BINARY_NAME)

.PHONY: all check fix fmt clippy test coverage coverage-html build build-musl build-compressed dist blog clean install-hooks install paper paper-mermaid paper-plantuml paper-ascii paper-arxiv paper-clean

# Default: build and install
all: build install

# === Quality Gates ===

check: fmt clippy test coverage
	@echo "✅ All checks passed (including coverage gate)"

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

# === Coverage (ADR-049) ===
# 100% coverage on testable code or failure
# Requires: cargo install cargo-llvm-cov

COVERAGE_THRESHOLD := 100

coverage:
	@echo "📊 Running coverage analysis (100% required - ADR-049)..."
	cargo +nightly llvm-cov --fail-under-lines $(COVERAGE_THRESHOLD) --fail-under-functions $(COVERAGE_THRESHOLD)
	@echo "✅ Coverage meets 100% threshold"

coverage-html:
	@echo "📊 Generating HTML coverage report..."
	cargo +nightly llvm-cov --html
	@echo "✅ Report at target/llvm-cov/html/index.html"
	open target/llvm-cov/html/index.html

# === Build ===

build:
	@echo "🔨 Building release..."
	cargo build $(CARGO_FLAGS)

install: build
	@echo "📦 Installing to ~/bin/daneel..."
	mkdir -p $(HOME)/bin
	cp $(BINARY_PATH) $(HOME)/bin/$(BINARY_NAME)
	@echo "✅ Installed to ~/bin/daneel"

# === Multi-Arch Builds (ADR-050) ===

MUSL_TARGET := x86_64-unknown-linux-musl
DIST_DIR := dist

build-musl:
	@echo "🔨 Building MUSL static binary..."
	cargo build --release --target $(MUSL_TARGET)
	@echo "✅ Built target/$(MUSL_TARGET)/release/$(BINARY_NAME)"

build-compressed: build-musl
	@echo "🗜️ Compressing with UPX..."
	@which upx > /dev/null || (echo "❌ UPX not found. Install with: brew install upx" && exit 1)
	cp target/$(MUSL_TARGET)/release/$(BINARY_NAME) target/$(MUSL_TARGET)/release/$(BINARY_NAME)-compressed
	upx --best --lzma target/$(MUSL_TARGET)/release/$(BINARY_NAME)-compressed
	@echo "✅ Compressed binary at target/$(MUSL_TARGET)/release/$(BINARY_NAME)-compressed"

dist: build
	@echo "📦 Creating distribution archive..."
	mkdir -p $(DIST_DIR)
	@if [ "$(UNAME_S)" = "Linux" ]; then \
		tar -czvf $(DIST_DIR)/$(BINARY_NAME)-$(MUSL_TARGET).tar.gz -C target/$(MUSL_TARGET)/release $(BINARY_NAME); \
	else \
		tar -czvf $(DIST_DIR)/$(BINARY_NAME)-$(shell uname -m)-apple-darwin.tar.gz -C target/release $(BINARY_NAME); \
	fi
	@echo "✅ Archive created in $(DIST_DIR)/"

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

# === Paper Generation ===
# Regenerate DANEEL_PAPER.pdf from markdown source
# Requires: pandoc, xelatex (brew install pandoc; brew install --cask mactex)

PAPER_DIR := paper
ARXIV_DIR := $(PAPER_DIR)/arxiv
PAPER_MD := $(PAPER_DIR)/DANEEL_PAPER.md
PAPER_TEX := $(ARXIV_DIR)/DANEEL_PAPER.tex
PAPER_PDF := $(ARXIV_DIR)/DANEEL_PAPER.pdf

paper: $(PAPER_PDF)
	@echo "✅ Paper generated: $(PAPER_PDF)"

$(PAPER_PDF): $(PAPER_MD) $(ARXIV_DIR)/diagrams.tex scripts/patch-paper-tex.py
	@echo "📝 Converting markdown to LaTeX..."
	pandoc $(PAPER_MD) -o $(PAPER_TEX) \
		--standalone \
		--from markdown+raw_tex \
		--to latex \
		--pdf-engine=xelatex
	@echo "🔧 Patching LaTeX for TikZ diagrams..."
	@python3 scripts/patch-paper-tex.py $(PAPER_TEX)
	@echo "🔨 Compiling PDF with XeLaTeX (pass 1)..."
	cd $(ARXIV_DIR) && xelatex -interaction=nonstopmode DANEEL_PAPER.tex > /dev/null 2>&1 || true
	@echo "🔨 Compiling PDF with XeLaTeX (pass 2)..."
	cd $(ARXIV_DIR) && xelatex -interaction=nonstopmode DANEEL_PAPER.tex > /dev/null 2>&1
	@echo "📄 Opening PDF..."
	open $(PAPER_PDF)

paper-mermaid:
	@echo "📝 Building paper with mermaid diagrams..."
	python3 scripts/build-paper-mermaid.py
	@echo "✅ Paper generated with mermaid diagrams"

paper-plantuml:
	@echo "📝 Building paper with PlantUML diagrams..."
	python3 scripts/build-paper-plantuml.py
	@echo "✅ Paper generated with PlantUML diagrams"

paper-ascii:
	@echo "📝 Building paper with ASCII diagrams..."
	python3 scripts/build-paper-ascii.py
	@echo "✅ Paper generated with ASCII diagrams"

paper-arxiv:
	@echo "📦 Building arXiv submission package..."
	python3 scripts/build-arxiv-package.py
	@echo "✅ arXiv package ready"

paper-clean:
	@echo "🧹 Cleaning paper build artifacts..."
	rm -f $(ARXIV_DIR)/*.aux $(ARXIV_DIR)/*.log $(ARXIV_DIR)/*.out $(ARXIV_DIR)/*.toc
	rm -f $(ARXIV_DIR)/*.pdf $(ARXIV_DIR)/*_processed.md
	rm -rf $(ARXIV_DIR)/diagrams
	@echo "✅ Paper artifacts cleaned"

# === Help ===

help:
	@echo "DANEEL Makefile"
	@echo ""
	@echo "Cross-platform Build:"
	@echo "  make all             Build and install (default)"
	@echo "  make build           Build release binary for current platform"
	@echo "  make install         Install binary to ~/bin/daneel"
	@echo "  make build-musl      Build static MUSL binary (Linux x64)"
	@echo "  make build-compressed Build MUSL + UPX compression"
	@echo "  make dist            Create release archive for current platform"
	@echo ""
	@echo "Quality Checks:"
	@echo "  make check        Run all quality checks (fmt, clippy, test, coverage)"
	@echo "  make fix          Auto-fix formatting and lint issues"
	@echo "  make fmt          Check code formatting"
	@echo "  make clippy       Run clippy lints"
	@echo "  make test         Run tests"
	@echo "  make coverage     Check coverage >= 95% (fails if below)"
	@echo "  make coverage-html Generate HTML coverage report"
	@echo ""
	@echo "Paper Generation:"
	@echo "  make paper          Generate PDF with TikZ diagrams"
	@echo "  make paper-ascii    Generate PDF with ASCII diagrams (recommended)"
	@echo "  make paper-arxiv    Generate arXiv LaTeX submission package"
	@echo "  make paper-mermaid  Generate PDF with mermaid PNGs"
	@echo "  make paper-clean    Clean paper build artifacts"
	@echo ""
	@echo "Other:"
	@echo "  make blog         Preview blog locally"
	@echo "  make install-hooks Install git pre-commit hook"
	@echo "  make clean        Clean build artifacts"

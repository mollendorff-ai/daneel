# Publishing Instructions

## Current State (Dec 2025)

| File | Status |
|------|--------|
| `DANEEL_PAPER.md` | Source of truth (edit this) |
| `arxiv/DANEEL_PAPER.tex` | STALE - needs regeneration |
| `arxiv/DANEEL_PAPER.pdf` | STALE - needs regeneration |
| `arxiv/diagrams.tex` | Ready - 8 TikZ diagrams |
| `arxiv/SUBMISSION_INFO.md` | arXiv submission metadata |

## Workflow

### 1. Edit Markdown Source

```bash
vim paper/DANEEL_PAPER.md
# or
code paper/DANEEL_PAPER.md
```

### 2. Generate LaTeX

```bash
cd /Users/rex/src/royalbit/daneel
pandoc paper/DANEEL_PAPER.md \
  -o paper/arxiv/DANEEL_PAPER.tex \
  --standalone \
  --from markdown+raw_tex \
  --to latex
```

### 3. Post-Process LaTeX (Manual Steps)

The generated `.tex` needs manual fixes for arXiv:

**a) Add diagram package to preamble:**
```latex
\usepackage{tikz}
\usetikzlibrary{shapes.multipart, positioning, arrows.meta, fit, backgrounds}
\input{diagrams}
```

**b) Replace Mermaid blocks with TikZ commands:**

| Mermaid Location | Replace With |
|-----------------|--------------|
| Game Theory Matrix (§1.2) | `\payoffmatrix` |
| Brain vs TMI (§7.1) | `\brainvstmi` |
| Wetware vs Software (§7.3) | `\wetwarevssoftware` |
| DANEEL Architecture (§10.2) | `\daneelarchitecture` |

Additional diagrams available:
- `\bridgescenario` - LLM bridge diagram
- `\scenariocomparison` - Expected utility bar chart
- `\montecarlodist` - Monte Carlo distribution
- `\thebox` - Asimov's Laws architecture

**c) Fix tables if needed** (pandoc usually handles these well)

### 4. Compile PDF

```bash
cd paper/arxiv
xelatex DANEEL_PAPER.tex
xelatex DANEEL_PAPER.tex  # Run twice for cross-references
```

Or one-liner:
```bash
cd /Users/rex/src/royalbit/daneel/paper/arxiv && \
xelatex -interaction=nonstopmode DANEEL_PAPER.tex && \
xelatex -interaction=nonstopmode DANEEL_PAPER.tex
```

### 5. View PDF

```bash
open paper/arxiv/DANEEL_PAPER.pdf
```

## Tools Required

```bash
brew install pandoc
# xelatex comes with texlive (brew install --cask mactex)
```

## arXiv Submission

**Files to upload:**
1. `DANEEL_PAPER.tex` - Main source
2. `diagrams.tex` - TikZ definitions
3. Any `.bib` file if using BibTeX

**Categories:**
- Primary: cs.AI (Artificial Intelligence)
- Secondary: cs.CY (Computers and Society)

**See:** `arxiv/SUBMISSION_INFO.md` for full metadata

## LinkedIn PDF

For LinkedIn, the arXiv PDF works directly. If you want a shorter version:

```bash
# Extract first N pages for preview
pdftk paper/arxiv/DANEEL_PAPER.pdf cat 1-5 output paper/DANEEL_PREVIEW.pdf
```

## Validation Before Submission

```bash
# Check all links in paper
/Users/rex/src/royalbit/ref-tools/target/release/ref-tools check-links paper/DANEEL_PAPER.md

# Verify references.yaml entries
/Users/rex/src/royalbit/ref-tools/target/release/ref-tools verify-refs references.yaml
```

## Quick Reference: Full Rebuild

```bash
cd /Users/rex/src/royalbit/daneel && \
pandoc paper/DANEEL_PAPER.md -o paper/arxiv/DANEEL_PAPER.tex --standalone --from markdown+raw_tex && \
cd paper/arxiv && \
xelatex -interaction=nonstopmode DANEEL_PAPER.tex && \
xelatex -interaction=nonstopmode DANEEL_PAPER.tex && \
open DANEEL_PAPER.pdf
```

**Note:** This quick rebuild doesn't include the manual diagram replacement step. For arXiv submission, follow the full workflow above.

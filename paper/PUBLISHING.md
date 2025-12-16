# Publishing Instructions

## Editing the Paper

Edit the markdown source:
```bash
# Use your preferred editor
vim paper/DANEEL_PAPER.md
# or
code paper/DANEEL_PAPER.md
```

## Generating LaTeX

After editing, regenerate LaTeX:
```bash
cd /Users/rex/src/royalbit/daneel
pandoc paper/DANEEL_PAPER.md -o paper/arxiv/DANEEL_PAPER.tex --standalone
```

## Generating PDF

Compile with xelatex (handles Unicode):
```bash
cd paper/arxiv
xelatex DANEEL_PAPER.tex
xelatex DANEEL_PAPER.tex  # Run twice for cross-references
```

Or one-liner from repo root:
```bash
cd /Users/rex/src/royalbit/daneel && \
pandoc paper/DANEEL_PAPER.md -o paper/arxiv/DANEEL_PAPER.tex --standalone && \
cd paper/arxiv && xelatex -interaction=nonstopmode DANEEL_PAPER.tex
```

## Viewing PDF

```bash
open paper/arxiv/DANEEL_PAPER.pdf
```

## Quick Rebuild Script

```bash
# From repo root
make pdf  # If we add a Makefile, or just:

cd /Users/rex/src/royalbit/daneel/paper/arxiv && \
pandoc ../DANEEL_PAPER.md -o DANEEL_PAPER.tex --standalone && \
xelatex -interaction=nonstopmode DANEEL_PAPER.tex && \
open DANEEL_PAPER.pdf
```

## Tools Required

- `pandoc` - Markdown to LaTeX conversion
- `xelatex` - LaTeX to PDF (part of texlive)

Install if missing:
```bash
brew install pandoc
# xelatex comes with texlive (already installed)
```

## arXiv Submission

1. Upload `paper/arxiv/DANEEL_PAPER.tex`
2. Categories: cs.AI (primary), cs.CY (cross-list)
3. Add any figures as separate files if needed

## Files

| File | Purpose |
|------|---------|
| `DANEEL_PAPER.md` | Source (edit this) |
| `arxiv/DANEEL_PAPER.tex` | Generated LaTeX |
| `arxiv/DANEEL_PAPER.pdf` | Generated PDF (gitignored) |

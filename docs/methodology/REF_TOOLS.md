# ref-tools: AI-Friendly Web Research

## The Problem

LLMs and AI agents need to verify sources, but traditional web scraping tools fail:

1. **curl/wget get blocked by bot protection** (403, 999, Cloudflare challenges)
2. **WebFetch tools often fail** on protected sites (news sites, paywalls, anti-bot systems)
3. **AI hallucinations require source verification** - you can't trust claimed statistics without checking
4. **LLMs need structured data** - raw HTML is token-inefficient and hard to parse

The fundamental issue: Most websites actively block automated access, but AI research requires automated verification of hundreds of URLs.

## The Solution: ref-tools

ref-tools is a **compact-JSON web browser designed for LLM consumption**.

**How it works:**
- Uses real browser automation (Chromium) to bypass bot protection
- Extracts structured content (headings, sections, links, text)
- Outputs minimal JSON optimized for LLM token consumption
- Handles JavaScript-heavy sites, paywalls, and anti-bot systems

**What makes it different:**
- **AI-first design**: Output is structured for LLM reasoning, not human reading
- **Bot protection bypass**: Real browser context defeats 403/999 blocks
- **Batch verification**: Parallel browser tabs for verifying hundreds of URLs
- **Status tracking**: Detects OK/paywall/404/error states automatically

## Usage with AI Agents

### Basic Fetch

```bash
ref-tools fetch "https://example.com/article"
```

### Output Format

The output is compact JSON with structured sections:

```json
{
  "url": "https://www.example.com",
  "status": "ok",
  "title": "Example Domain",
  "sections": [
    {
      "level": 1,
      "heading": "Example Domain",
      "content": "This domain is for use in documentation examples without needing permission. Avoid use in operations.\n\nLearn more"
    }
  ],
  "links": [
    {
      "text": "Learn more",
      "url": "https://iana.org/domains/example"
    }
  ],
  "chars": 127
}
```

**Field meanings:**
- `status`: `ok`, `paywall`, `404`, `error`, `timeout`
- `sections`: Hierarchical content structure (level 1-6 headings)
- `links`: Extracted hyperlinks with anchor text
- `chars`: Character count (for token estimation)

### Advanced Options

```bash
# Fetch multiple URLs in parallel (4 browser tabs)
ref-tools fetch "https://site1.com" "https://site2.com" "https://site3.com" -p 4

# Increase timeout for slow sites (30 seconds)
ref-tools fetch "https://slow-site.com" --timeout 30000

# Raw extraction (skip content cleaning)
ref-tools fetch "https://site.com" --raw

# Authenticated fetches (requires cookies file)
ref-tools fetch "https://paywalled-site.com" --cookies cookies.txt
```

## The references.yaml Workflow

### Purpose

The `references.yaml` file serves as a **central registry of all cited URLs** across the DANEEL project:

- **Verification status tracking**: Which URLs are alive, paywalled, or dead?
- **Category tagging**: Group URLs by topic (academic, tmi, neuromorphic, etc.)
- **Citation mapping**: Which documents cite which URLs?
- **Audit trail**: When was each URL last verified?

This enables **reproducible research** - anyone can re-verify all our sources.

### Structure

```yaml
meta:
  created: 2025-12-15
  last_verified: '2025-12-18T12:00:00+00:00'
  tool: ref-tools v1.0.0
  total_links: 89

references:
  - url: https://arxiv.org/abs/2405.02370
    title: Neuromorphic Correlates of Artificial Consciousness
    categories:
      - consciousness
      - arxiv
      - academic
    cited_in:
      - research/neuromorphic_landscape_2025.md
    status: ok
    verified: 2025-12-15 19:42:30.495729+00:00

  - url: https://ailabwatch.substack.com/p/xais-new-safety-framework-is-dreadful
    title: https://ailabwatch.substack.com/p/xais-new-safety-framework-is-dreadful
    categories:
      - general
    cited_in:
      - paper/DANEEL_PAPER.md
    status: paywall
    verified: 2025-12-15 19:42:21.808049+00:00
    notes: Paywall detected
```

**Key fields:**
- `url`: Canonical URL (normalized)
- `title`: Extracted page title (or URL if extraction failed)
- `categories`: Topic tags (multiple allowed)
- `cited_in`: List of markdown files that reference this URL
- `status`: `ok`, `paywall`, `404`, `error`
- `verified`: Timestamp of last successful verification
- `notes`: Optional human/automated notes

### Workflow

The typical ref-tools workflow for maintaining references:

#### 1. Scan for URLs

```bash
ref-tools scan "**/*.md" --output references.yaml --merge
```

This scans all markdown files for URLs and:
- Extracts all HTTP/HTTPS links
- Tracks which file cited each URL
- Merges with existing `references.yaml` (preserves verification status)

#### 2. Verify All References

```bash
ref-tools verify-refs references.yaml
```

This opens browser tabs to verify each URL:
- Updates `status` field (ok/paywall/404/error)
- Extracts page titles
- Records verification timestamp
- Writes results back to `references.yaml`

**Parallel verification:**
```bash
# Use 8 parallel browser tabs for faster verification
ref-tools verify-refs references.yaml -p 8
```

**Category filtering:**
```bash
# Only verify academic papers
ref-tools verify-refs references.yaml -c academic -c arxiv
```

**Dry run (no file changes):**
```bash
ref-tools verify-refs references.yaml --dry-run
```

#### 3. Fetch Content for Deep Verification

When you need to verify specific claims (like statistics):

```bash
# Get the actual content
ref-tools fetch "https://www.software.com/reports/state-of-software-development-2021"
```

Then analyze the JSON output to verify the claimed statistic appears in the source.

## Why This Matters for DANEEL

### Case Study: Open Source Dominance Model

In developing [ADR-029: Open Source Dominance Strategy](docs/adr/ADR-029-open-source-dominance-strategy.md), we needed to verify claims about developer productivity. Initial research cited several sources, but ref-tools verification revealed critical errors:

**What we found:**

| Source | Claim | ref-tools Result | Action |
|--------|-------|------------------|--------|
| Software.com 2021 | "11% coding time" | Verified: 52 min/day average across 250K developers | Keep |
| Atlassian 2025 | "16% heads-down work" | Verified: Exact statistic found in source | Keep |
| Clockwise 2022 | "Low coding time" | **Metric conflation**: Measured meeting time, NOT coding time | Remove |
| HBR 2017 | "Developer productivity" | **URL does not exist**: 404 error, unverifiable | Remove |

**Impact:**
- Removed 2 sources that would have undermined credibility
- Weighted average calculation now based on 253,500 verified samples
- Research foundation is **reproducible** - anyone can re-run verification

This is the difference between:
- "Some sources claim developers only code 11% of the time" (unreliable)
- "Analysis of 253,500 developers shows 11.1% coding time (verified via ref-tools)" (credible)

### The Broader Principle

DANEEL is about **trustworthy AI alignment**. We cannot build trust while citing unverifiable sources. ref-tools enables:

1. **Falsifiability**: All claims can be independently verified
2. **Audit trail**: Verification timestamps and status tracking
3. **Error detection**: Catches dead links, paywalls, metric conflation
4. **Reproducibility**: Anyone can re-run `verify-refs` and get the same results

If we're wrong about something, ref-tools helps others prove it and fix it.

## Other ref-tools Commands

### Check Links

Quick health check for URLs without full verification:

```bash
# Check single URL
ref-tools check-links "https://example.com"

# Check all links in a markdown file
ref-tools check-links paper/DANEEL_PAPER.md
```

### Initialize references.yaml

Create a template file:

```bash
ref-tools init
```

### PDF Extraction

Extract text from PDF files to structured JSON:

```bash
ref-tools pdf paper.pdf
```

Output includes:
- Text content per page
- Page count
- Character count
- Structured JSON for LLM analysis

### Refresh Data

Extract live data from URLs (market sizes, pricing, statistics):

```bash
ref-tools refresh-data references.yaml
```

This is useful for tracking time-sensitive information that changes frequently.

## Future: Open Sourcing ref-tools

We're considering open-sourcing ref-tools because:

1. **AI agents need reliable web access** - this is a general problem
2. **LLM verification is critical** - as AI generates more content, source verification becomes essential
3. **Community benefit** - many researchers face bot protection issues

Potential timeline: After DANEEL initial release (Q1 2026)

License considerations: MIT or Apache 2.0 (permissive, unlike DANEEL's AGPL)

## Installation

ref-tools is currently RoyalBit proprietary software.

**Requirements:**
- Rust 1.70+
- Chromium/Chrome installed
- Network access

**Build from source:**
```bash
cd ref-tools
cargo build --release
cargo install --path .
```

## Technical Details

**Architecture:**
- Written in Rust for performance and safety
- Uses headless Chrome via CDP (Chrome DevTools Protocol)
- Parallel browser tab management for batch operations
- Automatic retry logic with exponential backoff

**Why not puppeteer/playwright?**
- ref-tools is a single binary (no Node.js runtime)
- Faster startup and lower memory footprint
- JSON output optimized for LLM token efficiency
- Built-in references.yaml workflow

## Contributing

ref-tools development currently happens internally at RoyalBit.

If you encounter issues with specific URLs or have feature requests, contact: louis@royalbit.ai

---

**Version:** 1.0.0
**Last Updated:** 2025-12-19
**Maintainer:** RoyalBit Technologies

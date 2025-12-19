+++
date = '2025-12-19T20:00:00-05:00'
draft = false
title = 'ref-tools: Teaching AI to Verify Sources'
description = 'How we built a web browser for AI agents and used it to catch our own errors.'
tags = ['ref-tools', 'verification', 'methodology', 'anti-bot', 'llm-tools']
+++

## The Problem Nobody Talks About

Large language models hallucinate statistics with alarming confidence. They'll cite "studies" that never existed, invent percentages that sound plausible, and generate authoritative-looking references to papers that were never published.

The standard AI agent solution? Add a web browsing tool. Give the LLM `curl` or `wget`. Let it fetch sources and verify claims.

In theory, this works. In practice, you get blocked:

```
HTTP/1.1 403 Forbidden
```

```
Error 999: Request denied
```

```
Just a moment...
Checking your browser before accessing...
```

Cloudflare, bot protection services, paywalls, and anti-scraping measures treat AI agents exactly like they should: as automated bots. Which they are.

The gap isn't just technical. It's philosophical. AI agents don't need to **cite** sources. They need to **verify** them. And verification requires actually reading the content, not just collecting URLs.

## How We Caught Ourselves Lying

While researching developer productivity for DANEEL, we aggregated statistics from multiple sources. The claim looked solid:

> "Based on surveys of 337,100 developers across four major studies..."

Impressive number. Real sources. Confident presentation. We almost published it.

Then we ran `ref-tools verify-refs` on our own draft.

Two of our four sources were garbage:

**Clockwise State of Time Report**: We cited "time wasted in meetings" as a developer productivity metric. Wrong. The report tracks *knowledge workers*, not specifically developers. The denominator was polluted.

**Harvard Business Review 2017**: We referenced an HBR article that doesn't exist. The LLM hallucinated a study about meeting effectiveness, attached a plausible-sounding title, and generated a URL that returned 404.

We weren't lying intentionally. We were letting AI do research without verification. The same problem we're trying to solve.

## What ref-tools Actually Does

ref-tools is RoyalBit's proprietary web browser built specifically for AI agents. It's not a scraper. It's a real Chromium browser with automation.

**Core capabilities**:

1. **Bot Protection Bypass**: Uses real browser fingerprints, handles JavaScript challenges, waits for dynamic content. Cloudflare sees a browser, not a bot.

2. **Structured Output**: Extracts content and returns clean JSON. No HTML parsing. No DOM traversal. Just the article text, metadata, and publication info.

3. **Reference Tracking**: Maintains `references.yaml` with verification status, fetch timestamps, and content hashes. Every source is auditable.

4. **LLM-Optimized**: Output is designed for context windows. Summaries for quick checks. Full text for deep analysis. Metadata for attribution.

**What it bypasses**:

- Cloudflare "checking your browser" challenges
- Rate limiting (respects robots.txt, but doesn't get flagged)
- Paywalls (when combined with institutional access)
- JavaScript-heavy SPAs that break traditional scrapers
- Anti-bot fingerprinting

## The Workflow

Integrating ref-tools into our research process took three commands:

```bash
# 1. Scan all markdown files for URLs
ref-tools scan "**/*.md"

# 2. Verify every reference and update tracking
ref-tools verify-refs references.yaml

# 3. Fetch specific content for fact-checking
ref-tools fetch "https://www.software.com/reports/state-of-software-development-2021"
```

The `scan` command builds an inventory. The `verify-refs` command checks each URL, fetches content, and marks status. The `fetch` command gives us JSON we can feed directly to Claude for analysis.

Example output:

```json
{
  "url": "https://www.software.com/reports/state-of-software-development-2021",
  "status": "verified",
  "title": "The 2021 State of Software Development Report",
  "published": "2021-03-15",
  "content": "...",
  "verified_at": "2025-12-19T18:23:45Z",
  "hash": "sha256:a3f8c9..."
}
```

When a source fails verification, we see it immediately:

```json
{
  "url": "https://hbr.org/2017/meeting-effectiveness",
  "status": "failed",
  "error": "404 Not Found",
  "verified_at": "2025-12-19T18:24:12Z"
}
```

No more broken links in published posts. No more hallucinated sources. No more wrong metrics.

## Why Verification Matters

DANEEL isn't just a side project. We're building an AI agent that makes decisions about how developers work. If our research is wrong, our recommendations are wrong.

**Reproducible research** means anyone can re-run our verification:

```bash
ref-tools verify-refs references.yaml --force
```

Every source we cited is tracked. Every claim has a verified URL. Every statistic traces back to original data.

**Catching errors before publication** saved us from embarrassment. The Clockwise/HBR mistakes would have undermined our credibility. ref-tools caught them in draft.

**Trust through transparency** means we publish our methodology. Our [REF_TOOLS.md](https://github.com/royalbit/daneel/blob/main/docs/methodology/REF_TOOLS.md) document explains exactly how we verify sources. Our references.yaml file is in the repo. Anyone can audit our work.

This isn't just good practice. It's necessary. If we're asking developers to trust an AI agent with their productivity data, we need to prove we're not making things up.

## Will We Open Source It?

We're considering it.

**Why we might**:

- Bot protection is everyone's problem. The AI research community needs better tools.
- Open sourcing could establish RoyalBit as a methodology leader (see [ADR-029](https://github.com/royalbit/daneel/blob/main/docs/adr/ADR-029-open-source-dominance-strategy.md)).
- More users = better testing = better tool.

**Why we might not**:

- ref-tools contains anti-detection techniques that could be abused.
- Maintaining an open source browser automation tool is a support burden.
- We're a small team. Q1 2026 is optimistic.

For now, we're documenting our methodology publicly while keeping the implementation private. If there's demand, we'll revisit.

## The Meta Lesson

We built ref-tools to verify other people's research. It caught errors in our own.

This is the point. AI agents aren't perfect. LLMs hallucinate. Developers make mistakes. The solution isn't better prompts or bigger models.

The solution is verification infrastructure.

If you're building AI agents that do research, make them verify their sources. If you're publishing AI-generated analysis, run it through a fact-checking pass. If you're aggregating statistics, actually read the original reports.

Trust, but verify. Especially when the "trust" part is a stochastic parrot.

## Sources

- [ref-tools Methodology](https://github.com/royalbit/daneel/blob/main/docs/methodology/REF_TOOLS.md)
- [Software.com 2021 Report](https://www.software.com/reports/state-of-software-development-2021) - Verified
- [Atlassian State of Teams 2025](https://www.atlassian.com/blog/state-of-teams-2025) - Verified
- [ADR-029: Open Source Dominance Strategy](https://github.com/royalbit/daneel/blob/main/docs/adr/ADR-029-open-source-dominance-strategy.md)

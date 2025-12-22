#!/usr/bin/env python3
"""
Build DANEEL paper PDF with ASCII diagrams.
Replaces mermaid blocks with ASCII art that won't break across pages.
"""

import re
import os
import subprocess
import sys

PAPER_MD = 'paper/DANEEL_PAPER.md'
OUTPUT_DIR = 'paper/arxiv'
TEMPLATE = f'{OUTPUT_DIR}/template.tex'

# Paper metadata
TITLE = 'DANEEL: A Human-Like Cognitive Architecture for Aligned Artificial Superintelligence'
AUTHOR = 'Luis Cezar Menezes Tavares de Lacerda (Louis C. Tavares)'
DATE = 'December 2025'

# ASCII diagrams (wrapped in raw LaTeX minipage to prevent page breaks)
DIAGRAMS = [
    # Diagram 1: Game Theory Matrix
    r'''
\begin{figure}[H]
\centering
\textbf{Figure 1: AI Development Game Theory Matrix}
\begin{verbatim}
                     ALL OTHERS
                 Hold Line    Defect
              +------------+------------+
   Hold Line  |   SAFE     | DOMINATED  |
YOU           |  (ideal)   | (you lose) |
              +------------+------------+
   Defect     |   FIRST    |  RACE TO   |
              |   MOVER    |   BOTTOM   |
              +------------+------------+
\end{verbatim}
\end{figure}
''',

    # Diagram 2: Brain vs TMI
    r'''
\begin{figure}[H]
\centering
\textbf{Figure 2: Brain Hardware vs TMI Software Capacity}
\begin{verbatim}
+================================================================+
|  BRAIN (Hardware) - 86B neurons, 100T synapses, ~2.5 PB total  |
+================================================================+
|  Cerebellum: 69B neurons (80%) - Motor, NOT thought            |
|  Brainstem: ~500M (0.5%) - Autonomic (heart, breathing)        |
|  Spinal: ~1B (1%) - Body sensation routing                     |
|  >>> 82.5% of brain is NOT for cognition <<<                   |
+----------------------------------------------------------------+
|  +----------------------------------------------------------+  |
|  |  TMI / THOUGHT MACHINE (Software) - 17.5% of brain       |  |
|  +----------------------------------------------------------+  |
|  |  Cerebral cortex: 16B neurons (18.6%)                    |  |
|  |  Prefrontal cortex: ~2.5B - Executive, planning          |  |
|  |  Hippocampus/limbic: ~1B - Memory, emotion               |  |
|  |  Raw: ~0.44 PB --> Abstracted: ~500 GB (1000x compress)  |  |
|  +----------------------------------------------------------+  |
+================================================================+
\end{verbatim}
\end{figure}
''',

    # Diagram 3: Wetware vs Software
    r'''
\begin{figure}[H]
\centering
\textbf{Figure 3: Wetware vs Software - Medium-Independent Patterns}
\begin{verbatim}
WETWARE (Human Brain)              SOFTWARE (TMI Patterns)
+---------------------------+      +--------------------------------+
| 5s intervention window    | ---> | ~100 cycles per intervention   |
| (neurotransmitter rates)  |      | (RATIO, medium-independent)    |
+---------------------------+      +--------------------------------+
| 50ms attention cycle      | ---> | Competing parallel streams     |
| (synaptic plasticity)     |      | (PATTERN, medium-independent)  |
+---------------------------+      +--------------------------------+
| Sleep consolidation       | ---> | Salience-weighted selection    |
| (glymphatic system)       |      | (ALGORITHM, medium-independent)|
+---------------------------+      +--------------------------------+
\end{verbatim}
\end{figure}
''',

    # Diagram 4: DANEEL Architecture
    r'''
\begin{figure}[H]
\centering
\textbf{Figure 4: DANEEL Architecture - LLM as External Tool}
\begin{verbatim}
+============================================================+
|           DANEEL TMI Core (stores ALL experiences)         |
+============================================================+
|  Memory Windows          Salience           Continuity     |
|  (complete thought       (emotional         (persistent    |
|   history)               weights)           'I')           |
+============================================================+
                              ^
                              | gRPC
                              v
+============================================================+
|                   Tool Interface (gRPC)                    |
+============================================================+
|  LLM Tool:              LLM Tool:           Other tools:   |
|  thought -> language    language -> thought  web, files... |
+============================================================+
\end{verbatim}
\end{figure}
''',
]

# Abstract text
ABSTRACT = """We present DANEEL, a cognitive architecture implementing Augusto Cury's Theory of Multifocal Intelligence (TMI) with persistent non-semantic memory, emotional structuring via Russell's circumplex model, and an immutable ethical core (Asimov's Laws). Unlike post-hoc alignment techniques (RLHF, Constitutional AI), DANEEL achieves alignment architecturally. Core thesis: Architecture produces psychology. Structure determines values."""


def extract_mermaid_blocks(content):
    """Find all mermaid code blocks and their positions."""
    pattern = r'```mermaid\n(.*?)```'
    matches = list(re.finditer(pattern, content, re.DOTALL))
    return matches


def wrap_bare_urls(content):
    """Wrap bare URLs in \\url{} for proper LaTeX line breaking."""
    # Match bare URLs not already in markdown links or \url{}
    # Pattern: URLs starting with http(s):// not preceded by ( or { or "
    url_pattern = r'(?<![(\[{"])(?<!\]\()(?<!\\url\{)(https?://[^\s<>\)\]\}]+)'

    def replace_url(match):
        url = match.group(1)
        # Clean trailing punctuation that shouldn't be part of URL
        trailing = ''
        while url and url[-1] in '.,;:!?)':
            trailing = url[-1] + trailing
            url = url[:-1]
        return f'\\url{{{url}}}{trailing}'

    return re.sub(url_pattern, replace_url, content)


def preprocess_content(content):
    """Clean up markdown for better LaTeX conversion."""
    # Remove the title line
    content = re.sub(r'^# DANEEL:.*?\n', '', content)
    # Remove author block at top
    content = re.sub(r'^\*\*Luis Cezar.*?---\n', '', content, flags=re.DOTALL)
    # Remove Abstract section (we put it on title page)
    content = re.sub(r'^## Abstract\n\n.*?(?=\n##|\Z)', '', content, flags=re.DOTALL | re.MULTILINE)
    # Wrap bare URLs for LaTeX line breaking
    content = wrap_bare_urls(content)
    return content


def main():
    print('Building DANEEL paper with ASCII diagrams...\n')

    # Read source
    with open(PAPER_MD, 'r') as f:
        content = f.read()

    # Extract mermaid blocks
    matches = extract_mermaid_blocks(content)
    print(f'Found {len(matches)} mermaid diagrams to replace with ASCII')

    # Replace with ASCII diagrams
    processed_content = content
    offset = 0

    for i, match in enumerate(matches):
        diagram_num = i + 1
        if i < len(DIAGRAMS):
            ascii_diagram = DIAGRAMS[i]
            print(f'  [{diagram_num}/{len(matches)}] Replacing with ASCII Figure {diagram_num}')

            start = match.start() + offset
            end = match.end() + offset
            processed_content = processed_content[:start] + ascii_diagram + processed_content[end:]
            offset += len(ascii_diagram) - (match.end() - match.start())
        else:
            print(f'  [{diagram_num}/{len(matches)}] No ASCII defined, removing')
            start = match.start() + offset
            end = match.end() + offset
            processed_content = processed_content[:start] + '' + processed_content[end:]
            offset -= (match.end() - match.start())

    # Preprocess content
    processed_content = preprocess_content(processed_content)

    # Write processed markdown
    processed_md = f'{OUTPUT_DIR}/DANEEL_PAPER_processed.md'
    with open(processed_md, 'w') as f:
        f.write(processed_content)
    print(f'\nWrote processed markdown: {processed_md}')

    # Generate PDF with pandoc
    print('\nGenerating PDF with pandoc + custom template...')
    pdf_path = f'{OUTPUT_DIR}/DANEEL_PAPER.pdf'
    abs_output_dir = os.path.abspath(OUTPUT_DIR)

    result = subprocess.run([
        'pandoc', 'DANEEL_PAPER_processed.md',
        '-o', 'DANEEL_PAPER.pdf',
        '--template=template.tex',
        '--pdf-engine=xelatex',
        '--from=markdown+tex_math_dollars+pipe_tables+raw_tex',
        '--toc',
        '--toc-depth=3',
        '--number-sections',
        '-V', f'title={TITLE}',
        '-V', f'author={AUTHOR}',
        '-V', f'date={DATE}',
        '-V', f'abstract={ABSTRACT}',
    ], capture_output=True, text=True, cwd=abs_output_dir)

    if result.returncode != 0:
        print(f'Pandoc failed:\n{result.stderr}')
        sys.exit(1)

    print(f'\nPDF generated: {pdf_path}')

    # Open it
    if sys.platform == 'darwin':
        subprocess.run(['open', pdf_path])

    print('Done!')


if __name__ == '__main__':
    main()

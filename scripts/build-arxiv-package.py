#!/usr/bin/env python3
"""
Build arXiv submission package for DANEEL paper.
Generates standalone LaTeX source that compiles on arXiv's system.
"""

import re
import os
import subprocess
import sys
import shutil
import zipfile

PAPER_MD = 'paper/DANEEL_PAPER.md'
OUTPUT_DIR = 'paper/arxiv'
ARXIV_DIR = f'{OUTPUT_DIR}/arxiv-submission'

# Paper metadata
TITLE = 'DANEEL: A Human-Like Cognitive Architecture for Aligned Artificial Superintelligence'
AUTHOR = 'Louis C. Tavares'
DATE = 'December 2025'

# ASCII diagrams (same as build-paper-ascii.py)
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

ABSTRACT = """We present DANEEL, a cognitive architecture implementing Augusto Cury's Theory of Multifocal Intelligence (TMI) with persistent non-semantic memory, emotional structuring via Russell's circumplex model, and an immutable ethical core (Asimov's Laws). Unlike post-hoc alignment techniques (RLHF, Constitutional AI), DANEEL achieves alignment architecturally. Core thesis: Architecture produces psychology. Structure determines values."""


def extract_mermaid_blocks(content):
    """Find all mermaid code blocks and their positions."""
    pattern = r'```mermaid\n(.*?)```'
    matches = list(re.finditer(pattern, content, re.DOTALL))
    return matches


def wrap_bare_urls(content):
    """Wrap bare URLs in \\url{} for proper LaTeX line breaking."""
    url_pattern = r'(?<![(\[{"])(?<!\]\()(?<!\\url\{)(https?://[^\s<>\)\]\}]+)'

    def replace_url(match):
        url = match.group(1)
        trailing = ''
        while url and url[-1] in '.,;:!?)':
            trailing = url[-1] + trailing
            url = url[:-1]
        return f'\\url{{{url}}}{trailing}'

    return re.sub(url_pattern, replace_url, content)


def convert_unicode_to_latex(content):
    """Convert Unicode symbols to LaTeX commands for pdflatex compatibility."""
    replacements = {
        # Greek letters
        'α': r'$\alpha$',
        'β': r'$\beta$',
        'γ': r'$\gamma$',
        'δ': r'$\delta$',
        'ε': r'$\varepsilon$',
        'ζ': r'$\zeta$',
        'η': r'$\eta$',
        'θ': r'$\theta$',
        'ι': r'$\iota$',
        'κ': r'$\kappa$',
        'λ': r'$\lambda$',
        'μ': r'$\mu$',
        'ν': r'$\nu$',
        'ξ': r'$\xi$',
        'ο': r'$o$',
        'π': r'$\pi$',
        'ρ': r'$\rho$',
        'σ': r'$\sigma$',
        'τ': r'$\tau$',
        'υ': r'$\upsilon$',
        'φ': r'$\phi$',
        'χ': r'$\chi$',
        'ψ': r'$\psi$',
        'ω': r'$\omega$',
        'Σ': r'$\Sigma$',
        'Δ': r'$\Delta$',
        'Ω': r'$\Omega$',
        # Math symbols
        '≈': r'$\approx$',
        '≠': r'$\neq$',
        '≤': r'$\leq$',
        '≥': r'$\geq$',
        '→': r'$\rightarrow$',
        '←': r'$\leftarrow$',
        '↔': r'$\leftrightarrow$',
        '×': r'$\times$',
        '÷': r'$\div$',
        '±': r'$\pm$',
        '∞': r'$\infty$',
        '∑': r'$\sum$',
        '∏': r'$\prod$',
        '√': r'$\sqrt{}$',
        '∈': r'$\in$',
        '∉': r'$\notin$',
        '⊂': r'$\subset$',
        '⊃': r'$\supset$',
        '∪': r'$\cup$',
        '∩': r'$\cap$',
        '∧': r'$\land$',
        '∨': r'$\lor$',
        '¬': r'$\neg$',
        # Misc
        '—': '---',  # em dash
        '–': '--',   # en dash
        '…': '...',
        '•': r'\textbullet{}',
        '°': r'\textdegree{}',
    }
    for char, latex in replacements.items():
        content = content.replace(char, latex)
    return content


def preprocess_content(content):
    """Clean up markdown for better LaTeX conversion."""
    # Remove everything from start to first ---
    # This strips: title, authors, affiliations, correspondence, preprint notice
    content = re.sub(r'^.*?---\n', '', content, flags=re.DOTALL)
    # Remove Abstract section (we put it on title page)
    content = re.sub(r'^## Abstract\n\n.*?(?=\n##|\Z)', '', content, flags=re.DOTALL | re.MULTILINE)
    # Note: We keep Unicode - xelatex handles it natively
    # Wrap bare URLs for LaTeX line breaking
    content = wrap_bare_urls(content)
    return content


def main():
    print('Building arXiv submission package...\n')

    # Read source
    with open(PAPER_MD, 'r') as f:
        content = f.read()

    # Create arxiv submission directory
    if os.path.exists(ARXIV_DIR):
        shutil.rmtree(ARXIV_DIR)
    os.makedirs(ARXIV_DIR)

    # Extract mermaid blocks and replace with ASCII
    matches = extract_mermaid_blocks(content)
    print(f'Found {len(matches)} mermaid diagrams to replace with ASCII')

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
    processed_md = f'{ARXIV_DIR}/DANEEL_PAPER_processed.md'
    with open(processed_md, 'w') as f:
        f.write(processed_content)
    print(f'\nWrote processed markdown: {processed_md}')

    # Generate LaTeX with pandoc (not PDF)
    print('\nGenerating LaTeX with pandoc...')

    result = subprocess.run([
        'pandoc', 'DANEEL_PAPER_processed.md',
        '-o', 'main.tex',
        '--standalone',
        '--from=markdown+tex_math_dollars+pipe_tables+raw_tex',
        '--to=latex',
        '--toc',
        '--toc-depth=3',
        '--number-sections',
        '-V', f'title={TITLE}',
        '-V', f'author={AUTHOR}',
        '-V', f'date={DATE}',
        '-V', 'documentclass=article',
        '-V', 'geometry=margin=1in',
        '-V', 'fontsize=11pt',
    ], capture_output=True, text=True, cwd=ARXIV_DIR)

    if result.returncode != 0:
        print(f'Pandoc failed:\n{result.stderr}')
        sys.exit(1)

    # Read the generated tex and add our customizations
    tex_path = f'{ARXIV_DIR}/main.tex'
    with open(tex_path, 'r') as f:
        tex_content = f.read()

    # Add required packages and fixes after documentclass
    arxiv_preamble = """
% arXiv submission - DANEEL Paper
% Compile with: xelatex main.tex (run twice for TOC)

\\usepackage{float}  % For [H] figure placement
\\usepackage{url}
\\makeatletter
\\g@addto@macro{\\UrlBreaks}{\\UrlOrds}
\\makeatother

\\usepackage{hyperref}
\\hypersetup{
    colorlinks=true,
    linkcolor=blue,
    urlcolor=blue,
    citecolor=blue,
    breaklinks=true
}

% Fix section numbering
\\renewcommand{\\thesection}{\\arabic{section}}
\\renewcommand{\\thesubsection}{\\thesection.\\arabic{subsection}}
\\renewcommand{\\thesubsubsection}{\\thesubsection.\\arabic{subsubsection}}

% Abstract
\\renewcommand{\\abstractname}{Abstract}
"""

    # Insert after \begin{document}
    abstract_block = f"""
\\begin{{abstract}}
{ABSTRACT}
\\end{{abstract}}

"""

    # Find the end of documentclass block and insert preamble after
    # Pandoc outputs: \documentclass[\n  options,\n]{class}
    # We need to find ]{article} or similar
    documentclass_end = re.search(r'\]\{[a-z]+\}', tex_content)
    if documentclass_end:
        insert_pos = documentclass_end.end()
        tex_content = tex_content[:insert_pos] + '\n' + arxiv_preamble + tex_content[insert_pos:]

    # Add abstract after \maketitle
    tex_content = tex_content.replace(
        '\\maketitle',
        '\\maketitle\n' + abstract_block,
        1
    )

    # Write final tex
    with open(tex_path, 'w') as f:
        f.write(tex_content)

    print(f'Wrote LaTeX source: {tex_path}')

    # Clean up intermediate files
    os.remove(processed_md)

    # Create zip package
    zip_path = f'{OUTPUT_DIR}/daneel-arxiv-submission.zip'
    print(f'\nCreating zip package: {zip_path}')

    with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zf:
        zf.write(tex_path, 'main.tex')

    # Also keep unzipped for inspection
    print(f'\nLaTeX source ready at: {tex_path}')
    print(f'Zip package ready at: {zip_path}')

    # Test compile with xelatex (better Unicode support)
    print('\nTest compiling with xelatex...')
    result = subprocess.run(
        ['xelatex', '-interaction=nonstopmode', 'main.tex'],
        capture_output=True, text=True, cwd=ARXIV_DIR
    )

    if result.returncode == 0:
        # Run twice for TOC
        subprocess.run(
            ['xelatex', '-interaction=nonstopmode', 'main.tex'],
            capture_output=True, text=True, cwd=ARXIV_DIR
        )
        print('Compile successful!')
        pdf_path = f'{ARXIV_DIR}/main.pdf'
        print(f'Test PDF: {pdf_path}')
    else:
        print('Warning: xelatex compile had issues (check main.log)')
        print('arXiv may still compile successfully')

    print('\n' + '='*60)
    print('arXiv SUBMISSION PACKAGE READY')
    print('='*60)
    print(f'  Zip: {zip_path}')
    print(f'  LaTeX: {tex_path}')
    print(f'  Categories: cs.AI (primary), cs.CY (secondary)')
    print('='*60)
    print('\nUpload via: https://arxiv.org/submit')
    print('Done!')


if __name__ == '__main__':
    main()

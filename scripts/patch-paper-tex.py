#!/usr/bin/env python3
"""
Patch DANEEL_PAPER.tex after pandoc conversion:
1. Add TikZ packages for diagrams
2. Replace mermaid code blocks with TikZ diagram commands
"""

import re
import sys

def main():
    tex_file = sys.argv[1] if len(sys.argv) > 1 else 'paper/arxiv/DANEEL_PAPER.tex'

    with open(tex_file, 'r') as f:
        content = f.read()

    # 1. Add TikZ packages after \usepackage{bookmark}
    tikz_pkg = r"""
% Extra symbols (checkmark, etc.)
\usepackage{amssymb}

% TikZ for diagrams
\usepackage{tikz}
\usetikzlibrary{shapes.multipart, positioning, arrows.meta, fit, backgrounds}
\input{diagrams}
"""
    if r'\usepackage{tikz}' not in content:
        content = content.replace(
            r'\usepackage{bookmark}',
            r'\usepackage{bookmark}' + tikz_pkg
        )
        print('  ✓ Added TikZ packages')
    else:
        print('  ✓ TikZ packages already present')

    # 2. Replace Unicode characters with LaTeX equivalents
    unicode_map = {
        # Regular Greek letters
        'σ': r'$\sigma$',
        'α': r'$\alpha$',
        'β': r'$\beta$',
        'δ': r'$\delta$',
        'κ': r'$\kappa$',
        'τ': r'$\tau$',
        # Mathematical italic Greek (different Unicode codepoints)
        '𝜎': r'$\sigma$',
        '𝛼': r'$\alpha$',
        # Math symbols
        '≈': r'$\approx$',
        '≠': r'$\neq$',
        '→': r'$\rightarrow$',
        '←': r'$\leftarrow$',
        '↔': r'$\leftrightarrow$',
        '✓': r'$\checkmark$',
        '×': r'$\times$',
        '∞': r'$\infty$',
        '≥': r'$\geq$',
        '≤': r'$\leq$',
        '∼': r'$\sim$',
    }
    replacements = 0
    for char, latex in unicode_map.items():
        count = content.count(char)
        if count > 0:
            content = content.replace(char, latex)
            replacements += count
    if replacements > 0:
        print(f'  ✓ Replaced {replacements} Unicode characters with LaTeX')

    # 3. Replace mermaid Shaded blocks with TikZ commands
    # Pattern: \begin{Shaded} blocks containing mermaid graph/flowchart syntax
    pattern = r'\\begin\{Shaded\}\s*\\begin\{Highlighting\}\[\][\s\S]*?\\NormalTok\{(?:graph|flowchart) (?:TB|LR)\}[\s\S]*?\\end\{Highlighting\}\s*\\end\{Shaded\}'

    diagrams = [
        r'\payoffmatrix',      # Section 1.2 - Game Theory Matrix
        r'\brainvstmi',        # Section 7.1 - Brain capacity
        r'\wetwarevssoftware', # Section 7.3 - Wetware vs Software
        r'\daneelarchitecture' # Section 10.2 - LLM as External Tool
    ]

    matches = list(re.finditer(pattern, content, flags=re.DOTALL))
    print(f'  Found {len(matches)} mermaid blocks')

    # Replace from end to beginning to preserve positions
    for i, match in enumerate(reversed(matches)):
        idx = len(matches) - 1 - i
        if idx < len(diagrams):
            replacement = diagrams[idx]
            content = content[:match.start()] + replacement + content[match.end():]
            print(f'  ✓ Replaced block {idx+1} with {replacement}')
        else:
            content = content[:match.start()] + '% removed mermaid block' + content[match.end():]
            print(f'  ✓ Removed extra mermaid block {idx+1}')

    with open(tex_file, 'w') as f:
        f.write(content)

    print(f'  ✓ Wrote patched {tex_file}')

if __name__ == '__main__':
    main()

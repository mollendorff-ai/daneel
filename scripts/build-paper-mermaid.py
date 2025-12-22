#!/usr/bin/env python3
"""
Build DANEEL paper PDF with native mermaid diagram rendering.
Extracts mermaid blocks, renders to PNG via mmdc, replaces with image refs.
Uses custom LaTeX template for professional formatting.
"""

import re
import os
import subprocess
import sys
import tempfile
import shutil

PAPER_MD = 'paper/DANEEL_PAPER.md'
OUTPUT_DIR = 'paper/arxiv'
DIAGRAMS_DIR = f'{OUTPUT_DIR}/diagrams'
TEMPLATE = f'{OUTPUT_DIR}/template.tex'

# Paper metadata
TITLE = 'DANEEL: A Human-Like Cognitive Architecture for Aligned Artificial Superintelligence'
AUTHOR = 'Luis Cezar Menezes Tavares de Lacerda (Louis C. Tavares)'
DATE = 'December 2025'

# Diagram titles for captions
DIAGRAM_TITLES = [
    'AI Development Game Theory Matrix',
    'Brain Hardware vs TMI Software Capacity',
    'Wetware vs Software Architecture Comparison',
    'DANEEL TMI Core Architecture with LLM Tool Interface',
]

# Abstract text
ABSTRACT = """We present DANEEL, a cognitive architecture implementing Augusto Cury's Theory of Multifocal Intelligence (TMI) with persistent non-semantic memory, emotional structuring via Russell's circumplex model, and an immutable ethical core (Asimov's Laws). Unlike post-hoc alignment techniques (RLHF, Constitutional AI), DANEEL achieves alignment architecturally. Core thesis: Architecture produces psychology. Structure determines values."""


def extract_mermaid_blocks(content):
    """Find all mermaid code blocks and their positions."""
    pattern = r'```mermaid\n(.*?)```'
    matches = list(re.finditer(pattern, content, re.DOTALL))
    return matches


def render_mermaid(mermaid_code, output_path):
    """Render mermaid code to PNG using mmdc."""
    with tempfile.NamedTemporaryFile(mode='w', suffix='.mmd', delete=False) as f:
        f.write(mermaid_code)
        mmd_path = f.name

    try:
        result = subprocess.run([
            'mmdc',
            '-i', mmd_path,
            '-o', output_path,
            '-b', 'white',
            '-s', '3',  # 3x scale for hi-res
            '-t', 'neutral',  # Clean theme, no overlays
            '-w', '1200',  # Width for clarity
        ], capture_output=True, text=True, timeout=60)

        if result.returncode != 0:
            print(f'  Warning: mmdc failed: {result.stderr}')
            return False
        return True
    except subprocess.TimeoutExpired:
        print(f'  Warning: mmdc timed out')
        return False
    finally:
        os.unlink(mmd_path)


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


def preprocess_content(content):
    """Clean up markdown for better LaTeX conversion."""
    # Remove the title line (we'll use metadata instead)
    content = re.sub(r'^# DANEEL:.*?\n', '', content)

    # Remove author block at top (up to first ---)
    content = re.sub(r'^\*\*Luis Cezar.*?---\n', '', content, flags=re.DOTALL)

    # Remove the Abstract section header and content (we put it on title page)
    content = re.sub(r'^## Abstract\n\n.*?(?=\n##|\Z)', '', content, flags=re.DOTALL | re.MULTILINE)

    # Wrap bare URLs for LaTeX line breaking
    content = wrap_bare_urls(content)

    # Leave Unicode as-is - xelatex with fontspec will handle it
    return content


def main():
    print('Building DANEEL paper with mermaid diagrams...\n')

    # Read source
    with open(PAPER_MD, 'r') as f:
        content = f.read()

    # Create diagrams directory
    os.makedirs(DIAGRAMS_DIR, exist_ok=True)

    # Extract mermaid blocks
    matches = extract_mermaid_blocks(content)
    print(f'Found {len(matches)} mermaid diagrams')

    # Render each and replace
    processed_content = content
    offset = 0

    for i, match in enumerate(matches):
        diagram_num = i + 1
        mermaid_code = match.group(1)
        png_filename = f'diagram_{diagram_num}.png'
        png_path = f'{DIAGRAMS_DIR}/{png_filename}'

        print(f'  [{diagram_num}/{len(matches)}] Rendering {png_filename}...')

        if render_mermaid(mermaid_code, png_path):
            # Replace mermaid block with image reference + caption
            # Use relative path from OUTPUT_DIR
            caption = DIAGRAM_TITLES[i] if i < len(DIAGRAM_TITLES) else f'Diagram {diagram_num}'
            img_ref = f'![{caption}](diagrams/{png_filename}){{width=85%}}'

            start = match.start() + offset
            end = match.end() + offset
            processed_content = processed_content[:start] + img_ref + processed_content[end:]
            offset += len(img_ref) - (match.end() - match.start())
            print(f'    Rendered: {caption}')
        else:
            print(f'    Failed to render, keeping placeholder')

    # Preprocess content (remove duplicate title, convert unicode)
    processed_content = preprocess_content(processed_content)

    # Write processed markdown
    processed_md = f'{OUTPUT_DIR}/DANEEL_PAPER_processed.md'
    with open(processed_md, 'w') as f:
        f.write(processed_content)
    print(f'\nWrote processed markdown: {processed_md}')

    # Generate PDF with pandoc using custom template
    print('\nGenerating PDF with pandoc + custom template...')
    pdf_path = f'{OUTPUT_DIR}/DANEEL_PAPER.pdf'
    abs_output_dir = os.path.abspath(OUTPUT_DIR)

    result = subprocess.run([
        'pandoc', 'DANEEL_PAPER_processed.md',
        '-o', 'DANEEL_PAPER.pdf',
        '--template=template.tex',
        '--pdf-engine=xelatex',
        '--from=markdown+tex_math_dollars+pipe_tables+raw_tex+implicit_figures',
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
        # Show more context
        if result.stdout:
            print(f'Stdout:\n{result.stdout}')
        sys.exit(1)

    print(f'\nPDF generated: {pdf_path}')

    # Open it
    if sys.platform == 'darwin':
        subprocess.run(['open', pdf_path])

    print('Done!')


if __name__ == '__main__':
    main()

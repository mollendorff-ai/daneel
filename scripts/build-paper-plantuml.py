#!/usr/bin/env python3
"""
Build DANEEL paper PDF with PlantUML diagram rendering.
Extracts mermaid blocks, converts to PlantUML, renders via server to SVG/PNG.
Uses custom LaTeX template for professional formatting.
"""

import re
import os
import subprocess
import sys
import tempfile
import urllib.request
import urllib.parse
import zlib
import base64

PAPER_MD = 'paper/DANEEL_PAPER.md'
OUTPUT_DIR = 'paper/arxiv'
DIAGRAMS_DIR = f'{OUTPUT_DIR}/diagrams'
TEMPLATE = f'{OUTPUT_DIR}/template.tex'
PLANTUML_SERVER = 'https://www.plantuml.com/plantuml'

# Paper metadata
TITLE = 'DANEEL: A Human-Like Cognitive Architecture for Aligned Artificial Superintelligence'
AUTHOR = 'Luis Cezar Menezes Tavares de Lacerda (Louis C. Tavares)'
DATE = 'December 2025'

# Diagram titles and PlantUML code
DIAGRAMS = [
    {
        'title': 'AI Development Game Theory Matrix',
        'plantuml': '''@startuml
skinparam backgroundColor white
skinparam defaultFontSize 14
skinparam defaultFontName Arial
skinparam shadowing false
skinparam rectangleBorderThickness 2

title AI Development Game Theory Matrix

together {
  rectangle "**YOU: Hold Line**\\nALL OTHERS: Hold Line\\n───────────\\n**SAFE**\\n(ideal)" as A #90EE90
  rectangle "**YOU: Hold Line**\\nALL OTHERS: Defect\\n───────────\\n**DOMINATED**\\n(you lose)" as B #FFB6C6
}

together {
  rectangle "**YOU: Defect**\\nALL OTHERS: Hold Line\\n───────────\\n**FIRST MOVER**" as C #FFD700
  rectangle "**YOU: Defect**\\nALL OTHERS: Defect\\n───────────\\n**RACE TO BOTTOM**" as D #FF6B6B
}

A -[hidden]right- B
C -[hidden]right- D
A -[hidden]down- C

@enduml'''
    },
    {
        'title': 'Brain Hardware vs TMI Software Capacity',
        'plantuml': '''@startuml
skinparam backgroundColor white
skinparam defaultFontSize 12
skinparam defaultFontName Arial
skinparam shadowing false
skinparam rectangleBorderThickness 2

title Brain (Hardware) vs TMI (Software)

rectangle "**BRAIN (Hardware)**\\n86B neurons, 100T synapses, ~2.5 PB total" as BRAIN #E8E8E8 {
  rectangle "Cerebellum\\n69B neurons (80%)\\nMotor coordination, NOT thought" as CB #FFFFFF
  rectangle "Brainstem\\n~500M (0.5%)\\nAutonomic (heart, breathing)" as BS #FFFFFF
  rectangle "Spinal sensory\\n~1B (1%)\\nBody sensation routing" as SS #FFFFFF
  rectangle "**82.5% of brain capacity\\nis NOT for cognition**" as NC #FFE6E6

  rectangle "**TMI / THOUGHT MACHINE (Software)**\\n17.5% of brain" as TMI #B3D9FF {
    rectangle "Cerebral cortex\\n16B neurons (18.6%)" as CC #FFFFFF
    rectangle "Prefrontal cortex\\n~2.5B neurons\\nExecutive, planning" as PFC #FFFFFF
    rectangle "Hippocampus/limbic\\n~1B neurons\\nMemory, emotion" as HC #FFFFFF
    rectangle "Raw: ~0.44 PB → Abstracted: ~500 GB" as CAP #90EE90
  }
}

CB -[hidden]right- BS
BS -[hidden]right- SS
CC -[hidden]right- PFC
PFC -[hidden]right- HC

@enduml'''
    },
    {
        'title': 'Wetware vs Software Architecture Comparison',
        'plantuml': '''@startuml
skinparam backgroundColor white
skinparam defaultFontSize 12
skinparam defaultFontName Arial
skinparam shadowing false
skinparam rectangleBorderThickness 2

title Wetware vs Software: Medium-Independent Patterns

rectangle "**WETWARE (Human Brain)**" as WET #FFE6CC {
  rectangle "5s intervention window\\n(neurotransmitter rates)" as W1 #FFFFFF
  rectangle "50ms attention cycle\\n(synaptic plasticity)" as W2 #FFFFFF
  rectangle "Sleep consolidation\\n(glymphatic system)" as W3 #FFFFFF
}

rectangle "**SOFTWARE (TMI Patterns)**" as SOFT #CCE6FF {
  rectangle "~100 cycles per intervention\\n(RATIO, medium-independent)" as S1 #FFFFFF
  rectangle "Competing parallel streams\\n(PATTERN, medium-independent)" as S2 #FFFFFF
  rectangle "Salience-weighted selection\\n(ALGORITHM, medium-independent)" as S3 #FFFFFF
}

W1 -[hidden]down- W2
W2 -[hidden]down- W3
S1 -[hidden]down- S2
S2 -[hidden]down- S3

W1 ..> S1 : abstracted to
W2 ..> S2 : abstracted to
W3 ..> S3 : abstracted to

@enduml'''
    },
    {
        'title': 'DANEEL TMI Core Architecture with LLM Tool Interface',
        'plantuml': '''@startuml
skinparam backgroundColor white
skinparam defaultFontSize 12
skinparam defaultFontName Arial
skinparam shadowing false
skinparam rectangleBorderThickness 2

title DANEEL Architecture: LLM as External Tool

rectangle "**DANEEL TMI Core**\\n(stores ALL experiences)" as CORE #B3D9FF {
  rectangle "Memory Windows\\n(complete thought history)" as MW #FFFFFF
  rectangle "Salience\\n(emotional weights)" as SAL #FFFFFF
  rectangle "Continuity\\n(persistent 'I')" as CONT #FFFFFF
}

rectangle "**Tool Interface (gRPC)**" as TOOLS #FFE6CC {
  rectangle "LLM Tool:\\nConvert thought-structure → language" as LLM1 #E6E6FA
  rectangle "LLM Tool:\\nParse language → thought-structure" as LLM2 #E6E6FA
  rectangle "Other tools:\\nweb, files, APIs..." as OTHER #FFFFFF
}

MW -[hidden]right- SAL
SAL -[hidden]right- CONT
LLM1 -[hidden]right- LLM2
LLM2 -[hidden]right- OTHER

CORE <--> TOOLS

@enduml'''
    },
]

# Abstract text
ABSTRACT = """We present DANEEL, a cognitive architecture implementing Augusto Cury's Theory of Multifocal Intelligence (TMI) with persistent non-semantic memory, emotional structuring via Russell's circumplex model, and an immutable ethical core (Asimov's Laws). Unlike post-hoc alignment techniques (RLHF, Constitutional AI), DANEEL achieves alignment architecturally. Core thesis: Architecture produces psychology. Structure determines values."""


def plantuml_encode(text):
    """Encode PlantUML text for URL using PlantUML's custom encoding."""
    # PlantUML uses deflate (not zlib header) + custom base64
    compressed = zlib.compress(text.encode('utf-8'), 9)[2:-4]  # Remove zlib header/trailer

    # PlantUML's custom base64 alphabet
    alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_"

    result = []
    i = 0
    while i < len(compressed):
        if i + 2 < len(compressed):
            b1, b2, b3 = compressed[i], compressed[i+1], compressed[i+2]
            result.append(alphabet[b1 >> 2])
            result.append(alphabet[((b1 & 0x3) << 4) | (b2 >> 4)])
            result.append(alphabet[((b2 & 0xF) << 2) | (b3 >> 6)])
            result.append(alphabet[b3 & 0x3F])
            i += 3
        elif i + 1 < len(compressed):
            b1, b2 = compressed[i], compressed[i+1]
            result.append(alphabet[b1 >> 2])
            result.append(alphabet[((b1 & 0x3) << 4) | (b2 >> 4)])
            result.append(alphabet[(b2 & 0xF) << 2])
            i += 2
        else:
            b1 = compressed[i]
            result.append(alphabet[b1 >> 2])
            result.append(alphabet[(b1 & 0x3) << 4])
            i += 1

    return ''.join(result)


def render_plantuml(puml_code, output_path, fmt='png'):
    """Render PlantUML code to image via server."""
    encoded = plantuml_encode(puml_code)
    url = f'{PLANTUML_SERVER}/{fmt}/{encoded}'

    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
        with urllib.request.urlopen(req, timeout=30) as response:
            with open(output_path, 'wb') as f:
                f.write(response.read())
        return True
    except Exception as e:
        print(f'  Warning: PlantUML render failed: {e}')
        return False


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
    print('Building DANEEL paper with PlantUML diagrams...\n')

    # Read source
    with open(PAPER_MD, 'r') as f:
        content = f.read()

    # Create diagrams directory
    os.makedirs(DIAGRAMS_DIR, exist_ok=True)

    # Extract mermaid blocks
    matches = extract_mermaid_blocks(content)
    print(f'Found {len(matches)} mermaid diagrams to replace with PlantUML')

    # Render PlantUML diagrams
    processed_content = content
    offset = 0

    for i, match in enumerate(matches):
        diagram_num = i + 1

        if i < len(DIAGRAMS):
            diagram = DIAGRAMS[i]
            title = diagram['title']
            puml_code = diagram['plantuml']
            png_filename = f'diagram_{diagram_num}.png'
            png_path = f'{DIAGRAMS_DIR}/{png_filename}'

            print(f'  [{diagram_num}/{len(matches)}] Rendering: {title}')

            if render_plantuml(puml_code, png_path, 'png'):
                # Replace mermaid block with image reference
                img_ref = f'![{title}](diagrams/{png_filename}){{width=85%}}'

                start = match.start() + offset
                end = match.end() + offset
                processed_content = processed_content[:start] + img_ref + processed_content[end:]
                offset += len(img_ref) - (match.end() - match.start())
                print(f'    Success')
            else:
                print(f'    Failed - keeping placeholder')
        else:
            print(f'  [{diagram_num}/{len(matches)}] No PlantUML defined, removing block')
            start = match.start() + offset
            end = match.end() + offset
            processed_content = processed_content[:start] + '[Diagram placeholder]' + processed_content[end:]
            offset += len('[Diagram placeholder]') - (match.end() - match.start())

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

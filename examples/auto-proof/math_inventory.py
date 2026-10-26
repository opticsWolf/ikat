"""Token inventory over every `$$...$$` display span in the exported corpus."""
import re, collections
from pathlib import Path

spans = []
for md in sorted(Path(__file__).resolve().parent.joinpath('full').glob('*/*.md')):
    txt = md.read_text(encoding='utf-8', errors='replace')
    for m in re.finditer(r'^\$\$(.+)\$\$\s*$', txt, re.M):
        spans.append(m.group(1))
print('spans:', len(spans))

cmds = collections.Counter()
for s in spans:
    for m in re.finditer(r'\\([a-zA-Z]+)', s):
        cmds[m.group(1)] += 1
print('lettered commands:', len(cmds))
for c, n in cmds.most_common(200):
    print(f'  {c:24s} {n}')

na = collections.Counter()
for s in spans:
    for c in s:
        if ord(c) > 127:
            na[c] += 1
print('non-ascii:', {c: n for c, n in na.most_common()})

two = collections.Counter()
for s in spans:
    for m in re.finditer(r'\\([^a-zA-Z])', s):
        two[m.group(1)] += 1
print('punct commands:', dict(two))

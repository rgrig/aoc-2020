#!/usr/bin/env python3

from collections import defaultdict, deque
import sys

Grev = defaultdict(set)

for line in sys.stdin:
  ws = line.split()
  src = tuple(ws[0:2])
  i = 4
  while i < len(ws) and ws[i] != 'no':
    cnt = int(ws[i])
    tgt = tuple(ws[i+1:i+3])
    i += 4
    Grev[tgt].add(src)

todo = deque([('shiny','gold')])
seen = set([('shiny','gold')])
while todo:
  x = todo.popleft()
  for y in Grev[x]:
    if not (y in seen):
      seen.add(y)
      todo.append(y)
print(len(seen)-1)


#!/usr/bin/env python3

from collections import defaultdict, deque
import sys

G = defaultdict(set)

for line in sys.stdin:
  ws = line.split()
  src = tuple(ws[0:2])
  i = 4
  while i < len(ws) and ws[i] != 'no':
    cnt = int(ws[i])
    tgt = tuple(ws[i+1:i+3])
    i += 4
    G[src].add((cnt, tgt))

sg = ('shiny','gold')
todo = deque([sg])
seen = set([sg])
sz = {}

def dfs(x):
  sz[x] = 1
  for cnt, y in G[x]:
    if not (y in seen):
      seen.add(y)
      dfs(y)
    sz[x] += cnt * sz[y]

dfs(sg)
print(sz[sg]-1)


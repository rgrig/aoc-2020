#!/usr/bin/env python3
from collections import defaultdict
import sys

inp = [l.strip() for l in sys.stdin.readlines()]
active_nxt = set()
for i in range(len(inp)):
  for j in range(len(inp[0])):
    if inp[i][j] == '#':
      active_nxt.add((i,j,0,0))

print(sorted(active_nxt))

for _ in range(6):
  active_now, active_nxt = active_nxt, set()
  minx = min(x for x, _, _, _ in active_now)
  maxx = max(x for x, _, _, _ in active_now)
  miny = min(y for _, y, _, _ in active_now)
  maxy = max(y for _, y, _, _ in active_now)
  minz = min(z for _, _, z, _ in active_now)
  maxz = max(z for _, _, z, _ in active_now)
  minw = min(w for _, _, _, w in active_now)
  maxw = max(w for _, _, _, w in active_now)
  for x in range(minx-1, maxx+2):
    for y in range(miny-1, maxy+2):
      for z in range(minz-1, maxz+2):
        for w in range(minw-1, maxw+2):
          neigh = 0
          for dx in [-1,0,1]:
            for dy in [-1,0,1]:
              for dz in [-1,0,1]:
                for dw in [-1,0,1]:
                  if dx == 0 and dy == 0 and dz == 0 and dw == 0:
                    continue
                  if (x+dx,y+dy,z+dz,w+dw) in active_now:
                    neigh += 1
          active = (x,y,z,w) in active_now and neigh in [2,3]
          active = active or ((x,y,z,w) not in active_now and neigh == 3)
          if active:
            active_nxt.add((x,y,z,w))
  #print(sorted(active_nxt))
print(len(active_nxt))

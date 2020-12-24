#!/usr/bin/env python3
import sys

D = {'e':(2,0),'se':(1,-1),'sw':(-1,-1),'w':(-2,0),'nw':(-1,1),'ne':(1,1)}

def go(s):
  x, y = 0, 0
  i = 0
  while i < len(s):
    for k, (dx, dy) in D.items():
      if s[i:].startswith(k):
        x += dx
        y += dy
        i += len(k)
        break
  return (x, y)

black = set()

for line in sys.stdin:
  t = go(line.strip())
  if t in black:
    black.remove(t)
  else:
    black.add(t)

for _ in range(100):
  lookat = set(black)
  neigh = set()
  for x, y in lookat:
    for dx, dy in D.values():
      neigh.add((x+dx,y+dy))
  lookat |= neigh
  new_black = set(black)
  for x, y in lookat:
    neigh = 0
    for dx, dy in D.values():
      if (x+dx,y+dy) in black:
        neigh += 1
    if (x,y) in black and (neigh == 0 or neigh > 2):
      new_black.remove((x,y))
    elif (x,y) not in black and neigh == 2:
      new_black.add((x,y))
  black = new_black
  print(len(black))

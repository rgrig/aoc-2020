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
print(len(black))

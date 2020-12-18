#!/usr/bin/env python3
from collections import defaultdict
import sys

def num(x, i):
  if x[i] == '(':
    r, j = evaluate(x, i + 1)
    return (r, j + 1)
  else:
    r = int(x[i])
    return (r, i + 1)

def evaluate(x, i):
  #print('ev',x,i)
  p = 1
  s, i = num(x, i)
  while i < len(x) and x[i] != ')':
    op = x[i]
    a, i = num(x, i+1)
    if op == '+':
      s += a
    else:
      p *= s
      s = a
  return (p*s, i)

x=0
for line in sys.stdin:
  line = line.replace('(', '( ').replace(')', ' )')
  ws = line.split()
  r, _ = evaluate(ws, 0)
  #print(r)
  x += r
print(x)

#!/usr/bin/env python3

import sys

prog = [x.split() for x in sys.stdin.readlines()]
prog = [(a, int(b)) for [a,b] in prog]

seen = set()
ic = 0
acc = 0

while ic not in seen:
  seen.add(ic)
  if prog[ic][0] == 'acc':
    acc += prog[ic][1]
    ic += 1
  elif prog[ic][0] == 'jmp':
    ic += prog[ic][1]
  else:
    ic += 1

print(acc)

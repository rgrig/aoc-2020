#!/usr/bin/env python3
from collections import defaultdict
import sys

ormask = 0
fmask = 0

done = 0
memory = defaultdict(int)
for line in sys.stdin:
  if line.startswith('mask'):
    mstr = line.split()[2]
    ormask = fmask = 0
    for c in mstr:
      ormask = 2 * ormask
      fmask = 2 * fmask + 1
      if c == '1':
        ormask += 1
      elif c == 'X':
        fmask -= 1
        ormask += 1
  else:
    ws = line.replace('[',' ').replace(']',' ').split()
    addr = int(ws[1])
    val = int(ws[3])
    f = fmask
    while True:
      fa = (addr | ormask) & f
      memory[fa] = val
      if f == 2**36-1:
        break
      f = (f + 1) | fmask
  done += 1
ans = sum(v for v in memory.values())
print(ans)

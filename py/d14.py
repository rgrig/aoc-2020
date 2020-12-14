#!/usr/bin/env python3
from collections import defaultdict
import sys

ormask = 0
andmask = 0

memory = defaultdict(int)
for line in sys.stdin:
  if line.startswith('mask'):
    mstr = line.split()[2]
    ormask = andmask = 0
    for c in mstr:
      ormask = 2 * ormask
      andmask = 2 * andmask + 1
      if c == '1':
        ormask += 1
      elif c == '0':
        andmask -= 1
  else:
    ws = line.replace('[',' ').replace(']',' ').split()
    addr = int(ws[1])
    val = int(ws[3])
    memory[addr] = (val | ormask) & andmask
ans = sum(v for v in memory.values())
print(ans)

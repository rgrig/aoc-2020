#!/usr/bin/env python3
import sys

inp = [int(x) for x in sys.stdin.read().strip().split(',')]
last = {inp[i] : i for i in range(len(inp)-1)}
x = inp[-1]
i = len(inp)-1
while i != 30000000-1:
  #sys.stdout.write('{:3} {:3}\n'.format(i,x))
  if x not in last:
    y = 0
  else:
    y = i - last[x]
  last[x] = i
  i += 1
  x = y
print(x)

#!/usr/bin/env python3

magic=104054607
#magic=127

import sys

xs = [int(x) for x in sys.stdin.readlines()]
pre = [0]
pos = { 0 : 0 }
result = None
i = 0
for x in xs:
  s = pre[-1] + x
  i += 1
  pos[s] = i
  pre.append(s)
  if s - magic in pos:
    if result is not None:
      print("more?")
    result = (pos[s-magic], i)
    break
ys = xs[result[0]:result[1]]
r = min(ys) + max(ys)
sys.stdout.write('{}\n'.format(r))

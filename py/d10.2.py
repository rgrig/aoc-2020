#!/usr/bin/env python3
from collections import defaultdict
import sys

xs = [int(x) for x in sys.stdin.readlines()]

xs.append(0)
xs.append(3+max(xs))
count = [1]
xs.sort()
for i in range(1,len(xs)):
  c = 0
  k = 1
  while k <= i and xs[i-k] >= xs[i] - 3:
    c += count[i-k]
    k += 1
  count.append(c)
print(count[-1])

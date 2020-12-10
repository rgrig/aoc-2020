#!/usr/bin/env python3
from collections import defaultdict
import sys

xs = [int(x) for x in sys.stdin.readlines()]

xs.append(0)
xs.append(3+max(xs))
diffs = defaultdict(int)
xs.sort()
for i in range(1,len(xs)):
  diffs[xs[i]-xs[i-1]] += 1
print(diffs)
print(diffs[1]*diffs[3])

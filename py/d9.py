#!/usr/bin/env python3

import sys
n=25

xs = [int(x) for x in sys.stdin.readlines()]
for i in range(n,len(xs)):
  good = False
  for j in range(n):
    for k in range(j):
      if xs[i] == xs[i-j-1] + xs[i-k-1]:
        good = True
  if not good:
    print(xs[i])
    break


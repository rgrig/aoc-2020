#!/usr/bin/env python3
from collections import defaultdict
import sys

fields = defaultdict(list)
for line in sys.stdin:
  line = line.strip()
  if line == '':
    break
  [f, vs] = line.split(':')
  vs = vs.split('or')
  for v in vs:
    v = v.strip()
    v = v.split('-')
    fields[f].append((int(v[0]), int(v[1])))
print(fields)

sys.stdin.readline() # your ticket
sys.stdin.readline()
sys.stdin.readline()

nearby_tickets = []
sys.stdin.readline() # nearby tickets
for line in sys.stdin:
  t = [int(x) for x in line.split(',')]
  nearby_tickets.append(t)
print(nearby_tickets)


def can_be_valid(x):
  for lims in fields.values():
    for l, h in lims:
      if l <= x and x <= h:
        return True
  return False
ans = 0
for t in nearby_tickets:
  for v in t:
    if not can_be_valid(v):
      ans += v
print(ans)

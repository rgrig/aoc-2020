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
#print(fields)

sys.stdin.readline() # your ticket
line = sys.stdin.readline()
your_ticket = [int(x) for x in line.split(',')]
sys.stdin.readline()

nearby_tickets = []
sys.stdin.readline() # nearby tickets
for line in sys.stdin:
  t = [int(x) for x in line.split(',')]
  nearby_tickets.append(t)
#print(nearby_tickets)


def can_be_valid(x):
  for lims in fields.values():
    for l, h in lims:
      if l <= x and x <= h:
        return True
  return False

valid_tickets = []
for t in nearby_tickets:
  vs = [can_be_valid(v) for v in t]
  if all(vs):
    valid_tickets.append(t)
    n = len(t)
    #print(n)

can_be = { f : set(range(n)) for f in fields.keys() }
for t in valid_tickets:
  for i in range(n):
    for f, lims in fields.items():
      if i not in can_be[f]:
        continue
      ok = False
      for l, h in lims:
        ok |= l <= t[i] and t[i] <= h
      if not ok:
        can_be[f].remove(i)

foi = [None for _ in range(n)]
keep_going = True
while keep_going:
  keep_going = False
  for f, vs in can_be.items():
    if len(vs) == 1:
      keep_going = True
      for v in vs:
        pass
      foi[v] = f
      for ff, ws in can_be.items():
        if v in ws:
          ws.remove(v)
#print(can_be)
#print(foi)

ans = 1
for i in range(n):
  if foi[i].startswith('departure'):
    ans *= your_ticket[i]
print(ans)


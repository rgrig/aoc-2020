#!/usr/bin/env python3
from collections import defaultdict
import sys

inp = []
for line in sys.stdin:
  line = line.replace(')', '').replace(',', '')
  [xs, ys] = line.split('(contains')
  ingredients = set(xs.split())
  allergens = set(ys.split())
  inp.append((ingredients, allergens))

by_allergen = {}
for ingredients, allergens in inp:
  for a in allergens:
    if a not in by_allergen:
      by_allergen[a] = set(ingredients)
    else:
      by_allergen[a] = set(ingredients) & by_allergen[a]

fixed = True
while fixed:
  print(by_allergen)
  fixed = False
  ks = sorted(by_allergen.keys())
  for a in ks:
    if len(by_allergen[a]) == 1:
      for w in by_allergen[a]:
        pass
      for b in ks:
        if b == a:
          continue
        if w in by_allergen[b]:
          fixed = True
          by_allergen[b].remove(w)
ans = []
for k in sorted(by_allergen.keys()):
  for w in by_allergen[k]:
    pass
  ans.append(w)
ans = ','.join(ans)
print(ans)

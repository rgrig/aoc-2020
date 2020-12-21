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
bad = set()
for i in by_allergen.values():
  bad |= i
ans = 0
for ingredients, _ in inp:
  for i in ingredients:
    if i not in bad:
      ans += 1
print(ans)


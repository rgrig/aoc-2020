#!/usr/bin/env python3
from collections import defaultdict
import sys

Term = 0
NonTerm = 1

inp = sys.stdin.read().strip().split('\n\n')
rules_inp = inp[0].split('\n')
data = inp[1].split()

rules = defaultdict(list)
for r in rules_inp:
  [lhs,rhs] = r.split(':')
  lhs = int(lhs)
  rhs = rhs.strip()
  if rhs.startswith('"'):
    rules[lhs].append([(Term,rhs[1])])
  else:
    for nt in rhs.split('|'):
      rules[lhs].append([(NonTerm, int(x)) for x in nt.strip().split()])
rcnt = max(rules.keys())
for r in sorted(rules.keys()):
  rhs = rules[r]
  l = r
  for alt in rhs:
    while len(alt) > 2:
      rcnt += 1
      l = rcnt
      nalt = alt[1:]
      alt[1:] = [(NonTerm, rcnt)]
      alt = nalt
      rules[l] = [alt]
#print(rules)

cache = None

def match(text, tr, i, j):
  t, r = tr
  if t == Term:
    #print('NO',tr,i,j)
    return j == i + 1 and text[i] == r
  else:
    #print('REC',r,i,j)
    return parse(text, r, i, j) # we're promised no cycles in rules

def parse(text, r, i, j):
  global cache
  if cache[r][i][j] is not None:
    return cache[r][i][j]
  #print('try',r,i,j)
  cache[r][i][j] = False
  for alt in rules[r]:
    assert len(alt) in [1, 2]
    if len(alt) == 1:
      cache[r][i][j] = match(text, alt[0], i, j)
    else:
      for k in range(i + 1, j): # no empty word
        #print('try to split {}[{}:{}] into {}[{}:{}]+{}[{}:{}]'.format(
        #  r,i,j,alt[0][1],i,k,alt[1][1],k,j))
        if match(text, alt[0], i, k) and match(text, alt[1], k, j):
          cache[r][i][j] = True
          break
    if cache[r][i][j]:
      #print('Y',r,i,j,text[i:j],text)
      return True
  #print('N',r,i,j,text[i:j],text)
  return False


def initcache(text):
  global cache
  cache = [[[None for _ in range(len(text)+1)] for _ in range(len(text)+1)] for _ in range(rcnt+1)]

cnt = 0
for t in data:
  initcache(t)
  if parse(t, 0, 0, len(t)):
    cnt += 1
print(cnt)



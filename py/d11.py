#!/usr/bin/env python3
from collections import defaultdict
import sys

M = [x.strip() for x in sys.stdin.readlines()]
change = None

def step():
  global M
  global change
  change = False
  m = len(M)
  n = len(M[0])
  N = [[None for _ in range(n)] for _ in range(m)]
  for i in range(m):
    for j in range(n):
      neigh = 0
      for ii in [-1,0,1]:
        for jj in [-1,0,1]:
          if ii == 0 and jj == 0:
            continue
          iii=i+ii
          jjj=j+jj
          if not (0<=iii and iii<m):
            continue
          if not (0<=jjj and jjj<n):
            continue
          if M[iii][jjj] == '#':
            neigh += 1
      new = M[i][j]
      if M[i][j] == 'L' and neigh == 0:
        change = True
        new = '#'
      elif M[i][j] == '#' and neigh >= 4:
        change = True
        new = 'L'
      N[i][j] = new
  M = N

change = True
while change:
  step()

occ = 0
for ms in M:
  for m in ms:
    if m=='#':
      occ += 1
print(occ)

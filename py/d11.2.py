#!/usr/bin/env python3
from collections import defaultdict
import sys

dx = [+1,+1,0,-1,-1,-1,0,+1]
dy = [0,+1,+1,+1,0,-1,-1,-1]

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
      for d in range(8):
        k = 0
        while True:
          k += 1
          ii = i + k * dx[d]
          jj = j + k * dy[d]
          if not (0<=ii and ii<m):
            break
          if not (0<=jj and jj<n):
            break
          if M[ii][jj] == '#':
            neigh += 1
            break
          elif M[ii][jj] == 'L':
            break
      new = M[i][j]
      if M[i][j] == 'L' and neigh == 0:
        change = True
        new = '#'
      elif M[i][j] == '#' and neigh >= 5:
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

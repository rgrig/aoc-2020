#!/usr/bin/env python3
from collections import defaultdict
import sys

orig_image = sys.stdin.read().strip().split('\n')
pattern = ['                  # ','#    ##    ##    ###',' #  #  #  #  #  #   ']

def rot(p):
  NX = len(p)
  NY = len(p[0])
  return [[p[NX-j-1][i] for j in range(NX)] for i in range(NY)]

def flip(p):
  NX = len(p)
  NY = len(p[0])
  return [[p[i][NY-j-1] for j in range(NY)] for i in range(NX)]

def sym(p):
  for _ in range(4):
    p = rot(p)
    yield p
  p = flip(p)
  for _ in range(4):
    p = rot(p)
    yield p


for image in sym(orig_image):
  IX = len(image)
  IY = len(image[0])
  ans = [[image[i][j]=='#' for j in range(IY)] for i in range(IX)]
  p = pattern
  PX = len(p)
  PY = len(p[0])
  for i in range(PX,IX+1):
    for j in range(PY,IY+1):
      ok = True
      for x in range(PX):
        for y in range(PY):
          ok = ok and (p[x][y] != '#' or image[i-PX+x][j-PY+y] == '#')
      if ok:
        print('found one at ({},{})'.format(i-PX,j-PY))
        for x in range(PX):
          for y in range(PY):
            if p[x][y] == '#':
              ans[i-PX+x][j-PY+y] = False
  cnt = 0
  for r in ans:
    for b in r:
      if b:
        cnt += 1
  print(cnt)


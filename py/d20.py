#!/usr/bin/env python3
from collections import defaultdict
import sys

N = 10

inp = sys.stdin.read().split('\n\n')
by_id = {}
for tile in inp:
  tile = tile.strip().split('\n')
  tile_id = int(tile[0].split()[1].replace(':',' '))
  tile_body = tile[1:]
  #print(tile_id, tile_body)
  by_id[tile_id] = tile_body
M = 0
while M * M < len(by_id):
  M += 1

def rot(tile):
  return [[tile[N-j-1][i] for j in range(N)] for i in range(N)]

def flip(tile):
  return [[tile[i][N-j-1] for j in range(N)] for i in range(N)]

def sym(tile):
  for _ in range(4):
    tile = rot(tile)
    yield tile
  tile = flip(tile)
  for _ in range(4):
    tile = rot(tile)
    yield tile

state_id = [[None for _ in range(M)] for _ in range(M)]
state_im = [[None for _ in range(M)] for _ in range(M)]
available = set(by_id.keys())

def fill(row, col):
  # x<row and (x==row y<col) have been filled
  if row == M:
    return True
  if col == M:
    return fill(row+1, 0)
  top = None
  left = None
  if row > 0:
    top = state_im[row-1][col][N-1]
  if col > 0:
    left = [state_im[row][col-1][i][N-1] for i in range(N)]
  #print('fill ({},{}) top {} left {}'.format(row,col,top,left))
  for tile_id in sorted(available):
    tile = by_id[tile_id]
    available.remove(tile_id)
    state_id[row][col] = tile_id
    for tile in sym(tile):
      if top is not None and tile[0] != top:
        continue
      if left is not None and [tile[i][0] for i in range(N)] != left:
        continue
      state_im[row][col] = tile
      if fill(row, col+1):
        return True
      state_im[row][col] = None
    state_id[row][col] = None
    available.add(tile_id)
  return False

print(fill(0,0))
for i in range(M):
  for x in range(1,N-1):
    for j in range(M):
      for y in range(1,N-1):
        sys.stdout.write(state_im[i][j][x][y])
    sys.stdout.write('\n')
print(state_id[0][0] * state_id[0][M-1] * state_id[M-1][0] * state_id[M-1][M-1])

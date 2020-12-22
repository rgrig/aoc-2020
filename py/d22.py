#!/usr/bin/env python3
from collections import defaultdict
import sys

inp = sys.stdin.read().strip().split('\n\n')
players = []
for p in inp:
  p = p.strip().split('\n')
  p = p[1:]
  players.append([int(x) for x in p])

while players[0] and players[1]:
  assert players[0][0] != players[1][0]
  w = 0 if players[0][0] > players[1][0] else 1
  players[w] = players[w][1:] + [players[w][0], players[1-w][0]]
  players[1-w] = players[1-w][1:]
w = players[0] if players[0] else players[1]
w.reverse()
score = 0
for i in range(len(w)):
  score += (i + 1) * w[i]
print(score)

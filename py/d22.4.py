#!/usr/bin/env python3
from collections import defaultdict
from random import randrange
import sys

inp = sys.stdin.read().strip().split('\n\n')
players = []
for p in inp:
  p = p.strip().split('\n')
  p = p[1:]
  players.append([int(x) for x in p])

def play(P):
  global steps, level
  S = [set(), set()]
  while P[0] and P[1]:
    assert P[0][0] != P[1][0]
    if tuple(P[0]) in S[0] or tuple(P[1]) in S[1]:
      return 0
    S[0].add(tuple(P[0]))
    S[1].add(tuple(P[1]))
    if P[0][0] < len(P[0]) and P[1][0] < len(P[1]):
      w = play([P[0][1:P[0][0]+1], P[1][1:P[1][0]+1]])
    else:
      w = 0 if P[0][0] > P[1][0] else 1
    P[w] = P[w][1:] + [P[w][0], P[1-w][0]]
    P[1-w] = P[1-w][1:]
  return 0 if P[0] else 1
w = play(players)
#print(w,players)
w=players[w]
w.reverse()
score = 0
for i in range(len(w)):
  score += (i + 1) * w[i]
print(score)

#!/usr/bin/env python3
from collections import defaultdict
import sys

state = [9,6,3,2,7,5,4,8,1]
#state = [3,8,9,1,2,5,4,6,7]

M = max(state)
S = set(state)
for _ in range(3):
  destination = state[0] - 1
  if destination not in S:
    destination = M
  while destination in state[1:4]:
    destination -= 1
    if destination not in S:
      destination = M
  i = state.index(destination) + 1
  state = state[4:i]+ state[1:4] + state[i:] + [state[0]]
  print(state)
i = state.index(1)
state = state[i+1:] + state[:i]
N = len(state)
for i in range(len(state)):
  sys.stdout.write('{}'.format(state[i]))
sys.stdout.write('\n')




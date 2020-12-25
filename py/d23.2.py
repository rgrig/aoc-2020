#!/usr/bin/env python3

inp = [9,6,3,2,7,5,4,8,1]
#inp = [3,8,9,1,2,5,4,6,7]

T = 10000000
N = 1000000
while len(inp) < N:
  inp.append(1+len(inp))
nxt = [None for _ in range(N+1)]
prv = [None for _ in range(N+1)]

for i in range(N):
  nxt[inp[i]] = inp[(i+1)%N]
  prv[inp[i]] = inp[(i+N-1)%N]
c0 = inp[0]
inp = None

for _ in range(T):
  c1 = nxt[c0]
  c2 = nxt[c1]
  c3 = nxt[c2]
  d = c0
  while d in [c0,c1,c2,c3]:
    d -= 1
    if d < 1:
      d = N
  nxt[prv[c1]] = nxt[c3]
  prv[nxt[c3]] = prv[c1]
  prv[c1] = d
  nxt[c3] = nxt[d]
  nxt[prv[c1]] = c1
  prv[nxt[c3]] = c3
  c0 = nxt[c0]

c0 = 1
c1 = nxt[c0]
c2 = nxt[c1]
print(c1, c2)

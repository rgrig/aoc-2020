#!/usr/bin/env python3
import sys

def gcd(a,b):
  while b>0:
    a,b=b,a%b
  return a
def lcm(a,b):
  return a * b // gcd(a, b)

inp = [x.strip() for x in sys.stdin.readlines()]
start = int(inp[0])
ts = inp[1].replace(',',' ').split()
times = []
for i in range(len(ts)):
  if ts[i] != 'x':
    times.append((int(ts[i]), i))
times.sort()
times.reverse()
start = 0
g = 1
for m, k in times:
  print('mkg',m,k,g)
  while (start + k) % m != 0:
    start += g
  print('start',start)
  g = lcm(g, m)
sys.stdout.write('{}\n'.format(start))

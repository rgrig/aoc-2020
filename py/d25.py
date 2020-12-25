#!/usr/bin/env python3
import sys

M = 20201227
pub = [5099500, 7648211]

def logm(b):
  x = 1
  i = 0
  while x != b:
    x = (x * 7) % M
    i += 1
  return i

def expm(b):
  if b == 0:
    return 1
  bl = b % 2
  bh = b // 2
  ebh = expm(bh)
  x = ebh * ebh
  if bl:
    x *= 7
  return x % M

ka = logm(pub[0])
kb = logm(pub[1])
print(ka,kb,expm(ka*kb))

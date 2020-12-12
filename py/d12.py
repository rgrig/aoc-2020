#!/usr/bin/env python3
from collections import defaultdict
from math import sin, cos, pi
import sys

d = 0
nod = 'ENWS'
don = [0,90,180,270]

instructions = [x.strip() for x in sys.stdin.readlines()]
x, y = 0, 0

def move(deg, dist):
  global x, y
  x += dist * cos(deg * pi / 180)
  y += dist * sin(deg * pi / 180)

for instr in instructions:
  num = int(instr[1:])
  ad = nod.find(instr[0])
  if ad >= 0:
    move(don[ad], num)
  elif instr[0] == 'L':
    d += num
  elif instr[0] == 'R':
    d -= num
  elif instr[0] == 'F':
    move(d, num)
  sys.stdout.write('x {} y {} |x|+|y| {}\n'.format(x,y,abs(x)+abs(y)))
sys.stdout.write('x {} y {} |x|+|y| {}\n'.format(x,y,abs(x)+abs(y)))

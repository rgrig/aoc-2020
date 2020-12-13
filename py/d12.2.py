#!/usr/bin/env python3
from collections import defaultdict
from math import sin, cos, pi
import sys

dx = 10
dy = 1
nod = 'ENWS'
dxr = [+1,0,-1,0]
dyr = [0,+1,0,-1]

instructions = [x.strip() for x in sys.stdin.readlines()]
x, y = 0, 0

def move(dist):
  global x, y
  x += dist * dx
  y += dist * dy

for instr in instructions:
  num = int(instr[1:])
  ad = nod.find(instr[0])
  if ad >= 0:
    dx += dxr[ad] * num
    dy += dyr[ad] * num
  elif instr[0] == 'L':
    alpha = num * pi / 180
    dx, dy = cos(alpha) * dx - sin(alpha) * dy, sin(alpha) * dx + cos(alpha) * dy
  elif instr[0] == 'R':
    alpha = - num * pi / 180
    dx, dy = cos(alpha) * dx - sin(alpha) * dy, sin(alpha) * dx + cos(alpha) * dy
  elif instr[0] == 'F':
    move(num)
  sys.stdout.write('x {} y {} |x|+|y| {}\n'.format(x,y,abs(x)+abs(y)))
sys.stdout.write('x {} y {} |x|+|y| {}\n'.format(x,y,abs(x)+abs(y)))

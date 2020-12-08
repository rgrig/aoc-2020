#!/usr/bin/env python3

import sys

prog = [x.split() for x in sys.stdin.readlines()]
prog = [(a, int(b)) for [a,b] in prog]

for bug in range(len(prog)):
  if prog[bug][0] not in ['jmp','nop']:
    continue
  seen = set()
  ic = acc = 0
  while ic not in seen and ic < len(prog) and 0 <= ic:
    seen.add(ic)
    instr = prog[ic]
    if ic == bug:
      if instr[0] == 'jmp':
        instr = ('nop',0)
      elif instr[0] == 'nop':
        instr = ('jmp', instr[1])
    if instr[0] == 'acc':
      acc += instr[1]
      ic += 1
    elif instr[0] == 'jmp':
      ic += instr[1]
    else:
      ic += 1
  if ic == len(prog):
    print(acc)
    break

#!/usr/bin/env python3
import sys

inp = [x.strip() for x in sys.stdin.readlines()]
start = int(inp[0])
times = [int(x) for x in inp[1].replace(',',' ').split() if x!='x']
bt = None
for t in times:
  if bt is None or t - start % t < bt - start % bt:
    bt = t
sys.stdout.write('{}\n'.format(bt * (bt-start%bt)))

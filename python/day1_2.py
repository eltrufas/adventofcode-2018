import sys
from itertools import cycle, accumulate

freqs = set([])
for freq in accumulate(cycle(int(l) for l in sys.stdin)):
    if freq in freqs:
        break
    freqs.add(freq)

print(freq)

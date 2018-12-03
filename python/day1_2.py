import sys
import itertools

freqs = set([])
freq = 0
for df in itertools.cycle(int(l) for l in sys.stdin):
    freq += df
    if freq in freqs:
        break
    freqs.add(freq)

print(freq)

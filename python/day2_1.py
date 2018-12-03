import sys
from collections import Counter

twos = 0
threes = 0

for l in sys.stdin:
    cnt = Counter()
    for c in l:
        cnt[c] += 1

    if any(c == 2 for c in cnt.values()):
        twos += 1
    if any(c == 3 for c in cnt.values()):
        threes += 1

print(twos * threes)

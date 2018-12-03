import sys
import itertools

for (a, b) in itertools.combinations(sys.stdin, 2):
    diff = 0
    result = ""
    for (c, d) in zip(a, b):
        if c == d:
            result += c
    if len(result) - len(a) == -1:
        print(result)

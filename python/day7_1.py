import sys

lines = [(l[1], l[7]) for l in (l.split() for l in sys.stdin)]

steps = set([s[0] for s in lines] + [s[1] for s in lines])

order = ''
while steps:
    n = min(s for s in steps if all(b != s for (_, b) in l))
    order += n
    steps.remove(n)
    lines = [(a, b) for (a, b) in lines if a != n]

print(order)

import sys

lines = [(l[1], l[7]) for l in (l.split() for l in sys.stdin)]
steps = set([s[0] for s in lines] + [s[1] for s in lines])

N_WORKERS = 5

t = 0
workers, work = map(list, zip(*((0, None) for _ in range(N_WORKERS))))
while steps or any(w > 0 for w in workers):
    cand = [s for s in steps if all(b != s for (_, b) in lines)]
    cand.sort(reverse=True)

    for i in range(N_WORKERS):
        workers[i] -= 1
        if workers[i] <= 0:
            if work[i] is not None:
                lines = [(a, b) for (a, b) in lines if a != work[i]]
            if cand:
                n = cand.pop()
                workers[i] = 60 + ord(n) - ord('A')
                work[i] = n
                steps.remove(n)
    t += 1

print(t)

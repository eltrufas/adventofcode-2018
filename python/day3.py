import sys

lines = list(sys.stdin)
ss = [l.split(" ") for l in lines]

rects = []

for s in ss:
    print(s)
    x, y = s[2][:-1].split(',')
    w, h = s[3].split('x')
    x, y, w, h = int(x), int(y), int(w), int(h)

    x2, y2 = x + w, y + h
    rects.append((x, y, x2, y2))

cloth = [[0 for _ in range(1000)] for _ in range(1000)]

for r in rects:
    x, y, x1, y1 = r
    for i in range(x, x1):
        for j in range(y, y1):
            cloth[i][j] += 1

print(sum(c > 1 for r in cloth for c in r))

ids = [s[0] for s in ss]

for idx, r in enumerate(rects):
    x, y, x1, y1 = r
    if all(cloth[1] <= 1 for i in range(x, x1) for j in range(y, y1)):
        print(idx[idx])

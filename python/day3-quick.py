import sys

lines = list(sys.stdin)
ss = [l.split(" ") for l in lines]

rects = []

for s in ss:
    print(s)
    x, y = s[2][:-1].split(',')
    w, h = s[3].split('x')
    x = int(x)
    y = int(y)
    w = int(w)
    h = int(h)

    x2, y2 = x + w, y + h
    rects.append((x, y, x2, y2))

cloth = [[0 for _ in range(1000)] for _ in range(1000)]

for r in rects:
    x, y, x1, y1 = r
    for i in range(x, x1):
        for j in range(y, y1):
            cloth[i][j] += 1

filt = [c for r in cloth for c in r if c > 1]

print(len(filt))

ids = [s[0] for s in ss]

for idx, r in enumerate(rects):
    x, y, x1, y1 = r
    clean = True
    for i in range(x, x1):
        for j in range(y, y1):
            if cloth[i][j] > 1:
                clean = False
    if clean:
        print(ids[idx])

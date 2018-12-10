import sys
import re
from PIL import Image
import numpy as np
points = np.array(list(map(lambda s: tuple(map(int, re.findall(r'-?\d+', s))),
                  sys.stdin)))

coords, vels = points[:, 0:2].transpose(), points[:, 2:4].transpose()

t = 0
coords += t * vels
minarea = 1e15
while True:
    minx, miny = coords.min(axis=1)
    maxx, maxy = coords.max(axis=1)
    area = (maxx - minx) * (maxy - miny)
    if area > minarea:
        break
    minarea = area
    coords += vels
    t += 1

coords -= vels
sky = np.zeros((maxx - minx, maxy - miny), dtype='uint8')
sky[coords[0] - minx, coords[1] - miny] = 255
print(t - 1)
im = Image.fromarray(sky)
im.save("r/{}.jpeg".format(t - 1))

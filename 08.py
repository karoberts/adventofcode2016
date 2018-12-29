import re
from collections import Counter
from itertools import takewhile

pat = re.compile(r'^(rect) (\d+)x(\d+)|(rotate (?:row|column)) [yx]=(\d+) by (\d+)$')

def key(x,y):
    return str(x) + ',' + str(y)

grid = {key(x,y):False for x in range(0, 50) for y in range(0, 6)}
with open('8.txt') as f:
    for line in (l.strip() for l in f):
        m = pat.match(line)
        if m.group(1) == 'rect':
            for y in range(0, int(m.group(3))):
                for x in range(0, int(m.group(2))):
                    grid[key(x,y)] = True
            pass
        elif m.group(4) == 'rotate row':
            y = int(m.group(5))
            shift = int(m.group(6))
            newrow = {}
            for x in range(0, 50):
                newrow[key((x + shift) % 50, y)] = grid[key(x,y)]
            for c in newrow.keys():
                grid[c] = newrow[c]
            pass
        elif m.group(4) == 'rotate column':
            x = int(m.group(5))
            shift = int(m.group(6))
            newrow = {}
            for y in range(0, 6):
                newrow[key(x, (y + shift) % 6)] = grid[key(x,y)]
            for c in newrow.keys():
                grid[c] = newrow[c]

ct = 0
for v in grid.values():
    ct += 1 if v else 0
print('part1', ct)

for y in range(0, 6):
    for x in range(0, 50):
        print('#' if grid[key(x,y)] else '.', end='')
    print()
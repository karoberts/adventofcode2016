
import sys
from collections import defaultdict
import heapq

target_x = 31
target_y = 39

p = True

max_x = target_x + 10
max_y = target_y + 10

print(max_x, max_y)

grid = {}
ero_table = {}

def key(x,y):
    return str(x) + ',' + str(y)

def get_type(x,y):
    t = x*x + 3*x + 2*x*y + y + y*y + 1352
    bits = bin(t).count("1")
    return '.' if bits % 2 == 0 else '#'

for y in range(0, max_y + 1):
    for x in range(0, max_x + 1):
        grid[key(x,y)] = get_type(x,y)

for y in range(0, max_y):
    for x in range(0, max_x):
        c = grid[key(x,y)]
        if p:
            print(c, end='')
    if p:
        print()

min_found = 99999999999
best = defaultdict(lambda:999999)

def print_map(dist):
    for y in range(0, max_y):
        for x in range(0, max_x):
            k = key(x,y)
            c = grid[k]
            if k in dist:
                print(' {:02} '.format(dist[k]), end='')
            else:
                print(' ' + c + c + ' ', end='')
        print()
    print()

def dijkstra():
    tests = [ (1, 0), (-1, 0), (0, -1), (0, 1) ]
    def get_neighbors(x,y):
        k = key(x,y)
        c = grid[k]
        ns = []

        for t in tests:
            tx = x + t[0]
            ty = y + t[1]
            if tx < 0 or ty < 0 or tx >= max_x or ty >= max_y:
                continue
            tk = key(tx, ty)
            tt = grid[tk]

            if tt == '.':
                ns.append( [1, tx, ty, tk] )

        for n in ns:
            n.append(True)
        return ns

    dist['1,1'] = 0

    prev['1,1'] = None

    finder = {}

    inq = set()
    h = []
    heapq.heappush(h, [0, 1, 1, '1,1', True])
    finder['1,1'] = h[0]
    inq.add('1,1')

    while len(h) > 0:
        #print_map(dist);
        u = heapq.heappop(h)
        if not u[4]:
            continue
        inq.remove(u[3])
        #if u[1] == target_y and u[2] == target_y:
        #    return u
        uk = u[3]
        for v in get_neighbors(u[1], u[2]):
            alt = dist[uk] + v[0]
            if alt < dist[v[3]]:
                dist[v[3]] = alt
                prev[v[3]] = (uk, v[0], v[3], v[1], v[2])
                entry = [alt, v[1], v[2], v[3], True]
                if v[3] in inq:
                    finder[v[3]][4] = False
                inq.add(v[3])
                finder[v[3]] = entry

                heapq.heappush(h, entry)

prev = {}
dist = defaultdict(lambda :999999999)

x = dijkstra()

print(dist[key(target_x, target_y)])
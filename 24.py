import sys
from collections import defaultdict
import heapq
from copy import copy

p = False

grid = {}

def key(x,y):
    return str(x) + ',' + str(y)

cur_x = None
cuy_y = None
nums = 0

with open('24.txt') as f:
    y = 0
    for line in (l.strip() for l in f):
        x = 0
        for c in line:
            grid[key(x,y)] = c
            if c == '0':
                cur_x = x
                cur_y = y
            if c in '0123456789':
                nums += 1
            x += 1
        y += 1

max_x = x
max_y = y

print(nums, max_x, max_y, cur_x, cur_y)

for y in range(0, max_y):
    for x in range(0, max_x):
        c = grid[key(x,y)]
        print(c, end='')
    print()

def dijkstra(st_x,st_y,tgt):
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

            if tt == '#':
                continue

            ns.append( [1, tx, ty, tk, True] )

        return ns

    prev = {}
    dist = defaultdict(lambda :999999999)
    st = key(st_x, st_y)
    dist[st] = 0

    prev[st] = None

    finder = {}

    inq = set()
    h = []
    heapq.heappush(h, [0, st_x, st_y, st, True])
    finder[st] = h[0]
    inq.add(st)

    while len(h) > 0:
        #print_map(dist);
        u = heapq.heappop(h)
        if not u[4]:
            continue
        inq.remove(u[3])
        if grid[u[3]] == tgt:
            return u
        #if u[1] == target_y and u[2] == target_y:
            #return u
        uk = u[3]
        for v in get_neighbors(u[1], u[2]):
            alt = dist[uk] + v[0]
            if alt < dist[v[3]]:
                dist[v[3]] = alt
                #prev[v[3]] = (uk, v[0], v[4], v[1], v[2])
                entry = [alt, v[1], v[2], v[3], True]
                if v[3] in inq:
                    finder[v[3]][4] = False
                inq.add(v[3])
                finder[v[3]] = entry

                heapq.heappush(h, entry)

numsteps = []
for i in range(1, nums):
    x = dijkstra(cur_x, cur_y, str(i))
    numsteps.append([set([i]), x[0], x[1], x[2]])

for j in range(0, nums - 2):
    print('starting round', j+1)
    new_numsteps = []
    for n in numsteps:
        for i in range(1, nums):
            if i in n[0]: continue
            x = dijkstra(n[2], n[3], str(i))
            s = copy(n[0])
            s.add(i)
            new_numsteps.append([s, n[1] + x[0], x[1], x[2]])
    numsteps = new_numsteps

for nn in sorted(numsteps, key=lambda x:x[1]):
    print(nn)
    print('part1 steps', nn[1])
    break

seen = set()
new_numsteps = []
min_final = 999999999
for n in sorted(numsteps, key=lambda x:x[1]):
    if n[1] > min_final: continue
    k = str(n[1]) + '-' + key(n[2], n[3])
    if k in seen: continue
    seen.add(k)
    #print('current', n[1], 'at', n[2], n[3])
    x = dijkstra(n[2], n[3], '0')
    s = copy(n[0])
    s.add(i)
    new_numsteps.append([s, n[1] + x[0], x[1], x[2]])
    if n[1] + x[0] < min_final:
        min_final = n[1] + x[0]
        print('NEW MIN!!!', min_final)

for nn in sorted(new_numsteps, key=lambda x:x[1]):
    print(nn)
    print('part2 steps', nn[1])
    break
import re
from collections import Counter
from itertools import takewhile
import hashlib
import heapq
import recordtype
from collections import defaultdict

pat = re.compile(r'^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$')

nodes = {}
max_x = 0
max_y = 0
with open('22.txt') as f:
    for line in (l.strip() for l in f):
        m = pat.match(line)
        nk = m.group(1) + ',' + m.group(2)
        n = {'k': nk, 'x': int(m.group(1)), 'y': int(m.group(2)), 's': int(m.group(3)), 'u': int(m.group(4)), 'a': int(m.group(5)), 'p': int(m.group(6))}
        max_x = max(n['x']), max_x)
        max_y = max(n['y']), max_y)
        nodes[nk] = n

def key(x,y):
    return str(x) + ',' + str(y)

def dijkstra():
    tests = [ (1, 0), (-1, 0), (0, -1), (0, 1) ]
    def get_neighbors(x,y):
        k = key(x,y)
        ns = []

        for t in tests:
            tx = x + t[0]
            ty = y + t[1]
            if tx < 0 or ty < 0 or tx >= max_x or ty >= max_y:
                continue
            tk = key(tx, ty)
            if nodes[k]['u'] < nodes[tk]['a']:
                ns.append((1, tx, ty, tk))

        return ns

    QEntry = recordtype.recordtype('QEntry', 'x y k valid path')

    dist['0,0,'] = 0

    prev['0,0,'] = None

    finder = {}

    inq = set()
    h = []
    heapq.heappush(h, (0, 0, '', QEntry(x=0, y=0, k='0,0,', path='', valid=True)))
    finder['0,0,'] = h[0]
    inq.add('0,0,')

    while len(h) > 0:
        #print_map(dist);
        u = heapq.heappop(h)
        item = u[3]
        if not item.valid:
            continue
        inq.remove(item.k)
        if item.x == target_y and item.y == target_y:
            continue
        uk = item.k
        for v in get_neighbors(item.x, item.y, item.path):
            alt = dist[uk] + v[0]
            if alt > dist[v[3]]:
                dist[v[3]] = alt
                #prev[v[3]] = (uk, v[0], v[4], v[1], v[2])
                entry = QEntry(x=v[1], y=v[2], k=v[3], path=v[4], valid=True)
                if v[3] in inq:
                    finder[v[3]].valid = False
                inq.add(v[3])
                finder[v[3]] = entry

                heapq.heappush(h, (-alt, -len(entry.path), entry.path, entry))

prev = {}
dist = defaultdict(lambda :-1)

dijkstra()

target_key = key(target_x, target_y)
max_len = 0
for d in dist.keys():
    if d.startswith(target_key):
        max_len = max(max_len, dist[d])
        #print(d, dist[d], d[len(target_key):])
print(max_len)


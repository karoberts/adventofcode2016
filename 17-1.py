import hashlib
import heapq
import recordtype
from collections import defaultdict

passw = 'vkjiggvb'

target_x = 3
target_y = 3
max_x = 4
max_y = 4

def gethash(pw, route):
    return hashlib.md5(bytearray(pw + route, 'utf-8')).hexdigest()[0:4]

def key(x,y):
    return str(x) + ',' + str(y)

def dijkstra():
    tests = [ (1, 0, 3, 'R'), (-1, 0, 2, 'L'), (0, -1, 0, 'U'), (0, 1, 1, 'D') ]
    def get_neighbors(x,y,path):
        k = key(x,y)
        ns = []

        h = gethash(passw, path)

        for t in tests:
            tx = x + t[0]
            ty = y + t[1]
            if tx < 0 or ty < 0 or tx >= max_x or ty >= max_y:
                continue
            tk = key(tx, ty)
            if h[t[2]] in 'bcdef':
                ns.append((1, tx, ty, tk + path + t[3], path + t[3]))

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
            return u
        uk = item.k
        for v in get_neighbors(item.x, item.y, item.path):
            alt = dist[uk] + v[0]
            if alt < dist[v[3]]:
                dist[v[3]] = alt
                #prev[v[3]] = (uk, v[0], v[4], v[1], v[2])
                entry = QEntry(x=v[1], y=v[2], k=v[3], path=v[4], valid=True)
                if v[3] in inq:
                    finder[v[3]].valid = False
                inq.add(v[3])
                finder[v[3]] = entry

                heapq.heappush(h, (alt, len(entry.path), entry.path, entry))

prev = {}
dist = defaultdict(lambda :999999999)

dijkstra()

target_key = key(target_x, target_y)
for d in dist.keys():
    if d.startswith(target_key):
        print(d, dist[d], d[len(target_key):])

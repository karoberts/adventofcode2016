import re
from collections import Counter
from itertools import takewhile
import json
import hashlib
import heapq
import recordtype
from collections import defaultdict
from copy import deepcopy

def key(x,y):
    return str(x) + ',' + str(y)

pat = re.compile(r'^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$')

nodes = {}
max_x = 0
max_y = 0
with open('22.txt') as f:
    for line in (l.strip() for l in f):
        m = pat.match(line)
        nk = m.group(1) + ',' + m.group(2)
        n = {'k': nk, 'x': int(m.group(1)), 'y': int(m.group(2)), 's': int(m.group(3)), 'u': int(m.group(4)), 'a': int(m.group(5)), 'p': int(m.group(6)), 'v': False, 't': False}
        max_x = max(n['x'], max_x)
        max_y = max(n['y'], max_y)
        nodes[nk] = n

nodes[key(max_x, 0)]['t'] = True

target_x = max_x
target_y = 0

print(max_x, max_y, target_x, target_y)

def print_map(n):
    for y in range(0, max_y+1):
        for x in range(0, max_x+1):
            k = key(x,y)
            if n[k]['u'] == 0:
                print('{}     0    '.format('*' if n[k]['v'] else ' '), end='')
            else:
                print('{}{:>4d}/{:<4d}{}'.format('*' if n[k]['v'] else ' ', n[k]['u'], n[k]['s'], 'G' if n[k]['t'] else ' '), end='')
        print()

def deltkey(delts):
    if len(delts) == 0: return ''
    return '+' + '-'.join((k for k in sorted(delts.keys())))

def dijkstra(sx, sy):
    def hashnodes(ns):
        return str(hash(json.dumps(ns, sort_keys=True)))

    tests = [ (1, 0), (-1, 0), (0, -1), (0, 1) ]
    def get_neighbors(x,y, nns):
        k = key(x,y)
        ns = []

        for t in tests:
            tx = x + t[0]
            ty = y + t[1]
            if tx < 0 or ty < 0 or tx > max_x or ty > max_y:
                continue
            tk = key(tx, ty)

            if nns[tk]['v']:
                continue

            cur_avail = nns[k]['a']
            tgt_used = nns[tk]['u']

            if tgt_used < cur_avail:
                next_nodes = deepcopy(nns)
                next_nodes[k]['u'] += tgt_used
                next_nodes[k]['a'] -= tgt_used
                next_nodes[tk]['u'] = 0
                next_nodes[tk]['a'] = nns[tk]['s']
                next_nodes[tk]['v'] = True
                if next_nodes[tk]['t']:
                    next_nodes[k]['t'] = True
                    next_nodes[tk]['t'] = False
                hn = hashnodes(next_nodes)
                #ns.append((1, tx, ty, tk + '-' + hn, hn, next_nodes))
                ns.append((1, tx, ty, tk, hn, next_nodes))
                #print('try', key(x,y), 'to', key(tx,ty))

        return ns

    QEntry = recordtype.recordtype('QEntry', 'x y k valid hash nodes')

    sk = key(sx, sy)# + '-' + hashnodes(nodes)

    print('start at', sx, sy)

    dist = defaultdict(lambda :999999999)
    dist[sk] = 0
    prev = {}

    #prev['0,0,'] = None

    finder = {}

    inq = set()
    h = []
    nns = deepcopy(nodes)
    nns[key(sx,sy)]['v'] = True
    heapq.heappush(h, (0, hashnodes(nodes), sk, QEntry(x=sx, y=sy, k=sk, hash=hashnodes(nodes), nodes=nns, valid=True)))
    finder[sk] = h[0]
    inq.add(sk)

    while len(h) > 0:
        #print_map(dist);
        u = heapq.heappop(h)
        item = u[3]
        if not item.valid:
            continue
        inq.remove(item.k)
        #print(item.x, item.y, u[0])
        if item.x == target_x and item.y == target_y:
            return u
        uk = item.k
        for v in get_neighbors(item.x, item.y, item.nodes):
            alt = dist[uk] + v[0]
            if alt < dist[v[3]]:
                dist[v[3]] = alt
                #prev[v[3]] = (uk, v[0], v[4], v[1], v[2])
                entry = QEntry(x=v[1], y=v[2], k=v[3], hash=v[4], nodes=v[5], valid=True)
                if v[3] in inq:
                    finder[v[3]].valid = False
                inq.add(v[3])
                finder[v[3]] = entry

                heapq.heappush(h, (alt, v[4], v[3], entry))


def adjacent(a,b):
    if a['x'] == b['x'] and (a['y'] == b['y'] + 1 or a['y'] == b['y'] - 1):
        return True
    if a['y'] == b['y'] and (a['x'] == b['x'] + 1 or a['x'] == b['x'] - 1):
        return True
    return False

starts = dict()
for a in nodes.values():
    for b in nodes.values():
        if a['k'] == b['k'] or a['u'] == 0: continue
        if a['u'] <= b['a'] and adjacent(a, b):
            #print(a['k'], a['u'], b['k'], b['a'])
            starts[b['k']] = b
s = list(starts.values())[0]

#print_map(nodes)

x = dijkstra(s['x'], s['y'])
#x = bfs(s['x'], s['y'])
#print_map(x[3].nodes)
print(x[0], x[1], x[2])

# I printed and manually verified that it takes 5 moves to move the goal one to the right, and just repeat max_x-1 times
print('part2', (max_x-1) * 5 + x[0])



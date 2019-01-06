import re
from collections import Counter
from itertools import takewhile

pat = re.compile(r'^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$')

with open('22.txt') as f:
    nodes = []
    for line in (l.strip() for l in f):
        m = pat.match(line)
        nk = m.group(1) + ',' + m.group(2)
        n = {'k': nk, 's': int(m.group(3)), 'u': int(m.group(4)), 'a': int(m.group(5)), 'p': int(m.group(6))}
        nodes.append(n)

    ct = 0
    for i, a in enumerate(nodes):
        if a['u'] == 0: continue
        for j, b in enumerate(nodes):
            if a['k'] == b['k']: continue
            if a['u'] <= b['a']:
                print(a['k'], b['k'])
                ct += 1
    print(ct)

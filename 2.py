def key(x,y):
    return str(x) + ',' + str(y)

with open('2.txt') as f:
    pos = [0, 0]
    # part 1
    keys = {'0,0':5, '1,0':6, '-1,0':4, '-1,-1':1, '0,-1':2, '1,-1': 3, '-1,1':7, '0,1':8, '1,1':9}
    # part 2
    keys = {'0,0':5, '1,0':6, '2,0':7, '3,0':8, '4,0':9,   '1,-1': 2, '2,-1': 3, '3,-1':4,    '2,-2': 1,   '1,1': 'A', '2,1':'B', '3,1':'C',   '2,2': 'D'}
    code = []
    for line in (l.strip() for l in f):
        for c in line:
            npos = list(pos)
            if c == 'U':   npos[1] -= 1
            elif c == 'D': npos[1] += 1
            elif c == 'R': npos[0] += 1
            elif c == 'L': npos[0] -= 1
            nk = key(npos[0], npos[1])
            if nk in keys:
                pos = npos
        code.append(keys[key(pos[0], pos[1])])

    print(''.join((str(c) for c in code)))
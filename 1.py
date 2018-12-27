
with open('1.txt') as f:
    line = f.readline().strip()
    #line = 'R5, L5, R5, R3'
    #line = 'R8, R4, R4, R8'
    dirs = (x.strip() for x in line.split(','))
    dirs = [(d[0], int(d[1:])) for d in dirs]
    print(dirs)
    
pos = [0,0]
# 0=N, 1=W, 2=S, 3=E
direc = 0

def manhat_dist(x1,y1,x2,y2):
    return abs(x1 - x2) + abs(y1 - y2)

def key(x,y):
    return str(x) + ',' + str(y)

visits = {}
part2 = False

def check_visit(pos, i, d):
    global part2

    s = -1 if d < 0 else 1
    for c in range(pos[i] + s, pos[i] + d + s, s):
        x = pos[0] if i == 1 else c
        y = pos[1] if i == 0 else c
        k = key(x,y)
        if k in visits and not part2:
            print('part2', visits[k], manhat_dist(0,0,visits[k][0], visits[k][1]))
            part2 = True
        visits[k] = (x,y)


visits['0,0'] = (0,0)

for d in dirs:
    direc += (1 if d[0] == 'L' else -1)
    direc %= 4

    if direc == 0:
        check_visit(pos, 1, d[1])
        pos[1] += d[1]
    elif direc == 1:
        check_visit(pos, 0, -d[1])
        pos[0] -= d[1]
    elif direc == 2:
        check_visit(pos, 1, -d[1])
        pos[1] -= d[1]
    elif direc == 3:
        check_visit(pos, 0, d[1])
        pos[0] += d[1]

    #print(direc, pos)

print('part1', pos, manhat_dist(0,0,pos[0], pos[1]))
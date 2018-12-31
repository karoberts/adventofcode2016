import sys
import datetime
from copy import copy, deepcopy

sys.setrecursionlimit(4000)

floors = [None] * 4

#The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip, a promethium generator, a ruthenium generator, a ruthenium-compatible microchip, a cobalt generator, and a cobalt-compatible microchip.
#The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
#The third floor contains nothing relevant.
#The fourth floor contains nothing relevant.

# polonium generator, thulium generator, thulium microchip, promethium generator, ruthenium generator, ruthenium microchip, cobalt generator, cobalt microchip
generators = set(['PG', 'TG', 'RG', 'CG', 'pG'])
microchips = set(['PM', 'TM', 'RM', 'CM', 'pM'])
floors[0] = set(['PG', 'TG', 'TM', 'pG', 'RG', 'RM', 'CG', 'CM'])
floors[1] = set(['PM', 'pM'])
floors[2] = set([])
floors[3] = set([])
elev = 0

part2 = True
if part2:
    generators.update(['eG', 'dG'])
    microchips.update(['eM', 'dM'])
    floors[0].update(['eG', 'dG', 'eM', 'dM'])

# test
test = False
if test:
    generators = set(['HG', 'LG'])
    microchips = set(['HM', 'LM'])
    floors[0] = set(['HM', 'LM'])
    floors[1] = set(['HG'])
    floors[2] = set(['LG'])
    floors[3] = set([])

def to_gen(i):
    return i[0] + 'G'

def to_chip(i):
    return i[0] + 'M'

def is_valid_floor(f):
    if len(f & generators) > 0:
        for i in f:
            if i in microchips and to_gen(i) not in f:
                return False
    return True

def is_valid_move(old_floor, new_floor):
    return is_valid_floor(old_floor) and is_valid_floor(new_floor)

def is_valid(floors):
    for f in floors:
        if not is_valid_floor(f):
            return False
    return True

def is_done(floors, elev):
    if elev != 3: return False
    for fi in range(0, 3):
        if len(floors[fi]) != 0:
            return False
    return len(floors[3]) == len(generators) + len(microchips)

def floors_to_key(floors, elev):
    return str(elev) + ',' + str([sorted(i) for i in floors])

# this would take years
def recur(floors, elev, steps):
    global found_steps
    global used_floors

    if is_done(floors, elev):
        if steps < found_steps:
            found_steps = steps
            print(datetime.datetime.now(), steps)
        return steps

    if steps > found_steps:
        return 9999999999

    moves = []

    up = elev < 3
    down = elev > 0
    floor = floors[elev]

    used = set()
    for d in ['up' if up else None, 'down' if down else None]:
        if d is None: continue
        for i in floor:
            moves.append((d, i, None))
        for i in floor:
            for j in floor:
                if i == j: continue
                k = d + min(i,j) + max(i,j)
                if k in used: continue
                moves.append((d, i, j))
                used.add(k)

    #print(steps, moves)
    found = []
    for m in moves:
        new_floors = deepcopy(floors)
        nx_floor_n = elev + 1 if m[0] == 'up' else elev - 1
        cx_floor = new_floors[elev]
        nx_floor = new_floors[nx_floor_n]
        for mi in [1,2]:
            if m[mi] is not None:
                cx_floor.remove(m[mi])
                nx_floor.add(m[mi])
        if is_valid_move(cx_floor, nx_floor):
            nk = floors_to_key(new_floors, nx_floor_n)
            best_steps = used_floors[nk] if nk in used_floors else None
            if best_steps is None or steps < best_steps:
                used_floors[nk] = steps
                found.append(recur(new_floors, nx_floor_n, steps + 1))

    return 999999999 if len(found) == 0 else min(found)


def bfs(qnow):
    global used_floors

    qnext = []

    round = 0
    while True:
        print(round)
        round += 1
        for thismove in qnow:
            moves = []

            floors = thismove[0]
            elev = thismove[1]
            steps = thismove[2]

            if is_done(floors, elev):
                if steps < found_steps:
                    print(datetime.datetime.now(), steps)
                return steps

            up = elev < 3
            down = elev > 0
            floor = floors[elev]

            used = set()
            for d in ['up' if up else None, 'down' if down else None]:
                if d is None: continue
                for i in floor:
                    moves.append((d, i, None))
                for i in floor:
                    for j in floor:
                        if i == j: continue
                        k = d + min(i,j) + max(i,j)
                        if k in used: continue
                        moves.append((d, i, j))
                        used.add(k)

            #print(steps, moves)
            found = []
            for m in moves:
                new_floors = deepcopy(floors)
                nx_floor_n = elev + 1 if m[0] == 'up' else elev - 1
                cx_floor = new_floors[elev]
                nx_floor = new_floors[nx_floor_n]
                for mi in [1,2]:
                    if m[mi] is not None:
                        cx_floor.remove(m[mi])
                        nx_floor.add(m[mi])
                if is_valid_move(cx_floor, nx_floor):
                    nk = floors_to_key(new_floors, nx_floor_n)
                    best_steps = used_floors[nk] if nk in used_floors else None
                    if best_steps is None or steps < best_steps:
                        used_floors[nk] = steps
                        qnext.append((new_floors, nx_floor_n, steps + 1))

        qnow = qnext
        qnext = []

found_steps = 9999999999
#found_steps = 20

used_floors = {floors_to_key(floors, elev): 0}

#minsteps = recur(floors, elev, 0)

minsteps = bfs([(floors, elev, 0)])
print('answer', minsteps)
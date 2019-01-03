from copy import copy

row1 = '......^.^^.....^^^^^^^^^...^.^..^^.^^^..^.^..^.^^^.^^^^..^^.^.^.....^^^^^..^..^^^..^^.^.^..^^..^^^..'
nrows = 40 # part 1

nrows = 400_000 # part 2

#row1 = '.^^.^.^^^^'
#nrows = 10

def is_trap(prev_row, i):
    l = False if i == 0 else prev_row[i-1]
    c = prev_row[i]
    r = False if i == len(row1)-1 else prev_row[i+1]

    if l and c and not r:
        return True
    if r and c and not l:
        return True
    if l and not c and not r:
        return True
    if r and not c and not l:
        return True

    return False

row = [x == '^' for x in row1]
nsafe = 0
for n in row:
    if not n:
        nsafe += 1
print(''.join(['^' if x else '.' for x in row]))
next_row = [False] * len(row1)
for r in range(0, nrows - 1):
    for i in range(0, len(row1)):
        n = is_trap(row, i)
        next_row[i] = n
        if not n:
            nsafe += 1
    row = copy(next_row)
    #print(''.join(['^' if x else '.' for x in row]))
    if r % 10000 == 0:
       print(r)

print('safe', nsafe)
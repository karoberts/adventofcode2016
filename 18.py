from copy import copy

row1 = '......^.^^.....^^^^^^^^^...^.^..^^.^^^..^.^..^.^^^.^^^^..^^.^.^.....^^^^^..^..^^^..^^.^.^..^^..^^^..'
nrows = 40 # part 1

nrows = 400_000 # part 2

#row1 = '.^^.^.^^^^'
#nrows = 10

def is_trap(prev_row, i):
    l = False if i == 0 else prev_row[i-1]
    r = False if i == nlen-1 else prev_row[i+1]

    return not (l == r)

nlen = len(row1)
row = [x == '^' for x in row1]
nsafe = 0
for n in row:
    if not n:
        nsafe += 1
print(''.join(['^' if x else '.' for x in row]))

lists = [ row, [False] * len(row1) ]
lid = 0

for r in range(0, nrows - 1):
    nlid = (lid + 1) % 2
    row = lists[lid]
    nextrow = lists[nlid]
    for i in range(0, nlen):
        n = is_trap(row, i)
        nextrow[i] = n
        if not n:
            nsafe += 1
    lid = nlid
    #print(''.join(['^' if x else '.' for x in row]))
    if r % 10000 == 0:
       print(r)

print('safe', nsafe)

def test(a,b,c):
    cs = sorted([a,b,c])
    return cs[0] + cs[1] > cs[2]
        

ct = 0
with open('3.txt') as f:
    lines = []
    for line in (l.strip() for l in f):
        cs = [int(i) for i in line.split()]
        lines.append(cs)

    for i in range(0, len(lines), 3):
        for j in range(0, 3):
            if test(lines[i][j], lines[i + 1][j], lines[i + 2][j]):
                ct += 1

print(ct)
        

with open('3.txt') as f:
    ct = 0
    for line in (l.strip() for l in f):
        cs = sorted([int(i) for i in line.split()])
        if cs[0] + cs[1] > cs[2]:
            ct += 1
print(ct)
        
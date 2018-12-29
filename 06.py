from collections import Counter

with open('6.txt') as f:
    cols = []
    for i in range(0, 8): cols.append([])
    for line in (l.strip() for l in f):
        for i, c in enumerate(line):
            cols[i].append(c)

    part1 = ''
    part2 = ''
    for col in cols:
        ct = Counter(col)
        part1 += ct.most_common(1)[0][0]
        part2 += ct.most_common()[-1][0]
    print('part1', part1)
    print('part2', part2)
import re
from collections import Counter
from itertools import takewhile

pat = re.compile(r'swap position (\d) with position (\d)|swap letter ([a-z]) with letter ([a-z])|rotate (left|right) (\d) step[s]?|rotate based on position of letter ([a-z])|reverse positions (\d) through (\d)|move position (\d) to position (\d)')

pw = 'fbgdceah'
#pw = 'dgfaehcb'

def rotright(p, s):
    np = ''
    s = s % len(p)
    for i in range(len(p) - s, len(p)):
        np += p[i]
    for i in range(0, len(p) - s):
        np += p[i]
    return np

def rotleft(p, s):
    np = ''
    s = s % len(p)
    for i in range(s, len(p)):
        np += p[i]
    for i in range(0, s):
        np += p[i]
    return np

def swap(p, ia, ib):
    ta = ia
    ia = min(ia, ib)
    ib = max(ta, ib)
    return p[:ia] + p[ib] + p[ia+1:ib] + p[ia] + p[ib+1:]

mapping = { 0: 1, 1: 1, 2: 6, 3: 2, 4: 7, 5: 3, 6: 0, 7: 4}

with open('21.txt') as f:
    for line in reversed([l.strip() for l in f]):
        m = pat.match(line)
        if m.group(1):
            pw = swap(pw, int(m.group(2)), int(m.group(1)))
        elif m.group(3):
            ia = -1
            ib = -1
            for i, c in enumerate(pw):
                if c == m.group(3)[0]: ia = i
                if c == m.group(4)[0]: ib = i
            old = pw
            pw = swap(pw, ib, ia)
        elif m.group(5):
            left = m.group(5) == 'left'
            p = int(m.group(6))
            if left:
                pw = rotright(pw, p)
            else:
                pw = rotleft(pw, p)
        elif m.group(7):
            ia = -1
            for i, c in enumerate(pw):
                if c == m.group(7)[0]: ia = i
            pw = rotleft(pw, mapping[ia])
        elif m.group(8):
            ia = int(m.group(8))
            ib = int(m.group(9))
            pw = pw[:ia] + ''.join(reversed(pw[ia:ib+1])) + pw[ib+1:]
        elif m.group(10):
            ia = int(m.group(10))
            ib = int(m.group(11))
            pwa = list(pw)
            del pwa[ib]
            pwa.insert(ia, pw[ib])
            pw = ''.join(pwa)
        print(pw)

print('part2', pw)

# 0 => 1
# 1 => 2
# 2 => 3
# 3 => 4
# 4 => 6
# 5 => 7
# 6 => 8 (no change)
# 7 => 9 (1)


# 0 => 7 
# 1 => 0
# 2 => 4
# 3 => 1
# 4 => 5
# 5 => 2
# 6 => 6
# 7 => 3
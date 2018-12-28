import re
from collections import Counter
from itertools import takewhile

pat = re.compile(r'^([a-z\-]+)-(\d+)\[([a-z]{5})\]$')

# > 57317

def test(name, cksum):
    ct = Counter(name)
    mc = ct.most_common(26)
    exp = ''
    for i in takewhile(lambda _: len(exp) < 5, (x[0] for x in sorted(mc, key=lambda x:(-x[1], x[0])))):
        exp += i

    #print(exp, mc)
    return exp == cksum

def decrypt(name, sector):
    outp = ''
    for c in name:
        if c == '-':
            outp += ' '
        else:
            outp += chr(((ord(c) - ord('a')) + sector) % 26 + ord('a'))
    return outp

with open('4.txt') as f:
    for line in (l.strip() for l in f):
        m = pat.match(line)
        name = m.group(1).replace('-', '')
        sector = int(m.group(2))
        cksum = m.group(3)

        if test(name, cksum):
            v = decrypt(m.group(1), sector)
            if v.find('north') > -1:
                print('FOUND:', v, sector)
            
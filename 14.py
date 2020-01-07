import hashlib
import re

# takes about 7 minutes

salt = 'ngcjuoqr'
#salt = 'abc'

pat1 = re.compile(r'([a-z0-9])\1\1')
pat2 = re.compile(r'([a-z0-9])\1{4}')

def gethash(n):
    md5 = salt + str(n)
    for i in range(0, nstretch):
        md5 = hashlib.md5(bytearray(md5, 'utf-8')).hexdigest()
    return md5

def run(name):
    global nstretch
    hashes = {}
    keys = []
    n = 0
    while len(keys) < 80:
        md5 = hashes[n] if n in hashes else gethash(n)
        m = pat1.search(md5)
        if m:
            for i in range(0, 1000):
                nmd5 = None
                ni = n + i + 1
                if ni not in hashes:
                    nmd5 = gethash(ni)
                    hashes[ni] = nmd5
                else:
                    nmd5 = hashes[ni]

                m2 = pat2.search(nmd5)
                if m2 and m2.group(1) == m.group(1):
                    #print(n, ni, 'key:', md5)
                    keys.append((n, md5))
                    break

        n += 1

    print(name, keys[63])

nstretch = 1 # part1
run('part1')
nstretch = 2017 # part2
run('part2')

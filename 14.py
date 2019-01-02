import hashlib
import re

salt = 'ngcjuoqr'
#salt = 'abc'

n = 0

pat1 = re.compile(r'([a-z0-9])\1\1')
pat2 = re.compile(r'([a-z0-9])\1{4}')
hashes = {}
keys = []

while len(keys) < 80:
    md5 = hashes[n] if n in hashes else hashlib.md5(bytearray(salt + str(n), 'utf-8')).hexdigest()
    m = pat1.search(md5)
    if m:
        for i in range(0, 1000):
            nmd5 = None
            ni = n + i + 1
            if ni not in hashes:
                nmd5 = hashlib.md5(bytearray(salt + str(ni), 'utf-8')).hexdigest()
                hashes[ni] = nmd5
            else:
                nmd5 = hashes[ni]

            m2 = pat2.search(nmd5)
            if m2 and m2.group(1) == m.group(1):
                print(n, ni, 'key:', md5)
                keys.append((n, md5))
                break

    n += 1

print('part1', keys[63])
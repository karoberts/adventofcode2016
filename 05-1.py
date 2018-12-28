import hashlib

key = 'cxdnnyjw'
#key = 'abc'
n = 0
c = 8
password = ''

for i in range(0, c):
    while True:
        input = key + str(n)
        n += 1
        md5 = hashlib.md5(bytearray(input, 'utf-8')).hexdigest()
        if md5.startswith('00000'):
            password += md5[5]
            print(n, password)
            break

print(password)
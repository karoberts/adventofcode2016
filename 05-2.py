import hashlib

key = 'cxdnnyjw'
#key = 'abc'
n = 0
c = 8
password = ['-'] * 8

while '-' in password:
    while True:
        input = key + str(n)
        n += 1
        md5 = hashlib.md5(bytearray(input, 'utf-8')).hexdigest()
        if md5.startswith('00000'):
            if md5[5].isnumeric():
                idx = int(md5[5])
                if idx < 8 and password[idx] == '-':
                    password[idx] = md5[6]
                    print(n, password)
            break

print(''.join(password))
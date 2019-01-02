disk = 20
d = '10000'

# part 1
disk = 272
d = '10111011111001111'

# part 2
disk = 35651584

intab = "10"
outtab = "01"
trantab = str.maketrans(intab, outtab)

while len(d) < disk:
    a = d
    b = d[::-1].translate(trantab)
    d = a + '0' + b

d = d[:disk]
#print(d)

def checksum(d):
    ck = ''
    for i in range(0, len(d), 2):
        ck += '1' if d[i] == d[i+1] else '0'
    if len(ck) % 2 == 0:
        return checksum(ck)
    return ck

print(checksum(d))

import re

disk = 20
d = '10000'

# part 1
disk1 = 272
d = '10111011111001111'

# part 2
disk2 = 35651584

intab = "10"
outtab = "01"
trantab = str.maketrans(intab, outtab)

while len(d) < disk1:
    a = d
    b = d[::-1].translate(trantab)
    d = a + '0' + b

d = d[:disk1]

def checksum(d):
    ck = ''
    for i in range(0, len(d), 2):
        ck += '1' if d[i] == d[i+1] else '0'
    if len(ck) % 2 == 0:
        return checksum(ck)
    return ck

print('part1', checksum(d))

# this is from reddit, my part 2 is very slow
def f(input, maxlen):
  a = input
  while len(a) < maxlen:
    b = re.sub(r"0","#",a[::-1])
    b = re.sub(r"1","0",b[::-1])
    b = re.sub(r"#","1",b[::-1])
    a = a + "0" + b
  a = a[:maxlen]

  a = re.findall(r".{2}", a)
  chksum = "".join(map(lambda bc:"1" if bc[0]==bc[1] else "0", a))

  while len(chksum)>0 and len(chksum)%2==0:
    chksum = re.findall(r".{2}", chksum)
    chksum = "".join(map(lambda bc:"1" if bc[0]==bc[1] else "0", chksum))
  return chksum

print('part2', f(d, disk2))

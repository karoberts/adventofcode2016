
ranges = []
maxip = 4294967295
#maxip = 9
with open('20.txt') as f:
    for line in (l.strip() for l in f):
        rg = [int(x) for x in line.split('-')]
        ranges.append(rg)

ranges = list(sorted(ranges, key=lambda x:(x[0], x[1]-x[0])))

def findrange(ranges, ip):
    for rg in ranges:
        if rg[0] <= ip and rg[1] >= ip:
            return rg
    return None

def findnext(ranges, ip):
    for rg in ranges:
        if rg[0] > ip:
            return rg
    return None

ip = 0
ct = 0
while ip <= maxip:
    rg = findrange(ranges, ip)
    if rg is None:
        print(ip)
        nrg = findnext(ranges, ip) 
        if nrg is None:
            nrg = [maxip + 1, maxip + 1]
        ct += nrg[0] - ip
        ip = nrg[1]
    else:
        ip = rg[1]
        ip += 1

print('part2', ct)
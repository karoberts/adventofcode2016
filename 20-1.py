
ranges = []
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

ip = 0
while ip <= 4294967295:
    rg = findrange(ranges, ip)
    if rg is None:
        print('part1', ip)
        break
    ip = rg[1]
    print(ip)
    ip += 1
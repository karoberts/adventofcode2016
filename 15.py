from recordtype import recordtype

#Disc #1 has 5 positions; at time=0, it is at position 2.
#Disc #2 has 13 positions; at time=0, it is at position 7.
#Disc #3 has 17 positions; at time=0, it is at position 10.
#Disc #4 has 3 positions; at time=0, it is at position 2.
#Disc #5 has 19 positions; at time=0, it is at position 9.
#Disc #6 has 7 positions; at time=0, it is at position 0.

Disc = recordtype('Disc', 'np p last')

discs = []
discs.append(Disc(np=5, p=2, last=False))
discs.append(Disc(np=13, p=7, last=False))
discs.append(Disc(np=17, p=10, last=False))
discs.append(Disc(np=3, p=2, last=False))
discs.append(Disc(np=19, p=9, last=False))
discs.append(Disc(np=7, p=0, last=True))

# 0 1 2

part2 = True
if part2:
    discs[-1].last = False
    discs.append(Disc(np=11, p=0, last=True))

test = False
if test:
    discs = []
    discs.append(Disc(np=5, p=4, last=False))
    discs.append(Disc(np=2, p=1, last=True))

t = 0
t += 1
for d in discs:
    d.p = (d.p + 1) % d.np

def check():
    i = 0
    for d in discs:
        if d.p != abs(d.np - i) % d.np:
            return True
        i += 1
    return False

while True:
    tdelt = 0
    for d in discs:
        if (d.p + tdelt) % d.np != 0:
            break
        if d.last:
            print('t=',t-1)
            exit()
        tdelt += 1
    t += 1
    for d in discs:
        d.p = (d.p + 1) % d.np
    # optimization, make sure discs are in right position, skipping disc 0 to pos 0 each try
    # reduces part 2 by 75%
    while check():
        delt = discs[0].np - discs[0].p
        t += delt
        for d in discs:
            d.p = (d.p + delt) % d.np
            
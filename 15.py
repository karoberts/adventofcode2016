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

test = False
if test:
    discs = []
    discs.append(Disc(np=5, p=4, last=False))
    discs.append(Disc(np=2, p=1, last=True))

# > 22767

t = 0
t += 1
for d in discs:
    d.p = (d.p + 1) % d.np

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
            
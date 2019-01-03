
#nelves = 5
nelves = 3014603

elfpres = {n:1 for n in range(0, nelves)}

def inc(e):
    return (e + 1) % nelves

def find_e(e):
    ne = inc(e)
    while e != ne:
        if elfpres[ne] > 0:
            return ne
        ne = inc(ne)
    return -1

e = 0
e_no_p = set()
while True:
    if elfpres[e] == 0:
       e = inc(e) 
       continue
    e_with_p = find_e(e)
    if e_with_p == -1:
        print(e + 1)
        break

    elfpres[e] += elfpres[e_with_p]
    elfpres[e_with_p] = 0
    e_no_p.add(e_with_p)
    if len(e_no_p) % 100000 == 0:
        print(len(e_no_p))
    #e = inc(e)
    e = inc(e_with_p)


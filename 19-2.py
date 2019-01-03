import dllist

#nelves = 5
nelves = 3014603

# < 1507297
# > 1376176

def inc(e):
    if e.next is None:
        return elfpres.first
    return e.next

elfpres = dllist.dllist()
for i in range(0, nelves):
    elfpres.appendright(i+1)

def dec(e):
    if e == 0: return nelves - 1
    return (e - 1) % nelves

def find_e(e):
    sk = (len(elfpres) // 2)
    ne = e 
    for i in range(0, sk):
        #ne = ne.next
        ne = inc(ne)
    return ne

e = elfpres.first
#ce = find_e(e)
while True:
    #e_with_p = ce
    e_with_p = find_e(e)

    print(e.value,'steals from',e_with_p.value)
    #ce = inc(e_with_p)
    #ce = inc(ce)
    elfpres.remove(e_with_p)
    e = inc(e)
    if len(elfpres) % 10000 == 0:
        print(len(elfpres))
    if len(elfpres) == 1:
        print(elfpres, e)
        break


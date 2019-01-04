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
ce = find_e(e)
#ce = find_e(e)
ct = 0
while True:
    ct += 1
    #e_with_p = ce
    e_with_p = ce

    #print(e.value,'steals from',e_with_p.value)
    #print(e_with_p.value, e_with_p.value // 3)
    ce = inc(e_with_p)

    # i had originally approached it this way, but only got this mod 2 part from reading reddit thread
    # brute force approach took > 5 hours
    if len(elfpres) % 2 == 1:
        ce = inc(ce)
    elfpres.remove(e_with_p)
    e = inc(e)
    if len(elfpres) % 100000 == 0:
        print(len(elfpres))
    if len(elfpres) == 1:
        print(elfpres, e)
        break


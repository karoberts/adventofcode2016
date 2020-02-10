def test(s, abas):
    if len(s) < 3: return False
    if len(s) == 3:
        if s[0] == s[2] and s[0] != s[1]:
            abas.append((s[0], s[1], s[2]))
    for i, c in enumerate(s):
        if i > len(s) - 3: break
        if s[i] == s[i+2] and s[i] != s[i+1]:
            abas.append((s[i], s[i + 1], s[i + 2]))

# < 376

with open('7.txt') as f:
    ct = 0
    for line in (l.strip() for l in f):
        curlet = ''
        abas = []
        babs = []
        for c in line:
            if c == '[': 
                test(curlet, abas)
                curlet = ''
            elif c == ']': 
                test(curlet, babs)
                curlet = ''
            else:
                curlet += c
        test(curlet, abas)

        f = False
        for a in abas:
            for b in babs:
                if a[1] == b[0] and a[0] == b[1]:
                    ct += 1
                    f = True
                    break
            if f: break

print(ct)


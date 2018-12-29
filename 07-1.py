def testlet(s):
    if len(s) < 4: return False
    if len(s) == 4:
        return s[0] == s[3] and s[1] == s[2] and s[0] != s[1]
    for i, c in enumerate(s):
        if i > len(s) - 4: break
        if s[i] == s[i+3] and s[i+1] == s[i+2] and s[i] != s[i+1]:
            return True
    return False

with open('7.txt') as f:
    ct = 0
    for line in (l.strip() for l in f):
        st = 'LET'
        curlet = ''
        haslet = False
        hasinner = False
        for c in line:
            if c == '[': 
                st = 'INBR'
                haslet = haslet or testlet(curlet)
                curlet = ''
            elif c == ']': 
                st = 'LET'
                hasinner = hasinner or testlet(curlet)
                curlet = ''
            elif st == 'INBR':
                curlet += c
            elif st == 'LET':
                curlet += c
        haslet = haslet or testlet(curlet)
        if haslet and not hasinner:
            ct += 1

print(ct)


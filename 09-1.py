import re

pat = re.compile(r'\((\d+)x(\d+)\)')

def extract(l, p):
    m = pat.match(l, p)
    return (int(m.group(1)), int(m.group(2)), m.group(0))

with open('9.txt') as f:
    line = f.readline().strip()
    #line = 'X(8x2)(3x3)ABCY'
    #line = 'A(2x2)BCD(2x2)EFG'

    i = 0
    st = 'LET'
    mark = None
    outp = []
    while i < len(line):
        c = line[i]
        if c == '(':
            mark = extract(line, i)
            i += len(mark[2])
            for rep in range(0, mark[1]):
                for j in range(i, i + mark[0]):
                    outp.append(line[j])
            i += mark[0]
        else:
            outp.append(c)
            i += 1

    #print(''.join(outp))
    print(len(outp))
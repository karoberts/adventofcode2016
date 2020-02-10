import re

pat = re.compile(r'\((\d+)x(\d+)\)')

def extract(l, p):
    m = pat.match(l, p)
    if m is None: return None
    return (int(m.group(1)), int(m.group(2)), m.group(0))

def processq():
    global line

    ln = 0
    q = [(0, len(line), 1)]

    outer = 0
    inner = 0
    while len(q) > 0:
        outer += 1
        (pos, endpos, tx) = q.pop()
        beforeln = ln
        while (pos < endpos):
            inner += 1
            if line[pos] == '(':
                mark = extract(line, pos)
                if mark is None:
                    ln += 1
                    pos += 1
                    continue

                pos += len(mark[2])
                for rep in range(0, mark[1]):
                    for j in range(pos, pos + mark[0]):
                        # this block enables part 2
                        if line[j] == '(':
                            if rep == 0:
                                q.append((j, j + mark[0], mark[1] * tx))
                            break
                        else:
                            ln += 1
                pos += mark[0]
            else:
                ln += 1
                pos += 1
        ln += (ln - beforeln) * (tx - 1)
        if outer % 10000 == 0:
            print(len(q), outer, inner, ln)

    return ln

with open('9.txt') as f:
    line = f.readline().strip()
    #line = '(3x3)XYZ'
    #line = 'X(8x2)(3x3)ABCY'
    #line = '(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN'
    #line = '(27x12)(20x12)(13x14)(7x10)(1x12)A'

    #ln = 0
    #process(0, len(line))
    ln = processq()

    #print(''.join(outp))
    print(ln)
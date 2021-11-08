import re

# C# program finds the answer

program = []
with open('25.txt') as f:

    for line in (l.strip() for l in f):
        if line.startswith('#'):
            continue

        m = line.split(' ')
        program.append({'op': m[0], 'args': m[1:], 'l': line})

#printcode();
#exit()

def runit(a):

    regs = {'a': a, 'b': 0, 'c': 0, 'd': 0}

    # test
    #regs = {'a': 0, 'b': 0, 'c': 0, 'd': 0}

    ip = 0
    stmts = 0
    last_out = None
    outs = 0
    while True:
        if ip >= len(program) or ip < 0:
            print('HALT')
            break

        stmts += 1
        pline = program[ip]
        #print('ip={} {} {} ip='.format(ip, regs, pline['l']), end='')
        op = pline['op']
        i1 = pline['args'][0]
        n1 = int(i1) if i1 not in 'abcd' else None
        i2 = pline['args'][1] if len(pline['args']) > 1 else None
        n2 = int(i2) if i2 and i2 not in 'abcd' else None

        if op == 'cpy':
            if n2: continue
            regs[i2] = regs[i1] if n1 is None else n1
        elif op == 'inc':
            regs[i1] += 1
        elif op == 'dec':
            regs[i1] -= 1
        elif op == 'jnz':
            if (n1 and n1 != 0) or (n1 is None and regs[i1] != 0):
                ip += (n2 if n2 else regs[i2]) - 1
        elif op == 'out':
            #print(stmts, 'OUT:', regs[i1])
            if last_out is None:
                last_out = regs[i1]
            else:
                if regs[i1] == last_out:
                    #print(a, 'not right')
                    break
            last_out = regs[i1]
            outs += 1

        #print(ip, regs)

        ip += 1

        if (outs > 2500): 
            return True

        #print(ip, regs)

    return False

    #print(regs)

#runit(192)
#exit()

for a in range(0, 1000):
    if runit(a):
        print('found', a)
        exit()
print('none found')

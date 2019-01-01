import re

program = []
with open('12.txt') as f:

    for line in (l.strip() for l in f):
        if line.startswith('#'):
            continue

        m = line.split(' ')
        program.append({'op': m[0], 'args': m[1:], 'l': line})

#printcode();
#exit()

regs = {'a': 0, 'b': 0, 'c': 0, 'd': 0}

# part 2
regs = {'a': 0, 'b': 0, 'c': 1, 'd': 0}

ip = 0
stmts = 0
last_d = -1
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
        regs[i2] = regs[i1] if not n1 else n1
    elif op == 'inc':
        regs[i1] += 1
    elif op == 'dec':
        regs[i1] -= 1
    elif op == 'jnz':
        if (n1 and n1 != 0) or (regs[i1] != 0):
            ip += n2 - 1

    if regs['d'] != last_d :
        print(ip, regs)
        last_d = regs['d']

    ip += 1

    #print(ip, regs)

print(regs)

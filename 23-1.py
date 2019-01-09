import re

program = []
with open('23.txt') as f:

    for line in (l.strip() for l in f):
        if line.startswith('#'):
            continue

        m = line.split(' ')
        program.append({'op': m[0], 'args': m[1:], 'l': line})

#printcode();
#exit()

regs = {'a': 7, 'b': 0, 'c': 0, 'd': 0}

# test
#regs = {'a': 0, 'b': 0, 'c': 0, 'd': 0}

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
        if n2: continue
        regs[i2] = regs[i1] if n1 is None else n1
    elif op == 'inc':
        regs[i1] += 1
    elif op == 'dec':
        regs[i1] -= 1
    elif op == 'jnz':
        if (n1 and n1 != 0) or (regs[i1] != 0):
            ip += (n2 if n2 else regs[i2]) - 1
    elif op == 'tgl':
        inst = ip + (n1 if n1 else regs[i1])
        if inst >= 0 and inst < len(program):
            inst = program[inst]
            if inst['op'] == 'inc': inst['op'] = 'dec'
            elif inst['op'] == 'dec' or inst['op'] == 'tgl': inst['op'] = 'inc'
            elif inst['op'] == 'jnz': inst['op'] = 'cpy'
            elif inst['op'] == 'cpy': inst['op'] = 'jnz'


    #print(ip, regs)

    ip += 1

    #print(ip, regs)

print(regs)


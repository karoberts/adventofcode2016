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

ip = 0

regs = {'a': 7, 'b': 0, 'c': 0, 'd': 0}

# part 2
part2 = True
regs = {'a': 12, 'b': 0, 'c': 0, 'd': 0}
#regs = {'a': 479_001_600, 'b': 1, 'c': -16, 'd': 479_001_600}

# figured this out from the C# program (C# is SO much faster than python)
regs = {'a': 479001600,'b': 1,'c': 2,'d': 0}
program[20]['op'] = 'cpy'
program[22]['op'] = 'dec'
program[24]['op'] = 'dec'
ip = 16

#   ip = 5
#   regs = {'a': 479_001_499, 'b': 479001599, 'c': 100, 'd': 479001599}
#   regs = {'a': 958_003_188, 'b': 479001599, 'c': 10, 'd': 479001598}
#   regs = {'a': 1_437_004_787, 'b': 479001599, 'c': 10, 'd': 479001597}
#   regs = {'a': 1_916_006_376, 'b': 479001599, 'c': 10, 'd': 479001596}
#   regs = {'a': 229_442_531_365_555_202, 'b': 479001599, 'c': 10, 'd': 1}
#   ip = 13
#   regs = {'a': 229_442_531_365_555_212, 'b': 479001599, 'c': 479001599, 'd': 1}
#   ip = 5
#   regs = {'a': 479_001_589, 'b': 479001599, 'c': 10, 'd': 229442531365555212}

# test
#regs = {'a': 0, 'b': 0, 'c': 0, 'd': 0}

# > 479,001,600
# < 229442531365555202

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
            ipdelt = (n2 if n2 else regs[i2])
            ip += ipdelt - 1
    elif op == 'tgl':
        print('toggle', pline, regs, ip);
        inst = ip + (n1 if n1 else regs[i1])
        if inst >= 0 and inst < len(program):
            inst = program[inst]
            if inst['op'] == 'inc': inst['op'] = 'dec'
            elif inst['op'] == 'dec' or inst['op'] == 'tgl': inst['op'] = 'inc'
            elif inst['op'] == 'jnz': inst['op'] = 'cpy'
            elif inst['op'] == 'cpy': inst['op'] = 'jnz'

    #if regs['d'] > last_d:
        #last_d = regs['d']
        #print(ip, regs)

    ip += 1

    #print(ip, regs, pline)

print(regs)


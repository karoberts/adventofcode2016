import re
import json
from collections import Counter
from itertools import takewhile
from collections import namedtuple

pat = re.compile(r'^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)|value (\d+) goes to bot (\d+)$')

def give_chip(bot, chip):
    if bot['v1'] == -1:
        bot['v1'] = chip
    elif bot['v2'] == -1:
        bot['v2'] = chip
    else:
        raise('wtf')

def get_highlow(bot):
    return (min(bot['v1'], bot['v2']), max(bot['v1'], bot['v2']))

with open('10.txt') as f:
    bots = {}
    botvals = {}
    outputs = {}
    for line in (l.strip() for l in f):
        #print(line)
        m = pat.match(line)
        if m.group(1):
            bot_id = int(m.group(1))
            low_b = int(m.group(3))
            high_b = int(m.group(5))
            low_t = 'o' if m.group(2) == 'output' else 'b'
            high_t = 'o' if m.group(4) == 'output' else 'b'

            bots[bot_id] = {'id':bot_id, 'low':low_b, 'low_t':low_t, 'high':high_b, 'high_t': high_t, 'v1':-1, 'v2':-1}
            if low_b < 0: outputs[-low_b] = -1
            if high_b < 0: outputs[-high_b] = -1
        else:
            botvals[int(m.group(6))] = int(m.group(7))

    #print(json.dumps(bots, indent=2))
    #print(json.dumps(botvals, indent=2))
    #print(outputs)

    for chip, bot_id in botvals.items():
        give_chip(bots[bot_id], chip)

    while True:
        bots_who_gave = 0
        for bot in bots.values():
            if bot['v1'] != -1 and bot['v2'] != -1:
                bots_who_gave += 1
                (l, h) = get_highlow(bot)
                if l == 17 and h == 61:
                    print('part1 bot', bot['id'])
                if bot['low_t'] == 'b':
                    give_chip(bots[bot['low']], l)
                else:
                    outputs[bot['low']] = l
                if bot['high_t'] == 'b':
                    give_chip(bots[bot['high']], h)
                else:
                    outputs[bot['high']] = h
                bot['v1'] = -1
                bot['v2'] = -1
        if bots_who_gave == 0:
            break

    #print(json.dumps(outputs, indent=2))
    print('part2', outputs[0] * outputs[1] * outputs[2] )
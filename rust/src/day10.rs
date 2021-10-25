use regex::Regex;
use std::cell::{RefCell};

use super::utils;

lazy_static! {
    static ref PAT: Regex = Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)|value (\d+) goes to bot (\d+)$").unwrap();
}

struct Bot
{
    id: i32,
    low: i32,
    high: i32,
    high_t: char,
    low_t: char,
    v1: i32,
    v2: i32
}

fn give_chip(bot:&mut Bot, chip:i32)
{
    if bot.v1 == -1 {
        bot.v1 = chip;
    }
    else if bot.v2 == -1 {
        bot.v2 = chip;
    }
    else {
        panic!();
    }
}

fn get_highlow(bot:&Bot) -> (i32, i32) {
    (std::cmp::min(bot.v1, bot.v2), std::cmp::max(bot.v1, bot.v2))
}

pub fn _run() 
{
    let s = utils::run_timer_start(10, 1);

    let lines = utils::read_lines("../10.txt").unwrap();

    let mut bots : utils::HashMapFnv<i32, RefCell<Bot>> = fastmap!();
    let mut outputs : utils::HashMapFnv<i32, i32> = fastmap!();
    let mut botvals : utils::HashMapFnv<i32, i32> = fastmap!();

    for line in lines.map(|l| l.unwrap()) {
        let caps = PAT.captures(&line).unwrap();

        if caps.get(1).is_some() {
            let bot_id = utils::cap_to::<i32>(caps.get(1));
            let low_b = utils::cap_to::<i32>(caps.get(3));
            let high_b = utils::cap_to::<i32>(caps.get(5));
            let low_t_str = utils::cap_to_string(caps.get(2));
            let low_t = if low_t_str == "output" {'o'} else {'b'};
            let high_t_str = utils::cap_to_string(caps.get(4));
            let high_t = if high_t_str == "output" {'o'} else {'b'};

            let bot = Bot { id: bot_id, low: low_b, low_t: low_t, high: high_b, high_t: high_t, v1: -1, v2: -1 };
            bots.insert(bot_id, RefCell::new(bot));

            if low_b < 0 { outputs.insert(-low_b, -1); }
            if high_b < 0 { outputs.insert(-high_b, -1); }
        }
        else {
            botvals.insert( utils::cap_to::<i32>(caps.get(6)), utils::cap_to::<i32>(caps.get(7)) );
        }
    }

    for (chip, bot_id) in botvals {
        bots.entry(bot_id).and_modify(|e| give_chip(&mut e.borrow_mut(), chip));
    }

    loop {
        let mut bots_who_gave = 0;
        for (_, botcell) in bots.iter() {
            let bot = &mut botcell.borrow_mut();
            if bot.v1 != -1 && bot.v2 != -1 {
                bots_who_gave += 1;
                let (l, h) = get_highlow(&bot);
                if l == 17 && h == 61 {
                    utils::run_timer_end(s, bot.id);
                }
                if bot.low_t == 'b' {
                    let low_bot = bots.get(&bot.low).unwrap();
                    give_chip(&mut low_bot.borrow_mut(), l);
                }
                else {
                    outputs.insert(bot.low, l);
                }
                if bot.high_t == 'b' {
                    let high_bot = bots.get(&bot.high).unwrap();
                    give_chip(&mut high_bot.borrow_mut(), h);
                }
                else {
                    outputs.insert(bot.high, h);
                }
                bot.v1 = -1;
                bot.v2 = -1;
            }
        }
        if bots_who_gave == 0 {
            break;
        }
    }

    utils::run_timer_start(10, 2);
    utils::run_timer_end(s, outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap());
}

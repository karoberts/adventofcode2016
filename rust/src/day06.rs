use counter::Counter;

use super::utils;

pub fn _run() 
{
    let s = utils::run_timer_start(6, 1);

    let mut part1 = String::from("");
    let mut part2 = String::from("");

    let lines = utils::read_lines("../6.txt").unwrap();
    let mut cols : Vec<Vec<char>> = vec!();
    for _ in 0..8 {
        cols.push(Vec::new());
    }

    for line in lines.map(|l| l.unwrap()) {
        for (i, c) in line.chars().enumerate() {
            cols[i].push(c);
        }
    }

    for col in cols {
        let counter = col.iter().collect::<Counter<_>>();
        let mc = counter.most_common();

        part1.push( *mc[0].0 );
        part2.push( *mc.last().unwrap().0 );
    }

    utils::run_timer_end(s, part1);

    utils::run_timer_start(6, 2);
    utils::run_timer_end(None, part2);
}

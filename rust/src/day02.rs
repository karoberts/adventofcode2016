use std::iter::FromIterator;

use super::utils;

fn run(keys: &utils::HashMapFnv<(i32,i32), char>, code:&mut Vec<char>)
{
    let lines = utils::read_lines("../2.txt").unwrap();

    let mut pos = (0, 0);

    for line in lines.map(|l| l.unwrap()) {
        for c in line.chars() {
            let mut npos = pos.clone();
            match c {
                'U' => npos.1 -= 1,
                'D' => npos.1 += 1,
                'R' => npos.0 += 1,
                'L' => npos.0 -= 1,
                _ => panic!()
            }

            if keys.contains_key(&npos) {
                pos = npos
            }
        }
        code.push( *keys.get(&pos).unwrap() );
    }
}

pub fn _run() 
{
    let s = utils::run_timer_start(2, 1);
    let mut keys1 : utils::HashMapFnv<(i32,i32), char> = fastmap!();
    keys1.insert((0,0), '5');
    keys1.insert((1,0), '6');
    keys1.insert((-1,0), '4');
    keys1.insert((-1,-1), '1');
    keys1.insert((0,-1), '2');
    keys1.insert((1,-1),  '3');
    keys1.insert((-1,1), '7');
    keys1.insert((0,1), '8');
    keys1.insert((1,1), '9');
    let mut code : Vec<char> = vec!();
    run(&keys1, &mut code);
    utils::run_timer_end(s, String::from_iter(code));

    let s = utils::run_timer_start(2, 2);
    code = vec!();
    let mut keys2 : utils::HashMapFnv<(i32,i32), char> = fastmap!();
    keys2.insert((0,0), '5');
    keys2.insert((1,0), '6');
    keys2.insert((2,0), '7');
    keys2.insert((3,0), '8');
    keys2.insert((4,0), '9');
    keys2.insert((1,-1), '2');
    keys2.insert((2,-1), '3');
    keys2.insert((3,-1), '4');
    keys2.insert((2,-2), '1');
    keys2.insert((1,1), 'A');
    keys2.insert((2,1), 'B');
    keys2.insert((3,1), 'C');
    keys2.insert((2,2),  'D');

    run(&keys2, &mut code);
    utils::run_timer_end(s, String::from_iter(code));
}

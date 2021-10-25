use super::utils;
use std::fs;

fn manhat_dist(x1:i32,y1:i32,x2:i32,y2:i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn check_visit(pos:&mut (i32,i32), i:i32, d:i32, visits:&mut utils::HashSetFnv<(i32,i32)>, part2:&mut bool, part2ans:&mut Option<i32>)
{
    let s = if d < 0 {-1} else {1};
    let pi = if i == 1 {pos.1} else {pos.0};

    let start = pi + s;
    let end = pi + d + s;
    let mut cur = start;

    loop {
        let x = if i == 1 {pos.0} else {cur};
        let y = if i == 0 {pos.1} else {cur};
        let k = (x,y);
        if !*part2 && visits.contains(&k) {
            *part2ans = Some(manhat_dist(0,0,k.0,k.1));
            *part2 = true;
        }
        visits.insert(k);

        cur += s;
        if cur == end {
            break;
        }
    }

    if i == 1 {
        pos.1 += d;
    }
    else {
        pos.0 += d;
    }
}

pub fn _run() 
{
    let s = utils::run_timer_start(1, 1);

    let contents = fs::read_to_string("../1.txt").unwrap();

    let mut dirs : Vec<(&str, i32)> = vec!();
    for d in contents.split(",").map(|x| x.trim()) {
        dirs.push( (&d[0..1], d[1..].parse::<i32>().unwrap()) );
    }

    let mut visits : utils::HashSetFnv<(i32,i32)> = fastset!();

    let mut pos = (0,0);
    let mut direc = 0;
    let mut part2 = false;
    let mut part2ans : Option<i32> = None;

    visits.insert((0,0));

    for d in dirs {
        direc += if d.0 == "L" {1} else {-1};
        if direc < 0 {
            direc += 4;
        }
        else {
            direc %= 4;
        }

        match direc {
            0 => check_visit(&mut pos, 1, d.1, &mut visits, &mut part2, &mut part2ans),
            1 => check_visit(&mut pos, 0, -d.1, &mut visits, &mut part2, &mut part2ans),
            2 => check_visit(&mut pos, 1, -d.1, &mut visits, &mut part2, &mut part2ans),
            3 => check_visit(&mut pos, 0, d.1, &mut visits, &mut part2, &mut part2ans),
            _ => { println!("direct = {}", direc); panic!();}
        }
    }

    utils::run_timer_end(s, manhat_dist(0,0,pos.0,pos.1));

    utils::run_timer_start(1, 2);
    utils::run_timer_end(None, part2ans.unwrap());
}

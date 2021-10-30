use super::utils;

fn findrange(ranges:&Vec<(i64, i64)>, ip:i64) -> Option<&(i64, i64)>
{
    for rg in ranges {
        if rg.0 <= ip && rg.1 >= ip {
            return Some(rg);
        }
    }
    return None
}

fn findnext(ranges:&Vec<(i64, i64)>, ip:i64) -> Option<&(i64, i64)>
{
    for rg in ranges {
        if rg.0 > ip {
            return Some(rg);
        }
    }
    return None;
}

const MAX_IP : i64 = 4294967295;

pub fn _run() 
{
    let s = utils::run_timer_start(20, 1);

    let lines = utils::read_lines("../20.txt").unwrap();
    let mut ranges : Vec<(i64, i64)> = vec!();

    for line in lines.map(|l| l.unwrap()) {
        let sides : Vec<i64> = line.split('-').map(|x| x.parse::<i64>().unwrap()).collect();
        ranges.push( (sides[0], sides[1]) );
    }

    ranges.sort_by(|a, b| {
        if a.0 == b.0 { (a.1 - a.0).cmp(&(b.1 - b.0)) } else { a.0.cmp(&b.0) }
    });

    let mut ip : i64 = 0;
    let mut ct = 0;
    let mut part1 = false;
    while ip <= MAX_IP {
        let rg = findrange(&ranges, ip);
        if rg.is_none() {
            if !part1 {
                utils::run_timer_end(None, ip);
                utils::run_timer_start(20, 2);
                part1 = true;
            }
            let mut nrg = findnext(&ranges, ip);
            if nrg.is_none() {
                nrg = Some(&(MAX_IP + 1, MAX_IP + 1));
            }
            ct += nrg.unwrap().0 - ip;
            ip = nrg.unwrap().1;
        }
        else {
            ip = rg.unwrap().1;
            ip += 1;
        }
    }

    utils::run_timer_end(s, ct);
}

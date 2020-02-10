use regex::Regex;
use counter::Counter;

use super::utils;

fn test(name:&String, cksum:&String) -> bool {

    let counter = name.chars().collect::<Counter<_>>();
    let by_common = counter.most_common_tiebreaker(|&a, &b| a.cmp(&b));
    let ckc : Vec<char> = cksum.chars().collect();
    let mut ckpos : usize = 0;

    for c in by_common.iter() {
        if c.0 != ckc[ckpos] {
            return false;
        }
        ckpos += 1;
        if ckpos == 5 {
            return true;
        }
    }

    false
}

fn decrypt(name:&String, sector:i32) -> String {
    let mut outp = String::from("");
    for c in name.chars() {
        if c == '-' {
            outp.push(' ');
        }
        else {
            let diff = (c as u8) - ('a' as u8);
            let rot = ((diff as i32) + sector) % 26;
            let newc = (rot as u8) + ('a' as u8);
            outp.push( newc as char );
        }
    }
    return outp
}

pub fn _run() 
{
    let pat = Regex::new(r"^([a-z\-]+)-(\d+)\[([a-z]{5})\]$").unwrap();
    let lines = utils::read_lines("../4.txt").unwrap();
    let mut secsum = 0;
    let mut part2 : Option<i32> = None;

    for line in lines.map(|l| l.unwrap()) {
        let m = pat.captures(&line).unwrap();

        let name = utils::cap_to_string(m.get(1)).replace('-', "");
        let sector = utils::cap_to::<i32>(m.get(2));
        let cksum = utils::cap_to_string(m.get(3));

        if test(&name, &cksum) {
            secsum += sector;

            if part2.is_none() {
                let v = decrypt(&name, sector);
                if v.starts_with("north") {
                    part2 = Some(sector);
                }
            }
        }
    }

    println!("day04-1: {}", secsum);
    println!("day04-2: {}", part2.unwrap());
}

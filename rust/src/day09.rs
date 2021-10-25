use regex::Regex;
use std::collections::VecDeque;
use std::fs;

use super::utils;

lazy_static! {
    static ref PAT: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
}

fn extract(l:&String, p:usize) -> Option<(usize, usize, String)> {
    let m = PAT.captures(&l[p..]);
    if m.is_none() {
        return None;
    }
    let caps = m.unwrap();
    return Some(( utils::cap_to::<usize>(caps.get(1)),
        utils::cap_to::<usize>(caps.get(2)),
        utils::cap_to_string(caps.get(0)))
    );
}

fn processq(line_str:&String, part:i32) -> u64
{
    let line: Vec<char> = line_str.chars().collect();

    let mut ln : u64 = 0;
    let mut q: VecDeque<(usize, usize, u64)> = VecDeque::new();
    q.push_back((0, line.len(), 1));

    while q.len() > 0 {
        let (mut pos, endpos, tx) = q.pop_front().unwrap();
        let beforeln = ln;
        while pos < endpos {
            if line[pos] == '(' {
                let mark_opt = extract(line_str, pos);
                if mark_opt.is_none() {
                    ln += 1;
                    pos += 1;
                    continue;
                }

                let mark = mark_opt.unwrap();
                pos += mark.2.len();
                for rep in 0..mark.1 {
                    for j in pos..pos + mark.0 {
                        if part == 1 {
                            ln += 1;
                        }
                        else {
                            if line[j] == '(' {
                                if rep == 0 {
                                    q.push_back((j, j + mark.0, mark.1 as u64 * tx));
                                }
                                break;
                            }
                            else {
                                ln += 1;
                            }
                        }
                    }
                }
                pos += mark.0;
            }
            else {
                ln += 1;
                pos += 1;
            }
        }
        ln += (ln - beforeln) * (tx - 1);
    }

    return ln;
}

pub fn _run() 
{
    let line = fs::read_to_string("../9.txt").unwrap().trim_end().to_owned();

    let s = utils::run_timer_start(9, 1);
    utils::run_timer_end(s, processq(&line, 1));

    let s = utils::run_timer_start(9, 2);
    utils::run_timer_end(s, processq(&line, 2));
}

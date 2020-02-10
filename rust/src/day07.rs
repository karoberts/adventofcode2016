use super::utils;

fn testlet(s:&Vec<char>) -> bool {
    if s.len() < 4 { return false; }

    if s.len() == 4 {
        return s[0] == s[3] && s[1] == s[2] && s[0] != s[1];
    }
    for i in 0..s.len() {
        if i > s.len() - 4 { break; }
        if s[i] == s[i+3] && s[i+1] == s[i+2] && s[i] != s[i+1] {
            return true;
        }
    }
    return false;
}

pub fn _run() 
{
    let lines = utils::read_lines("../7.txt").unwrap();

    let mut ct = 0;
    for line in lines.map(|l| l.unwrap()) {
        let mut curlet : Vec<char> = vec!();
        let mut haslet = false;
        let mut hasinner = false;
        for c in line.chars() {
            match c {
                '[' => {
                    haslet |= testlet(&curlet);
                    curlet.clear();
                },
                ']' => {
                    hasinner |= testlet(&curlet);
                    curlet.clear();
                },
                _ => {
                    curlet.push(c);
                }
            }
        }
        haslet |= testlet(&curlet);
        if haslet && !hasinner {
            ct += 1;
        }
    }

    println!("day07-1: {}", ct);
}

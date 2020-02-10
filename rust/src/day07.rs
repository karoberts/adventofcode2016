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

fn test(s:&Vec<char>, abas:&mut Vec<(char, char, char)>) {
    if s.len() < 3 { return; }

    if s.len() == 3 {
        if s[0] == s[2] && s[0] != s[1] {
            abas.push( (s[0], s[1], s[2]) );
        }
        return;
    }

    for i in 0..s.len() {
        if i > s.len() - 3 { break; }
        if s[i] == s[i+2] && s[i] != s[i+1] {
            abas.push( (s[i], s[i + 1], s[i + 2]) );
        }
    }
}

pub fn _run() 
{
    let lines = utils::read_lines("../7.txt").unwrap();

    let mut ct = 0;
    let mut ct2 = 0;
    for line in lines.map(|l| l.unwrap()) {
        let mut curlet : Vec<char> = vec!();
        let mut haslet = false;
        let mut hasinner = false;

        let mut abas : Vec<(char,char,char)> = vec!();
        let mut babs : Vec<(char,char,char)> = vec!();

        for c in line.chars() {
            match c {
                '[' => {
                    haslet |= testlet(&curlet);
                    test(&curlet, &mut abas);
                    curlet.clear();
                },
                ']' => {
                    hasinner |= testlet(&curlet);
                    test(&curlet, &mut babs);
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

        test(&curlet, &mut abas);
        let mut f = false;
        for a in abas.iter() {
            for b in babs.iter() {
                if a.1 == b.0 && a.0 == b.1 {
                    ct2 += 1;
                    f = true;
                    break;
                }
            }
            if f { break; }
        }
    }

    println!("day07-1: {}", ct);
    println!("day07-2: {}", ct2);
}

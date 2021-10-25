use super::utils;

fn test(a:i32, b:i32, c:i32) -> bool {
    let mut v = vec![a, b, c];
    v.sort();
    v[0] + v[1] > v[2]
}

pub fn _run() 
{
    let s = utils::run_timer_start(3, 1);
    let lines = utils::read_lines("../3.txt").unwrap();
    let mut ct = 0;
    let mut groupings : Vec<Vec<i32>> = vec!();
    for line in lines.map(|l| l.unwrap()) {
        let mut ns : Vec<i32> = line.split(' ')
            .filter(|x| x.len() > 0)
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        groupings.push(ns.clone());
        ns.sort();
        if ns[0] + ns[1] > ns[2] {
            ct += 1;
        }
    }
    utils::run_timer_end(s, ct);

    let s = utils::run_timer_start(3, 2);
    ct = 0;
    for i in (0..groupings.len()).step_by(3) {
        for j in 0..3 {
            if test(groupings[i][j], groupings[i + 1][j], groupings[i + 2][j]) {
                ct += 1;
            }
        }
    }

    utils::run_timer_end(s, ct);
}

use super::utils;

#[derive(Clone, Debug)]
struct Disc
{
    p: i32,
    np: i32
}
            
// Disc #1 has 5 positions; at time=0, it is at position 2.
// Disc #2 has 13 positions; at time=0, it is at position 7.
// Disc #3 has 17 positions; at time=0, it is at position 10.
// Disc #4 has 3 positions; at time=0, it is at position 2.
// Disc #5 has 19 positions; at time=0, it is at position 9.
// Disc #6 has 7 positions; at time=0, it is at position 0.

fn check(discs: &Vec<Disc>) -> bool
{
    let mut i = 0;
    for d in discs {
        if d.p != (d.np - i).abs() % d.np {
            return true;
        }
        i += 1;
    }
    return false;
}

pub fn run(part2: bool) -> i32
{
    let mut discs : Vec<Disc> = vec!();

    let test = false;

    if test {
        discs.push(Disc { np:5, p:4 });
        discs.push(Disc { np:2, p:1 });
    }
    else
    {
        discs.push(Disc { np:5, p:2 });
        discs.push(Disc { np:13, p:7 });
        discs.push(Disc { np:17, p:10 });
        discs.push(Disc { np:3, p:2 });
        discs.push(Disc { np:19, p:9 });
        discs.push(Disc { np:7, p:0 });

        if part2 {
            discs.push(Disc { np:11, p:0 });
        }
    }

    for mut d in &mut discs {
        d.p = (d.p + 1) % d.np;
    }

    let mut t = 1;
    loop 
    {
        let mut tdelt = 0;
        let mut broke = false;
        for d in &discs {
            if (d.p + tdelt) % d.np != 0 {
                broke = true;
                break;
            }
            tdelt += 1;
        }

        if !broke {
            return t-1;
        }

        t += 1;
        for mut d in &mut discs {
            d.p = (d.p + 1) % d.np
        }
        // optimization, make sure discs are in right position, skipping disc 0 to pos 0 each try
        // reduces part 2 by 75%
        while check(&discs) {
            let delt = discs[0].np - discs[0].p;
            t += delt;
            for mut d in &mut discs {
                d.p = (d.p + delt) % d.np
            }
        }
    }
}

pub fn _run()
{
    utils::run_timer(|| run(false), 15, 1);
    utils::run_timer(|| run(true), 15, 2);
}
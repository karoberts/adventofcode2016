use super::utils::{self, HashSetFnv, HashMapFnv};
use std::time::{Instant, Duration};

fn part1(nelves:i32) -> i32
{
    let mut e = 0;
    let mut e_no_p : HashSetFnv<i32> = fastset!();
    let mut elfpres : HashMapFnv<i32,i32> = (0..nelves).map(|x| (x,1)).into_iter().collect();

    #[inline(always)]
    fn inc(e:i32, nelves:i32) -> i32
    {
        return (e + 1) % nelves;
    }

    fn find_e(e:i32, nelves:i32, elfpres:&HashMapFnv<i32,i32>) -> i32 
    {
        let mut ne = inc(e, nelves);
        while e != ne {
            if elfpres[&ne] > 0 {
                return ne;
            }
            ne = inc(ne, nelves);
        }
        return -1;
    }

    loop {
        if elfpres[&e] == 0 {
            e = inc(e, nelves); 
            continue;
        }
        let e_with_p = find_e(e, nelves, &elfpres);
        if e_with_p == -1 {
            return e + 1;
        }

        *elfpres.entry(e).or_default() += elfpres[&e_with_p];
        *elfpres.entry(e_with_p).or_default() = 0;
        e_no_p.insert(e_with_p);
        e = inc(e_with_p, nelves);
    }
}

fn part2(nelves:i32) -> i32
{
    let mut len : usize = nelves as usize;
    let mut elfpres : Vec<Option<i32>> = Vec::with_capacity(len);
    for i in 0..nelves {
        elfpres.push(Some(i+1));
    }

    fn go_next(e:usize, elfpres:&Vec<Option<i32>>, head: usize, tail: usize) -> usize
    {
        if e == tail {
            return head;
        }
        else {
            let mut ep = e + 1;
            while elfpres[ep].is_none() {
                ep += 1;
            }
            return ep;
        }
    }

    fn find_e(e:usize, elfpres:&Vec<Option<i32>>, head: usize, tail: usize, len:usize) -> usize
    {
        let sk = (len / 2) as usize;
        let mut ne = e;
        for _ in 0..sk {
            ne = go_next(ne, &elfpres, head, tail);
        }
        return ne;
    }

    let mut start = Instant::now();

    let mut head : usize = 0;
    let mut tail : usize = len - 1;
    let mut e : usize = head;
    loop {
        if len % 10000 == 0 {
            let duration = start.elapsed();
            println!("{:?}: head = {}, tail = {}, len = {}", duration, head, tail, len);
            start = Instant::now();
        }

        let e_with_p = find_e(e, &elfpres, head, tail, len);

        //print(e.value,'steals from',e_with_p.value)
        //println!("{} steals from {}", elfpres[e].unwrap(), elfpres[e_with_p].unwrap());
        elfpres[e_with_p] = None;
        len -= 1;
        if e_with_p == tail {
            while elfpres[tail].is_none() {
                tail -= 1;
            }
        }
        else if e_with_p == head {
            while elfpres[head].is_none() {
                head += 1;
            }
        }
        if len == 1 {
            for x in elfpres {
                if x.is_some() {
                    println!("{}", x.unwrap());
                }
            }
            break;
        }
        e = go_next(e, &elfpres, head, tail);
    }
    return 0;
}

pub fn _run() 
{
    //println!("day19-1: {}", part1(3014603));
    println!("day19-2: {}", part2(3014603));
}

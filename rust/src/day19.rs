use super::utils::{self, HashSetFnv, HashMapFnv};


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

pub fn _run() 
{
    println!("day19-1: {}", part1(3014603));
}

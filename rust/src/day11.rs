use super::utils;

fn setup(generators : &mut utils::HashSetFnv<String>, microchips : &mut utils::HashSetFnv<String>, floors : &mut Vec<utils::HashSetFnv<String>>, test: bool)
{
    floors.push(utils::HashSetFnv::<String>::default());
    floors.push(utils::HashSetFnv::<String>::default());
    floors.push(utils::HashSetFnv::<String>::default());
    floors.push(utils::HashSetFnv::<String>::default());

    if test {
        generators.insert("HG".to_owned());
        generators.insert("LG".to_owned());

        microchips.insert("HM".to_owned());
        microchips.insert("LM".to_owned());

        floors[0].insert("HM".to_owned());
        floors[0].insert("LM".to_owned());
        floors[1].insert("HG".to_owned());
        floors[2].insert("LG".to_owned());
    }
    else {
        generators.insert("PG".to_owned());
        generators.insert("TG".to_owned());
        generators.insert("RG".to_owned());
        generators.insert("CG".to_owned());
        generators.insert("pG".to_owned());

        microchips.insert("PM".to_owned());
        microchips.insert("TM".to_owned());
        microchips.insert("RM".to_owned());
        microchips.insert("CM".to_owned());
        microchips.insert("pM".to_owned());

        floors[0].insert("PG".to_owned());
        floors[0].insert("TG".to_owned());
        floors[0].insert("TM".to_owned());
        floors[0].insert("pG".to_owned());
        floors[0].insert("RG".to_owned());
        floors[0].insert("RM".to_owned());
        floors[0].insert("CG".to_owned());
        floors[0].insert("CM".to_owned());

        floors[1].insert("PM".to_owned());
        floors[1].insert("pM".to_owned());
    }
}

fn floors_to_key(floors: &Vec<utils::HashSetFnv<String>>, elev:i32) -> String
{
    let mut ab_s : String = String::new();

    for f in floors {
        let mut a_s = 0;
        let mut b_s = 0;
        for i in f {
            if i.ends_with("G") {
                a_s += 1;
            }
            else {
                b_s += 1;
            }
        }
        //ABs.append(('A' * As) + ('B' * Bs))
        let ab : String = std::iter::repeat("A").take(a_s).chain( std::iter::repeat("B").take(b_s) ).collect::<String>();
        ab_s.push_str(":");
        ab_s.push_str(ab.as_str());
    }

    //# optimization borrowed from reddit thread (after answering), part 1 now 2s, part 2 now 11s
    return elev.to_string() + "," + ab_s.as_str();
    //# my original, part 1 took 1m23s, part 2 took 85m
    //return str(elev) + ',' + str([sorted(i) for i in floors])
}

fn is_done(generators : &utils::HashSetFnv<String>, microchips : &utils::HashSetFnv<String>, floors : &Vec<utils::HashSetFnv<String>>, elev: i32) -> bool
{
    if elev != 3 {
        return false;
    }

    for fi in 0..3 {
        if floors[fi].len() != 0 {
            return false;
        }
    }
    return floors[3].len() == generators.len() + microchips.len();
}

fn to_gen(i:&String) -> String
{
    let chs = i.chars().collect::<Vec<char>>();
    let mut ret = String::new();
    ret.push(chs[0]);
    ret.push('G');
    return ret;
}

fn is_valid_floor(f: &utils::HashSetFnv<String>, generators : &utils::HashSetFnv<String>, microchips : &utils::HashSetFnv<String>) -> bool
{
    //println!("count = {:?} {:?} {:?}", generators.intersection(f).count(), f, generators.intersection(f));
    if generators.intersection(f).count() > 0 {
        for i in f.iter() {
            if microchips.contains(i) && !f.contains(&to_gen(i)) {
                return false;
            }
        }
    }

    return true;
}

fn is_valid_move(old_floor: &utils::HashSetFnv<String>, new_floor: &utils::HashSetFnv<String>, generators : &utils::HashSetFnv<String>, microchips : &utils::HashSetFnv<String>) -> bool
{
    //println!("v1: old {:?} new {:?}", old_floor, new_floor);
    let ov = is_valid_floor(old_floor, generators, microchips);
    let nv = is_valid_floor(new_floor, generators, microchips);
    //println!("v2: old {:?} new {:?}", ov, nv);
    return ov && nv;
}

#[derive(Clone, Debug)]
struct QItem
{
    floors: Vec<utils::HashSetFnv<String>>,
    elev: i32,
    steps: i32
}

fn deepcopy_floors(input : &Vec<utils::HashSetFnv<String>>) -> Vec<utils::HashSetFnv<String>>
{
    return input.iter().map(|h| h.clone()).collect();
}

fn deepcopy_qitems(input : &Vec<QItem>) -> Vec<QItem>
{
    return input.iter().map(|h| {
        let mut n = h.clone();
        n.floors = deepcopy_floors(&h.floors);
        return n;
    } ).collect();
}

fn bfs(generators : &utils::HashSetFnv<String>, microchips : &utils::HashSetFnv<String>, floors : &Vec<utils::HashSetFnv<String>>) -> i32
{
    let mut used_floors : utils::HashMapFnv<String, i32> = fastmap!();
    used_floors.insert(floors_to_key(&floors, 0), 0);

    let mut qnext : Vec<QItem> = vec!();
    let mut qnow : Vec<QItem> = vec!();

    qnow.push(QItem { floors: deepcopy_floors(floors), elev: 0, steps: 0});

    while qnow.len() > 0 {
        //println!("loop");
        for thismove in qnow.iter() {
            //println!("  iter {:?}", thismove);
            if is_done(generators, microchips, &thismove.floors, thismove.elev) {
                return thismove.steps;
            }

            let up = thismove.elev < 3;
            let mut down = thismove.elev > 0;
            let floor = &thismove.floors[thismove.elev as usize];

            // minor optimization, no need to move stuff down to lower, empty floors
            if thismove.elev == 1 && thismove.floors[0].len() == 0 {
                down = false;
            }
            if thismove.elev == 2 && thismove.floors[0].len() == 0 && thismove.floors[1].len() == 0 {
                down = false;
            }

            //println!("    up {:?} down {:?} floor {:?}", up, down, floor);

            let mut used : utils::HashSetFnv<String> = fastset!();
            let mut moves : Vec<(i32, String, Option<String>)> = vec!();

            // 0 == up
            // 1 == down
            for d in 0..2 {
                if d == 0 && !up { continue; }
                if d == 1 && !down { continue; }
                for i in floor.iter() {
                    moves.push((d, i.clone(), None));
                }
                for (ix, i) in floor.iter().enumerate() {
                    for (jx, j) in floor.iter().enumerate() {
                        if ix == jx { continue; }
                        let k = d.to_string() + std::cmp::min(i, j) + std::cmp::max(i, j);
                        if used.contains(&k) { continue; }
                        moves.push( (d, i.clone(), Some(j.clone())) );
                        used.insert(k);
                    }
                }
            }

            //println!("    moves {:?}", moves);

            for m in moves {
                //println!("      move {:?}", m);
                let mut new_floors = deepcopy_floors(&thismove.floors);
                let nx_floor_n = if m.0 == 0 {thismove.elev + 1} else {thismove.elev - 1};

                //println!("        new_floors {:?}", new_floors);

                new_floors[thismove.elev as usize].remove(&m.1);
                new_floors[nx_floor_n as usize].insert(m.1);

                match m.2 {
                    Some(x) => {
                        new_floors[thismove.elev as usize].remove(&x);
                        new_floors[nx_floor_n as usize].insert(x);
                    },
                    None => ()
                }
                //println!("        new_floors2 {:?}", new_floors);
                if is_valid_move(&new_floors[thismove.elev as usize], &new_floors[nx_floor_n as usize], generators, microchips) {
                    //println!("        valid");
                    let nk = floors_to_key(&new_floors, nx_floor_n);
                    let best_steps = if used_floors.contains_key(&nk) {used_floors.get(&nk)} else {None};
                    if best_steps.is_none() || thismove.steps < *best_steps.unwrap() {
                        used_floors.insert(nk, thismove.steps);
                        let q = QItem { floors: new_floors, elev: nx_floor_n, steps: thismove.steps + 1};
                        //println!("        next added {:?}", q);
                        qnext.push(q);
                    }
                }
            }

            //println!("    qnext {:?}, qnow {:?}", qnext, qnow);
            //return 0;
        }

        qnow = deepcopy_qitems(&qnext);
        qnext.clear();
    }
    return -1;
}

pub fn _run() 
{
    let mut generators : utils::HashSetFnv<String> = fastset!();
    let mut microchips : utils::HashSetFnv<String> = fastset!();
    let mut floors : Vec<utils::HashSetFnv<String>> = vec!();

    let test = false;

    setup(&mut generators, &mut microchips, &mut floors, test);

    let mut minsteps = bfs(&generators, &microchips, &floors);
    println!("day11-1: {}", minsteps);

    if test
    {
        return;
    }

    setup(&mut generators, &mut microchips, &mut floors, false);

    generators.insert("eG".to_owned());
    generators.insert("dG".to_owned());
    microchips.insert("eM".to_owned());
    microchips.insert("dM".to_owned());
    floors[0].insert("eG".to_owned());
    floors[0].insert("dG".to_owned());
    floors[0].insert("eM".to_owned());
    floors[0].insert("dM".to_owned());

    minsteps = bfs(&generators, &microchips, &floors);
    println!("day11-2: {}", minsteps);
}

use super::utils;

fn setup(generators : &mut utils::HashSetFnv<String>, microchips : &mut utils::HashSetFnv<String>, floors : &mut Vec<utils::HashSetFnv<String>>)
{
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

    floors.push(utils::HashSetFnv::<String>::default());
    floors.push(utils::HashSetFnv::<String>::default());
    floors.push(utils::HashSetFnv::<String>::default());
    floors.push(utils::HashSetFnv::<String>::default());

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
    if generators.union(f).count() > 0 {
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
    return is_valid_floor(old_floor, generators, microchips) && is_valid_floor(new_floor, generators, microchips);
}

#[derive(Clone, Debug)]
struct QItem
{
    floors: Vec<utils::HashSetFnv<String>>,
    elev: i32,
    steps: i32
}

fn bfs(generators : &utils::HashSetFnv<String>, microchips : &utils::HashSetFnv<String>, floors : &mut Vec<utils::HashSetFnv<String>>) -> i32
{
    let mut used_floors : utils::HashMapFnv<String, i32> = fastmap!();
    used_floors.insert(floors_to_key(&floors, 0), 0);

    let mut qnext : Vec<QItem> = vec!();
    let mut qnow : Vec<QItem> = vec!();

    qnow.push(QItem { floors: floors.clone(), elev: 0, steps: 0});

    loop {
        for thismove in qnow.iter() {
            if is_done(generators, microchips, &thismove.floors, thismove.elev) {
                return thismove.steps;
            }

            let up = thismove.elev < 3;
            let mut down = thismove.elev > 0;
            let floor = &floors[thismove.elev as usize];

            // minor optimization, no need to move stuff down to lower, empty floors
            if thismove.elev == 1 && thismove.floors[0].len() == 0 {
                down = false;
            }
            if thismove.elev == 2 && thismove.floors[0].len() == 0 && thismove.floors[1].len() == 0 {
                down = false;
            }

            let mut used : utils::HashSetFnv<String> = fastset!();
            let mut moves : Vec<(i32, String, Option<String>)> = vec!();

            for d in 0..1 {
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

            for m in moves {
                let mut new_floors = floors.clone();
                let nx_floor_n = if m.0 == 0 {thismove.elev + 1} else {thismove.elev - 1};

                new_floors[thismove.elev as usize].remove(&m.1);
                new_floors[nx_floor_n as usize].insert(m.1);
                match m.2 {
                    Some(x) => {
                        new_floors[thismove.elev as usize].remove(&x);
                        new_floors[nx_floor_n as usize].insert(x);
                    },
                    None => ()
                }
                if is_valid_move(&new_floors[thismove.elev as usize], &new_floors[nx_floor_n as usize], generators, microchips) {
                    let nk = floors_to_key(&new_floors, nx_floor_n);
                    let best_steps = if used_floors.contains_key(&nk) {used_floors.get(&nk)} else {None};
                    if best_steps.is_none() || thismove.steps < *best_steps.unwrap() {
                        used_floors.insert(nk, thismove.steps);
                        qnext.push(QItem { floors: new_floors, elev: nx_floor_n, steps: thismove.steps + 1});
                    }
                }
            }

            println!("qnext {:?}", qnext);
            return 0;
        }

        qnow = qnext.clone();
        qnext.clear();
    }
}

pub fn _run() 
{
    let mut generators : utils::HashSetFnv<String> = fastset!();
    let mut microchips : utils::HashSetFnv<String> = fastset!();
    let mut floors : Vec<utils::HashSetFnv<String>> = vec!();

    setup(&mut generators, &mut microchips, &mut floors);

    let minsteps = bfs(&generators, &microchips, &mut floors);
    println!("day11-01: {}", minsteps);
}

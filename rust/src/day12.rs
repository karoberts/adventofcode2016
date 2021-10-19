use super::utils;

#[derive(Clone, Debug)]
struct ProgItem
{
    op: String,
    arg1: String,
    arg1i: Option<i32>,
    arg2: Option<String>,
    arg2i: Option<i32>,
    line: String
}

fn is_abcd(s: &String) -> bool
{
    return match s.as_str()
    {
        "a" | "b" | "c" | "d" => true,
        _ => false
    }
}

fn run(program : &Vec<ProgItem>, regs: &mut utils::HashMapFnv<String, i32>)
{
    let mut ip : i32 = 0;
    let mut last_d : i32 = -1;
    loop
    {
        if ip >= program.len() as i32 || ip < 0
        {
            //println!("HALT");
            break;
        }

        let pline = &program[ip as usize];

        match pline.op.as_str() {
            "cpy" => regs.insert(pline.arg2.as_ref().unwrap().to_owned(), pline.arg1i.or_else(|| Some(regs[&pline.arg1])).unwrap()),
            "inc" => regs.insert(pline.arg1.to_owned(), regs[&pline.arg1] + 1),
            "dec" => regs.insert(pline.arg1.to_owned(), regs[&pline.arg1] - 1),
            "jnz" => None,
            _ => panic!("unsupported op")
        };

        if pline.op == "jnz" {
            if (pline.arg1i.is_some() && pline.arg1i.unwrap() != 0) || (regs[&pline.arg1] != 0) {
                ip += pline.arg2i.unwrap() - 1;
            }
        }

        if regs["d"] != last_d
        {
            last_d = regs["d"];
        }

        ip += 1;
    }
}

pub fn _run() 
{
    let lines = utils::read_lines("../12.txt").unwrap();
    let mut program : Vec<ProgItem> = vec!();

    for line in lines.map(|l| l.unwrap()) {
        if line.starts_with('#') { continue; }

        let m : Vec<&str> = line.split(' ').collect();
        let p : ProgItem = ProgItem {
            op: m[0].to_owned(), 
            arg1: m[1].to_owned(), 
            arg1i: if is_abcd(&m[1].to_owned()) { None } else { Some(m[1].parse::<i32>().expect("")) },
            arg2: if m.len() == 2 {None} else {Some(m[2].to_owned())},
            arg2i: if m.len() == 2 || is_abcd(&m[2].to_owned()) { None } else { Some(m[2].parse::<i32>().expect("")) },
            line: line
        };
        program.push(p);
    }

    let mut regs = hashmap!["a".to_owned() => 0, "b".to_owned() => 0, "c".to_owned() => 0, "d".to_owned() => 0];
    run(&program, &mut regs);
    println!("day12-1: {}", regs["a"]);

    regs = hashmap!["a".to_owned() => 0, "b".to_owned() => 0, "c".to_owned() => 1, "d".to_owned() => 0];
    run(&program, &mut regs);
    println!("day12-2: {}", regs["a"]);
}

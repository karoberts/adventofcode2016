use super::utils;

#[derive(Clone, Debug)]
struct ProgItem
{
    op: String,
    arg1: Option<char>,
    arg1i: Option<i32>,
    arg2: Option<char>,
    arg2i: Option<i32>
}

fn is_abcd(s: &String) -> bool
{
    return match s.as_str()
    {
        "a" | "b" | "c" | "d" => true,
        _ => false
    }
}

fn run(program : &Vec<ProgItem>, regs: &mut utils::HashMapFnv<char, i32>) -> bool
{
    let mut ip : i32 = 0;
    let mut last_out : Option<i32> = None;
    let mut outs = 0;
    loop
    {
        if ip >= program.len() as i32 || ip < 0
        {
            println!("HALT");
            return false;
        }

        let pline = &program[ip as usize];
        let op = pline.op.as_str();

        if op == "cpy" {
            let arg = pline.arg1i.or_else(|| Some(regs[&pline.arg1.unwrap()]));
            *regs.entry(pline.arg2.unwrap()).or_default() = arg.unwrap();
        }
        else if op == "inc" {
            *regs.entry(pline.arg1.unwrap()).or_default() += 1;
        }
        else if op == "dec" { 
            *regs.entry(pline.arg1.unwrap()).or_default() -= 1;
        }
        else if op == "jnz" {
            //println!("line: {:?}", pline);
            if (pline.arg1i.is_some() && pline.arg1i.unwrap() != 0) || (pline.arg1i.is_none() && regs[&pline.arg1.unwrap()] != 0) {
                ip += pline.arg2i.unwrap() - 1;
            }
        }
        else if op == "out" {
            //println!("OUT: {}", regs[&pline.arg1.unwrap()]);
            if last_out.is_some() {
                if last_out.unwrap() == regs[&pline.arg1.unwrap()] {
                    return false;
                }
            }
            last_out = Some(regs[&pline.arg1.unwrap()]);
            outs += 1;
        }

        if outs > 2500 {
            return true;
        }

        ip += 1;
    }
}

pub fn _run() 
{
    let lines = utils::read_lines("../25.txt").unwrap();
    let mut program : Vec<ProgItem> = vec!();

    for line in lines.map(|l| l.unwrap()) {
        if line.starts_with('#') { continue; }

        let m : Vec<&str> = line.split(' ').collect();
        let p : ProgItem = ProgItem {
            op: m[0].to_owned(), 
            arg1: if is_abcd(&m[1].to_owned()) { Some(m[1].chars().nth(0).unwrap()) } else { None }, 
            arg1i: if is_abcd(&m[1].to_owned()) { None } else { Some(m[1].parse::<i32>().expect("")) },
            arg2: if m.len() == 3 && is_abcd(&m[2].to_owned()) { Some(m[2].chars().nth(0).unwrap()) } else { None }, 
            arg2i: if m.len() == 2 || is_abcd(&m[2].to_owned()) { None } else { Some(m[2].parse::<i32>().expect("")) }
        };
        program.push(p);
    }

    let s = utils::run_timer_start(25, 1);

    for a in 0..1000 {
        let mut regs = hashmap!['a' => a, 'b' => 0, 'c' => 0, 'd' => 0];
        if run(&program, &mut regs) {
            utils::run_timer_end(s, a);
            break;
        }
    }
}

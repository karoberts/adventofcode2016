use super::utils;

#[derive(Clone, Debug)]
struct ProgItem
{
    op: String,
    arg1: Option<char>,
    arg1i: Option<i32>,
    arg2: Option<char>,
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

fn run(program : &Vec<ProgItem>, regs: &mut utils::HashMapFnv<char, i32>)
{
    let mut ip : i32 = 0;
    loop
    {
        if ip >= program.len() as i32 || ip < 0
        {
            //println!("HALT");
            break;
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
            if (pline.arg1i.is_some() && pline.arg1i.unwrap() != 0) || (regs[&pline.arg1.unwrap()] != 0) {
                ip += pline.arg2i.unwrap() - 1;
            }
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
            arg1: if is_abcd(&m[1].to_owned()) { Some(m[1].chars().nth(0).unwrap()) } else { None }, 
            arg1i: if is_abcd(&m[1].to_owned()) { None } else { Some(m[1].parse::<i32>().expect("")) },
            arg2: if m.len() == 3 && is_abcd(&m[2].to_owned()) { Some(m[2].chars().nth(0).unwrap()) } else { None }, 
            arg2i: if m.len() == 2 || is_abcd(&m[2].to_owned()) { None } else { Some(m[2].parse::<i32>().expect("")) },
            line: line
        };
        program.push(p);
    }

    let mut regs = hashmap!['a' => 0, 'b' => 0, 'c' => 0, 'd' => 0];
    run(&program, &mut regs);
    println!("day12-1: {}", regs[&'a']);

    let mut regs = hashmap!['a' => 0, 'b' => 0, 'c' => 1, 'd' => 0];
    run(&program, &mut regs);
    println!("day12-2: {}", regs[&'a']);
}

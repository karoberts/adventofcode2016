extern crate crypto;
  
use crypto::md5::Md5;
use crypto::digest::Digest;
use super::utils;

fn gethash(n : i32, stretch: i32, hasher : &mut Md5) -> String
{
    let mut md5 = format!("ngcjuoqr{}", n);
    for _ in 0..stretch {
        hasher.input(md5.as_bytes());

        md5 = hasher.result_str();
        hasher.reset();
    }
    return md5;
}

fn has_triplet(s: &String) -> Option<char>
{
    let cs : Vec<char> = s.chars().collect();
    for i in 0..(cs.len()-2) {
        if cs[i] == cs[i+1] && cs[i] == cs[i+2] {
            return Some(cs[i]);
        }
    }
    return None;
}

fn has_sextupletlet(s: &String, c: char) -> bool
{
    let cs : Vec<char> = s.chars().collect();
    for i in 0..(cs.len()-5) {
        if cs[i] == c && cs[i+1] == c && cs[i+2] == c && cs[i+3] == c && cs[i+4] == c {
            return true;
        }
    }
    return false;
}

fn run(stretch : i32) -> i32 {
    let mut hashes : utils::HashMapFnv<i32, String> = fastmap!();
    let mut keys : Vec<i32> = vec!();
    let mut n : i32 = 0;
    let mut hasher = Md5::new();

    while keys.len() < 80 
    {
        let m1: char;
        {
            let md5 : &String = hashes.entry(n).or_insert_with(|| gethash(n, stretch, &mut hasher));
            //println!("md5 = {:?}", md5);
            let maybem1 = has_triplet(md5);
            if maybem1.is_none() {
                n += 1;
                continue;
            }
            m1 = maybem1.unwrap();
            //println!("md5: {:?} {}", md5, m1);
        }

        for i in 0..1000 {
            let ni = n + i + 1;
            let nmd5 : &String = hashes.entry(ni).or_insert_with(|| gethash(ni, stretch, &mut hasher));

            if has_sextupletlet(nmd5, m1)
            {
                //println!("KEY: {:?}", nmd5);
                keys.push(n);
                break;
            }
        }

        n += 1;
    }

    return keys[63];
}

pub fn _run() 
{    
    println!("day14-1: {}", run(1));
    println!("day14-1: {}", run(2017));
}

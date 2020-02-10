extern crate crypto;
  
use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn to_chr(c:u8) -> char {
    return if c < 10 {
        c + ('0' as u8) 
    } 
    else { 
        c - 10 + ('a' as u8) 
    } as char
}

pub fn _run() 
{    
    let key = "cxdnnyjw".as_bytes();
    let mut n = 1;

    let mut hasher = Md5::new();

    let mut password1 = String::from("");
    let mut password2 = vec!['-'; 8];
    let mut p2reps = 0;

    while p2reps < 8
    {
        hasher.input(key);
        hasher.input(n.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        // "00000" is 2.5 bytes of 0
        if output[0] == 0 && output[1] == 0 && output[2] & 0xF0 == 0 {
            let c = (output[2] & 0x0F) as u8;
            if password1.len() < 8 {
                password1.push(to_chr(c));
            }

            if c < 8 && password2[c as usize] == '-' {
                let c2 = ((output[3] as u32 >> 4) & 0x0F) as u8;
                password2[c as usize] = to_chr(c2);
                p2reps += 1;
            }
        }

        n += 1;
        hasher.reset();
    }

    println!("day05-1: {}", password1);
    println!("day05-2: {}", password2.iter().collect::<String>());
}

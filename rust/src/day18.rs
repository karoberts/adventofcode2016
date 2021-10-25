use super::utils;

fn is_trap(prev_row : &Vec<bool>, i: usize, nlen: usize) -> bool
{
    let l = if i == 0 { false } else { prev_row[i-1] };
    let r = if i == nlen-1 { false } else { prev_row[i+1] };

    return ! (l == r);
}

fn run(nrows:i32) -> usize
{
    let row1 = "......^.^^.....^^^^^^^^^...^.^..^^.^^^..^.^..^.^^^.^^^^..^^.^.^.....^^^^^..^..^^^..^^.^.^..^^..^^^..";

    let nlen = row1.len();
    let mut row_data : Vec<bool> = row1.chars().map(|c| c == '^').collect();
    let mut nsafe = row_data.iter().filter(|x| !*x).count();

    let mut next_row_data = (0..nlen).map(|_| false).collect();
    let lists : [&mut Vec<bool>; 2] = [&mut row_data, &mut next_row_data];
    let mut lid = 0;

    for _ in 0..nrows - 1 {
        let nlid = (lid + 1) % 2;
        for i in 0..nlen {
            let n = is_trap(&lists[lid], i, nlen);
            lists[nlid][i] = n;
            if ! n {
                nsafe += 1;
            }
        }
        lid = nlid;
    }

    return nsafe;
}

pub fn _run() 
{
    utils::run_timer(|| run(40), 18, 1);
    utils::run_timer(|| run(400_000), 18, 2);
}

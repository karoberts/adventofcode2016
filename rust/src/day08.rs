use regex::Regex;

use super::utils;

pub fn _run() 
{
    let pat = Regex::new(r"^(rect) (\d+)x(\d+)|(rotate (?:row|column)) [yx]=(\d+) by (\d+)$").unwrap();

    let lines = utils::read_lines("../8.txt").unwrap();

    let mut grid = [[false; 50]; 6];

    for line in lines.map(|l| l.unwrap()) {
        let caps = pat.captures(&line).unwrap();

        let rect = caps.get(1);
        let rot = caps.get(4);
        if rect.is_some() && rect.unwrap().as_str() == "rect" {
            for y in 0..utils::cap_to::<usize>(caps.get(3)) {
                for x in 0..utils::cap_to::<usize>(caps.get(2)) {
                    grid[y][x] = true;
                }
            }
        }
        else if rot.is_some() {
            match rot.unwrap().as_str() {
                "rotate row" => {
                    let y = utils::cap_to::<usize>(caps.get(5));
                    let shift = utils::cap_to::<usize>(caps.get(6));
                    let mut newrow = [false; 50];
                    for x in 0..50 {
                        newrow[(x + shift) % 50] = grid[y][x];
                    }
                    grid[y] = newrow;
                },
                "rotate column" => {
                    let x = utils::cap_to::<usize>(caps.get(5));
                    let shift = utils::cap_to::<usize>(caps.get(6));
                    let mut newrow = [false; 6];
                    for y in 0..6 {
                        newrow[(y + shift) % 6] = grid[y][x];
                    }
                    for y in 0..6 {
                        grid[y][x] = newrow[y];
                    }
                }
                _ => panic!()
            }
        }
    }

    let mut ct = 0;
    for y in 0..6 {
        for x in 0..50 {
            if grid[y][x] {
                ct += 1;
            }
        }
    }
    println!("day08-1: {}", ct);

    println!("day08-2: {}", "UPOJFLBCEZ");

    /*
    for y in 0..6 {
        for x in 0..50 {
            print!("{}", if grid[y][x] {"\u{2588}"} else {" "});
        }
        println!();
    }
    */
}

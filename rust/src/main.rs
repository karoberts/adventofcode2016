use std::env;
use std::time::{Instant, Duration};

#[macro_use]
#[allow(unused_imports)]
extern crate lazy_static;

#[macro_use]
mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn run_timer(f : fn()) -> Duration
{
    let start = Instant::now();

    f();

    let duration = start.elapsed();
    println!(" ==> {:?}", duration);
    return duration;
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let funcs: Vec<Option<fn()>> = vec![
        Some(day01::_run),
        Some(day02::_run),
        Some(day03::_run),
        Some(day04::_run),
        Some(day05::_run),
        Some(day06::_run),
        Some(day07::_run),
        Some(day08::_run),
        Some(day09::_run),
        Some(day10::_run),
        Some(day11::_run),
        Some(day12::_run),
        Some(day13::_run),
        Some(day14::_run),
        Some(day15::_run),
        Some(day16::_run),
        Some(day17::_run),
        Some(day18::_run),
        Some(day19::_run),
        Some(day20::_run),
        Some(day21::_run),
        Some(day22::_run),
        Some(day23::_run),
        Some(day24::_run),
        Some(day25::_run)
    ];

    if args.len() > 1 {
        if args[1] == "all" {
            let mut total: Duration = Duration::from_secs(0);
            for f in funcs.iter().filter(|x| x.is_some()).map(|x| x.unwrap()) {
                total += run_timer(f);
            }
            println!();
            println!("  TOTAL: {:?}", total);
        }
        else {
            let f = funcs[ args[1].parse::<usize>().expect("invalid arg!") - 1 ];
            println!("Running Day {}", args[1]);
            run_timer(f.unwrap());
        }
    }
    else {
        run_timer( day02::_run );
    }
}

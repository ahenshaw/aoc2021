use argh::FromArgs;
use chrono::{Utc, Datelike}; 
use aoc2021::day_12::{input_generator, part1, part2};
use std::time::Instant;

#[derive(FromArgs)]
/// advent of code runner
struct Args {
    /// day
    #[argh(option, short = 'd')]
    day: Option<u32>,
    /// test mode
    #[argh(switch, short = 't')]
    test: bool,
}

fn main() {
    let args: Args = argh::from_env();
    let day = args.day.unwrap_or(Utc::now().day());
    let mode = if args.test {"test"} else {"day"};
    let filename = format!("input/2021/{}{}.txt", mode, day);
    println!("Using {}", filename);
    let input_day = std::fs::read_to_string(filename).unwrap();

    let input = input_generator(&input_day);

    let start = Instant::now();
    let result1 = part1(&input);
    println!("part 1: {} in {:?}", result1, Instant::now() - start);

    let start = Instant::now();
    let result2 = part2(&input);
    println!("part 2: {} in {:?}", result2, Instant::now() - start);
}

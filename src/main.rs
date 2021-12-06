use aoc2021::day_6::{part1, part2, input_generator};

fn main() {
    let input_day = include_str!("../input/2021/day6.txt");

    let input = input_generator(&input_day);

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}
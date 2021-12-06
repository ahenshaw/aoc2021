use sscanf::scanf;
use itertools::Itertools;


// #[derive(Debug)]
// pub struct Input {
//     a: usize,
// }
type Input = Vec<usize>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| {
        scanf!(line, "{}", usize).unwrap()
    }).collect()
}

pub fn part1(_input: &Input) -> usize {
    todo!()
}

pub fn part2(_input: &Input) -> usize {
    todo!()
}

type Input = Vec<i32>;

pub fn input_generator(input: &str) -> Input {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let max = input.iter().max().unwrap();
    (0..*max).map(|x| input.iter().map(|pos| (*pos-x).abs()).sum()).min().unwrap()
}

pub fn part2(input: &Input) -> i32 {
    let max = input.iter().max().unwrap();
    (0..*max).map(|x| input.iter().map(|pos| {
        let dx = (*pos-x).abs();
        ((dx + 1) * dx) / 2 
    })
    .sum())
    .min().unwrap()
}

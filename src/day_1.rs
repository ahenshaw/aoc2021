use sscanf::scanf;

type Input = Vec<usize>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| scanf!(line, "{}", usize).unwrap())
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input.windows(2).filter(|&x| x[1] > x[0]).count()
}

pub fn part2(input: &Input) -> usize {
    let w: Vec<usize> = input.windows(3).map(|x| x.iter().sum()).collect();
    w.windows(2).filter(|&x| x[1] > x[0]).count()
}

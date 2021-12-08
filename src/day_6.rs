type Input = Vec<usize>;

pub fn input_generator(input: &str) -> Input {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

pub fn part1(input: &Input) -> usize {
    calc(input, 80)
}

pub fn part2(input: &Input) -> usize {
    calc(input, 256)
}

fn calc(input: &Input, num_days: usize) -> usize {
    let mut swarm = Vec::from([0; 9]);
    input.iter().for_each(|&f| swarm[f] += 1);
    for _day in 0..num_days {
        swarm.rotate_left(1);
        swarm[6] += swarm[8];
    }
    swarm.iter().sum()
}

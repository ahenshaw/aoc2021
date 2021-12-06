type Input = Vec<usize>;

pub fn input_generator(input: &str) -> Input {
    input.split(',').map(|x| x.parse::<usize>().unwrap()).collect()
}

pub fn part1(input: &Input) -> usize {
    calc(input, 80)
}

pub fn part2(input: &Input) -> usize {
    calc(input, 256)
}

fn calc(input: &Input, num_days: usize) -> usize {
    let mut swarm: std::collections::VecDeque<usize> = std::collections::VecDeque::from([0;9]);
    input.iter().for_each(|&f| swarm[f] += 1);
    for _day in 0..num_days {
        let spawners = swarm.pop_front().unwrap();
        swarm[6] += spawners;
        swarm.push_back(spawners);
    }
    swarm.iter().sum()
}
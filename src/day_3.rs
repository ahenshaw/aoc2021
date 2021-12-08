use counter::Counter;

type Input = Vec<String>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|s| s.to_string()).collect()
}

fn get_stats(bits: &Input) -> Vec<Counter<char>> {
    (0..bits[0].len())
        .map(|i| {
            let mut count = Counter::<char>::new();
            bits.iter().for_each(|row| {
                count[&row.chars().nth(i).unwrap()] += 1;
            });
            count
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let stats = get_stats(&input);
    let gamma_s: String = stats.iter().map(|x| x.most_common()[0].0).collect();
    let epsilon_s: String = stats.iter().map(|x| x.most_common()[1].0).collect();
    let gamma = usize::from_str_radix(&gamma_s, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_s, 2).unwrap();
    gamma * epsilon
}

pub fn part2(input: &Input) -> usize {
    let mut oxygen = input.clone();
    let mut co2 = input.clone();
    let n = input[0].len();
    for i in 0..n {
        while oxygen.len() > 1 {
            let desired = get_stats(&oxygen)[i].most_common()[0].0;
            oxygen = oxygen
                .iter()
                .filter(|bits| bits.chars().nth(i).unwrap() == desired)
                .map(|x| x.clone())
                .collect();
        }
        while co2.len() > 1 {
            let desired = get_stats(&co2)[i].most_common()[1].0;
            co2 = co2
                .iter()
                .filter(|bits| bits.chars().nth(i).unwrap() == desired)
                .map(|x| x.clone())
                .collect();
        }
    }

    let co2 = usize::from_str_radix(&co2[0], 2).unwrap();
    let oxygen = usize::from_str_radix(&oxygen[0], 2).unwrap();
    co2 * oxygen
}

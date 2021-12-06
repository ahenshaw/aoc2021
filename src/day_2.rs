use sscanf::scanf;

type Input = Vec<(String, i32)>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| {
        scanf!(line, "{} {}", String, i32).unwrap()
    }).collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut depth = 0;
    let mut x = 0;
    for (cmd, value) in input {
        match cmd.as_ref() {
            "forward" => x += value,
            "down"    => depth += value,
            "up"      => depth -= value,
            _         => (),
        }
    }
    depth * x
}

pub fn part2(input: &Input) -> i32 {
    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;
    for (cmd, value) in input {
        match cmd.as_ref() {
            "forward" => {
                x     += value;
                depth += value*aim;
            },
            "down" => aim += value,
            "up"   => aim -= value,
            _      => (),
        }
    }
    depth * x
}

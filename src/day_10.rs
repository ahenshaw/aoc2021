type Input = Vec<Vec<char>>;

const ERROR_SCORE: [usize;4] = [3, 57, 1197, 25137];
const OPENER: &str = "([{<";
const CLOSER: &str = ")]}>";

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| {
        line.trim().chars().collect()
    }).collect()
}

pub fn part1(_: &Input) -> &str {
    "Answer given in part 2"
}

pub fn part2(input: &Input) -> usize {
    let mut total_error_score = 0;
    let mut scores = Vec::<usize>::new();
    'outer: for line in input {
        let mut stack = Vec::<usize>::new();
        for ch in line {
            if let Some(index) = OPENER.find(*ch) {
                stack.push(index)
            };
            if let Some(index) = CLOSER.find(*ch) {
                if index != stack.pop().unwrap() {
                    total_error_score += ERROR_SCORE[index];
                    continue 'outer;
                }    
            };
        }
        scores.push(stack.iter().rev().fold(0, |acc, &index| 5 * acc + index + 1));
    }
    println!("part 1: {}", total_error_score);
    scores.sort();
    scores[scores.len()/2]
}
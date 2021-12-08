use std::collections::HashSet;
type Item = HashSet<char>;
type Items = Vec<Item>;
type Input<'a> = Vec<(Items, Items)>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| {
        let (signals, digits) = line.split_once('|').unwrap();
        let signals: Items = signals.split_whitespace().map(|x| x.chars().collect()).collect();
        let digits:  Items = digits.split_whitespace().map(|x| x.chars().collect()).collect();
        (signals, digits)
    }).collect()
}

/// Ignore the signals for Part 1 - just need to count easily identifiable digits
pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(_, digits)| {
            digits.iter().filter(|digit| [2, 3, 4, 7].contains(&digit.len())).count()
        }).sum()
}

/// Identify the signals by deduction, then decode the digits
pub fn part2(input: &Input) -> usize {
    let mut d2s: Vec<Item> = (0..10).map(|_| HashSet::<char>::new()).collect();
    let mut total: usize = 0;
    for (sigs, digits) in input {
        d2s[1] = sigs.iter().filter(|s| s.len()==2).next().unwrap().clone();
        d2s[8] = sigs.iter().filter(|s| s.len()==7).next().unwrap().clone();
        d2s[7] = sigs.iter().filter(|s| s.len()==3).next().unwrap().clone();
        d2s[4] = sigs.iter().filter(|s| s.len()==4).next().unwrap().clone();

        d2s[9] = sigs.iter().filter(|s| s.len()==6 && s.is_superset(&d2s[4])).next().unwrap().clone();
        d2s[6] = sigs.iter().filter(|s| s.len()==6 && s.intersection(&d2s[1]).copied().collect::<Item>().len()==1).next().unwrap().clone();
        d2s[0] = sigs.iter().filter(|s| s.len()==6 && *s != &d2s[9] && *s != &d2s[6]).next().unwrap().clone();

        d2s[3] = sigs.iter().filter(|s| s.len()==5 && s.is_superset(&d2s[1])).next().unwrap().clone();
        d2s[2] = sigs.iter().filter(|s| s.len()==5 && s.intersection(&d2s[4]).copied().collect::<Item>().len()==2).next().unwrap().clone();
        d2s[5] = sigs.iter().filter(|s| s.len()==5 && *s != &d2s[3] && *s != &d2s[2]).next().unwrap().clone();

        let num: String = digits.iter().map(|digit| d2s.iter().position(|sig| sig==digit).unwrap().to_string()).collect();
        total += num.parse::<usize>().unwrap();
    }
    total
}

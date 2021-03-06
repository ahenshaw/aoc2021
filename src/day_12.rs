use sscanf::scanf;
use std::collections::{HashMap, HashSet};

type Input = HashMap<String, Vec<String>>;

pub fn input_generator(input: &str) -> Input {
    let mut tunnels = HashMap::<String, Vec<String>>::new();
    input.lines().for_each(|line| {
        let (left, right) = scanf!(line.trim(), "{}-{}", String, String).unwrap();
        tunnels.entry(left.clone()).or_insert(vec![]).push(right.clone());
        tunnels.entry(right).or_insert(vec![]).push(left);
    });
    tunnels
}

fn explore(tunnels: &Input, src: &str, mut path: Vec<String>, distinct: &mut HashSet<String>, is_part_2: bool) {
    path.push(src.to_owned());
    if src == "end" {
        distinct.insert(path.join("-"));
    } else {
        for dest in &tunnels[src] {
            let lower: Vec<&String> = path.iter().filter(|x| x.chars().nth(0).unwrap().is_ascii_lowercase()).collect();
            let any_doubles = lower.len() != lower.iter().collect::<HashSet<&&String>>().len();
            // println!("doubles: {}, lower: {:?} {:?}", &any_doubles, &lower, lower.iter().collect::<HashSet<&&String>>());
            if dest != "start" {
                let dest_cnt = path.iter().filter(|&x| x==dest).count();
                if dest.chars().nth(0).unwrap().is_ascii_uppercase() || dest_cnt < 1 || (is_part_2 && !any_doubles && dest_cnt < 2)  {
                     explore(tunnels, dest, path.clone(), distinct, is_part_2);
                }
            }
        }
    }
}

fn solve(tunnels: &Input, is_part_2: bool) -> usize {
    let mut distinct = HashSet::<String>::new();
    let path: Vec<String> = vec!["start".to_owned()];
    for dst in &tunnels["start"] {
        explore(tunnels, &dst, path.clone(), &mut distinct, is_part_2);
    }
    distinct.len()
}

pub fn part1(tunnels: &Input) -> usize {solve(tunnels, false)}
pub fn part2(tunnels: &Input) -> usize {solve(tunnels, true)}
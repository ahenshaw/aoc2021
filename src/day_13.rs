use sscanf::scanf;
use std::collections::HashSet;

type Grid = HashSet<(u32, u32)>;
#[derive(Debug)]
pub struct Input<'a> {
    grid: Grid,
    folds: std::str::Lines<'a>,
}

pub fn input_generator(input: &str) -> Input {
    let mut grid = Grid::new();
    let mut lines = input.lines();
    for line in &mut lines {
        if let Some(coord) = scanf!(line, "{},{}", u32, u32) {
            grid.insert(coord);
        } else {
            break;
        }
    }
    Input{grid, folds:lines} // unused portion of input is the fold instructions
}

pub fn part1(_input: &Input) -> String {
    "See answer in part 2".to_owned()
}

pub fn part2(input: &Input) -> usize {
    let mut grid = input.grid.clone();
    for fold in input.folds.clone() {
        let (axis, val) = scanf!(fold, "fold along {}={}", char, u32).unwrap();
        let mut folded = Grid::new();
        for (x, y) in grid {
            let y = if axis=='y' && y > val {2 * val - y} else {y};
            let x = if axis=='x' && x > val {2 * val - x} else {x};
            folded.insert((x, y));
        }
        grid = folded;
        println!("Active: {}", grid.len());
    }
    let cols = grid.iter().map(|g| g.0).max().unwrap();
    let rows = grid.iter().map(|g| g.1).max().unwrap();
    (0..=rows).for_each(|y| {
        (0..=cols).for_each(|x| {
            print!("{}", if grid.contains(&(x, y)) {'#'} else {' '});
        });
        println!();
    });
    0
}

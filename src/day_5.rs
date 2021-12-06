use sscanf::scanf;
use num::signum;

const N: usize = 1000;
type Input = Vec<(i32, i32, i32, i32)>;
type Grid = [[usize;N];N];

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| {
        scanf!(line, "{},{} -> {},{}", i32, i32, i32, i32).unwrap()
    }).collect()
}

fn draw_line(grid: &mut Grid, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dx = signum(x2 - x1);
    let dy = signum(y2 - y1);
    let mut x = x1;
    let mut y = y1;
    loop {
        grid[y as usize][x as usize] += 1;
        if x == x2 && y == y2 {
            break
        }
        x += dx; y += dy;
    }
}

pub fn part1(input: &Input) -> usize {
    let mut grid: Grid = [[0; N]; N];
    input.iter().for_each(|(x1, y1, x2, y2)| if x1==x2 || y1==y2 {draw_line(&mut grid, *x1, *y1, *x2, *y2)});
    grid.iter().flatten().filter(|&x| *x >= 2).count()
}

pub fn part2(input: &Input) -> usize {
    let mut grid: Grid = [[0; N]; N];
    input.iter().for_each(|(x1, y1, x2, y2)| draw_line(&mut grid, *x1, *y1, *x2, *y2));
    grid.iter().flatten().filter(|&x| *x >= 2).count()
}

use num::signum;
use sscanf::scanf;

const N: usize = 1000;
type Matrix = [[usize; N]; N];

#[derive(PartialEq, Clone, Copy)]
pub struct Point {
    x: i32, 
    y: i32
}
type Input = Vec<(Point, Point)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (x1, y1, x2, y2) = scanf!(line, "{},{} -> {},{}", i32, i32, i32, i32).unwrap();
            (Point{x: x1, y:y1}, Point{x: x2, y: y2})
        })
        .collect()
}

/// Draw horizontal, vertical, or strictly diagonal lines into a Matrix
fn draw_line(grid: &mut Matrix, start: Point, end: Point) {
    let dx = signum(end.x - start.x);
    let dy = signum(end.y - start.y);
    let mut p: Point = start;
    loop {
        grid[p.y as usize][p.x as usize] += 1;
        if p == end {
            break
        }
        p.x += dx;
        p.y += dy;
    }
}

pub fn part1(lines: &Input) -> usize {
    let mut grid: Matrix = [[0; N]; N];
    lines.iter()
        .filter(|(start, end)| start.x == end.x || start.y == end.y)  // only draw horizontal or vertical lines
        .for_each(|(start, end)| draw_line(&mut grid, *start, *end));
    grid.iter().flatten().filter(|&x| *x >= 2).count()
}

pub fn part2(input: &Input) -> usize {
    let mut grid: Matrix = [[0; N]; N];
    input
        .iter()
        .for_each(|(start, end)| draw_line(&mut grid, *start, *end));
    grid.iter().flatten().filter(|&x| *x >= 2).count()
}

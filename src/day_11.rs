use itertools::iproduct;
use itertools::Itertools;

type Input = Vec<Vec<u8>>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| {
        line.bytes().map(|b| b - b'0').collect()
    }).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut grid = input.clone();
    (0..100).map(|_| do_step(&mut grid)).sum::<usize>()
}

pub fn part2(input: &Input) -> usize {
    let mut grid = input.clone();
    (1..usize::MAX)
        .find(|_| do_step(&mut grid) == grid.len() * grid[0].len())
        .unwrap()
}

fn do_step(grid: &mut Input) -> usize {
    let cols = grid[0].len();
    let rows = grid.len();
    let mut stack: std::collections::VecDeque<(usize, usize)> = iproduct!(0..rows, 0..cols).collect();
    while let Some((y, x)) = stack.pop_front() {
        match grid[y][x] {
            10 => (),
            9 => {
                grid[y][x] += 1;
                stack.extend(neighbors(y, x, cols, rows));
             },
            _ => grid[y][x] += 1,
        }    
    }
    grid.iter_mut()
        .flat_map(|row| row.iter_mut())
        .filter_map(|x| if *x == 10 {Some(*x=0)} else {None})
        .count()
}

fn neighbors(y: usize, x: usize, cols: usize, rows: usize) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1)
    .cartesian_product(-1..=1)
    .map(move |(dy, dx)| ((y as i32 + dy) as usize, (x as i32 + dx) as usize))
    .filter(move |(ny, nx)| ny >= &0 && ny < &rows && nx >= &0 && nx < &cols && (ny != &y || nx != &x))
}
use ndarray::{Array2, s};
use itertools::iproduct;
use std::collections::HashSet;

type Matrix = Array2<u32>;
type Index = (usize, usize);

pub fn input_generator(input: &str) -> Matrix {
    let rows = input.lines().count();
    let d: Vec<u32> = input.lines().map(|line| {
        line.trim().chars().map(|c| c.to_digit(10).unwrap())
    }).flatten().collect();
    let cols = d.len()/rows;
    Array2::from_shape_vec((rows, cols), d).unwrap()
}

fn padded(m: &Matrix, p: u32) -> Matrix {
    let (rows, cols) = m.dim();
    let mut new = Matrix::from_elem((rows+2, cols+2), p);
    let mut a = new.slice_mut(s![1..rows+1,1..cols+1]);
    a.fill(0);
    a += m;
    new
}

fn low_points(m: &Matrix) -> Vec<((usize, usize), u32)> {
    let (rows, cols) = m.dim();
    let neighbors = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
    iproduct!(1..rows-1, 1..cols-1)
        .map(|(r, c)| ((r, c), m[[r, c]]))
        .filter(|((r, c), v)| {
            neighbors.iter().all(|(dr, dc)| v < &m[[(*r as i32+dr) as usize, (*c as i32+dc) as usize]])
        }).collect()
}

fn find_basin(m: &Matrix, p: &Index, visited: &mut HashSet<Index>) -> usize {
    let mut size: usize = 1;
    let neighbors = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
    visited.insert(*p);
    for (dr, dc) in neighbors {
        let row = ((p.0 as i32) + dr) as usize;
        let col = ((p.1 as i32) + dc) as usize;
        let index:Index = (row, col);
        if !visited.contains(&index) {
            let v = m[index];
            if v != 9 && v > m[[p.0, p.1]] {
                size += find_basin(m, &index, visited);
            }
        }
    }
    size
}

pub fn part1(input: &Matrix) -> u32 {
    let m = padded(input, 9);
    low_points(&m).iter().map(|(_, v)| v+1).sum()
}

pub fn part2(input: &Matrix) -> usize {
    let m = padded(input, 9);
    let mut sizes= Vec::<usize>::new();
    for ((r, c), _) in low_points(&m) {
        let mut visited = HashSet::<Index>::new();
        sizes.push(find_basin(&m, &(r, c), &mut visited));
    }
    sizes.sort();
    sizes.iter().rev().take(3).product()
}
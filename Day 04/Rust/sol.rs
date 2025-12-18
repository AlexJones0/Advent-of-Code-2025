/*
 * FILE: Day 04/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 4 problems (7 & 8) for Advent of Code 2025, solved in Rust.
 */
use std::fs;

const ADJACENT_DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type Pos = (usize, usize);

fn in_bounds(grid: &[Vec<u8>], pos: Pos) -> bool {
    pos.0 < grid.len() && pos.1 < grid[pos.0].len()
}

fn roll_neighbours(grid: &[Vec<u8>], pos: Pos) -> Vec<Pos> {
    ADJACENT_DIRS
        .iter()
        .map(|&(dx, dy)| ((pos.0 as i32 + dy) as usize, (pos.1 as i32 + dx) as usize))
        .filter(|&adj| in_bounds(grid, adj) && grid[adj.0][adj.1] == b'@')
        .collect()
}

fn remove_rolls(grid: &[Vec<u8>]) -> u32 {
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != b'@' {
                continue;
            }
            let neighbours = roll_neighbours(grid, (y, x)).len();
            total += (neighbours < 4) as u32;
        }
    }
    total
}

fn remove_all_rolls(grid: &mut [Vec<u8>]) -> u32 {
    let mut removed = 0;
    let mut queue = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == b'@' {
                queue.push((y, x));
            }
        }
    }

    while let Some(pos) = queue.pop() {
        if grid[pos.0][pos.1] != b'@' {
            continue;
        }
        let neighbours = roll_neighbours(grid, pos);
        if neighbours.len() < 4 {
            removed += 1;
            grid[pos.0][pos.1] = b'.';
            queue.extend(neighbours.iter());
        }
    }
    removed
}

pub fn solve() {
    let mut data: Vec<Vec<u8>> = fs::read("Day 04/data.txt")
        .expect("Could not read data file")
        .trim_ascii()
        .split(|&b| b == b'\n')
        .map(|line| line.iter().copied().filter(|&b| b != b'\r').collect())
        .collect();

    println!("Problem 7: {}", remove_rolls(&data));

    println!("Problem 8: {}", remove_all_rolls(&mut data));
}

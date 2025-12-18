/*
 * FILE: Day 07/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 7 problems (13 & 14) for Advent of Code 2025, solved in Rust.
 */
use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};
use std::fs;

fn advance_step(beams: &mut BTreeSet<(usize, usize)>, grid: &[Vec<u8>]) -> u32 {
    let mut removed = Vec::new();
    let mut added = Vec::new();
    for pos in beams.iter() {
        removed.push(*pos);
        if grid[pos.0 + 1][pos.1] == b'^' {
            added.push((pos.0 + 1, pos.1 - 1));
            added.push((pos.0 + 1, pos.1 + 1));
        } else {
            added.push((pos.0 + 1, pos.1));
        }
    }
    let splits = added.len() - removed.len();
    for beam in removed {
        beams.remove(&beam);
    }
    for beam in added {
        beams.insert(beam);
    }
    splits as u32
}

type Memo = HashMap<(usize, usize), u64>;
fn num_splits(memo: &mut Memo, pos: (usize, usize), grid: &[Vec<u8>]) -> u64 {
    let entry = memo.entry(pos);
    if let Entry::Occupied(e) = entry {
        *e.get()
    } else if pos.0 == grid.len() - 1 {
        *entry.or_insert(1)
    } else {
        let result = if grid[pos.0 + 1][pos.1] != b'^' {
            num_splits(memo, (pos.0 + 1, pos.1), grid)
        } else {
            let left = num_splits(memo, (pos.0 + 1, pos.1 - 1), grid);
            let right = num_splits(memo, (pos.0 + 1, pos.1 + 1), grid);
            left + right
        };
        memo.insert(pos, result);
        result
    }
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 07/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<Vec<u8>> = contents.split("\n").map(|s| s.bytes().collect()).collect();

    let start = (0, data[0].iter().position(|&b| b == b'S').unwrap());
    let mut beams = BTreeSet::new();
    beams.insert(start);

    println!(
        "Problem 13: {}",
        (1..data.len())
            .map(|_| advance_step(&mut beams, &data))
            .sum::<u32>()
    );

    let mut memoized = Memo::new();
    println!("Problem 14: {}", num_splits(&mut memoized, start, &data));
}

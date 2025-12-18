/*
 * FILE: Day 05/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 5 problems (9 & 10) for Advent of Code 2025, solved in Rust.
 */
use std::fs;

use itertools::Itertools;

pub fn solve() {
    let contents: String = fs::read_to_string("Day 05/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let (ranges, ids) = contents
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<_>>())
        .collect_tuple()
        .unwrap();

    let ids: Vec<u64> = ids.iter().map(|s| s.parse::<u64>().unwrap()).collect();
    let mut franges: Vec<(u64, u64)> = Vec::new();
    let mut ranges: Vec<(u64, u64)> = ranges
        .iter()
        .map(|s| {
            s.split("-")
                .map(|v| v.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .sorted()
        .collect();
    for i in 1..ranges.len() {
        let j = unsafe { i.unchecked_sub(1) };
        if ranges[j].1 < ranges[i].0 {
            franges.push(ranges[j]);
        } else {
            ranges[i].0 = ranges[j].0;
            ranges[i].1 = ranges[i].1.max(ranges[j].1);
        }
    }
    franges.push(*ranges.last().unwrap());

    println!(
        "Problem 9: {}",
        ids.iter()
            .filter(|id| franges.iter().any(|(start, end)| end >= id && start <= id))
            .count()
    );
    println!(
        "Problem 10: {}",
        franges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum::<u64>()
    );
}

/*
 * FILE: Day 03/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 3 problems (5 & 6) for Advent of Code 2025, solved in Rust.
 */
use std::cmp::Ordering;
use std::fs;

fn max_joltage(num_batteries: usize, bank: &str) -> u64 {
    let mut total = 0u64;
    let (mut start, mut end) = (0, bank.len() - num_batteries);
    for _ in 0..num_batteries {
        let (idx, val) = bank
            .chars()
            .enumerate()
            .skip(start)
            .take(end - start + 1)
            .max_by(|(_, a), (_, b)| {
                let cmp = a.cmp(b);
                if cmp == Ordering::Equal {
                    return Ordering::Greater;
                }
                cmp
            })
            .unwrap();
        total = total * 10 + val.to_digit(10).unwrap() as u64;
        (start, end) = (idx + 1, end + 1);
    }
    total
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 03/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n").collect();

    println!(
        "Problem 5: {}",
        data.iter().map(|bank| max_joltage(2, bank)).sum::<u64>()
    );
    println!(
        "Problem 6: {}",
        data.iter().map(|bank| max_joltage(12, bank)).sum::<u64>()
    );
}

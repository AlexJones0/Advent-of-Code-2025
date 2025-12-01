/*
 * FILE: Day 01/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 1 problems (1 & 2) for Advent of Code 2025, solved in Rust.
 */
use std::fs;

const DIAL_SIZE: i32 = 100;

pub fn solve() {
    let contents: String = fs::read_to_string("Day 01/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let turns: Vec<i32> = contents
        .split("\n")
        .map(|s| {
            let mut chars = s.chars();
            let direction = if chars.next().unwrap() == 'R' { 1 } else { -1 };
            let clicks = chars.as_str().parse::<i32>().unwrap();
            direction * clicks
        })
        .collect();

    let (mut dial, mut zeroes) = (50, 0);
    for turn in &turns {
        dial = (dial + turn).rem_euclid(DIAL_SIZE);
        zeroes += (dial == 0) as i32;
    }

    println!("Problem 1: {}", zeroes);

    let (mut dial, mut zeroes) = (50, 0);
    for turn in turns {
        let was_zero = dial == 0;
        dial += turn;
        let rots = dial.div_euclid(DIAL_SIZE);
        dial = dial.rem_euclid(DIAL_SIZE);
        zeroes += rots.abs();
        zeroes += (rots <= 0 && dial == 0) as i32;
        zeroes -= (rots < 0 && was_zero) as i32;
    }

    println!("Problem 2: {}", zeroes);
}

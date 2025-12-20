/*
 * FILE: Day 12/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 12 problems (23 & 24) for Advent of Code 2025, solved in Rust.
 */
use std::fs;

use itertools::Itertools;

pub fn solve() {
    let contents: String = fs::read_to_string("Day 12/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n\n").collect();

    let shape_sizes = data
        .iter()
        .take(data.len() - 1)
        .map(|s| s.chars().filter(|&c| c == '#').count())
        .collect::<Vec<_>>();
    println!(
        "Problem 23: {}",
        data.last()
            .unwrap()
            .split("\n")
            .filter(|region| {
                let parts: (&str, &str) = region.split(": ").collect_tuple().unwrap();
                let area: u32 = parts
                    .0
                    .split("x")
                    .map(|n| n.parse::<u32>().unwrap())
                    .product();
                let min_area: u32 = parts
                    .1
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .zip(shape_sizes.iter())
                    .map(|(num, size)| num * (*size as u32))
                    .sum();
                min_area <= area
            })
            .count()
    );
    println!("Problem 24: Christmas (Spirit) Freebie :)");
}

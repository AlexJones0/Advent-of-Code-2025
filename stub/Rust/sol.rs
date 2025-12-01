/*
 * FILE: Day XX/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day X problems (X & X) for Advent of Code 2025, solved in Rust.
 */
use std::fs;

const NOT_IMPLEMENTED: &str = "Not Yet Implemented";


pub fn solve() {
    let contents: String = fs::read_to_string("Day XX/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents
        .split("\n")
        .collect();


    println!("Problem X: {}", NOT_IMPLEMENTED);

    println!("Problem X: {}", NOT_IMPLEMENTED);
}

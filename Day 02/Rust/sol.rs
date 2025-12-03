/*
 * FILE: Day 02/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 2 problems (3 & 4) for Advent of Code 2025, solved in Rust.
 */
use std::collections::HashSet;
use std::fs;

// Look-up table for the first 10 powers of 10 for improved performance
const POW10: [u64; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

pub fn solve() {
    let contents: String = fs::read_to_string("Day 02/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: &str = contents.split("\n").collect::<Vec<_>>().first().unwrap();

    let (mut p1_total, mut p2_total) = (0, 0);
    let mut seen = HashSet::new();
    for id_range in data.split(",") {
        let id_range = id_range.split("-").collect::<Vec<_>>();
        let (start_str, end_str) = (id_range[0], id_range[1]);
        let start_val = start_str.parse::<u64>().unwrap();
        let end_val = end_str.parse::<u64>().unwrap();

        let max_part_len = end_str.len() / 2;
        for part_len in (1..=max_part_len).rev() {
            for target_len in start_str.len()..=end_str.len() {
                let num_parts = target_len / part_len;
                let remainder = target_len % part_len;
                if remainder != 0 || num_parts <= 1 {
                    continue;
                }

                for part in POW10[part_len - 1]..POW10[part_len] {
                    let mut invalid_id = part;
                    for _ in 1..num_parts {
                        invalid_id *= POW10[part_len];
                        invalid_id += part;
                    }

                    if invalid_id < start_val || invalid_id > end_val || seen.contains(&invalid_id)
                    {
                        continue;
                    }
                    seen.insert(invalid_id);
                    p2_total += invalid_id;
                    if num_parts == 2 {
                        p1_total += invalid_id;
                    }
                }
            }
        }
    }

    println!("Problem 3: {}", p1_total);
    println!("Problem 4: {}", p2_total);
}

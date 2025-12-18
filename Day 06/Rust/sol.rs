/*
 * FILE: Day 06/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 6 problems (11 & 12) for Advent of Code 2025, solved in Rust.
 */
use std::collections::HashMap;
use std::fs;

type BinOp = fn(u64, u64) -> u64;

fn get_operation_lengths(operator_row: &str) -> Vec<usize> {
    let mut lengths = Vec::new();
    let mut length = 0;
    for char in operator_row.bytes() {
        if char != b' ' && length != 0 {
            lengths.push(length - 1);
            length = 1;
            continue;
        }
        length += 1;
    }
    lengths.push(length);
    lengths
}

fn get_operations(data: &[&str], op_lengths: &[usize]) -> Vec<Vec<Vec<u8>>> {
    let mut items = Vec::with_capacity(data.len());
    for row in data {
        let mut item = Vec::<Vec<u8>>::with_capacity(op_lengths.len());
        let mut row = row.as_bytes().iter();
        for &length in op_lengths {
            item.push(row.by_ref().take(length).copied().collect());
            row.next();
        }
        items.push(item);
    }
    items
}

fn eval_horizontal(item: &[Vec<u8>], operations: &HashMap<u8, BinOp>) -> u64 {
    let operator = item.last().unwrap().trim_ascii()[0];
    let operator = operations.get(&operator).unwrap();
    let num_operands = item.len() - 1;
    let mut operands = item.iter().take(num_operands).map(|bs| {
        String::from_utf8(bs.clone())
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap()
    });
    let first = operands.next().unwrap();
    operands.fold(first, operator)
}

fn eval_vertical(item: &[Vec<u8>], operations: &HashMap<u8, BinOp>) -> u64 {
    let operator = item.last().unwrap().trim_ascii()[0];
    let operator = operations.get(&operator).unwrap();
    let num_operand_rows = item.len() - 1;
    let num_operands = item[0].len();
    let mut operands = (0..num_operands).map(|i| {
        let digits: Vec<u8> = item.iter().take(num_operand_rows).map(|bs| bs[i]).collect();
        String::from_utf8(digits)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap()
    });
    let first = operands.next().unwrap();
    operands.fold(first, operator)
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 06/data.txt")
        .expect("Could not read data file")
        .replace("\r", "")
        .trim_end_matches("\n")
        .to_string();
    let data: Vec<&str> = contents.split("\n").collect();

    let operators: [(u8, BinOp); 2] = [(b'*', |x, y| x * y), (b'+', |x, y| x + y)];

    let lengths = get_operation_lengths(data.last().unwrap());
    let items = get_operations(&data, &lengths);

    // Transpose
    let items: Vec<Vec<Vec<u8>>> = (0..items[0].len())
        .map(|i| items.iter().map(|inner| inner[i].clone()).collect())
        .collect();

    let operators = HashMap::from(operators);

    println!(
        "Problem 11: {}",
        items
            .iter()
            .map(|i| eval_horizontal(i, &operators))
            .sum::<u64>()
    );
    println!(
        "Problem 12: {}",
        items
            .iter()
            .map(|i| eval_vertical(i, &operators))
            .sum::<u64>()
    );
}

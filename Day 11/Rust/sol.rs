/*
 * FILE: Day 11/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 11 problems (21 & 22) for Advent of Code 2025, solved in Rust.
 */
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

type Memo<'a> = HashMap<(&'a str, &'a str), u64>;
fn paths<'a>(memo: &mut Memo<'a>, start: &'a str, end: &'a str, graph: &'a Graph) -> u64 {
    let entry = memo.entry((start, end));
    if let Entry::Occupied(e) = entry {
        *e.get()
    } else if start == end {
        *entry.or_insert(1)
    } else {
        let result = graph
            .get(start)
            .unwrap_or(&vec![])
            .iter()
            .map(|ngb| paths(memo, ngb, end, graph))
            .sum::<u64>();
        memo.insert((start, end), result);
        result
    }
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 11/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let graph: Graph = Graph::from_iter(contents.split("\n").map(|row| {
        let mut parts = row.split(": ");
        let node = parts.next().unwrap();
        let ngbs = parts
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .collect::<Vec<_>>();
        (node, ngbs)
    }));

    let mut memoized = Memo::new();
    println!("Problem 21: {}", paths(&mut memoized, "you", "out", &graph));

    // Graph is a DAG (acyclic), so either we have `fft` -> `dac` or vice versa
    let num_paths = paths(&mut memoized, "fft", "dac", &graph);
    let (fst, snd, mut num_paths) = if num_paths > 0 {
        ("fft", "dac", num_paths)
    } else {
        ("dac", "fft", paths(&mut memoized, "dac", "fft", &graph))
    };
    num_paths *= paths(&mut memoized, "svr", fst, &graph);
    num_paths *= paths(&mut memoized, snd, "out", &graph);
    println!("Problem 22: {}", num_paths);
}

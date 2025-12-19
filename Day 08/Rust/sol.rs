/*
 * FILE: Day 08/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 8 problems (15 & 16) for Advent of Code 2025, solved in Rust.
 */
use std::collections::BinaryHeap;
use std::fs;

use itertools::Itertools;

const NUM_CONNECTIONS: usize = 1000;

type Node = (u32, u32, u32);
type NodeHeap = BinaryHeap<(i64, usize, usize)>;
fn try_make_connection(heap: &mut NodeHeap, circuits: &mut Vec<Vec<usize>>, nodes: &[Node]) -> u64 {
    let (_, i, j) = heap.pop().unwrap();
    let mut i_circuit = circuits.iter_mut().position(|c| c.contains(&i)).unwrap();
    let j_circuit = circuits.iter().position(|c| c.contains(&j)).unwrap();
    if i_circuit != j_circuit {
        i_circuit -= (i_circuit > j_circuit) as usize;
        let j_circuit = circuits.remove(j_circuit);
        circuits[i_circuit].extend(j_circuit);
        if circuits.len() == 1 {
            return nodes[i].0 as u64 * nodes[j].0 as u64;
        }
    }
    0
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 08/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let data: Vec<&str> = contents.split("\n").collect();

    let nodes: Vec<Node> = data
        .iter()
        .map(|s| {
            s.split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let num_pairs = nodes.len() * (nodes.len() - 1);
    let mut pairs: NodeHeap = BinaryHeap::with_capacity(num_pairs);
    for i in 0..nodes.len() {
        for j in 0..i {
            let (ni, nj) = (nodes[i], nodes[j]);
            let square = |x| x * x;
            let dist = square(ni.0 as i64 - nj.0 as i64)
                + square(ni.1 as i64 - nj.1 as i64)
                + square(ni.2 as i64 - nj.2 as i64);
            pairs.push((-dist, j, i));
        }
    }

    let mut circuits: Vec<Vec<usize>> = Vec::from_iter((0..nodes.len()).map(|i| vec![i]));
    for _ in 0..NUM_CONNECTIONS {
        try_make_connection(&mut pairs, &mut circuits, &nodes);
    }
    let lens: Vec<u64> = circuits
        .iter()
        .map(|c| c.len() as u64)
        .sorted()
        .rev()
        .collect();
    println!("Problem 15: {}", lens[..3].iter().product::<u64>());

    let mut ans = 0;
    while ans == 0 {
        ans = try_make_connection(&mut pairs, &mut circuits, &nodes);
    }
    println!("Problem 16: {}", ans);
}

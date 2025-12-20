/*
 * FILE: Day 10/Rust/sol.rs
 * Author: Alex Jones
 * Desc: Solution to day 10 problems (19 & 20) for Advent of Code 2025, solved in Rust.
 */
use std::collections::{BTreeSet, BinaryHeap};
use std::fs;

use z3::{ast::Int, Optimize, SatResult::Sat};

fn remove_start_and_end(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
struct Machine {
    indicators: u32,
    buttons: Vec<Vec<usize>>,
    counters: Vec<u32>,
}

impl Machine {
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let indicators = remove_start_and_end(parts[0])
            .chars()
            .enumerate()
            .map(|(i, c)| if c == '.' { 0 } else { 1 << i })
            .sum();
        let buttons = parts
            .iter()
            .take(parts.len() - 1)
            .skip(1)
            .map(|b| {
                remove_start_and_end(b)
                    .split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let counters = remove_start_and_end(parts.last().unwrap())
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Self {
            indicators,
            buttons,
            counters,
        }
    }

    fn min_indicator_presses(&self) -> Option<u32> {
        let initial = 0u32;
        if self.indicators == initial {
            return Some(0);
        }
        let mut seen = BTreeSet::from([initial]);
        let mut queue = BinaryHeap::from([(0i32, initial)]);
        while let Some((neg_length, state)) = queue.pop() {
            for button in &self.buttons {
                let next_state = button
                    .iter()
                    .map(|pos| 1u32 << pos)
                    .fold(state, |a, b| a ^ b);
                if seen.contains(&next_state) {
                    continue;
                }
                seen.insert(next_state);
                if next_state == self.indicators {
                    return Some(-neg_length as u32 + 1);
                } // `BinaryHeap` is a max heap
                queue.push((neg_length - 1, next_state));
            }
        }
        None
    }

    fn min_counter_presses(&self) -> Option<u32> {
        if self.counters.iter().all(|&c| c == 0) {
            return Some(0);
        }

        // Just keep it simple and solve with Z3
        // First create integer symbols for each button
        let button_symbols = (0..self.buttons.len())
            .map(|i| Int::fresh_const(format!("b_{}", i).as_str()))
            .collect::<Vec<_>>();
        let optimizer = Optimize::new();
        // Buttons must be pressed a non-negative number of times
        button_symbols
            .iter()
            .for_each(|b| optimizer.assert(&b.ge(0)));
        // For all buttons that influence a counter, the sum of the presses
        // for each button should equal that counter (for each number)
        for (i, &count) in self.counters.iter().enumerate() {
            let counter_symbols = &self
                .buttons
                .iter()
                .zip(button_symbols.iter())
                .filter(|(button, _)| button.contains(&i))
                .map(|(_, sym)| sym)
                .collect::<Vec<_>>();
            optimizer.assert(&Int::add(counter_symbols).eq(count));
        }
        // Our goal is to minimize the total button presses
        optimizer.minimize(&Int::add(&button_symbols));
        // Solve with a SAT solver
        assert_eq!(optimizer.check(&[]), Sat);
        let model = optimizer.get_model()?;
        button_symbols
            .iter()
            .map(|b| model.eval(b, true)?.as_u64().map(|v| v as u32))
            .sum()
    }
}

pub fn solve() {
    let contents: String = fs::read_to_string("Day 10/data.txt")
        .expect("Could not read data file")
        .trim()
        .replace("\r", "");
    let machines: Vec<Machine> = contents.split("\n").map(Machine::from_str).collect();

    println!(
        "Problem 19: {}",
        machines
            .iter()
            .map(|m| m.min_indicator_presses().unwrap())
            .sum::<u32>()
    );

    println!(
        "Problem 20: {}",
        machines
            .iter()
            .map(|m| m.min_counter_presses().unwrap())
            .sum::<u32>()
    );
}

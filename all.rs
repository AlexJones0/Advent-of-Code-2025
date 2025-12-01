use std::collections::HashMap;

use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;

// There's probably a nicer way to do this while maintaining my existing directory structure,
// but I'm doing this hackily for now to avoid spending a lot of time on this.

#[path = "./Day 01/Rust/sol.rs"]
mod one;

type Func = fn() -> ();
lazy_static! {
    static ref sols: HashMap<u8, Func> = #[allow(clippy::zero_prefixed_literal)]
    {
        let mut m = HashMap::new();
        m.insert(01u8, one::solve as Func);
        m
    };
}

#[derive(Parser)]
struct Opts {
    #[arg(short, long, value_name = "DATE")]
    day: Option<u8>,
}

fn main() {
    let opts = Opts::parse();

    if opts.day.is_some() {
        let day = opts.day.unwrap();
        if sols.contains_key(&day) {
            sols[&day]();
        } else {
            println!("Invalid day input - no solution exists for day {}", day);
        }
    } else {
        for day in sols.keys().sorted() {
            sols[day]();
        }
    }
}

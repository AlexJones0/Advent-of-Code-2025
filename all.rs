use std::collections::HashMap;

use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;

// There's probably a nicer way to do this while maintaining my existing directory structure,
// but I'm doing this hackily for now to avoid spending a lot of time on this.

#[path = "./Day 08/Rust/sol.rs"]
mod eight;
#[path = "./Day 05/Rust/sol.rs"]
mod five;
#[path = "./Day 04/Rust/sol.rs"]
mod four;
#[path = "./Day 01/Rust/sol.rs"]
mod one;
#[path = "./Day 07/Rust/sol.rs"]
mod seven;
#[path = "./Day 06/Rust/sol.rs"]
mod six;
#[path = "./Day 03/Rust/sol.rs"]
mod three;
#[path = "./Day 02/Rust/sol.rs"]
mod two;

type Func = fn() -> ();
lazy_static! {
    static ref sols: HashMap<u8, Func> = #[allow(clippy::zero_prefixed_literal)]
    {
        let mut m = HashMap::new();
        m.insert(01u8, one::solve as Func);
        m.insert(02u8, two::solve as Func);
        m.insert(03u8, three::solve as Func);
        m.insert(04u8, four::solve as Func);
        m.insert(05u8, five::solve as Func);
        m.insert(06u8, six::solve as Func);
        m.insert(07u8, seven::solve as Func);
        m.insert(08u8, eight::solve as Func);
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

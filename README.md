# Advent of Code 2025

Repository storing my solutions for Advent of Code 2025.
Solutions are in whatever language I feel like using that day.

## Running

Each day is stored in a separate `Day XX/language/sol.ext` file.

For python, you can run an individual day by running e.g. `python3 'Day\ XX/Python/sol.py`.
All solutions can be run in one go using `python3 all.py`.

For Rust, you can run an individual day by running e.g. `cargo run -- --day XX`
All solutions can be run in one go by running normally, e.g. `cargo run`.

## Runtime

Timings on my system (AMD Ryzen AI 7 350 @ 3.40 MHz):
* **Python**: 999.1 ms (± 14.9 ms) using `hyperfine --warmup=5 'python3 all.py'`
* **Rust**: 234.3 ms (± 4.3 ms) using `cargo build --release && hyperfine --warmup=5 './target/release/advent_of_code_2025'`

With day 10 removed (a notable slow day, as this dispatches to z3 to handle the general case):
* **Python**: 785.0 ms (± 7.2 ms) using `hyperfine --warmup=5 'python3 all.py'`
* **Rust**: 35.9 ms (± 2.9 ms) using `cargo build --release && hyperfine --warmup=5 './target/release/advent_of_code_2025'`
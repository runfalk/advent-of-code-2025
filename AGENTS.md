# Purpose & layout
Advent of Code 2024 solutions. Entry point `src/main.rs` dispatches to
`src/dayN.rs` modules; inputs live in `data/dayN.txt`. Run a solution with
`cargo run -- --day <N> [--input <path>]`. Each day is split into part A and
B. The problem for part B isn't exposed until part A is solved, so only solve
for part A until more instructions are provided.


# Coding conventions
Each day exposes `pub fn main(input: &str) -> anyhow::Result<(A, Option<B>)>`.
`A` and `B` must implement `std::fmt::Display` and should generally be
integers. Solutions for new days are registered in `src/main.rs` such that they
can be called using the CLI. Keep parsing and solving code in the day module.
Each solution should be self contained within the corresponding `src/dayN.rs`
file and no code should be shared across days unless explicitly instructed. All
problems come with example input and output and unit tests should be created to
cover all examples.

Solutions should be split into separate functions `part_a` and `part_b` that
take the parsed input as an argument. The parsing can happen either in `main`
or in a separate function to make handling easier for tests.

Use `anyhow::{Result, Context}` for errors with helpful messages; favor
iterator/functional style; avoid `dbg!` (denied at crate root); prefer
deterministic, allocation-aware code (dev profile is optimized for speed);
keep tests near implementations with clear example cases.

`src/utils.rs` only holds the `test_real_input!` macro—do not add other shared
helpers there. Solutions should stay self contained in their `src/dayN.rs`
modules.


# Testing
Run everything with `cargo test`. Target a single day with `cargo test day7`
(name filter). Use the `test_real_input!(day, answer_a, answer_b)` macro to
assert solutions against `data/dayN.txt`. Always finish the solution by adding
a unit test against the real data once the correctness of the solution is
asserted.


# Linting & formatting
`cargo fmt` for formatting; `cargo clippy -- -D warnings` for linting
(matches `pre-commit`). Keep the tree ASCII-only unless existing files already
use other characters.

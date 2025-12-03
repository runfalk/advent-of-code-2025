# Purpose & layout
Advent of Code 2024 solutions. Entry point `src/main.rs` dispatches to
`src/dayN.rs` modules; inputs live in `data/dayN.txt`. Run a solution with
`cargo run -- --day <N> [--input <path>]`. Each day is split into part A and
B. The problem for part B isn't exposed until part A is solved.


# Coding conventions
* Add a doc comment for each file that explains the problem in a concise way
  for each part and omit the story telling details of the problem statement. The
  description should be verbose enough that the problem can be solved without
  access to the problem statement. The doc comment should wrap at 100 characters
  per line.
* Each day exposes `pub fn main(input: &str) -> anyhow::Result<(A, Option<B>)>`.
  `A` and `B` must implement `std::fmt::Display` and should generally be
  integers.
* Each solution should be self contained within the corresponding
  `src/dayN.rs`.
* Solutions for new days are registered in `src/main.rs` such that they
  can be called using the CLI.
* Parsing should be strict and things like additional spaces are invalid. There
  is no need to support malformed input. The first line may however be blank, so
  it's recommended to use inside the parse function `.trim()` once on the whole
  input string such that unit tests can be nicely formatted using the
  `dedent!()` macro.
* Whenever the `dedent!()` macro is used for test data, the input should be
  indented one more level than the let statement it's assigned to. Raw multi-line
  strings should be used and `r#"` and `"` should be on their own lines.
* All problems come with example input and output and unit tests should be
  created to cover all examples.
* Solutions should be split into separate functions `part_a` and `part_b` that
  take the parsed input as an argument.
* When logic is re-used in multiple places within the same day, create a helper
  function.
* Input parsing should be its own function.
* It is OK to combine both `part_a` and `part_b` into a single function in cases
  where the same function can be used to solve the problem.
* Tests don't need doc comments.
* Use `src/day1.rs` as a reference for how the code should be structured.
* Structs should at least derive `Debug` such that it can easily be printed.
* Integer types should be `usize` or `isize` to avoid casting.
* Only declare types for `let` statements when the correct type can't
  automatically be inferred.
* Use named constants instead of "magic" numbers.
* Use `anyhow::{Result, Context}` for errors with helpful messages.
* Favor iterator/functional over imperative style. `.fold()` should be avoided
  unless the accumulator is a single value (i.e. not a tuple).
* Doc comments should be used for all functions to describe in concise wording
  that ignores the story-telling aspects of the problem statement. `main` only
  needs a comment if it includes the parsing implementation.
* Doc comments should not use redundant information like `Part A:` for the
  `part_a` function.
* `src/utils.rs` only holds the `test_real_input!` macro. Don't add other
  shared helpers there. Solutions should stay self contained in their
  `src/dayN.rs` modules.


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

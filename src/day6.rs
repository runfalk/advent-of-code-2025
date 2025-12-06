//! # Day 6: Trash Compactor
//! Input is four lines representing many column-aligned math problems placed horizontally. The
//! first `n-1` lines hold the operands, each problem stacked vertically with arbitrary internal
//! spacing. A full column of spaces separates problems. The final line contains either `+` or `*`
//! per problem to indicate whether to sum or multiply that column's operands.
//!
//! ## Part A
//! Split the grid into problems, parse the operands above each operator, evaluate the indicated
//! addition or multiplication, and return the sum of all problem results.
//!
//! ## Part B
//! Cephalopod numbers are vertical, most significant digit at the top. Each column within a problem
//! is one number. Read problems right-to-left column by column, build numbers from top-to-bottom
//! digits, evaluate, and sum the results.
use anyhow::{Context, Result, bail};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    horizontal: Vec<usize>,
    vertical: Vec<usize>,
    op: Operation,
}

/// Parse the column-aligned worksheet into a list of problems with their operands and operator.
fn parse_input(input: &str) -> Result<Vec<Problem>> {
    let lines: Vec<&str> = input.trim().lines().collect();
    if lines.len() < 2 {
        bail!("Expected at least two lines for operands and operators");
    }

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let padded: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect();

    let operator_row = padded.len() - 1;
    let mut problems = Vec::new();
    let mut col = 0;
    while col < width {
        while col < width && padded.iter().all(|row| row[col] == ' ') {
            col += 1;
        }
        if col >= width {
            break;
        }
        let start = col;
        while col < width && padded.iter().any(|row| row[col] != ' ') {
            col += 1;
        }
        let end = col;

        let mut horizontal = Vec::new();
        let mut vertical = Vec::new();
        for (row, chars) in padded.iter().take(operator_row).enumerate() {
            let slice: String = chars[start..end].iter().collect();
            let trimmed = slice.trim();
            if trimmed.is_empty() {
                continue;
            }
            let value = trimmed
                .parse::<usize>()
                .with_context(|| format!("Invalid number {trimmed:?} on line {}", row + 1))?;
            horizontal.push(value);
        }

        if horizontal.is_empty() {
            bail!("Problem spanning columns {start}-{end} has no operands");
        }

        let op_slice: String = padded[operator_row][start..end].iter().collect();
        let op = match op_slice.trim() {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            other => bail!("Unknown operator {other:?} at columns {start}-{end}"),
        };

        for c in (start..end).rev() {
            let mut digits = String::new();
            for (row, chars) in padded.iter().take(operator_row).enumerate() {
                let ch = chars[c];
                if ch.is_ascii_digit() {
                    digits.push(ch);
                } else if ch != ' ' {
                    bail!("Invalid character {ch:?} in column {c} on line {}", row + 1);
                }
            }

            if digits.is_empty() {
                bail!("Column {c} inside problem spanning {start}-{end} has no digits");
            }

            let value = digits
                .parse::<usize>()
                .with_context(|| format!("Invalid column number {digits:?} at column {}", c + 1))?;
            vertical.push(value);
        }

        problems.push(Problem {
            horizontal,
            vertical,
            op,
        });
    }

    Ok(problems)
}

/// Evaluate a list of operands using the given operation.
fn evaluate(op: Operation, operands: &[usize]) -> usize {
    match op {
        Operation::Add => operands.iter().copied().sum(),
        Operation::Multiply => operands.iter().copied().product(),
    }
}

/// Sum the results of every parsed problem.
fn part_a(problems: &[Problem]) -> usize {
    problems
        .iter()
        .map(|problem| evaluate(problem.op, &problem.horizontal))
        .sum()
}

/// Sum the results of every parsed problem when numbers are read right-to-left column-wise.
fn part_b(problems: &[Problem]) -> usize {
    problems
        .iter()
        .map(|problem| evaluate(problem.op, &problem.vertical))
        .sum()
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let problems = parse_input(input)?;
    Ok((part_a(&problems), Some(part_b(&problems))))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(6, 4_719_804_927_602, Some(9_608_327_000_261));

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            123 328  51 64 
             45 64  387 23 
              6 98  215 314
            *   +   *   +  
        "#
    );

    #[test]
    fn example_a() {
        assert_eq!(part_a(&parse_input(EXAMPLE_INPUT).unwrap()), 4_277_556);
    }

    #[test]
    fn example_b() {
        assert_eq!(part_b(&parse_input(EXAMPLE_INPUT).unwrap()), 3_263_827);
    }
}

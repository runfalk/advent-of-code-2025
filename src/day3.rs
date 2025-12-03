//! # Day 3: Lobby
//!
//! Pick exactly two batteries from each line of digits (in order) to form the largest possible
//! two-digit joltage, summing the per-bank maxima. Part B instead picks twelve batteries per bank
//! to maximize the resulting 12-digit joltage.
use anyhow::{Result, bail};

const PICK_A: usize = 2;
const PICK_B: usize = 12;

/// Parse banks of battery ratings (digits 1-9).
fn parse_input(input: &str) -> Result<Vec<Vec<usize>>> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let line_no = idx + 1;
            line.chars()
                .map(|ch| match ch.to_digit(10) {
                    Some(0) | None => {
                        bail!("Invalid battery rating `{}` on line {}", ch, line_no)
                    }
                    Some(value) => Ok(value as usize),
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect()
}

/// Build the largest possible `num_picks`-digit number by keeping digits in order.
fn max_bank_joltage(batteries: &[usize], num_picks: usize) -> Result<usize> {
    if batteries.len() < num_picks {
        bail!(
            "Bank needs at least {} batteries but only has {}",
            num_picks,
            batteries.len()
        );
    }

    let mut stack = Vec::with_capacity(num_picks);
    let mut remaining = batteries.len();

    // Remove smaller leading digits while enough remain to reach length.
    for &digit in batteries {
        while !stack.is_empty()
            && stack.len() + remaining > num_picks
            && stack.last() < Some(&digit)
        {
            stack.pop();
        }
        if stack.len() < num_picks {
            stack.push(digit);
        }
        remaining -= 1;
    }

    Ok(stack.into_iter().fold(0, |acc, digit| acc * 10 + digit))
}

/// Sum the highest two-digit values obtainable from each bank.
fn part_a(banks: &[Vec<usize>]) -> Result<usize> {
    banks.iter().try_fold(
        0usize,
        |acc, bank| Ok(acc + max_bank_joltage(bank, PICK_A)?),
    )
}

/// Sum the highest 12-digit values obtainable from each bank.
fn part_b(banks: &[Vec<usize>]) -> Result<usize> {
    banks.iter().try_fold(
        0usize,
        |acc, bank| Ok(acc + max_bank_joltage(bank, PICK_B)?),
    )
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let banks = parse_input(input)?;
    Ok((part_a(&banks)?, Some(part_b(&banks)?)))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(3, 16_946, 168_627_047_606_506);

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "#
    );

    #[test]
    fn example_a() {
        let banks = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_a(&banks).unwrap(), 357);
    }

    #[test]
    fn example_b() {
        let banks = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_b(&banks).unwrap(), 3_121_910_778_619);
    }
}

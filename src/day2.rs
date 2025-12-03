//! # Day 2: Gift Shop
//! Input is a single line of comma-separated inclusive ID ranges `start-end` with no leading
//! zeroes.
//!
//! ## Part A
//! Find IDs within the ranges whose digits are a non-empty sequence repeated exactly twice; sum
//! all invalid IDs.
//!
//! ## Part B
//! IDs are invalid if their digits are any sequence repeated two or more times; sum all invalid IDs
//! in the ranges.
use anyhow::{Context, Result, bail};

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

/// Parse strict inclusive ranges of the form `start-end` separated by commas on a single line.
fn parse_input(input: &str) -> Result<Vec<Range>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("Input must contain at least one range");
    }

    trimmed
        .split(',')
        .enumerate()
        .map(|(idx, part)| {
            let range_str = part.trim();
            if range_str.is_empty() {
                bail!("Empty range at position {}", idx + 1);
            }
            let (start, end) = range_str
                .split_once('-')
                .with_context(|| format!("Missing dash in range {}", idx + 1))?;
            let start = start
                .parse()
                .with_context(|| format!("Invalid start in range {}", idx + 1))?;
            let end = end
                .parse()
                .with_context(|| format!("Invalid end in range {}", idx + 1))?;
            if start > end {
                bail!("Range {} has start greater than end", idx + 1);
            }
            Ok(Range { start, end })
        })
        .collect()
}

/// Generate all numbers up to `max_value` whose decimal digits are formed by repeating a base
/// sequence `repeat_count` times, retaining only repeat counts accepted by `filter_repeat`.
fn repeated_numbers<F: Fn(usize) -> bool>(max_value: usize, filter_repeat: F) -> Vec<usize> {
    let mut numbers = Vec::new();
    let max_digits = max_value.to_string().len();

    for base_len in 1..=max_digits {
        let pow_base = 10usize.pow(base_len as u32);
        let base_start = pow_base / 10;
        let base_end = pow_base - 1;
        for num_repeats in 2..=max_digits / base_len {
            if !filter_repeat(num_repeats) {
                continue;
            }
            let pow_total = 10usize.pow((base_len * num_repeats) as u32);
            let factor = (pow_total - 1) / (pow_base - 1);
            for base in base_start..=base_end {
                let candidate = base * factor;
                if candidate > max_value {
                    break;
                }
                numbers.push(candidate);
            }
        }
    }

    numbers.sort_unstable();
    numbers.dedup();
    numbers
}

/// Sum every repeated-half number that falls inside any of the provided inclusive ranges.
fn part_a(ranges: &[Range]) -> usize {
    let max_value = ranges.iter().map(|range| range.end).max().unwrap_or(0);
    if max_value == 0 {
        return 0;
    }

    let doubles = repeated_numbers(max_value, |num_repeats| num_repeats == 2);
    ranges
        .iter()
        .map(|range| {
            let start_idx = doubles.partition_point(|&value| value < range.start);
            let end_idx = doubles.partition_point(|&value| value <= range.end);
            doubles[start_idx..end_idx].iter().sum::<usize>()
        })
        .sum()
}

/// Sum every repeated-sequence number (two or more repeats) that falls inside any of the ranges.
fn part_b(ranges: &[Range]) -> usize {
    let max_value = ranges.iter().map(|range| range.end).max().unwrap_or(0);
    if max_value == 0 {
        return 0;
    }

    let repeated = repeated_numbers(max_value, |num_repeats| num_repeats >= 2);
    ranges
        .iter()
        .map(|range| {
            let start_idx = repeated.partition_point(|&value| value < range.start);
            let end_idx = repeated.partition_point(|&value| value <= range.end);
            repeated[start_idx..end_idx].iter().sum::<usize>()
        })
        .sum()
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let ranges = parse_input(input)?;
    Ok((part_a(&ranges), Some(part_b(&ranges))))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(2, 38_310_256_125, 58_961_152_806);

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,
            824824821-824824827,2121212118-2121212124
        "#
    );

    #[test]
    fn example_a() {
        assert_eq!(part_a(&parse_input(EXAMPLE_INPUT).unwrap()), 1_227_775_554);
    }

    #[test]
    fn example_b() {
        assert_eq!(part_b(&parse_input(EXAMPLE_INPUT).unwrap()), 4_174_379_265);
    }
}

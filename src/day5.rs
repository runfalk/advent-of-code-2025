//! # Day 5: Cafeteria
//! Input lists inclusive fresh ingredient ID ranges, then a blank line, followed by ingredient IDs
//! to evaluate.
//!
//! ## Part A
//! Count how many available ingredient IDs fall within any listed fresh range.
//!
//! ## Part B
//! Count how many distinct ingredient IDs are covered by the fresh ranges.
use std::ops::Range;

use anyhow::{Context, Result, bail};

fn parse_input(input: &str) -> Result<(Vec<Range<usize>>, Vec<usize>)> {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut lines = input.trim().lines().enumerate();

    // Iterate through lines until we spot a blank line without completely consuming the iterator.
    for (idx, line) in &mut lines {
        let line_no = idx + 1;
        if line.trim().is_empty() {
            break;
        }

        let (start, end) = line
            .split_once('-')
            .with_context(|| format!("Missing dash in range on line {}", line_no))?;
        let start = start
            .parse::<usize>()
            .with_context(|| format!("Invalid range start on line {}", line_no))?;
        let end_inclusive = end
            .parse::<usize>()
            .with_context(|| format!("Invalid range end on line {}", line_no))?;
        if start > end_inclusive {
            bail!("Range start exceeds end on line {}", line_no);
        }
        ranges.push(start..(end_inclusive + 1));
    }

    for (idx, line) in lines {
        ids.push(
            line.parse::<usize>()
                .with_context(|| format!("Invalid ingredient ID on line {}", idx + 1))?,
        );
    }

    let mut ranges_sorted = ranges;
    ranges_sorted.sort_unstable_by_key(|range| range.start);
    let mut merged_ranges: Vec<Range<usize>> = Vec::with_capacity(ranges_sorted.len());
    for range in ranges_sorted {
        if let Some(last) = merged_ranges.last_mut()
            && range.start <= last.end
        {
            last.end = last.end.max(range.end);
            continue;
        }
        merged_ranges.push(range);
    }

    Ok((merged_ranges, ids))
}

/// Count ingredient IDs that are contained in any fresh range.
fn part_a(ranges: &[Range<usize>], ids: &[usize]) -> usize {
    ids.iter()
        .filter(|&&id| {
            let idx = ranges.partition_point(|range| range.end <= id);
            idx < ranges.len() && ranges[idx].contains(&id)
        })
        .count()
}

/// Return the total number of unique ingredient IDs covered by any fresh range.
fn part_b(ranges: &[Range<usize>]) -> usize {
    ranges.iter().map(Range::len).sum()
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let (ranges, ids) = parse_input(input)?;
    Ok((part_a(&ranges, &ids), Some(part_b(&ranges))))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(5, 517, 336_173_027_056_994);

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "#
    );

    #[test]
    fn example_a() {
        let (ranges, ids) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_a(&ranges, &ids), 3);
    }

    #[test]
    fn example_b() {
        let (ranges, _) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_b(&ranges), 14);
    }

    #[test]
    fn accepts_ranges_only() {
        let input = "1-3\n5-5\n";
        let (ranges, ids) = parse_input(input).unwrap();
        assert_eq!(ids.len(), 0);
        assert_eq!(part_b(&ranges), 4);
    }

    #[test]
    fn accepts_empty_input() {
        let (ranges, ids) = parse_input("").unwrap();
        assert!(ranges.is_empty());
        assert!(ids.is_empty());
        assert_eq!(part_a(&ranges, &ids), 0);
        assert_eq!(part_b(&ranges), 0);
    }
}

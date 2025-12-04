//! # Day 4: Printing Department
//! Input is a rectangular grid of `@` (paper rolls) and `.` (empty).
//!
//! ## Part A
//! A roll is accessible when fewer than four of its eight neighbors also contain rolls; count all
//! accessible rolls.
//!
//! ## Part B
//! Repeatedly remove every currently accessible roll (fewer than four neighboring rolls). Each
//! removal can expose more rolls; count how many rolls can be removed before no new rolls become
//! accessible.
use anyhow::{Result, bail};
use std::collections::{HashMap, HashSet};

/// Maximum number of rolls in neighboring cells that still permits access.
const ACCESS_THRESHOLD: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: isize,
    y: isize,
}

impl Cell {
    /// Return all eight neighboring cells (including diagonals).
    fn neighbors(self) -> impl Iterator<Item = Cell> {
        (-1..=1).flat_map(move |dx| {
            (-1..=1).filter_map(move |dy| {
                (dx != 0 || dy != 0).then(|| Cell {
                    x: self.x + dx,
                    y: self.y + dy,
                })
            })
        })
    }
}

/// Parse a grid of `@` rolls and `.` empty spaces into neighbor counts for each roll.
fn parse_input(input: &str) -> Result<HashMap<Cell, usize>> {
    let mut rolls = HashSet::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    rolls.insert(Cell {
                        x: y as isize,
                        y: x as isize,
                    });
                }
                '.' => {}
                _ => bail!("Invalid character `{c}` at row {}, col {}", y + 1, x + 1),
            }
        }
    }

    Ok(rolls
        .iter()
        .map(|&cell| {
            let count = cell
                .neighbors()
                .filter(|neighbor| rolls.contains(neighbor))
                .count();
            (cell, count)
        })
        .collect())
}

/// Count rolls with fewer than four neighboring rolls.
fn part_a(num_neighbors: &HashMap<Cell, usize>) -> usize {
    num_neighbors
        .values()
        .filter(|&&count| count < ACCESS_THRESHOLD)
        .count()
}

/// Remove accessible rolls until no more become accessible; return the total removed.
fn part_b(mut num_neighbors: HashMap<Cell, usize>) -> usize {
    let mut queue: Vec<Cell> = num_neighbors
        .iter()
        .filter_map(|(&coord, &count)| (count < ACCESS_THRESHOLD).then_some(coord))
        .collect();

    let mut num_removed = 0;
    while let Some(cell) = queue.pop() {
        if num_neighbors.remove(&cell).is_none() {
            continue;
        }
        num_removed += 1;

        for neighbor in cell.neighbors() {
            if let Some(count) = num_neighbors.get_mut(&neighbor) {
                *count -= 1;
                if *count < ACCESS_THRESHOLD {
                    queue.push(neighbor);
                }
            }
        }
    }

    num_removed
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let num_neighbors = parse_input(input)?;
    Ok((part_a(&num_neighbors), Some(part_b(num_neighbors.clone()))))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(4, 1587, 8946);

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "#
    );

    #[test]
    fn example_a() {
        let neighbors = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_a(&neighbors), 13);
    }

    #[test]
    fn example_b() {
        let neighbors = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_b(neighbors), 43);
    }
}

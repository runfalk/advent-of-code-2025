//! # Day 7: Laboratories
//! Input is a rectangular map of `.` empty space, `^` splitters, and exactly one `S` start
//! location. A tachyon beam begins directly below `S` and always moves downward.
//!
//! ## Part A
//! A beam passes through empty space unchanged. When it reaches a splitter, that beam stops and
//! two new beams start in the same row immediately to the left and right, continuing downward.
//! Beams sharing the same column from the same or later rows are equivalent and merge. Count how
//! many times any beam reaches a splitter before every beam exits the map.
//!
//! ## Part B
//! A single particle splits timelines at every splitter: the particle takes both left and right
//! paths, creating a separate timeline for each choice. Timelines that later share the same path
//! still remain distinct. Count how many timelines exist after the particle finishes traversing
//! the manifold.
use anyhow::{Context, Result, bail};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

type Cell = (usize, usize);

#[derive(Debug)]
struct Manifold {
    splitters: HashSet<Cell>,
    start: Cell,
    height: usize,
    width: usize,
}

impl Manifold {
    fn next_splitter(&self, x: usize, y: usize) -> Option<Cell> {
        (y..self.height).find_map(|ny| self.splitters.contains(&(x, ny)).then_some((x, ny)))
    }
}
/// Parse the manifold into splitter coordinates and locate the start cell.
fn parse_input(input: &str) -> Result<Manifold> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut width = 0;
    let mut splitters = HashSet::new();
    let mut start = None;

    for (y, line) in lines.iter().enumerate() {
        width = width.max(line.len());
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {}
                '^' => {
                    splitters.insert((x, y));
                }
                'S' => {
                    if start.replace((x, y)).is_some() {
                        bail!("Second start position found on line {}", y + 1);
                    }
                }
                other => bail!("Invalid character {other:?} on line {}", y + 1),
            }
        }
    }

    Ok(Manifold {
        splitters,
        start: start.context("Missing start position S")?,
        height: lines.len(),
        width,
    })
}

/// Count how often beams are split until every beam exits the manifold.
fn part_a(manifold: &Manifold) -> usize {
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    let mut splits = 0;

    queue.push((manifold.start.0, manifold.start.1 + 1));
    while let Some((x, y)) = queue.pop() {
        if let Some((hit_x, hit_y)) = manifold.next_splitter(x, y)
            && visited.insert((hit_x, hit_y))
        {
            splits += 1;
            if hit_x > 0 {
                queue.push((hit_x - 1, hit_y));
            }
            if hit_x + 1 < manifold.width {
                queue.push((hit_x + 1, hit_y));
            }
        }
    }
    debug_assert!(splits <= manifold.splitters.len());
    splits
}

/// Count how many distinct timelines exist when the particle splits at every encountered splitter.
fn part_b(manifold: &Manifold) -> usize {
    let mut counts: HashMap<Cell, usize> = HashMap::new();
    let mut heap = std::collections::BinaryHeap::new();
    let mut timelines = 0usize;

    let start_y = manifold.start.1 + 1;
    if let Some(start_splitter) = manifold.next_splitter(manifold.start.0, start_y) {
        counts.insert(start_splitter, 1);
        heap.push(Reverse((start_splitter.1, start_splitter.0)));
    } else {
        // We never hit any splitter
        return 1;
    }

    while let Some(Reverse((y, x))) = heap.pop() {
        let count = counts.remove(&(x, y)).unwrap_or(0);
        if count == 0 {
            continue;
        }

        for next_x in [
            x.checked_sub(1),
            x.checked_add(1).filter(|&nx| nx < manifold.width),
        ] {
            let Some(next_x) = next_x else {
                timelines += count;
                continue;
            };

            if let Some((sx, sy)) = manifold.next_splitter(next_x, y) {
                let entry = counts.entry((sx, sy)).or_insert(0);
                if *entry == 0 {
                    heap.push(Reverse((sy, sx)));
                }
                *entry += count;
                continue;
            }

            timelines += count;
        }
    }

    timelines
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let manifold = parse_input(input)?;
    Ok((part_a(&manifold), Some(part_b(&manifold))))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(7, 1507, Some(1_537_373_473_728));

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "#
    );

    #[test]
    fn example_a() {
        let manifold = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_a(&manifold), 21);
    }

    #[test]
    fn example_b() {
        let manifold = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_b(&manifold), 40);
    }
}

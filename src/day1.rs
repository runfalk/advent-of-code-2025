//! # Day 1: Secret Entrance
//! Input is a list of dial rotations on a 0-99 circle starting at 50, each as `L|R<clicks>` on its
//! own line.
//!
//! ## Part A
//! Apply rotations and count how many end with the dial at 0.
//!
//! ## Part B
//! Count every click that passes through 0 during rotations, including intermediate clicks on long
//! moves.
use anyhow::{Context, Result, bail};

const DIAL_SIZE: usize = 100;
const START_POS: usize = 50;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: Rotation,
    clicks: usize,
}

impl Instruction {
    /// Advance the dial by this rotation and return the new position.
    fn rotate(self, position: usize) -> usize {
        let delta = self.clicks % DIAL_SIZE;
        match self.dir {
            Rotation::Left => (position + DIAL_SIZE - delta) % DIAL_SIZE,
            Rotation::Right => (position + delta) % DIAL_SIZE,
        }
    }
}

/// Parse strict rotation instructions of form `L|R<clicks>` into direction-click pairs.
fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let line_no = idx + 1;
            let mut chars = line.chars();
            let dir = match chars
                .next()
                .with_context(|| format!("Missing direction on line {}", line_no))?
            {
                'L' => Rotation::Left,
                'R' => Rotation::Right,
                other => bail!("Unknown direction {other} on line {}", line_no),
            };

            let clicks = chars
                .as_str()
                .parse()
                .with_context(|| format!("Invalid click count on line {}", line_no))?;
            Ok(Instruction { dir, clicks })
        })
        .collect()
}

/// Count how often the dial ends a rotation at 0 on a 0-99 circle starting from 50.
fn part_a(rotations: &[Instruction]) -> usize {
    let mut position = START_POS;
    rotations
        .iter()
        .filter_map(|instruction| {
            position = instruction.rotate(position);
            (position == 0).then_some(1)
        })
        .sum()
}

/// Count every click landing on 0 (including mid-rotation) across all rotations.
fn part_b(rotations: &[Instruction]) -> usize {
    let mut position = START_POS;
    let mut hits = 0;
    for &instruction in rotations {
        let offset = match instruction.dir {
            Rotation::Left => position,
            Rotation::Right => DIAL_SIZE - position,
        };
        let clicks_to_zero = if offset == 0 { DIAL_SIZE } else { offset };
        if clicks_to_zero <= instruction.clicks {
            hits += 1 + (instruction.clicks - clicks_to_zero) / DIAL_SIZE;
        }
        position = instruction.rotate(position);
    }
    hits
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let rotations = parse_input(input)?;
    Ok((part_a(&rotations), Some(part_b(&rotations))))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(1, 1034, 6166);

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "#
    );

    #[test]
    fn example_a() {
        assert_eq!(part_a(&parse_input(EXAMPLE_INPUT).unwrap()), 3);
    }

    #[test]
    fn example_b() {
        assert_eq!(part_b(&parse_input(EXAMPLE_INPUT).unwrap()), 6);
    }
}

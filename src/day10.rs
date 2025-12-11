//! # Day 10: Factory
//! Input lists machines, one per line, each with an indicator target in brackets, a set of button
//! wiring diagrams in parentheses, and per-light joltage requirements in braces.
//!
//! ## Part A
//! Indicator lights start off. Pushing a button toggles the listed lights; pushes stack, so pushing
//! the same button twice cancels its effect. Find the minimum number of button pushes needed to
//! reach the target indicator pattern for every machine and sum those counts.
//!
//! ## Part B
//! Switch the buttons to increase joltage counters instead: each machine lists required counter
//! values in braces and buttons add 1 to the listed counters. Starting from all-zero counters,
//! find the minimum presses to reach each machine's exact joltage requirements and sum the presses.
use anyhow::{Context, Result, bail};
use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
    target: u16,
    button_masks: Vec<u16>,
    requirements: Vec<usize>,
    lights: usize,
}

/// Parse a machine line like `[.#.] (0,2) (0,1) {3,5,7}` into target mask, button masks, and
/// joltage requirements.
fn parse_machine(line: &str) -> Result<Machine> {
    let line = line.trim();
    let mut chars = line.chars();
    if chars.next() != Some('[') {
        bail!("Machine description must start with '['");
    }
    let end_indicator = line
        .find(']')
        .context("Missing closing ']' for indicator diagram")?;
    let diagram = &line[1..end_indicator];
    let lights = diagram.len();
    if lights == 0 {
        bail!("Indicator diagram must contain at least one light");
    }

    let mut target: u16 = 0;
    for (idx, ch) in diagram.chars().enumerate() {
        match ch {
            '.' => {}
            '#' => target |= 1 << idx,
            other => bail!("Invalid indicator character '{other}'"),
        }
    }

    let rest = line[end_indicator + 1..].trim();
    let brace_start = rest
        .rfind('{')
        .context("Missing joltage requirement block")?;
    let buttons_part = rest[..brace_start].trim();
    let jolts_part = rest[brace_start..].trim();
    if !jolts_part.ends_with('}') {
        bail!("Missing closing '}}' for joltage requirements");
    }
    let jolts_str = &jolts_part[1..jolts_part.len() - 1];
    let jolts: Vec<usize> = jolts_str
        .split(',')
        .map(|value| value.parse().context("Invalid joltage value"))
        .collect::<Result<_>>()?;
    if jolts.len() != lights {
        bail!("Expected {} joltage entries, found {}", lights, jolts.len());
    }

    let mut button_masks = Vec::new();
    let mut idx = 0;
    while idx < buttons_part.len() {
        while idx < buttons_part.len() && buttons_part.as_bytes()[idx].is_ascii_whitespace() {
            idx += 1;
        }
        if idx == buttons_part.len() {
            break;
        }
        if buttons_part.as_bytes()[idx] != b'(' {
            bail!("Expected '(' when parsing button definition");
        }
        let after_open = idx + 1;
        let close = buttons_part[after_open..]
            .find(')')
            .with_context(|| format!("Missing ')' for button starting at {}", idx))?
            + after_open;
        let button_def = &buttons_part[after_open..close];
        let mut mask: u16 = 0;
        if !button_def.is_empty() {
            for entry in button_def.split(',') {
                let light_idx: usize = entry
                    .parse()
                    .with_context(|| format!("Invalid light index '{entry}'"))?;
                if light_idx >= lights {
                    bail!(
                        "Light index {} out of bounds for {lights}-light machine",
                        light_idx
                    );
                }
                mask ^= 1 << light_idx;
            }
        }
        button_masks.push(mask);
        idx = close + 1;
    }

    if button_masks.is_empty() {
        bail!("Machine must list at least one button");
    }

    Ok(Machine {
        target,
        button_masks,
        requirements: jolts,
        lights,
    })
}

/// Parse all machine definitions from the input.
fn parse_input(input: &str) -> Result<Vec<Machine>> {
    input.trim().lines().map(parse_machine).collect()
}

/// Return the minimum number of button presses needed to reach the target pattern.
fn part_a(machines: &[Machine]) -> Result<usize> {
    machines.iter().try_fold(0, |acc, machine| {
        let states = 1usize << machine.lights;
        let mut dist: Vec<Option<usize>> = vec![None; states];
        let mut queue = VecDeque::new();
        dist[0] = Some(0);
        queue.push_back(0usize);
        while let Some(state) = queue.pop_front() {
            if state as u16 == machine.target {
                break;
            }
            let next_dist = dist[state].unwrap() + 1;
            for &mask in &machine.button_masks {
                let next = state ^ mask as usize;
                if dist[next].is_none() {
                    dist[next] = Some(next_dist);
                    queue.push_back(next);
                }
            }
        }
        let presses = dist[machine.target as usize]
            .with_context(|| "Target configuration unreachable with given buttons")?;
        Ok(acc + presses)
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fraction {
    num: i128,
    den: i128,
}

impl Fraction {
    fn new(num: i128, den: i128) -> Self {
        debug_assert!(den != 0);
        let sign = if den < 0 { -1 } else { 1 };
        let mut num = num * sign;
        let mut den = den.abs();
        let gcd = num.abs().gcd(&den);
        num /= gcd;
        den /= gcd;
        Self { num, den }
    }

    fn from_int(value: i128) -> Self {
        Self { num: value, den: 1 }
    }

    fn is_zero(self) -> bool {
        self.num == 0
    }

    fn scaled(self, denom: i128) -> i128 {
        debug_assert!(denom % self.den == 0);
        self.num * (denom / self.den)
    }
}

impl std::ops::Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lcm = self.den.lcm(&rhs.den);
        let lhs = self.num * (lcm / self.den);
        let rhs = rhs.num * (lcm / rhs.den);
        Fraction::new(lhs + rhs, lcm)
    }
}

impl std::ops::Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let lcm = self.den.lcm(&rhs.den);
        let lhs = self.num * (lcm / self.den);
        let rhs = rhs.num * (lcm / rhs.den);
        Fraction::new(lhs - rhs, lcm)
    }
}

impl std::ops::Mul<i128> for Fraction {
    type Output = Self;

    fn mul(self, rhs: i128) -> Self::Output {
        Fraction::new(self.num * rhs, self.den)
    }
}

impl std::ops::Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl std::ops::Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.den, self.den * rhs.num)
    }
}

trait GcdExt {
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
}

impl GcdExt for i128 {
    fn gcd(&self, other: &Self) -> Self {
        let mut a = self.abs();
        let mut b = other.abs();
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    fn lcm(&self, other: &Self) -> Self {
        if *self == 0 || *other == 0 {
            0
        } else {
            (self / self.gcd(other)) * other.abs()
        }
    }
}

/// Bring a matrix to reduced row echelon form while applying the same operations to the right-hand
/// side vector. Returns the pivot column index for each row.
fn rref(matrix: &mut [Vec<Fraction>], rhs: &mut [Fraction]) -> Result<Vec<Option<usize>>> {
    let rows = matrix.len();
    let cols = matrix.first().map_or(0, Vec::len);
    let mut pivot_cols = vec![None; rows];
    let mut row = 0;

    for col in 0..cols {
        if row == rows {
            break;
        }
        let pivot_row = (row..rows).find(|&r| !matrix[r][col].is_zero());
        let Some(pivot_row) = pivot_row else {
            continue;
        };
        matrix.swap(row, pivot_row);
        rhs.swap(row, pivot_row);

        let pivot = matrix[row][col];
        for entry in matrix[row].iter_mut().skip(col) {
            *entry = *entry / pivot;
        }
        rhs[row] = rhs[row] / pivot;

        for r in 0..rows {
            if r == row || matrix[r][col].is_zero() {
                continue;
            }
            let factor = matrix[r][col];
            let pivot_row = matrix[row].clone();
            for (c, value) in matrix[r].iter_mut().enumerate().skip(col) {
                *value = *value - factor * pivot_row[c];
            }
            rhs[r] = rhs[r] - factor * rhs[row];
        }

        pivot_cols[row] = Some(col);
        row += 1;
    }

    for (r, pivot) in pivot_cols.iter().enumerate() {
        if pivot.is_none() && !rhs[r].is_zero() {
            bail!("System of equations inconsistent");
        }
    }

    Ok(pivot_cols)
}

struct PivotExpr {
    column: usize,
    denom: i128,
    base: i128,
    coeffs: Vec<(usize, i128)>,
}

fn build_pivot_expressions(
    matrix: &[Vec<Fraction>],
    rhs: &[Fraction],
    pivot_cols: &[Option<usize>],
    free_cols: &[usize],
) -> Vec<PivotExpr> {
    let mut expressions = Vec::new();
    for (row, pivot_col) in pivot_cols.iter().enumerate() {
        let Some(column) = pivot_col else {
            continue;
        };
        let mut denom = rhs[row].den;
        for &free_col in free_cols {
            denom = denom.lcm(&matrix[row][free_col].den);
        }

        let base = rhs[row].scaled(denom);
        let mut coeffs = Vec::new();
        for (free_idx, &free_col) in free_cols.iter().enumerate() {
            let coeff = matrix[row][free_col];
            if !coeff.is_zero() {
                coeffs.push((free_idx, coeff.scaled(denom)));
            }
        }
        expressions.push(PivotExpr {
            column: *column,
            denom,
            base,
            coeffs,
        });
    }
    expressions
}

fn evaluate_solution(
    free_values: &[usize],
    pivot_exprs: &[PivotExpr],
    button_caps: &[usize],
) -> Option<usize> {
    let mut total = free_values.iter().sum::<usize>();
    for expr in pivot_exprs {
        let mut numerator = expr.base;
        for (idx, coeff) in &expr.coeffs {
            numerator -= *coeff * free_values[*idx] as i128;
        }
        if numerator % expr.denom != 0 {
            return None;
        }
        let value = numerator / expr.denom;
        if value < 0 {
            return None;
        }
        let value = value as usize;
        if value > button_caps[expr.column] {
            return None;
        }
        total += value;
    }
    Some(total)
}

fn search_free_values(
    idx: usize,
    free_caps: &[usize],
    free_values: &mut [usize],
    partial_sum: usize,
    pivot_exprs: &[PivotExpr],
    button_caps: &[usize],
    best: &mut Option<usize>,
) {
    if idx == free_caps.len() {
        match evaluate_solution(free_values, pivot_exprs, button_caps) {
            Some(cost) if best.is_none_or(|best_cost| cost < best_cost) => {
                *best = Some(cost);
            }
            _ => {}
        }
        return;
    }

    for value in 0..=free_caps[idx] {
        let new_sum = partial_sum + value;
        if best.is_some_and(|b| new_sum >= b) {
            continue;
        }
        free_values[idx] = value;
        search_free_values(
            idx + 1,
            free_caps,
            free_values,
            new_sum,
            pivot_exprs,
            button_caps,
            best,
        );
    }
}

/// Return the minimum presses to reach the exact joltage requirements for one machine.
fn min_presses_counters(machine: &Machine) -> Result<usize> {
    if machine.requirements.iter().all(|&req| req == 0) {
        return Ok(0);
    }

    let button_caps: Vec<usize> = machine
        .button_masks
        .iter()
        .map(|&mask| {
            let mut cap = usize::MAX;
            for (idx, &req) in machine.requirements.iter().enumerate() {
                if mask & (1 << idx) != 0 {
                    cap = cap.min(req);
                }
            }
            if cap == usize::MAX { 0 } else { cap }
        })
        .collect();

    let rows = machine.lights;
    let cols = machine.button_masks.len();
    let mut matrix = vec![vec![Fraction::from_int(0); cols]; rows];
    for (col, &mask) in machine.button_masks.iter().enumerate() {
        for (row_idx, row) in matrix.iter_mut().enumerate() {
            if mask & (1 << row_idx) != 0 {
                row[col] = Fraction::from_int(1);
            }
        }
    }
    let mut rhs: Vec<Fraction> = machine
        .requirements
        .iter()
        .map(|&req| Fraction::from_int(req as i128))
        .collect();

    let pivot_cols = rref(&mut matrix, &mut rhs)?;
    let mut pivot_mask = vec![false; cols];
    for pivot in pivot_cols.iter().flatten() {
        pivot_mask[*pivot] = true;
    }

    if pivot_mask.iter().all(|&p| !p) {
        // No constraints left; the only way to stay within bounds is to press no buttons.
        return Ok(0);
    }

    let free_cols: Vec<usize> = (0..cols).filter(|&col| !pivot_mask[col]).collect();
    let pivot_exprs = build_pivot_expressions(&matrix, &rhs, &pivot_cols, &free_cols);
    let free_caps: Vec<usize> = free_cols.iter().map(|&col| button_caps[col]).collect();
    let mut free_values = vec![0usize; free_caps.len()];
    let mut best = None;
    search_free_values(
        0,
        &free_caps,
        &mut free_values,
        0,
        &pivot_exprs,
        &button_caps,
        &mut best,
    );

    best.context("Joltage requirements unreachable")
}

/// Return the minimum presses to satisfy all joltage requirements across machines.
fn part_b(machines: &[Machine]) -> Result<usize> {
    machines.iter().try_fold(0usize, |acc, machine| {
        Ok(acc + min_presses_counters(machine)?)
    })
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let machines = parse_input(input)?;
    Ok((part_a(&machines)?, Some(part_b(&machines)?)))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(10, 438, Some(16463));

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "#
    );

    #[test]
    fn example_a() {
        let machines = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_a(&machines).unwrap(), 7);
    }

    #[test]
    fn example_b() {
        let machines = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_b(&machines).unwrap(), 33);
    }
}

//! # Day 8: Playground
//! Input is a list of junction box coordinates, one per line, formatted as `x,y,z` integers.
//!
//! ## Part A
//! Compute straight-line distances between every pair of boxes and order the unique pairs by
//! increasing distance (breaking ties by input order). Connect the 1000 closest pairs in that
//! order, treating each connection as undirected; connecting boxes already in the same circuit
//! leaves the circuit sizes unchanged. After all connections, find the size of every resulting
//! circuit and return the product of the three largest sizes.
//!
//! ## Part B
//! Keep connecting boxes in that same order until all boxes belong to a single circuit. Return the
//! product of the X coordinates of the final connection that merges the circuits into one.
use anyhow::{Context, Result, bail};
use std::cmp::Reverse;
use std::collections::HashMap;

const CONNECTIONS: usize = 1000;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
            size: vec![1; len],
        }
    }

    fn find(&mut self, idx: usize) -> usize {
        if self.parent[idx] == idx {
            return idx;
        }
        let root = self.find(self.parent[idx]);
        self.parent[idx] = root;
        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for idx in 0..self.parent.len() {
            let root = self.find(idx);
            *counts.entry(root).or_insert(0) += 1;
        }
        counts.into_values().collect()
    }
}

/// Parse a list of strict `x,y,z` coordinate triples into points.
fn parse_input(input: &str) -> Result<Vec<Point>> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let line_no = idx + 1;
            let mut parts = line.split(',');
            let x = parts
                .next()
                .context("Missing X coordinate")?
                .parse()
                .with_context(|| format!("Invalid X value on line {}", line_no))?;
            let y = parts
                .next()
                .context("Missing Y coordinate")?
                .parse()
                .with_context(|| format!("Invalid Y value on line {}", line_no))?;
            let z = parts
                .next()
                .context("Missing Z coordinate")?
                .parse()
                .with_context(|| format!("Invalid Z value on line {}", line_no))?;

            if parts.next().is_some() {
                bail!("Too many comma-separated values on line {}", line_no);
            }

            Ok(Point { x, y, z })
        })
        .collect()
}

fn squared_distance(a: &Point, b: &Point) -> u128 {
    let dx = a.x.abs_diff(b.x) as u128;
    let dy = a.y.abs_diff(b.y) as u128;
    let dz = a.z.abs_diff(b.z) as u128;
    dx * dx + dy * dy + dz * dz
}

fn sorted_edges(points: &[Point]) -> Vec<(u128, usize, usize)> {
    let mut edges = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            edges.push((squared_distance(&points[i], &points[j]), i, j));
        }
    }

    edges.sort_by_key(|&(dist, i, j)| (dist, i, j));
    edges
}

fn connect(points: &[Point], edges: &[(u128, usize, usize)], connection_limit: usize) -> usize {
    let mut uf = UnionFind::new(points.len());
    for (_, a, b) in edges.iter().copied().take(connection_limit) {
        uf.union(a, b);
    }

    let mut sizes = uf.component_sizes();
    sizes.sort_unstable_by_key(|&size| Reverse(size));
    debug_assert!(sizes.len() >= 3);
    sizes.iter().take(3).product()
}

fn final_connection(points: &[Point], edges: &[(u128, usize, usize)]) -> usize {
    let mut uf = UnionFind::new(points.len());
    let mut components = points.len();
    for &(_, a, b) in edges {
        let ra = uf.find(a);
        let rb = uf.find(b);
        if ra == rb {
            continue;
        }
        uf.union(ra, rb);
        components -= 1;
        if components == 1 {
            return points[a].x * points[b].x;
        }
    }
    unreachable!("All points should eventually connect");
}

/// Connect the 1000 closest pairs of boxes and multiply the three largest circuit sizes.
fn part_a(points: &[Point]) -> usize {
    let edges = sorted_edges(points);
    connect(points, &edges, CONNECTIONS)
}

/// Multiply X coordinates of the final connection that joins all boxes.
fn part_b(points: &[Point]) -> usize {
    let edges = sorted_edges(points);
    final_connection(points, &edges)
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let points = parse_input(input)?;
    Ok((part_a(&points), Some(part_b(&points))))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(8, 175_440, Some(3_200_955_921usize));

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "#
    );

    #[test]
    fn example_a() {
        let points = parse_input(EXAMPLE_INPUT).unwrap();
        let edges = sorted_edges(&points);
        assert_eq!(connect(&points, &edges, 10), 40);
    }

    #[test]
    fn example_b() {
        let points = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_b(&points), 25_272);
    }
}

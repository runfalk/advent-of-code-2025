//! # Day 9: Movie Theater
//! Input is a loop of red tile coordinates as `x,y` pairs, one per line, listed in order around
//! the perimeter. Consecutive tiles share a row or column and all tiles between them are green;
//! the first and last tiles are also connected. All tiles enclosed by this perimeter are green
//! too.
//!
//! ## Part A
//! Pick any two red tiles as opposite corners of an axis-aligned rectangle. Return the largest
//! possible area, counting every tile inside the inclusive rectangle spanned by those corners.
//!
//! ## Part B
//! Red corners still define the rectangle, but every tile it covers must be red or green (inside
//! the perimeter). Find the largest possible area under this restriction.
use anyhow::{Context, Result, bail};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    a: Point,
    b: Point,
}

impl Rect {
    /// Create an inclusive axis-aligned rectangle from two opposite corners.
    fn new(a: Point, b: Point) -> Self {
        Rect {
            a: Point {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
            b: Point {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
        }
    }

    /// Return the inclusive area of the rectangle.
    fn area(&self) -> usize {
        (self.b.x - self.a.x + 1) * (self.b.y - self.a.y + 1)
    }
}

/// Parse strict `x,y` coordinate pairs for red tiles.
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

            if parts.next().is_some() {
                bail!("Too many comma-separated values on line {}", line_no);
            }

            Ok(Point { x, y })
        })
        .collect()
}

/// Return the largest possible rectangle area using any two red tiles as opposite corners.
fn part_a(points: &[Point]) -> usize {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| {
            points
                .iter()
                .skip(i + 1)
                .map(move |&b| Rect::new(a, b).area())
        })
        .max()
        .unwrap_or(0)
}

/// Return the largest rectangle that fits fully inside the green area with red opposite corners.
fn part_b(points: &[Point]) -> Result<usize> {
    let is_axis_aligned = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .all(|(a, b)| a.x == b.x || a.y == b.y);
    if !is_axis_aligned {
        bail!("Perimeter contains diagonal edge");
    }
    let min_y = points
        .iter()
        .map(|p| p.y)
        .min()
        .with_context(|| "Missing minimum Y value")?;
    let max_y = points
        .iter()
        .map(|p| p.y)
        .max()
        .with_context(|| "Missing maximum Y value")?;
    let height = max_y - min_y + 1;
    let mut scanlines: Vec<Vec<usize>> = vec![Vec::new(); height];
    let mut ranges_by_y: Vec<Vec<(usize, usize)>> = vec![Vec::new(); height];

    for (&a, &b) in points.iter().zip(points.iter().cycle().skip(1)) {
        if a.y == b.y {
            let (x1, x2) = (a.x.min(b.x), a.x.max(b.x));
            ranges_by_y[a.y - min_y].push((x1, x2));
        } else if a.x == b.x {
            let y_start = a.y.min(b.y);
            let y_end = a.y.max(b.y);
            for y in y_start..y_end {
                scanlines[y - min_y].push(a.x);
            }
        }
    }

    for (offset, xs) in scanlines.into_iter().enumerate() {
        let mut xs = xs;
        xs.sort_unstable();
        if xs.len() % 2 != 0 {
            bail!(
                "Uneven number of intersections on scanline {}",
                offset + min_y
            );
        }
        for pair in xs.chunks_exact(2) {
            ranges_by_y[offset].push((pair[0], pair[1]));
        }
    }

    for ranges in &mut ranges_by_y {
        ranges.sort_unstable_by_key(|&(start, _)| start);
        let mut merged: Vec<(usize, usize)> = Vec::new();
        for (start, end) in ranges.drain(..) {
            if let Some((_, last_end)) = merged.last_mut()
                && start <= *last_end + 1
            {
                *last_end = (*last_end).max(end);
                continue;
            }
            merged.push((start, end));
        }
        *ranges = merged;
    }

    let max_area = points
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| points.iter().skip(i + 1).map(move |&b| Rect::new(a, b)))
        .filter(|rect| {
            (rect.a.y..=rect.b.y).all(|y| {
                ranges_by_y[y - min_y]
                    .iter()
                    .any(|&(start, end)| start <= rect.a.x && rect.b.x <= end)
            })
        })
        .map(|rect| rect.area())
        .max()
        .unwrap_or(0);
    Ok(max_area)
}

pub fn main(input: &str) -> Result<(usize, Option<usize>)> {
    let points = parse_input(input)?;
    Ok((part_a(&points), Some(part_b(&points)?)))
}

#[cfg(test)]
mod test {
    use dedent::dedent;

    use super::*;

    test_real_input!(9, 4_771_508_457, Some(1_539_809_693));

    const EXAMPLE_INPUT: &str = dedent!(
        r#"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "#
    );

    #[test]
    fn example_a() {
        let points = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_a(&points), 50);
    }

    #[test]
    fn example_b() {
        let points = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part_b(&points).unwrap(), 24);
    }

    #[test]
    fn parses_single_coordinate() {
        let points = parse_input("1,2").unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].x, 1);
        assert_eq!(points[0].y, 2);
    }
}

extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::common::*;
use aoc_lib::data::Point2;
use aoc_lib::harness::*;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day15;
type Input = Vec<Sensor>;
type Output = i64;
impl Solution<Input, Output> for Day15 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Beacon Exclusion Zone", 2022, 15)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let lines = resource.as_str_lines()?;
        lines
            .iter()
            .map(|x| {
                // "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
                let parts: Vec<&str> = x.split_whitespace().collect();
                Ok(Sensor {
                    pos: Pos::new(chop(parts[2], 2, 1)?, chop(parts[3], 2, 1)?),
                    beacon: Pos::new(chop(parts[8], 2, 1)?, chop(parts[9], 2, 0)?),
                })
            })
            .collect()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        scan_y(input, 2_000_000)
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        scan_xy(input, 4_000_000)
    }
}

type Pos = Point2<i32>;

#[derive(Debug, PartialEq, Eq)]
pub struct Sensor {
    pos: Pos,
    beacon: Pos,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Span {
    from: i32,
    to: i32,
}

/// Given a string slice, chops off start and end and parses to i32
fn chop<T: FromStr>(text: &str, drop_start: usize, drop_end: usize) -> Result<T> {
    parse_str(&text[drop_start..text.len() - drop_end])
    // Ok(text[drop_start..text.len() - drop_end].parse::<T>()?)
}

/// Calculates all covered spans
fn calc_spans(input: &Input, y: i32) -> Result<Vec<Span>> {
    // (from, to-inclusive) ranges of coverage
    let mut spans: Vec<Span> = Vec::with_capacity(input.len());
    for s in input {
        // Row coverage is a function of S->B distance and S.y->scan.y separation
        // dist = manhattan distance from S to B
        // Range centred on S.x
        // At y = S.y + dist, range would just be S.x
        // At y = S.y + dist - 1, range would be S.x - 1 -> S.x + 1
        let sb_dist = (s.pos - s.beacon).manhattan();
        let y_dist = (s.pos.y - y).abs();
        let span_extent = sb_dist - y_dist;
        if span_extent >= 0 {
            spans.push(Span { from: s.pos.x - span_extent, to: s.pos.x + span_extent });
        }
    }
    combine_spans(&spans)
}

/// Combines spans that overlap
fn combine_spans(spans: &Vec<Span>) -> Result<Vec<Span>> {
    // Sort the covered spans
    let mut spans = spans.to_owned();
    spans.sort_unstable_by(|a, b| a.from.cmp(&b.from));
    // Smush em
    let mut current = spans.first().expect("spans should not be empty").clone();
    let mut result: Vec<Span> = Vec::with_capacity(spans.len());
    for span in spans {
        if span.from > current.to + 1 {
            result.push(current);
            current = span.clone();
        }
        current.to = current.to.max(span.to);
    }
    result.push(current);
    Ok(result)
}

/// Total horizontal beacon-free coverage on a row
fn scan_y(input: &Input, y: i32) -> Result<i64> {
    let spans = calc_spans(input, y)?;
    // We should subtract any beacons on the row.
    // It works without, but only because doing to-from is 1 off (because "to" is inclusive),
    // and both test and real input have exactly one beacon on the scan row :)
    let mut bxs: HashSet<i32> = HashSet::new();
    input.iter().for_each(|x| {
        if x.beacon.y == y {
            bxs.insert(x.beacon.x);
        }
    });
    let area: i32 = spans.iter().map(|x| x.to + 1 - x.from).sum();
    Ok((area as usize - bxs.len()) as i64)
}

/// Find the first uncovered point in (0..=max_xy) along each axis
fn scan_xy(input: &Input, max_xy: i32) -> Result<i64> {
    // Perhaps there is a cleverer way than just checking every row, but it still runs in <1s
    for i in (0..max_xy).rev() {
        let spans = calc_spans(input, i)?;
        for s in spans {
            if s.to >= 0 && s.to < max_xy {
                return Ok(i as i64 + ((s.to + 1) as i64 * 4_000_000));
            }
        }
    }
    Err(anyhow!("No result"))
}

fn main() -> Result<()> {
    run_solution(&Day15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = Day15
            .parse_input(&InlineResource::new(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            ))
            .unwrap();
        assert_eq!(input, vec![Sensor { pos: Pos::new(2, 18), beacon: Pos::new(-2, 15) }]);
    }

    #[test]
    fn test_part1() {
        let input = Day15.parse_input(&FileResource::new("test", 2022, 15)).unwrap();
        assert_eq!(scan_y(&input, 10).unwrap(), 26);
    }

    #[test]
    fn test_part2() {
        let input = Day15.parse_input(&FileResource::new("test", 2022, 15)).unwrap();
        assert_eq!(scan_xy(&input, 20).unwrap(), 56000011);
    }
}

extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::common;
use aoc_lib::harness::*;

pub struct Day04;
type Input = Vec<(SecRange, SecRange)>;
type Output = u32;
impl Solution<Input, Output> for Day04 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Camp Cleanup", 2022, 4)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        parse_sec_range_pairs(&resource.as_str()?)
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().filter(|(a, b)| contains(a, b) || contains(b, a)).count() as u32)
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().filter(|(a, b)| overlaps(a, b)).count() as u32)
    }
}

/// Range of Section IDs.
/// Could also use e.g. RangeInclusive, but it doesn't really provide anything useful.
pub struct SecRange {
    from: u32,
    to: u32,
}

/// Convert "xxx-yyy" into SecRange
fn parse_sec_range(sec: &str) -> Result<SecRange> {
    let parts: Vec<&str> = sec.split('-').collect();
    Ok(SecRange { from: parts[0].parse()?, to: parts[1].parse()? })
}

/// Convert "a-b,c-d" into a pair of SecRanges
fn parse_sec_range_pairs(content: &str) -> Result<Vec<(SecRange, SecRange)>> {
    let lines = common::split_lines(content);
    let mut result = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        result.push((parse_sec_range(parts[0])?, parse_sec_range(parts[1])?));
    }
    Ok(result)
}

/// True if given range contains the other range.
fn contains(a: &SecRange, b: &SecRange) -> bool {
    a.from <= b.from && a.to >= b.to
}

/// True if the two ranges overlap.
fn overlaps(a: &SecRange, b: &SecRange) -> bool {
    a.from <= b.to && b.from <= a.to
}

fn main() -> Result<()> {
    run_solution(&Day04)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day04, SolutionPart::One), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day04, SolutionPart::Two), 4);
    }
}

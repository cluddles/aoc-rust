extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::harness::*;

pub struct Day06;
type Input = Vec<u8>;

/// True if the given slice contains any duplicate values
fn contains_duplicates(text: &[u8]) -> bool {
    for i in 0..text.len() {
        for j in (i + 1)..text.len() {
            if text[i] == text[j] {
                return true;
            }
        }
    }
    false
}

/// Returns end position of first non-duplicate segment of given size
fn find_unique_marker(text: &[u8], len: usize) -> Result<usize> {
    Ok((0..text.len() - len)
        .find(|&i| !contains_duplicates(&text[i..i + len]))
        .ok_or_else(|| anyhow!("No unique marker"))?
        + len)
}

impl Solution<Input, usize> for Day06 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Tuning Trouble", 2022, 6)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_u8()
    }

    /// Unique segment, length 4
    fn solve_part1(&self, input: &Input) -> Result<usize> {
        find_unique_marker(input, 4)
    }

    /// Unique segment, length 14
    fn solve_part2(&self, input: &Input) -> Result<usize> {
        find_unique_marker(input, 14)
    }
}

fn main() -> Result<()> {
    run_solution(&Day06)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let solution = Day06;
        let test = |x| test_inline(&solution, SolutionPart::One, x);
        assert_eq!(test("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(test("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(test("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(test("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(test("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        let solution = Day06;
        let test = |x| test_inline(&solution, SolutionPart::Two, x);
        assert_eq!(test("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(test("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(test("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(test("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(test("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

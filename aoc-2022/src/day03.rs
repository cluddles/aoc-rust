extern crate aoc_lib;

use aoc_lib::common;
use aoc_lib::harness::*;

/// Split each line into 2 compartments
fn parse_compartments(content: &str) -> Vec<Vec<&str>> {
    common::split_lines(content)
        .iter()
        .map(|x| x.split_at(x.len() / 2))
        .map(|x| vec![x.0, x.1])
        .collect()
}

/// Split by groups of 3 lines
fn parse_elf_groups(content: &str) -> Vec<Vec<&str>> {
    let lines = common::split_lines(content);
    let mut result = Vec::new();
    for i in (0..lines.len()).step_by(3) {
        result.push(vec![lines[i], lines[i + 1], lines[i + 2]]);
    }
    result
}

/// Find duplicates in the given strings
fn dup(val: &[&str]) -> u8 {
    val[0]
        .chars()
        .find(|&x| (1..val.len()).all(|y| val[y].contains(x)))
        .unwrap() as u8
}

/// Convert char to priority value
fn priority(val: u8) -> u8 {
    match val {
        b'A'..=b'Z' => val - b'A' + 27,
        b'a'..=b'z' => val - b'a' + 1,
        _ => panic!(),
    }
}

/// Sum of all duplicate char priorities
fn sum_priority(input: &[Vec<&str>]) -> u32 {
    input.iter().map(|x| priority(dup(x)) as u32).sum()
}

type Input = String;
type Output = u32;
pub struct Day03;
impl Solution<Input, Output> for Day03 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Rucksack Reorganization", 2022, 3)
    }

    fn parse_input(&self, resource: &dyn Resource) -> DynResult<Input> {
        Ok(resource.as_str())
    }

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        Ok(sum_priority(&parse_compartments(input)))
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        Ok(sum_priority(&parse_elf_groups(input)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day03, SolutionPart::One), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day03, SolutionPart::Two), 70);
    }
}

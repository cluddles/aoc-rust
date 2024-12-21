extern crate aoc_lib;

use anyhow::Result;

use aoc_lib::common;
use aoc_lib::harness::*;

pub struct Day07;

type Input = Vec<u32>;
type Output = u32;

impl Solution<Input, Output> for Day07 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("The Treachery of Whales", 2021, 7)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        common::tokenize_first_line(&resource.as_str()?, ',')
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(score_linear(input, median(input)))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(score_tri(input, mean(input).round() as u32))
    }
}

/// Calculate distance between two points
fn dist(a: u32, b: u32) -> u32 {
    (a as i32 - b as i32).unsigned_abs()
}

/// Linear score, where each point of distance costs a flat 1
fn score_linear(state: &[u32], pos: u32) -> u32 {
    state.iter().map(|x| dist(*x, pos)).sum()
}

/// Calculate triangle value (this isn't factorial; my brain broke)
fn tri(x: u32) -> u32 {
    (x * (x + 1)) / 2
}

/// Triangle score, where each point of distance costs 1 more than the previous
fn score_tri(state: &[u32], pos: u32) -> u32 {
    state.iter().map(|x| tri(dist(*x, pos))).sum()
}

/// Calculate the median of the given Vec
fn median(input: &[u32]) -> u32 {
    let mut state = input.to_owned();
    state.sort_unstable();
    state[state.len() / 2]
}

/// Calculate the mean of the given Vec
fn mean(input: &[u32]) -> f64 {
    input.iter().sum::<u32>() as f64 / input.len() as f64
}

fn main() -> Result<()> {
    run_solution(&Day07)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day07, SolutionPart::One), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day07, SolutionPart::Two), 168);
    }
}

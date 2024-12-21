extern crate aoc_lib;

use anyhow::Result;

use aoc_lib::common;
use aoc_lib::harness::*;

pub struct Day01;

type Input = Vec<u32>;
type Output = usize;

impl Solution<Input, Output> for Day01 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Sonar Sweep", 2021, 1)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        common::tokenize(&resource.as_str()?, '\n')
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().enumerate().skip(1).filter(|(i, &x)| x > input[i - 1]).count())
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let mut prev = 0;
        let mut count = 0;
        for i in 0..input.len() - 2 {
            let sum3 = input[i..i + 2].iter().sum();
            if prev != 0 && sum3 > prev {
                count += 1;
            }
            prev = sum3
        }
        Ok(count)
    }
}

fn main() -> Result<()> {
    run_solution(&Day01)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day01, SolutionPart::One), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day01, SolutionPart::Two), 5);
    }
}

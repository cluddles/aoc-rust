extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::common;
use aoc_lib::harness::*;

#[derive(Debug, Clone, Default)]
pub struct Elf {
    carried: Vec<u32>,
    total: u32,
}

type Input = Vec<Elf>;
type Output = u32;
pub struct Day01;
impl Solution<Input, Output> for Day01 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Calorie Counting", 2022, 1)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let text = resource.as_str()?;
        let lines = common::split_lines_keep_empty(&text);
        let mut result: Vec<Elf> = Vec::new();
        let mut elf = Elf::default();
        for l in lines {
            if l.chars().count() == 0 {
                // Empty line - commit the current elf, start a new one
                result.push(elf);
                elf = Elf::default();
            } else {
                // Update current elf
                let val = l.trim().parse::<u32>()?;
                elf.carried.push(val);
                elf.total += val;
            }
        }
        // Don't forgot to commit the last elf!
        if elf.total != 0 {
            result.push(elf);
        }
        Ok(result)
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().map(|x| x.total).max().ok_or_else(|| anyhow!("No max"))?)
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let mut max_first = input.to_vec();
        max_first.sort_by(|a, b| b.total.cmp(&a.total));
        Ok(max_first.iter().take(3).map(|x| x.total).sum())
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
        assert_eq!(test_solution(&Day01, SolutionPart::One), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day01, SolutionPart::Two), 45000);
    }
}

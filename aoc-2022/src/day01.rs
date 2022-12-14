extern crate aoc_lib;

use aoc_lib::common;
use aoc_lib::harness::*;

#[derive(Debug, Clone)]
pub struct Elf {
    carried: Vec<u32>,
    total: u32,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            carried: Vec::new(),
            total: 0,
        }
    }
}

type Input = Vec<Elf>;
type Output = u32;
pub struct Day01;
impl Solution<Input, Output> for Day01 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Calorie Counting", 2022, 1)
    }

    fn parse_input(&self, resource: &dyn Resource) -> DynResult<Input> {
        let text = resource.as_str();
        let lines = common::split_lines_keep_empty(&text);
        let mut result: Vec<Elf> = Vec::new();
        let mut elf = Elf::new();
        for l in lines {
            if l.chars().count() == 0 {
                // Empty line - commit the current elf, start a new one
                result.push(elf);
                elf = Elf::new();
            } else {
                // Update current elf
                let val = l.trim().parse::<u32>().unwrap();
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

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        Ok(input.iter().map(|x| x.total).max().unwrap())
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        let mut max_first = input.to_vec();
        max_first.sort_by(|a, b| b.total.cmp(&a.total));
        Ok(max_first.iter().take(3).map(|x| x.total).sum())
    }
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

extern crate aoc_lib;

use anyhow::Result;

use aoc_lib::harness::*;

pub struct Day02;

type Input = Vec<Instruction>;
type Output = u32;

impl Solution<Input, Output> for Day02 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Dive!", 2021, 2)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()?.iter().map(|x| parse_instruction(x)).collect::<Result<_, _>>()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(part1(input))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(part2(input))
    }
}

pub struct Instruction {
    name: String,
    amount: u32,
}

/// Parse a single instruction from a line of text
fn parse_instruction(line: &str) -> Result<Instruction> {
    let parts: Vec<&str> = line.trim().split(' ').collect();
    Ok(Instruction { name: parts[0].to_string(), amount: parts[1].parse()? })
}

/// Horizontal * depth after running instructions
fn part1(instructions: &Vec<Instruction>) -> u32 {
    let mut horiz: u32 = 0;
    let mut depth: u32 = 0;
    for instruction in instructions {
        match instruction.name.as_str() {
            "forward" => horiz += instruction.amount,
            "up" => depth -= instruction.amount,
            "down" => depth += instruction.amount,
            _ => (),
        }
    }
    horiz * depth
}

/// With additional "aim"
fn part2(instructions: &Vec<Instruction>) -> u32 {
    let mut horiz: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    for instruction in instructions {
        match instruction.name.as_str() {
            "forward" => {
                horiz += instruction.amount;
                depth += aim * instruction.amount
            }
            "up" => aim -= instruction.amount,
            "down" => aim += instruction.amount,
            _ => (),
        }
    }
    horiz * depth
}

fn main() -> Result<()> {
    run_solution(&Day02)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day02, SolutionPart::One), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day02, SolutionPart::Two), 900);
    }
}

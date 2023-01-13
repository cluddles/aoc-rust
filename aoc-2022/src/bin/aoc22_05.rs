extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::common;
use aoc_lib::harness::*;

pub struct Day05;
type Input = (Vec<Vec<u8>>, Vec<Move>);
type Output = String;
impl Solution<Input, Output> for Day05 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Supply Stacks", 2022, 5)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let content = resource.as_str()?;
        let parts: Vec<&str> = content.split("\n\n").collect();
        Ok((parse_crates(parts[0])?, parse_moves(parts[1])?))
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        let (crates, moves) = input;
        let mut state = crates.to_owned();
        for m in moves {
            for _ in 0..m.quantity {
                let popped = state[m.from - 1].pop().ok_or_else(|| anyhow!("Nothing to pop"))?;
                state[m.to - 1].push(popped);
            }
        }
        summarise(&state)
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let (crates, moves) = input;
        let mut state = crates.to_owned();
        for m in moves {
            let mut popped = Vec::new();
            (0..m.quantity).for_each(|_| popped.push(state[m.from - 1].pop().unwrap()));
            popped.iter().rev().for_each(|&p| state[m.to - 1].push(p));
        }
        summarise(&state)
    }
}

#[derive(Debug)]
pub struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

/// Parse the crate block of the input, including the final line with column indexes
fn parse_crates(content: &str) -> Result<Vec<Vec<u8>>> {
    let lines = common::split_lines(content);
    // Get number of columns from the last line
    let cols =
        lines.last().ok_or_else(|| anyhow!("Unable to find columns"))?.split_whitespace().count();
    let mut result = vec![Vec::new(); cols];
    // Iterate backwards over lines (except the last)
    for line in lines.iter().rev().skip(1) {
        // [A] [B] [C] etc: meaningful chars are 1, 5, 9, ...
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                result[i].push(c as u8)
            }
        }
    }
    Ok(result)
}

/// Parse a single move: "move 1 from 1 to 2"
fn parse_move(line: &str) -> Result<Move> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Ok(Move { quantity: parts[1].parse()?, from: parts[3].parse()?, to: parts[5].parse()? })
}

/// Parse the move block of the input
fn parse_moves(content: &str) -> Result<Vec<Move>> {
    common::split_lines(content).iter().map(|x| parse_move(x)).collect()
}

/// Summarise the result, by taking the top crate from each stack
fn summarise(crates: &[Vec<u8>]) -> Result<String> {
    crates.iter().map(|x| Ok(*x.last().ok_or(anyhow!("No crate data"))? as char)).collect()
}

fn main() -> Result<()> {
    run_solution(&Day05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day05, SolutionPart::One), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day05, SolutionPart::Two), "MCD");
    }
}

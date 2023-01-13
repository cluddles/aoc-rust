extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::harness::*;

pub struct Day11;
type Input = Vec<Monkey>;
type Output = u64;
impl Solution<Input, Output> for Day11 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Monkey in the Middle", 2022, 11)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str()?.split("\n\n").map(Monkey::parse).collect()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(run_sim(input, true, 20))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(run_sim(input, false, 10000))
    }
}

/// Supported operations: add (fixed value), multiply (fixed value and self, i.e. squaring)
#[derive(Debug)]
enum Op {
    Add(u64),
    Mult(u64),
    Sq(),
}

impl Op {
    fn parse(op: &str, val: &str) -> Result<Op> {
        Ok(match (op, val) {
            ("*", "old") => Op::Sq(),
            ("*", v) => Op::Mult(v.parse::<u64>()?),
            ("+", v) => Op::Add(v.parse::<u64>()?),
            (op, v) => return Err(anyhow!(format!("could not parse op: '{}' '{}'", op, v))),
        })
    }
}

/// Monkey definition
pub struct Monkey {
    items: Vec<u64>,
    op: Op,
    div_by: u64,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn parse(text: &str) -> Result<Monkey> {
        let lines: Vec<&str> = text.split('\n').collect();
        let items: &str = lines[1].split(':').collect::<Vec<&str>>()[1];
        let items = items.split(',').map(|x| x.trim().parse::<u64>()).collect::<Result<_, _>>()?;
        let op: Vec<&str> = lines[2].split_whitespace().collect();
        let op = Op::parse(op[4], op[5])?;
        let last_num = |x: &str| x.split_whitespace().last().unwrap_or("").parse();
        Ok(Monkey {
            items,
            op,
            div_by: last_num(lines[3])? as u64,
            true_target: last_num(lines[4])?,
            false_target: last_num(lines[5])?,
        })
    }
}

/// Monkey state as sim runs
pub struct MonkeyState {
    items: Vec<u64>,
    inspections: u64,
}

/// Run sim for given number of rounds
fn run_sim(input: &Input, reduce_worry: bool, num_rounds: u32) -> u64 {
    // Manage "ridiculous" worry levels (part 2) using common multiple
    let common_multiple: Option<u64> =
        if reduce_worry { None } else { Some(input.iter().map(|x| x.div_by).product()) };
    let mut state: Vec<MonkeyState> =
        input.iter().map(|x| MonkeyState { items: x.items.to_owned(), inspections: 0 }).collect();
    for _ in 0..num_rounds {
        run_sim_once(&mut state, input, common_multiple);
    }
    let mut inspections: Vec<u64> = state.iter().map(|x| x.inspections).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

/// Run sim for a single round
fn run_sim_once(state: &mut [MonkeyState], input: &Input, common_multiple: Option<u64>) {
    for (i, monkey) in input.iter().enumerate() {
        // This isn't great
        let b = state[i].items.clone();
        b.iter().for_each(|x| {
            let worry = match monkey.op {
                Op::Add(v) => x + v,
                Op::Mult(v) => x * v,
                Op::Sq() => x * x,
            };
            let worry = match common_multiple {
                Some(v) => worry % v,
                None => worry / 3,
            };
            let target = match worry % monkey.div_by == 0 {
                true => monkey.true_target,
                false => monkey.false_target,
            };
            state[target].items.push(worry);
        });
        state[i].inspections += state[i].items.len() as u64;
        state[i].items.clear();
    }
}

fn main() -> Result<()> {
    run_solution(&Day11)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day11, SolutionPart::One), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day11, SolutionPart::Two), 2713310158);
    }
}

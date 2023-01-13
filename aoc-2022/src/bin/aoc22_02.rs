extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::common;
use aoc_lib::harness::*;

type Input = Vec<Strategy>;
type Output = u32;
pub struct Day02;
impl Solution<Input, Output> for Day02 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Rock Paper Scissors", 2022, 2)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        parse_strategy_guide(&resource.as_str_lines()?)
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().map(score_part1).sum())
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().map(score_part2).sum())
    }
}

/// Player choice
#[derive(Debug, PartialEq, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    /// The opposing choice that this beats
    fn beats(&self) -> Choice {
        match *self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    /// The opposing choice that this loses to
    fn loses_to(&self) -> Choice {
        match *self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    /// Score for this choice
    fn score(&self) -> u32 {
        match *self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

/// The outcome of a single round
#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    /// Score for this outcome
    fn score(&self) -> u32 {
        match *self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

/// A single entry from the strategy guide
#[derive(Debug)]
pub struct Strategy {
    opp: char,
    strat: char,
}

/// Lookup choice from strategy guide text
fn parse_choice(choice: char) -> Choice {
    match choice {
        'A' | 'X' => Choice::Rock,
        'B' | 'Y' => Choice::Paper,
        'C' | 'Z' => Choice::Scissors,
        _ => panic!("Unrecognised choice: {}", choice),
    }
}

/// Lookup outcome from strategy guide test
fn parse_outcome(outcome: char) -> Outcome {
    match outcome {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Unrecognised outcome: {}", outcome),
    }
}

/// Parse strategy for a round from a line of text
fn parse_strategy(line: &str) -> Result<Strategy> {
    let parts: Vec<char> = common::tokenize(line, ' ')?;
    Ok(Strategy { opp: parts[0], strat: parts[1] })
}

/// Parse complete strategy guide
fn parse_strategy_guide(lines: &[String]) -> Result<Vec<Strategy>> {
    lines.iter().map(|x| parse_strategy(x)).collect()
}

/// Calculate the outcome of both players' choices
fn outcome(opp: &Choice, mine: &Choice) -> Outcome {
    if mine == opp {
        return Outcome::Draw;
    } else if &mine.beats() == opp {
        return Outcome::Win;
    }
    Outcome::Lose
}

/// Score part 1: X,Y,Z map directly to our choices
fn score_part1(round: &Strategy) -> u32 {
    let (opp, mine) = (parse_choice(round.opp), parse_choice(round.strat));
    outcome(&opp, &mine).score() + mine.score()
}

/// Given opponent choice and our desired outcome, what choice should we make?
fn choice_for_outcome(opp: &Choice, target_outcome: &Outcome) -> Choice {
    match target_outcome {
        Outcome::Draw => *opp,
        Outcome::Win => opp.loses_to(),
        Outcome::Lose => opp.beats(),
    }
}

/// Score part 2: X,Y,Z map to our desired outcomes
fn score_part2(round: &Strategy) -> u32 {
    let opp = parse_choice(round.opp);
    let target_outcome = parse_outcome(round.strat);
    let mine = choice_for_outcome(&opp, &target_outcome);
    outcome(&opp, &mine).score() + mine.score()
}

fn main() -> Result<()> {
    run_solution(&Day02)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day02, SolutionPart::One), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day02, SolutionPart::Two), 12);
    }
}

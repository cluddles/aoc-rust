extern crate aoc;

use aoc::shared;

const DAY: &str = "2022/02";

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
struct Strategy {
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
fn parse_strategy(line: &str) -> Strategy {
    let parts: Vec<char> = shared::tokenize(line, ' ');
    Strategy {
        opp: parts[0],
        strat: parts[1],
    }
}

/// Parse complete strategy guide
fn parse_strategy_guide(content: &str) -> Vec<Strategy> {
    let lines = shared::split_lines(content);
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

fn part1(moves: &[Strategy]) -> u32 {
    moves.iter().map(score_part1).sum()
}

fn part2(moves: &[Strategy]) -> u32 {
    moves.iter().map(score_part2).sum()
}

fn main() {
    let content = shared::input_as_str(DAY, "input");
    let rounds = parse_strategy_guide(&content);
    println!("Part 1: {}", part1(&rounds));
    println!("Part 2: {}", part2(&rounds));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_moves() -> Vec<Strategy> {
        parse_strategy_guide(&shared::input_as_str(DAY, "input.test"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_test_moves()), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_test_moves()), 12);
    }
}

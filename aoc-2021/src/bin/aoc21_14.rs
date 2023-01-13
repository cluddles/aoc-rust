extern crate aoc_lib;

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use aoc_lib::harness::*;

pub struct Day14;

type Pair = (char, char);

// The trick is to count pairs, instead of simulating every pair individually.
// Order doesn't matter - just count pairs.
type PairCount = HashMap<Pair, u64>;

// Technically a map of Pair to (Pair, Pair), but easier to iterate over Vec...
type Rules = HashMap<Pair, Vec<Pair>>;

#[derive(Debug)]
struct Input {
    template: String,
    rules: Rules,
}

impl Solution<Input, u64> for Day14 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Extended Polymerization", 2021, 14)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        parse(&resource.as_str_lines()?)
    }

    fn solve_part1(&self, input: &Input) -> Result<u64> {
        solve(input, 10)
    }

    fn solve_part2(&self, input: &Input) -> Result<u64> {
        solve(input, 40)
    }
}

fn parse(lines: &[String]) -> Result<Input> {
    let template = lines[0].to_string();
    // Remember that resource.as_str_lines() will strip out the empty line
    let rules = lines
        .iter()
        .skip(1)
        .map(|line| -> Result<(Pair, Vec<Pair>)> {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let from: Pair =
                parts[0].chars().collect_tuple().ok_or_else(|| anyhow!("Cannot create pair"))?;
            let to_char = parts[1].chars().next().ok_or_else(|| anyhow!("Char missing"))?;
            Ok((from, vec![(from.0, to_char), (to_char, from.1)]))
        })
        .collect::<Result<_, _>>()?;
    Ok(Input { template, rules })
}

/// Run simulation for the required number of steps and then score it
fn solve(input: &Input, steps: usize) -> Result<u64> {
    let start = to_pairs(&input.template);
    let pair_counts = sim(&start, &input.rules, steps);
    score(&input.template, &pair_counts)
}

/// Convert a line of text into a count of pairs of chars
fn to_pairs(line: &str) -> PairCount {
    line.chars().collect::<Vec<char>>().windows(2).fold(PairCount::new(), |mut acc, x| {
        *acc.entry((x[0], x[1])).or_insert(0) += 1;
        acc
    })
}

/// Run the simulation for the given number of steps
fn sim(start: &PairCount, rules: &Rules, steps: usize) -> PairCount {
    let mut current = start.clone();
    for _ in 0..steps {
        current = current.into_iter().fold(PairCount::new(), |mut acc, (pair, count)| {
            // I don't think a rule is ever *not* present for given input...
            if let Some(sub) = rules.get(&pair) {
                sub.iter().for_each(|&target| *acc.entry(target).or_insert(0) += count);
            } else {
                *acc.entry(pair).or_insert(0) += count;
            }
            acc
        })
    }
    current
}

/// Score the simulation state
fn score(start_polymer: &str, pair_count: &PairCount) -> Result<u64> {
    // Add the first of each pair
    let mut char_count = pair_count.iter().fold(HashMap::new(), |mut acc, (pair, count)| {
        *acc.entry(pair.0).or_insert(0) += count;
        acc
    });
    // Add the last character of the input polymer
    *char_count.entry(start_polymer.chars().last().unwrap()).or_insert(0) += 1;
    // Max - min scoring
    let max = char_count.values().max().ok_or_else(|| anyhow!("No max"))?;
    let min = char_count.values().min().ok_or_else(|| anyhow!("No min"))?;
    Ok(max - min)
}

fn main() -> Result<()> {
    run_solution(&Day14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day14, SolutionPart::One), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day14, SolutionPart::Two), 2188189693529);
    }
}

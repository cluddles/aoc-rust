extern crate aoc_lib;

use std::collections::HashMap;
use itertools::Itertools;
use aoc_lib::harness::*;

pub struct Day14;

type Output = u64;

#[derive(Debug)]
struct Input {
    template: String,
    rules: HashMap<String, char>,
}

impl Solution<Input, Output> for Day14 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Extended Polymerization", 2021, 14)
    }

    fn parse_input(&self, resource: &dyn Resource) -> DynResult<Input> {
        parse(&resource.as_str_lines()?)
    }

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        solve(input, 10)
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        // The current implementation is totally inappropriate for 40 iterations
        todo!()
    }
}

fn parse(lines: &[String]) -> DynResult<Input> {
    let template = lines[0].to_string();
    // Remember that resource.as_str_lines() will strip out the empty line
    let rules = lines.iter().skip(1).map(|line| {
        let parts: Vec<&str> = line.split(" -> ").collect();
        return (parts[0].to_string(), parts[1].chars().collect::<Vec<char>>()[0]);
    }).collect();
    //println!("{:?}", rules);
    Ok(Input { template, rules })
}

fn solve(input: &Input, steps: usize) -> DynResult<u64> {
    let mut current = input.template.to_string();
    for i in 0..steps {
        println!("{}", i);
        current = expand_polymer(input, &current);
    }
    let char_counts = char_counts(&current);
    //println!("{:?}", char_counts);
    let (_, min) = char_counts.iter().min_by(|(_, a), (_, b)| a.cmp(b)).ok_or_else(|| SimpleError::new_dyn("Min required"))?;
    let (_, max) = char_counts.iter().max_by(|(_, a), (_, b)| a.cmp(b)).ok_or_else(|| SimpleError::new_dyn("Max required"))?;
    Ok(max - min)
}

fn expand_polymer(input: &Input, polymer: &str) -> String {
    let chars: Vec<char> = polymer.chars().collect();
    let mut result = String::new();
    result.push(chars[0]);
    for i in 1..chars.len() {
        let mut str = String::from(chars[i-1]);
        str.push(chars[i]);
        if let Some(rule) = input.rules.get(&str) {
            result.push(*rule);
        }
        result.push(chars[i]);
    }
    println!("{:?}", char_counts(&result).iter().sorted());
    result
}

fn char_counts(text: &str) -> HashMap<char, u64> {
    let mut result = HashMap::new();
    text.chars().for_each(|c| { let entry = result.entry(c).or_insert(0); *entry += 1; });
    result
}

fn main() -> DynResult<()> {
    run_solution(&Day14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let input = Day14.parse_input(&FileResource::new("input.test", 2021, 14)).unwrap();
        assert_eq!(expand_polymer(&input, "NNCB"), "NCNBCHB");
        assert_eq!(expand_polymer(&input, "NCNBCHB"), "NBCCNBBBCBHCB");
        assert_eq!(expand_polymer(&input, "NBCCNBBBCBHCB"), "NBBBCNCCNBBNBNBBCHBHHBCHB");
        assert_eq!(expand_polymer(&input, "NBBBCNCCNBBNBNBBCHBHHBCHB"), "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day14, SolutionPart::One), 1588);
    }

    #[test]
    fn test_part2() {
        //assert_eq!(test_solution(&Day14, SolutionPart::Two), 2188189693529);
    }

}

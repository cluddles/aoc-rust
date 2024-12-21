extern crate aoc_lib;

use anyhow::Result;

use aoc_lib::harness::*;

pub struct Day03;

type Input = Vec<String>;
type Output = usize;

impl Solution<Input, Output> for Day03 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Binary Diagnostic", 2021, 3)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        let gamma = common_binary(input, true);
        Ok(binary_to_int(&gamma) * binary_to_int(&inv_binary(&gamma)))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let oxy_bin = filter_binary(input, true);
        let co2_bin = filter_binary(input, false);
        Ok(binary_to_int(&oxy_bin) * binary_to_int(&co2_bin))
    }
}

/// Find most/least common bit value at given position
fn common_bit(lines: &Vec<String>, pos: usize, most_common: bool) -> char {
    let count = lines.iter().filter(|l| l.chars().nth(pos).unwrap() == '1').count();
    if (count >= ((lines.len() + 1) / 2)) == most_common {
        '1'
    } else {
        '0'
    }
}

/// Invert binary string: 00101 becomes 11010
fn inv_binary(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        result.push(match c {
            '0' => '1',
            _ => '0',
        });
    }
    result
}

/// Analyse lines and find most common bit for each position
fn common_binary(lines: &Vec<String>, most_common: bool) -> String {
    let mut result = String::new();
    // .first().unwrap() is a thing, but might as well just use [0]
    for i in 0..lines[0].len() {
        result.push(common_bit(lines, i, most_common));
    }
    if !most_common {
        result = inv_binary(&result);
    }
    result
}

/// Converts binary string to int value
fn binary_to_int(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

/// Filters lines down to single line matching criteria
fn filter_binary(lines: &Vec<String>, most_common: bool) -> String {
    let line_len = lines[0].len();
    let mut working: Vec<String> = lines.to_owned();
    for i in 0..line_len {
        let ch = common_bit(&working, i, most_common);
        working.retain(|l| l.chars().nth(i).unwrap() == ch);
        if working.len() == 1 {
            break;
        }
    }
    working.first().unwrap().to_string()
}

fn main() -> Result<()> {
    run_solution(&Day03)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day03, SolutionPart::One), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day03, SolutionPart::Two), 230);
    }
}

extern crate aoc_lib;

use anyhow::Result;

use aoc_lib::harness::*;

pub struct Day08;

type Input = Vec<String>;
type Output = u32;

impl Solution<Input, Output> for Day08 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Seven Segment Search", 2021, 8)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(part1(input))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(part2(input))
    }
}

/// Count digits with 2, 3, 4 and 7 segments
fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|x| x.split(" | ").collect::<Vec<&str>>()[1])
        .flat_map(|x| x.split(' '))
        .filter(|x| x.len() <= 4 || x.len() == 7)
        .count() as u32
}

/// Parses patterns from either side of input delimiter, sorting segment chars
fn parse_patterns(text: &str) -> Vec<String> {
    text.split(' ')
        .map(|x| {
            let mut c: Vec<char> = x.chars().collect();
            c.sort_unstable();
            c.iter().collect()
        })
        .collect()
}

/// Segments in p1 not in p2
fn subtract(p1: &str, p2: &str) -> String {
    p1.chars().filter(|&x| !p2.contains(x)).collect()
}

/// True if all segments in p2 are also in p1
fn contains(p1: &str, p2: &str) -> bool {
    subtract(p2, p1).is_empty()
}

/// Adds pattern to dictionary based on given predicate
fn add_to_dict<'a>(
    src_patterns: &'a [String],
    dict: &mut [Option<u8>],
    val: u8,
    predicate: impl Fn(&str) -> bool,
) -> &'a str {
    let (index, pattern) = src_patterns
        .iter()
        .enumerate()
        .filter(|(i, _)| dict[*i].is_none())
        .find(|(_, x)| predicate(x))
        .unwrap();
    dict[index] = Some(val);
    pattern
}

/// Builds up the dictionary for patterns to digits
fn build_dict(src_patterns: &[String]) -> Vec<Option<u8>> {
    let mut dict: Vec<Option<u8>> = vec![None; 10];
    // Only "1" has two segments
    let d1 = add_to_dict(src_patterns, &mut dict, 1, |x| x.len() == 2);
    // Only "7" has three segments
    add_to_dict(src_patterns, &mut dict, 7, |x| x.len() == 3);
    // Only "4" has four segments
    let d4 = add_to_dict(src_patterns, &mut dict, 4, |x| x.len() == 4);
    // Only "8" has seven segments
    add_to_dict(src_patterns, &mut dict, 8, |x| x.len() == 7);
    // Five segments: 2, 3, 5
    // Only "5" contains the part of "4" that isn't in "1"
    let tl_mid = subtract(d4, d1);
    let d5 = add_to_dict(src_patterns, &mut dict, 5, |x| x.len() == 5 && contains(x, &tl_mid));
    // Six segments: 0, 6, 9
    // Only "9" contains all of "4"
    let d9 = add_to_dict(src_patterns, &mut dict, 9, |x| x.len() == 6 && contains(x, d4));
    let tr = subtract(d9, d5);
    // Only "0" contains the top-right segment
    let d0 = add_to_dict(src_patterns, &mut dict, 0, |x| x.len() == 6 && contains(x, &tr));
    // "6" is the only six segment left
    add_to_dict(src_patterns, &mut dict, 6, |x| x.len() == 6);
    // Only "2" contains the bottom-left segment
    let bl = subtract(d0, d9);
    add_to_dict(src_patterns, &mut dict, 2, |x| x.len() == 5 && contains(x, &bl));
    // "3" is the only five segment left
    add_to_dict(src_patterns, &mut dict, 3, |x| x.len() == 5);
    dict
}

/// Converts digits to numeric value using the given dictionary
fn digits_to_num(digits: &[String], src_patterns: &[String], dict: &[Option<u8>]) -> u32 {
    let mut current: u32 = 0;
    for d in digits {
        current *= 10;
        for (i, p) in src_patterns.iter().enumerate() {
            if p == d {
                current += dict[i].unwrap() as u32;
                break;
            }
        }
    }
    current
}

/// Sum all RHS values
fn part2(input: &Input) -> u32 {
    let mut result = 0;
    for line in input {
        let split_line: Vec<&str> = line.split(" | ").collect();
        let src_patterns = parse_patterns(split_line[0]);
        let digits = parse_patterns(split_line[1]);
        let dict = build_dict(&src_patterns);
        let value = digits_to_num(&digits, &src_patterns, &dict);
        result += value;

        // println!("{:?} -> {:?} = {:?}", patterns, vals, current);
    }
    result
}

fn main() -> Result<()> {
    run_solution(&Day08)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day08, SolutionPart::One), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day08, SolutionPart::Two), 61229);
    }
}

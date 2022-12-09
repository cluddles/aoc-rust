extern crate aoc_lib;

use aoc_lib::common;

const DAY: &str = "2021/10";

fn parse_input(content: &str) -> Vec<&str> {
    common::split_lines(content)
}

/// Returns closing counterpart for opening character
fn closer_for(c: u8) -> u8 {
    match c {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => panic!("Undefined matching_brace: {}", c as char),
    }
}

/// Score for misplaced closer char
fn score_syntax_error_closer(c: u8) -> u32 {
    match c {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!("Undefined score_syntax_error: {}", c as char),
    }
}

/// Score for autocomplete char
fn score_autocomplete_closer(c: u8) -> u64 {
    match c {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => panic!("Undefined score_autocomplete: {}", c as char),
    }
}

/// Find error char (if any), and state of stack at end of execution
fn find_error(line: &str) -> (Option<u8>, Vec<u8>) {
    let mut stack: Vec<u8> = Vec::new();
    for c in line.chars() {
        let c = c as u8;
        match c {
            b'(' | b'[' | b'{' | b'<' => stack.push(c),
            _ => {
                if closer_for(stack.pop().unwrap()) != c {
                    return (Some(c), stack);
                }
            }
        }
    }
    (None, stack)
}

/// Score line for syntax error
fn score_syntax_error_line(line: &str) -> u32 {
    match find_error(line).0 {
        Some(v) => score_syntax_error_closer(v),
        None => 0,
    }
}

/// Score line for autocomplete
fn score_autocomplete_line(line: &str) -> u64 {
    match find_error(line) {
        (None, stack) => stack
            .iter()
            .rev()
            .fold(0, |x, &y| x * 5 + score_autocomplete_closer(closer_for(y))),
        _ => 0,
    }
}

fn part1(input: &[&str]) -> u32 {
    input.iter().map(|x| score_syntax_error_line(x)).sum()
}

fn part2(input: &[&str]) -> u64 {
    let mut scores: Vec<u64> = input
        .iter()
        .map(|x| score_autocomplete_line(x))
        .filter(|&x| x != 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let content = common::input_as_str(DAY, "input");
    let input = parse_input(&content);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = &common::input_as_str(DAY, "input.test");
        assert_eq!(part1(&parse_input(input)), 26397);
    }

    #[test]
    fn test_part2() {
        let input = &common::input_as_str(DAY, "input.test");
        assert_eq!(part2(&parse_input(input)), 288957);
    }
}

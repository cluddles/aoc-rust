extern crate aoc;

use aoc::shared;

const DAY: &str = "2021/07";

fn parse_input(content: &str) -> Vec<u32> {
    shared::tokenize_first_line(content, ',')
}

/// Calculate distance between two points
fn dist(a: u32, b: u32) -> u32 {
    (a as i32 - b as i32).unsigned_abs()
}

/// Linear score, where each point of distance costs a flat 1
fn score_linear(state: &[u32], pos: u32) -> u32 {
    state.iter().map(|x| dist(*x, pos)).sum()
}

/// Calculate factorial
fn fac(x: u32) -> u32 {
    (x * (x + 1)) / 2
}

/// Factorial score, where each point of distance costs 1 more than the previous
fn score_fac(state: &[u32], pos: u32) -> u32 {
    state.iter().map(|x| fac(dist(*x, pos))).sum()
}

/// Calculate the median of the given Vec
fn median(input: &[u32]) -> u32 {
    let mut state = input.to_owned();
    state.sort_unstable();
    state[state.len() / 2]
}

/// Calculate the mean of the given Vec
fn mean(input: &[u32]) -> f64 {
    input.iter().sum::<u32>() as f64 / input.len() as f64
}

fn part1(input: &[u32]) -> u32 {
    score_linear(input, median(input))
}

fn part2(input: &[u32]) -> u32 {
    score_fac(input, mean(input).round() as u32)
}

fn main() {
    let input = parse_input(&shared::input_as_str(DAY, "input"));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_input() -> Vec<u32> {
        parse_input(&shared::input_as_str(DAY, "input.test"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_input()), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_input()), 168);
    }

}

extern crate aoc;

use aoc::shared;

const DAY: &str = "2022/04";

/// Range of Section IDs.
/// Could also use e.g. RangeInclusive, but it doesn't really provide anything useful.
struct SecRange {
    from: u32,
    to: u32,
}

/// Convert "xxx-yyy" into SecRange
fn parse_sec_range(sec: &str) -> SecRange {
    let parts: Vec<&str> = sec.split('-').collect();
    SecRange {
        from: parts[0].parse().unwrap(),
        to: parts[1].parse().unwrap(),
    }
}

/// Convert "a-b,c-d" into a pair of SecRanges
fn parse_sec_range_pairs(content: &str) -> Vec<(SecRange, SecRange)> {
    let lines = shared::split_lines(content);
    let mut result = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        result.push((parse_sec_range(parts[0]), parse_sec_range(parts[1])));
    }
    result
}

/// True if given range contains the other range.
fn contains(a: &SecRange, b: &SecRange) -> bool {
    a.from <= b.from && a.to >= b.to
}

/// True if the two ranges overlap.
fn overlaps(a: &SecRange, b: &SecRange) -> bool {
    a.from <= b.to && b.from <= a.to
}

/// Count pairs where either range contains the other.
fn part1(input: &[(SecRange, SecRange)]) -> u32 {
    input
        .iter()
        .filter(|(a, b)| contains(a, b) || contains(b, a))
        .count() as u32
}

/// Count pairs where ranges overlap.
fn part2(input: &[(SecRange, SecRange)]) -> u32 {
    input.iter().filter(|(a, b)| overlaps(a, b)).count() as u32
}

fn main() {
    let content = shared::input_as_str(DAY, "input");
    let input = parse_sec_range_pairs(&content);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_input() -> Vec<(SecRange, SecRange)> {
        parse_sec_range_pairs(&shared::input_as_str(DAY, "input.test"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_input()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_input()), 4);
    }
}

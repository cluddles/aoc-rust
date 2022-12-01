extern crate aoc;
use aoc::shared;

/// Find most/least common bit value at given position
fn common_bit(lines: &Vec<&str>, pos: usize, most_common: bool) -> char {
    let count = lines
        .iter()
        .filter(|l| l.chars().nth(pos).unwrap() == '1')
        .count();
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
        result.push(if c == '0' { '1' } else { '0' });
    }
    result
}

/// Analyse lines and find most common bit for each position
fn common_binary(lines: &Vec<&str>, most_common: bool) -> String {
    let mut result = String::new();
    // .first().unwrap() is a thing, but might as well just use [0]
    for i in 0..lines[0].len() {
        result.push(common_bit(&lines, i, most_common));
    }
    if !most_common {
        result = inv_binary(&result);
    }
    result
}

/// Converts binary string to int value
fn binary_to_int(s: &str) -> usize {
    usize::from_str_radix(&s, 2).unwrap()
}

/// Filters lines down to single line matching criteria
fn filter_binary(lines: &Vec<&str>, most_common: bool) -> String {
    let line_len = lines[0].len();
    let mut working: Vec<&str> = lines.to_owned();
    for i in 0..line_len {
        let ch = common_bit(&working, i, most_common);
        working = working
            .into_iter()
            .filter(|l| l.chars().nth(i).unwrap() == ch)
            .collect();
        if working.len() == 1 {
            break;
        }
    }
    working.first().unwrap().to_string()
}

fn part1(content: &String) -> usize {
    let lines = shared::split_lines(&content);
    let gamma = common_binary(&lines, true);
    binary_to_int(&gamma) * binary_to_int(&inv_binary(&gamma))
}

fn part2(content: &String) -> usize {
    let lines = shared::split_lines(&content);
    let oxy_bin = filter_binary(&lines, true);
    let co2_bin = filter_binary(&lines, false);
    binary_to_int(&oxy_bin) * binary_to_int(&co2_bin)
}

fn main() {
    let content = shared::read_resource("2021/03/input");
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let content = shared::read_resource("2021/03/input.test");
        let lines = shared::split_lines(&content);
        let most_common = common_binary(&lines, true);
        assert_eq!(most_common, "10110");
        assert_eq!(inv_binary(&most_common), "01001");
        assert_eq!(part1(&content), 198);
    }

    #[test]
    fn test_part2() {
        let content = shared::read_resource("2021/03/input.test");
        let lines = shared::split_lines(&content);
        let oxy_bin = filter_binary(&lines, true);
        let co2_bin = filter_binary(&lines, false);
        assert_eq!(oxy_bin, "10111");
        assert_eq!(co2_bin, "01010");
        assert_eq!(part2(&content), 230);
    }
}

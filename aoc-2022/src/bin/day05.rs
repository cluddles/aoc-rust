extern crate aoc_lib;

use aoc_lib::common;

const DAY: &str = "2022/05";

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

/// Parse the crate block of the input, including the final line with column indexes
fn parse_crates(content: &str) -> Vec<Vec<u8>> {
    let lines = common::split_lines(content);
    // Get number of columns from the last line
    let cols = lines.last().unwrap().split_whitespace().count();
    let mut result = vec![Vec::new(); cols];
    // Iterate backwards over lines (except the last)
    for line in lines.iter().rev().skip(1) {
        // [A] [B] [C] etc: meaningful chars are 1, 5, 9, ...
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                result[i].push(c as u8)
            }
        }
    }
    result
}

/// Parse a single move: "move 1 from 1 to 2"
fn parse_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Move {
        quantity: parts[1].parse().unwrap(),
        from: parts[3].parse().unwrap(),
        to: parts[5].parse().unwrap(),
    }
}

/// Parse the move block of the input
fn parse_moves(content: &str) -> Vec<Move> {
    common::split_lines(content)
        .iter()
        .map(|x| parse_move(x))
        .collect()
}

/// Parse the whole input
fn parse_input(content: &str) -> (Vec<Vec<u8>>, Vec<Move>) {
    let parts: Vec<&str> = content.split("\n\n").collect();
    (parse_crates(parts[0]), parse_moves(parts[1]))
}

/// Summarise the result, by taking the top crate from each stack
fn summarise(crates: &[Vec<u8>]) -> String {
    crates.iter().map(|x| *x.last().unwrap() as char).collect()
}

/// Pop/push crates individually
fn part1(crates: &[Vec<u8>], moves: &Vec<Move>) -> String {
    //println!("{:?}", crates);
    //println!("{:?}", moves);
    let mut state = crates.to_owned();
    for m in moves {
        (0..m.quantity).for_each(|_| {
            let popped = state[m.from - 1].pop().unwrap();
            state[m.to - 1].push(popped);
        });
    }
    summarise(&state)
}

/// Pop/push multiple crates at once
fn part2(crates: &[Vec<u8>], moves: &[Move]) -> String {
    let mut state = crates.to_owned();
    for m in moves {
        let mut popped = Vec::new();
        (0..m.quantity).for_each(|_| popped.push(state[m.from - 1].pop().unwrap()));
        popped.iter().rev().for_each(|&p| state[m.to - 1].push(p));
    }
    summarise(&state)
}

fn main() {
    let content = common::input_as_str(DAY, "input");
    let (crates, moves) = parse_input(&content);
    println!("Part 1: {}", part1(&crates, &moves));
    println!("Part 2: {}", part2(&crates, &moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_input() -> (Vec<Vec<u8>>, Vec<Move>) {
        parse_input(&common::input_as_str(DAY, "input.test"))
    }

    #[test]
    fn test_part1() {
        let (crates, moves) = gen_input();
        assert_eq!(part1(&crates, &moves), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (crates, moves) = gen_input();
        assert_eq!(part2(&crates, &moves), "MCD");
    }
}

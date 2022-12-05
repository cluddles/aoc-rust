extern crate aoc;

use aoc::shared;

const DAY: &str = "2022/05";

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

/// Parse the crate block of the input, including the final line with column indexes
fn parse_crates(content: &str) -> Vec<Vec<u8>> {
    let lines = shared::split_lines(content);
    let cols = lines[lines.len() - 1]
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut result = vec![Vec::new(); cols];
    for i in (0..lines.len() - 1).rev() {
        let line: Vec<char> = lines[i].chars().collect();
        for j in 0..((line.len() + 1) / 4) {
            let c = line[(j * 4) + 1];
            if c != ' ' {
                result[j].push(c as u8)
            }
        }
    }
    result
}

/// Parse a single move: "move 1 from 1 to 2"
fn parse_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split(' ').collect();
    Move {
        quantity: parts[1].parse().unwrap(),
        // Apply the index-1 correction here to avoid having to do it everywhere later
        from: parts[3].parse::<usize>().unwrap() - 1,
        to: parts[5].parse::<usize>().unwrap() - 1,
    }
}

/// Parse the move block of the input
fn parse_moves(content: &str) -> Vec<Move> {
    shared::split_lines(content)
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
    crates
        .iter()
        .map(|x| *x.iter().rev().next().unwrap() as char)
        .collect()
}

/// Pop/push crates individually
fn part1(crates: &[Vec<u8>], moves: &Vec<Move>) -> String {
    //println!("{:?}", crates);
    //println!("{:?}", moves);
    let mut state = crates.to_owned();
    for m in moves {
        (0..m.quantity).for_each(|_| {
            let popped = state[m.from].pop().unwrap();
            state[m.to].push(popped);
        });
    }
    summarise(&state)
}

/// Pop/push multiple crates at once
fn part2(crates: &[Vec<u8>], moves: &[Move]) -> String {
    let mut state = crates.to_owned();
    for m in moves {
        let mut popped = Vec::new();
        (0..m.quantity).for_each(|_| popped.push(state[m.from].pop().unwrap()));
        popped.iter().rev().for_each(|&p| state[m.to].push(p));
    }
    summarise(&state)
}

fn main() {
    let content = shared::read_res_day(DAY, "input");
    let (crates, moves) = parse_input(&content);
    println!("Part 1: {}", part1(&crates, &moves));
    println!("Part 2: {}", part2(&crates, &moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_input() -> (Vec<Vec<u8>>, Vec<Move>) {
        parse_input(&shared::read_res_day(DAY, "input.test"))
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

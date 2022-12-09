extern crate aoc_lib;

use aoc_lib::common;

const DAY: &str = "2022/01";

#[derive(Debug, Clone)]
struct Elf {
    carried: Vec<u32>,
    total: u32,
}

/// Creates a new elf, carrying nothing
fn gen_elf() -> Elf {
    Elf {
        carried: Vec::new(),
        total: 0,
    }
}

/// Given input text, create corresponding elves
fn gen_elves(content: &str) -> Vec<Elf> {
    let lines = common::split_lines_keep_empty(content);
    let mut result: Vec<Elf> = Vec::new();
    let mut elf = gen_elf();
    for l in lines {
        if l.chars().count() == 0 {
            // Empty line - commit the current elf, start a new one
            result.push(elf);
            elf = gen_elf();
        } else {
            // Update current elf
            let val = l.trim().parse::<u32>().unwrap();
            elf.carried.push(val);
            elf.total += val;
        }
    }
    // Don't forgot to commit the last elf!
    if elf.total != 0 {
        result.push(elf);
    }
    result
}

/// Find max total value
fn part1(elves: &[Elf]) -> u32 {
    elves.iter().map(|x| x.total).max().unwrap()
}

/// Find sum of 3 max total values
fn part2(elves: &[Elf]) -> u32 {
    let mut max_first = elves.to_vec();
    max_first.sort_by(|a, b| b.total.cmp(&a.total));
    max_first.iter().take(3).map(|x| x.total).sum()
}

fn main() {
    let content = common::input_as_str(DAY, "input");
    let elves = gen_elves(&content);
    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_elves() -> Vec<Elf> {
        gen_elves(&common::input_as_str(DAY, "input.test"))
    }

    #[test]
    fn test_gen_elves() {
        let elves = gen_test_elves();
        assert_eq!(elves.len(), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_test_elves()), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_test_elves()), 45000);
    }
}

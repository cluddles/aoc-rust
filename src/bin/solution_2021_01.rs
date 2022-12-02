extern crate aoc;
use aoc::shared;

fn generate_sonar(content: &str) -> Vec<u32> {
    shared::tokenize(content, '\n')
}

/// How many values that are greater than previous value?
fn part1(sonar: &Vec<u32>) -> u32 {
    let mut count: u32 = 0;
    for i in 1..sonar.len() {
        if sonar[i] > sonar[i - 1] {
            count += 1;
        }
    }
    count
}

/// How many triplets ("sliding window") with sum greater than the previous triplet?
fn part2(sonar: &Vec<u32>) -> u32 {
    let mut prev: u32 = 0;
    let mut count: u32 = 0;
    for i in 2..sonar.len() {
        let sum3 = sonar[i - 2] + sonar[i - 1] + sonar[i];
        if prev != 0 && sum3 > prev {
            count += 1;
        }
        prev = sum3
    }
    count
}

fn main() {
    let content = shared::read_resource("2021/01/input");
    let sonar = generate_sonar(&content);
    println!("Part 1: {}", part1(&sonar));
    println!("Part 2: {}", part2(&sonar));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_sonar() -> Vec<u32> {
        generate_sonar(&shared::read_resource("2021/01/input.test"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_test_sonar()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_test_sonar()), 5);
    }
}

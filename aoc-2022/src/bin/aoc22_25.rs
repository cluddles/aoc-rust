extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::harness::*;

pub struct Day25;

type Input = Vec<String>;
type Output = String;

impl Solution<Input, Output> for Day25 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Full of Hot Air", 2022, 25)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        // Looks like you could also do the addition directly in SNAFU if you so desired
        Ok(dec_to_snafu(input.iter().map(|x| snafu_to_dec(x)).sum()))
    }

    fn solve_part2(&self, _: &Input) -> Result<Output> {
        // There is no part 2
        Ok("YAY".to_string())
    }
}

fn snafu_to_dec(text: &str) -> i64 {
    let mut result = 0;
    for c in text.chars() {
        result *= 5;
        result += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Unsupported digit: {}", c),
        };
    }
    result
}

fn dec_to_snafu(val: i64) -> String {
    let mut result = vec![];
    let mut wip = val;
    while wip != 0 {
        let digit = match wip.rem_euclid(5) {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                wip += 2;
                '='
            }
            4 => {
                wip += 1;
                '-'
            }
            _ => panic!("Impossible"),
        };
        result.push(digit);
        wip /= 5;
    }
    result.iter().rev().collect()
}

fn main() -> Result<()> {
    run_solution(&Day25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_snafu() {
        assert_eq!(snafu_to_dec("1=-0-2"), 1747);
        assert_eq!(snafu_to_dec("12111"), 906);
        assert_eq!(snafu_to_dec("2=0="), 198);
        assert_eq!(snafu_to_dec("21"), 11);
        assert_eq!(snafu_to_dec("2=01"), 201);
        assert_eq!(snafu_to_dec("111"), 31);
        assert_eq!(snafu_to_dec("20012"), 1257);
        assert_eq!(snafu_to_dec("112"), 32);
        assert_eq!(snafu_to_dec("1=-1="), 353);
        assert_eq!(snafu_to_dec("1-12"), 107);
        assert_eq!(snafu_to_dec("12"), 7);
        assert_eq!(snafu_to_dec("1="), 3);
        assert_eq!(snafu_to_dec("122"), 37);
    }

    #[test]
    fn test_from_dec() {
        assert_eq!(dec_to_snafu(1), "1");
        assert_eq!(dec_to_snafu(2), "2");
        assert_eq!(dec_to_snafu(3), "1=");
        assert_eq!(dec_to_snafu(4), "1-");
        assert_eq!(dec_to_snafu(5), "10");
        assert_eq!(dec_to_snafu(6), "11");
        assert_eq!(dec_to_snafu(7), "12");
        assert_eq!(dec_to_snafu(8), "2=");
        assert_eq!(dec_to_snafu(9), "2-");
        assert_eq!(dec_to_snafu(10), "20");
        assert_eq!(dec_to_snafu(15), "1=0");
        assert_eq!(dec_to_snafu(20), "1-0");
        assert_eq!(dec_to_snafu(353), "1=-1=");
        assert_eq!(dec_to_snafu(2022), "1=11-2");
        assert_eq!(dec_to_snafu(12345), "1-0---0");
        assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_bidirectional() {
        let dec = 314159265;
        assert_eq!(snafu_to_dec(&dec_to_snafu(dec)), dec);
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day25, SolutionPart::One), "2=-1=0");
    }
}

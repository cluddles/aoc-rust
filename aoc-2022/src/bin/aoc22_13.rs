extern crate aoc_lib;

use anyhow::{bail, Error, Result};
use aoc_lib::harness::*;
use std::cmp::Ordering;
use std::str::FromStr;

pub struct Day13;
type Input = Vec<Packet>;
type Output = usize;
impl Solution<Input, Output> for Day13 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Distress Signal", 2022, 13)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()?.iter().map(|x| Packet::from_str(x)).collect::<Result<_, _>>()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        // Work on pairs of packets
        Ok((0..input.len())
            .step_by(2)
            .filter(|&i| cmp_lists(&input[i].0, &input[i + 1].0).is_le())
            .map(|i| (i / 2) + 1)
            .sum())
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let dividers = vec![
            Packet(vec![El::List(vec![El::Int(2)])]),
            Packet(vec![El::List(vec![El::Int(6)])]),
        ];
        // We need to clone the input to a) sort it and b) add extra divider packets
        let mut sorted = input.to_owned();
        dividers.iter().for_each(|d| sorted.push(d.clone()));
        sorted.sort_unstable_by(|a, b| cmp_lists(&a.0, &b.0));
        // Now just find the dividers
        Ok(sorted
            .into_iter()
            .enumerate()
            .filter(|(_, x)| dividers.contains(x))
            .map(|(i, _)| i + 1)
            .product())
    }
}

#[derive(PartialEq, Debug, Clone)]
enum El {
    List(Vec<El>),
    Int(u32),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Packet(Vec<El>);

impl FromStr for Packet {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars();
        if chars.next() != Some('[') {
            bail!("Doesn't start with '['");
        }
        Ok(Packet(parse_list(&mut chars)?))
    }
}

fn parse_list(chars: &mut dyn Iterator<Item = char>) -> Result<Vec<El>> {
    let mut elements: Vec<El> = Vec::new();
    let mut val = None;
    while let Some(c) = chars.next() {
        match c {
            '[' => {
                elements.push(El::List(parse_list(chars)?));
            }
            ',' => {
                if let Some(v) = val {
                    elements.push(El::Int(v));
                    val = None;
                }
            }
            ']' => {
                if let Some(v) = val {
                    elements.push(El::Int(v));
                }
                return Ok(elements);
            }
            x => val = Some(val.unwrap_or(0) * 10 + (x as u8 - b'0') as u32),
        }
    }
    bail!("Invalid input")
}

fn cmp_lists(left: &[El], right: &[El]) -> Ordering {
    let mut ileft = left.iter();
    let mut iright = right.iter();
    loop {
        match (ileft.next(), iright.next()) {
            (Some(El::Int(i1)), Some(El::Int(i2))) => {
                let c = i1.cmp(i2);
                if c.is_ne() {
                    return c;
                }
            }
            (Some(El::List(l1)), Some(El::List(l2))) => {
                let c = cmp_lists(l1, l2);
                if c.is_ne() {
                    return c;
                }
            }
            (Some(El::List(l1)), Some(El::Int(i2))) => return cmp_lists(l1, &[El::Int(*i2)]),
            (Some(El::Int(i1)), Some(El::List(l2))) => return cmp_lists(&[El::Int(*i1)], l2),
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            _ => return Ordering::Greater,
        }
    }
}

fn main() -> Result<()> {
    run_solution(&Day13)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Packet::from_str("[]").unwrap().0, Vec::new());
        assert_eq!(Packet::from_str("[1]").unwrap().0, vec![El::Int(1)]);
        assert_eq!(
            Packet::from_str("[1,[2,3],4]").unwrap().0,
            vec![El::Int(1), El::List(vec![El::Int(2), El::Int(3)]), El::Int(4),]
        );
        assert_eq!(
            Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap().0,
            vec![
                El::Int(1),
                El::List(vec![
                    El::Int(2),
                    El::List(vec![
                        El::Int(3),
                        El::List(vec![
                            El::Int(4),
                            El::List(vec![El::Int(5), El::Int(6), El::Int(7),])
                        ])
                    ]),
                ]),
                El::Int(8),
                El::Int(9),
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day13, SolutionPart::One), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day13, SolutionPart::Two), 140);
    }
}

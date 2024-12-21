extern crate aoc_lib;

use anyhow::Result;

use aoc_lib::harness::*;

pub struct Day19;

type Input = Vec<Scanner>;
type Output = u64;

type Pos = (i32, i32, i32);

struct Scanner {
    beacons: Vec<Pos>,
}

impl Solution<Input, Output> for Day19 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Beacon Scanner", 2021, 19)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        for line in resource.as_str_lines()? {
            if line.starts_with("---") {
                if !current.is_empty() {
                    result.push(Scanner { beacons: current });
                    current = Vec::new();
                }
            } else {
                let parts: Vec<&str> = line.split(',').collect();
                current.push((parts[0].parse()?, parts[1].parse()?, parts[2].parse()?))
            }
        }
        if !current.is_empty() {
            result.push(Scanner { beacons: current });
        }
        Ok(result)
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        let dists: Vec<Vec<u32>> = input.iter().map(beacon_pair_dists).collect();
        for i in 0..dists.len()-1 {
            for j in i+1..dists.len() {
                let mut count = 0;
                for a in &dists[i] {
                    for b in &dists[j] {
                        if a == b { count += 1; break; }
                    }
                }
                if count >= 12 * (12 - 1) / 2 {
                    println!("{} vs {}: count = {}", i, j, count);
                }
            }
        }
        todo!()
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        todo!()
    }
}

fn beacon_pair_dists(scanner: &Scanner) -> Vec<u32> {
    let mut result = Vec::new();
    let b = &scanner.beacons;
    for i in 0..b.len()-1 {
        for j in i+1..b.len() {
            let (x1, y1, z1) = b[i];
            let (x2, y2, z2) = b[j];
            result.push(((x2-x1).abs() + (y2-y1).abs() + (z2-z1).abs()) as u32);
        }
    }
    result
}

fn main() -> Result<()> {
    run_solution(&Day19)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // assert_eq!(test_solution(&Day19, SolutionPart::One), 79);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(test_solution(&Day19, SolutionPart::Two), 0);
    }
}

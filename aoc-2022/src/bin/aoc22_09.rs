extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::data::{Dir4, Point2};
use aoc_lib::harness::*;
use std::collections::HashSet;

type Motion = (Dir4, u8);
type MotionList = Vec<Motion>;
type Pos = Point2<i32>;
type RopePos = Vec<Pos>;

pub struct Day09;
type Input = MotionList;
type Output = usize;
impl Solution<Input, Output> for Day09 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Rope Bridge", 2022, 9)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()?.iter().map(|x| parse_motion(x)).collect()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(simulate_rope(input, 2))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(simulate_rope(input, 10))
    }
}

fn parse_motion(s: &str) -> Result<Motion> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    Ok((parts[0].parse::<Dir4>()?, parts[1].parse::<u8>()?))
}

fn move_in_dir4(p: &mut Pos, dir: &Dir4) {
    match dir {
        Dir4::Up => p.y -= 1,
        Dir4::Down => p.y += 1,
        Dir4::Left => p.x -= 1,
        Dir4::Right => p.x += 1,
    }
}

/// Apply a single move step to rope
fn rope_step(rope_pos: &mut RopePos, dir: &Dir4) {
    move_in_dir4(&mut rope_pos[0], dir);
    for i in 1..rope_pos.len() {
        let prev = rope_pos[i - 1];
        let sep = prev - rope_pos[i];
        let delta = Pos::new(sep.x.signum(), sep.y.signum());
        if sep.x.abs() >= 2 || sep.y.abs() >= 2 {
            rope_pos[i] += delta;
        }
    }
    // println!("{:?}: {:?}", dir, rope_pos);
}

/// Apply all motions to rope of specified length.
fn simulate_rope(motions: &MotionList, rope_length: usize) -> usize {
    let mut rope_pos: RopePos = vec![Point2::default(); rope_length];
    let mut result: HashSet<Pos> = HashSet::new();
    for motion in motions {
        for _ in 0..motion.1 {
            rope_step(&mut rope_pos, &motion.0);
            result.insert(*rope_pos.last().unwrap());
        }
    }
    result.len()
}

fn main() -> Result<()> {
    run_solution(&Day09)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(test_ext(&Day09, SolutionPart::One, "test.1"), 13);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(test_ext(&Day09, SolutionPart::Two, "test.1"), 1);
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(test_ext(&Day09, SolutionPart::Two, "test.2"), 36);
    }
}

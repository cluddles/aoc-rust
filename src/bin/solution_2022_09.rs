extern crate aoc;

use std::collections::HashSet;
use aoc::shared;
use aoc::shared::Point2;
use std::str::FromStr;

const DAY: &str = "2022/09";

type Motion = (Dir4, u8);
type MotionList = Vec<Motion>;
type Pos = Point2<i32>;
type RopePos = Vec<Pos>;

// This is very similar (identical?) to Dir4 in 2022/08...
#[derive(Debug)]
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

impl Dir4 {
    pub const VALUES: [Dir4; 4] = [Dir4::Up, Dir4::Down, Dir4::Left, Dir4::Right];
}

impl FromStr for Dir4 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "U" => Dir4::Up,
            "D" => Dir4::Down,
            "L" => Dir4::Left,
            "R" => Dir4::Right,
            _ => return Err(()),
        };
        Ok(result)
    }
}

fn move_in_dir4(p: &mut Pos, dir: &Dir4) {
    match dir {
        Dir4::Up => p.y -= 1,
        Dir4::Down => p.y += 1,
        Dir4::Left => p.x -= 1,
        Dir4::Right => p.x += 1,
    }
}

fn parse_input(content: &str) -> MotionList {
    let lines = shared::split_lines(content);
    lines
        .iter()
        .map(|x| {
            let parts: Vec<&str> = x.split_whitespace().collect();
            (
                parts[0].parse::<Dir4>().unwrap(),
                parts[1].parse::<u8>().unwrap(),
            )
        })
        .collect()
}

/// Apply a single move step to rope
fn rope_step(rope_pos: &mut RopePos, dir: &Dir4) {
    move_in_dir4(&mut rope_pos[0], dir);
    for i in 1..rope_pos.len() {
        let prev = rope_pos[i-1];
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

fn part1(motions: &MotionList) -> usize {
    simulate_rope(motions, 2)
}

fn part2(motions: &MotionList) -> usize {
    simulate_rope(motions, 10)
}

fn main() {
    let input = parse_input(&shared::input_as_str(DAY, "input"));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_input(filename: &str) -> MotionList {
        parse_input(&shared::input_as_str(DAY, filename))
    }

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&gen_test_input("input.test.1")), 13);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&gen_test_input("input.test.1")), 1);
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&gen_test_input("input.test.2")), 36);
    }

}

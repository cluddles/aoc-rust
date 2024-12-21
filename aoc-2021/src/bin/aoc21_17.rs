extern crate aoc_lib;

use anyhow::{bail, Result};
use aoc_lib::data::Point2;

use aoc_lib::harness::*;

pub struct Day17;

type Input = Area;
type Output = i32;

type Pos = Point2<i32>;

#[derive(Debug)]
struct Area {
    from: Pos,
    to: Pos,
}

impl Area {
    fn contains(&self, pos: &Pos) -> bool {
        self.from.x <= pos.x && self.from.y <= pos.y && self.to.x > pos.x && self.to.y > pos.y
    }
}

impl Solution<Input, Output> for Day17 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Trick Shot", 2021, 17)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        use regex::Regex;
        let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")?;
        if let Some(cap) = re.captures_iter(&resource.as_str()?).next() {
            return Ok(Area {
                from: Pos::new(cap[1].parse()?, cap[3].parse()?),
                to: Pos::new(cap[2].parse::<i32>()? + 1, cap[4].parse::<i32>()? + 1),
            });
        }
        bail!("No target area found")
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(part1(input))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(part2(input))
    }
}

fn part1(target: &Area) -> Output {
    // The target zone y is always negative
    // At y=0, downward vy is the same as initial upward value
    // Next tick, vy will be -= 1
    // In order to land in target zone, vy at y=0 must be target.from.y - 1
    let n = target.from.y.abs() - 1;
    // Sum of 1..n to get height at top of curve
    n * (n + 1) / 2
}

/// All x-velocities that will touch the target area
fn part2(target: &Area) -> Output {
    // This gives us the final x position for an initial velocity.x
    //   tx = (x * (x + 1)) / 2
    // Inverse to get the velocity.x required to reach min required x
    //   x = -1 + sqrt(1 + 8 * tx) / 2
    let min_vx = (-1.0 + ((1 + 8 * target.from.x) as f64).sqrt() * 0.5).ceil() as i32;
    let mut count = 0;
    // ...aaaaand brute force it
    for vx in min_vx..target.to.x {
        for vy in target.from.y..target.from.y.abs() {
            if sim(&Pos::new(vx, vy), target).is_some() {
                count += 1;
            }
        }
    }
    count
}

#[allow(clippy::if_same_then_else)]
fn sim(vel: &Pos, target: &Area) -> Option<i32> {
    let mut pos = Pos::new(0, 0);
    let mut vel = vel.to_owned();
    let mut max_y = pos.y;
    loop {
        // Update position
        pos += vel;
        max_y = max_y.max(pos.y);
        // Apply x-drag
        vel.x += match vel.x {
            _ if vel.x < 0 => 1,
            _ if vel.x > 0 => -1,
            _ => 0,
        };
        // Apply gravity
        vel.y -= 1;
        // Stop
        if target.contains(&pos) {
            return Some(max_y)
        } else if vel.y < 0 && pos.y < target.from.y {
            // Can never reach target min Y
            return None;
        } else if pos.x >= target.to.x {
            // Overshot target max X
            return None;
        }
    }
}

fn main() -> Result<()> {
    run_solution(&Day17)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidates() {
        let area = Day17.parse_input(&FileResource::new("input.test", 2021, 17)).unwrap();
        assert!(sim(&Pos::new(7, 2), &area).is_some());
        assert!(sim(&Pos::new(6, 3), &area).is_some());
        assert!(sim(&Pos::new(9, 0), &area).is_some());
        assert!(sim(&Pos::new(17, -4), &area).is_none());
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day17, SolutionPart::One), 45);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day17, SolutionPart::Two), 112);
    }
}

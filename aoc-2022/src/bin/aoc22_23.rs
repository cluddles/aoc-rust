extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::data::Point2;
use aoc_lib::harness::*;
use std::collections::{HashMap, HashSet};

pub struct Day23;

type Pos = Point2<i32>;

type Input = HashSet<Pos>;
type Output = u32;

impl Solution<Input, Output> for Day23 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Unstable Diffusion", 2022, 23)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let lines = resource.as_str_lines()?;
        let mut elves = HashSet::new();
        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    elves.insert(Pos::new(x as i32, y as i32));
                }
            }
        }
        Ok(elves)
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(score(&simulate(input, 10)))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let mut elves = input.clone();
        let mut i = 0;
        loop {
            let next = tick(&elves, i);
            if next.eq(&elves) {
                return Ok(1 + i as u32);
            }
            elves = next;
            i += 1;
        }
    }
}

/// Adjacent cells - no diagonals. In order of elf preference...
const ADJ_4: [Pos; 4] = [Pos::new(0, -1), Pos::new(0, 1), Pos::new(-1, 0), Pos::new(1, 0)];
/// Adjacent cells including diagonals
const ADJ_8: [Pos; 8] = [
    Pos::new(0, -1),
    Pos::new(1, -1),
    Pos::new(1, 0),
    Pos::new(1, 1),
    Pos::new(0, 1),
    Pos::new(-1, 1),
    Pos::new(-1, 0),
    Pos::new(-1, -1),
];

/// Build up map of proposed moves: Key: To, Value: From
fn determine_proposed_moves(elves: &HashSet<Pos>, tick: usize) -> HashMap<Pos, Vec<Pos>> {
    let mut proposed_moves: HashMap<Pos, Vec<Pos>> = HashMap::new();
    // N, S, W, E
    'per_elf: for e in elves {
        // Check for adjacency
        let mut found = false;
        for d in ADJ_8 {
            if elves.contains(&(*e + d)) {
                found = true;
            }
        }
        if !found {
            continue 'per_elf;
        }
        // Check for directed moves
        // Don't forget that the order of moves to check changes per tick
        for i in tick..tick + 4 {
            let d = ADJ_4[i % 4];
            let p1 = *e + d;
            let p2 = Pos::new(p1.x + d.y, p1.y + d.x);
            let p3 = Pos::new(p1.x - d.y, p1.y - d.x);
            if !elves.contains(&p1) && !elves.contains(&p2) && !elves.contains(&p3) {
                proposed_moves.entry(p1).or_default().push(*e);
                continue 'per_elf;
            }
        }
    }
    proposed_moves
}

/// Apply all valid proposed moves to the given elves
fn apply_proposed(elves: &HashSet<Pos>, proposed_moves: &HashMap<Pos, Vec<Pos>>) -> HashSet<Pos> {
    let mut result = elves.clone();
    for (k, v) in proposed_moves {
        // Only move if there's no conflict in proposed moves
        if v.len() == 1 {
            if let Some(p) = v.iter().next() {
                // Update elf position
                result.remove(p);
                result.insert(*k);
            }
        }
    }
    result
}

/// Simulate a single tick
fn tick(elves: &HashSet<Pos>, tick: usize) -> HashSet<Pos> {
    apply_proposed(elves, &determine_proposed_moves(elves, tick))
}

/// Simulate the given number of ticks
fn simulate(elves: &HashSet<Pos>, num_ticks: usize) -> HashSet<Pos> {
    // Feels like this shouldn't be required - but I guess you could be running 0 ticks
    let mut elves = elves.clone();
    for i in 0..num_ticks {
        elves = tick(&elves, i);
    }
    elves
}

/// Score the given state - empty cells in the relevant area.
fn score(elves: &HashSet<Pos>) -> u32 {
    let mut min = Pos::new(i32::MAX, i32::MAX);
    let mut max = Pos::new(i32::MIN, i32::MIN);
    for e in elves {
        min.x = min.x.min(e.x);
        min.y = min.y.min(e.y);
        max.x = max.x.max(e.x);
        max.y = max.y.max(e.y);
    }
    ((max.x - min.x + 1) * (max.y - min.y + 1)) as u32 - elves.len() as u32
}

fn main() -> Result<()> {
    run_solution(&Day23)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn elf(x: i32, y: i32) -> Pos {
        Pos::new(x, y)
    }

    #[test]
    fn test_one_elf() {
        let e = HashSet::from([elf(0, 0)]);
        let e = simulate(&e, 1);
        assert_eq!(e, HashSet::from([elf(0, 0)]));
    }

    #[test]
    fn test_example() {
        let e = HashSet::from([elf(2, 1), elf(3, 1), elf(2, 2), elf(2, 4), elf(3, 4)]);
        let e = tick(&e, 0);
        assert_eq!(
            e,
            HashSet::from([elf(2, 0), elf(3, 0), elf(2, 2), elf(3, 3), elf(2, 4)]),
            "after 1 tick"
        );
        let e = tick(&e, 1);
        assert_eq!(
            e,
            HashSet::from([elf(2, 1), elf(3, 1), elf(1, 2), elf(4, 3), elf(2, 5)]),
            "after 2 ticks"
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day23, SolutionPart::One), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day23, SolutionPart::Two), 20);
    }
}

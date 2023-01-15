extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::data::Point3;
use aoc_lib::harness::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day18;

type Pos = Point3<i32>;
type Input = HashSet<Pos>;
type Output = u32;

/// x, y, z deltas representing adjacent positions
const ADJACENTS: &[Pos; 6] = &[
    Pos::new(1, 0, 0),
    Pos::new(-1, 0, 0),
    Pos::new(0, 1, 0),
    Pos::new(0, -1, 0),
    Pos::new(0, 0, 1),
    Pos::new(0, 0, -1),
];

#[derive(Copy, Clone)]
enum MapCell {
    Solid,
    Exterior,
}

impl Solution<Input, Output> for Day18 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Boiling Boulders", 2022, 18)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()?.iter().map(|i| i.parse()).collect::<Result<_, _>>()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().map(|x| count_edges(input, *x)).sum())
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let mut map = HashMap::new();
        for p in input {
            map.insert(*p, MapCell::Solid);
        }
        // Flood fill, check for exterior adjacents
        fill_exterior(input, &mut map);
        Ok(input.iter().map(|x| count_exterior_edges(&map, *x)).sum())
    }
}

/// Part one - count adjacents that are empty
fn count_edges(input: &Input, pos: Pos) -> u32 {
    ADJACENTS.iter().filter(|&&i| !input.contains(&(pos + i))).count() as u32
}

/// Fill exterior volume with suitable cells
fn fill_exterior(input: &Input, map: &mut HashMap<Pos, MapCell>) {
    // Work out min and max bounds; assume cuboid
    use std::iter::once;
    let vals: Vec<i32> =
        input.iter().flat_map(|i| once(i.x).chain(once(i.y).chain(once(i.z)))).collect();
    // Pad min and max by one so that we can fill around shape
    let min = vals.iter().min().expect("must have min") - 1;
    let max = vals.iter().max().expect("must have max") + 1;
    // Flood fill without recursion
    let start = Pos::new(min, min, min);
    let mut q = VecDeque::new();
    q.push_back(start);
    while !q.is_empty() {
        let pos = q.pop_front().expect("cannot be empty");
        map.entry(pos).or_insert(MapCell::Exterior);
        ADJACENTS.iter().for_each(|i| {
            let next_pos = pos + *i;
            if check_fill(map, &next_pos, &q, min, max) {
                q.push_back(next_pos);
            }
        });
    }
}

/// True if the given cell can be filled
fn check_fill(
    map: &HashMap<Pos, MapCell>,
    pos: &Pos,
    q: &VecDeque<Pos>,
    min: i32,
    max: i32,
) -> bool {
    pos.x >= min
        && pos.x <= max
        && pos.y >= min
        && pos.y <= max
        && pos.z >= min
        && pos.z <= max
        && !map.contains_key(pos)
        && !q.contains(pos)
}

/// Part two - count adjacents that are tagged as exterior
fn count_exterior_edges(map: &HashMap<Pos, MapCell>, pos: Pos) -> u32 {
    ADJACENTS.iter().filter(|&&i| matches!(map.get(&(pos + i)), Some(MapCell::Exterior))).count()
        as u32
}

fn main() -> Result<()> {
    run_solution(&Day18)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_min() {
        assert_eq!(test_inline(&Day18, SolutionPart::One, "1,1,1\n2,1,1"), 10);
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day18, SolutionPart::One), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day18, SolutionPart::Two), 58);
    }
}

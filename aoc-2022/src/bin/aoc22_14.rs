extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::data::{Grid, GridChar, GridPos};
use aoc_lib::harness::*;

pub struct Day14;
type Input = Cave;
type Output = usize;
impl Solution<Input, Output> for Day14 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Regolith Reservoir", 2022, 14)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let lines = resource.as_str_lines()?;
        // Read input into line segments
        let data: Vec<CaveLine> = lines.iter().map(|x| parse_line(x)).collect::<Result<_, _>>()?;
        // Work out the cave bounds we care about
        // (remember that sand enters at 500, 0 - so stretch out from there)
        let mut min: CavePos = ENTRY_POINT;
        let mut max: CavePos = ENTRY_POINT;
        data.iter().flat_map(|x| x.iter()).for_each(|x| {
            min.x = min.x.min(x.x);
            min.y = min.y.min(x.y);
            max.x = max.x.max(x.x);
            max.y = max.y.max(x.y);
        });
        // Apply leeway so sand can trickle over
        // Also note that the line positions are inclusive...
        //max += CavePos::new(2, 2);
        //min.x -= 1;
        // For part 2 we need to crank this right up; may as well just do it always
        max += CavePos::new(max.y - min.y, 2);
        min.x -= max.y - min.y;
        // Make the empty cave
        //println!("Bounds: {:?} -> {:?}", min, max);
        let size = max - min;
        //println!("Size: {:?}", size);
        let mut cave = Cave {
            grid: Grid::new_default(size.x as usize, size.y as usize),
            entry_point: ENTRY_POINT - min,
        };
        // Add the lines
        data.into_iter().for_each(|x| cave.apply_line(&x, &min));
        // println!("{}", cave.grid);
        Ok(cave)
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(simulate(input, false))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(simulate(input, true))
    }
}

const ENTRY_POINT: CavePos = CavePos::new(500, 0);

type CavePos = GridPos;
type CaveLine = Vec<CavePos>;

#[derive(Clone)]
pub struct Cave {
    grid: Grid<CaveCell>,
    entry_point: CavePos,
}

impl Cave {
    /// Add line (multiple segments) of rocks to cave
    fn apply_line(&mut self, cl: &CaveLine, offset: &CavePos) {
        cl.windows(2).for_each(|x| self.apply_line_seg(&x[0], &x[1], offset));
    }

    /// Add single line segment of rocks to cave
    fn apply_line_seg(&mut self, p1: &CavePos, p2: &CavePos, offset: &CavePos) {
        // Need to make sure p1 < p2 otherwise ranges don't do anything
        if p1.y == p2.y {
            for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
                self.grid.set(x - offset.x, p1.y - offset.y, CaveCell::Rock);
            }
        } else if p1.x == p2.x {
            for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                self.grid.set(p1.x - offset.x, y - offset.y, CaveCell::Rock);
            }
        } else {
            panic!("Only horizontal/vertical lines supported! {:?}->{:?}", p1, p2);
        }
    }
}

#[derive(Default, Clone, PartialEq)]
enum CaveCell {
    #[default]
    Air,
    Rock,
    Sand,
}

impl GridChar for CaveCell {
    fn to_grid_char(&self) -> char {
        match self {
            CaveCell::Air => '.',
            CaveCell::Rock => '#',
            CaveCell::Sand => 'o',
        }
    }
}

fn parse_line(text: &str) -> Result<CaveLine> {
    text.split(" -> ").map(|y| y.parse()).collect()
}

/// Run simulation until sand drops off or blocks entry point
fn simulate(cave_in: &Cave, floor: bool) -> usize {
    let mut cave = cave_in.to_owned();
    let mut ticks = 0;
    // Improvement: Track path taken by sand. Next sand can start at penultimate position.
    // This makes part 2 about 10 times quicker...
    let mut path: Vec<CavePos> = Vec::with_capacity(cave.grid.dim().y as usize);
    loop {
        // println!("\n{}", cave.grid);
        if floor && cave.grid.get(cave.entry_point.x, cave.entry_point.y) == &CaveCell::Sand {
            return ticks;
        }
        // This is where the sand came to rest. We don't care.
        path.pop();
        // This is where the sand was before that. Use this as the start, if available...
        let start = path.pop().unwrap_or(cave_in.entry_point);
        if !tick(&mut cave, &mut path, start, floor) {
            return ticks;
        }
        ticks += 1;
    }
}

/// Run a single simulation tick. Returns true if sand came to rest.
fn tick(cave: &mut Cave, path: &mut Vec<CavePos>, start: CavePos, floor: bool) -> bool {
    let mut p = start;
    while p.y < cave.grid.dim().y - 1 {
        // println!("{:?}", p);
        path.push(p);
        if cave.grid.get(p.x, p.y + 1) == &CaveCell::Air {
            p.y += 1;
        } else if cave.grid.get(p.x - 1, p.y + 1) == &CaveCell::Air {
            p.x -= 1;
            p.y += 1;
        } else if cave.grid.get(p.x + 1, p.y + 1) == &CaveCell::Air {
            p.x += 1;
            p.y += 1;
        } else {
            cave.grid.set(p.x, p.y, CaveCell::Sand);
            return true;
        }
    }
    if floor {
        cave.grid.set(p.x, p.y, CaveCell::Sand);
        return true;
    }
    false
}

fn main() -> Result<()> {
    run_solution(&Day14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day14, SolutionPart::One), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day14, SolutionPart::Two), 93);
    }
}

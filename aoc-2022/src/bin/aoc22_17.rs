extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::data::Point2;
use aoc_lib::harness::*;
use std::collections::HashSet;

pub struct Day17;

type Output = u64;
type Pos = Point2<u64>;

#[derive(Debug)]
struct Input {
    jets: Vec<Jet>,
    rocks: Vec<Rock>,
}

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

#[derive(Debug)]
struct Rock {
    size: Pos,
    open_cells: HashSet<Pos>,
}

impl Solution<Input, Output> for Day17 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Pyroclastic Flow", 2022, 17)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let rocks = vec![
            Rock { size: Pos::new(4, 1), open_cells: HashSet::new() },
            Rock {
                size: Pos::new(3, 3),
                open_cells: HashSet::from([
                    Pos::new(0, 0),
                    Pos::new(0, 2),
                    Pos::new(2, 0),
                    Pos::new(2, 2),
                ]),
            },
            Rock {
                size: Pos::new(3, 3),
                open_cells: HashSet::from([
                    Pos::new(0, 1),
                    Pos::new(0, 2),
                    Pos::new(1, 1),
                    Pos::new(1, 2),
                ]),
            },
            Rock { size: Pos::new(1, 4), open_cells: HashSet::new() },
            Rock { size: Pos::new(2, 2), open_cells: HashSet::new() },
        ];
        let jets = resource
            .as_str()?
            .chars()
            .filter(|&c| c as u8 >= 32)
            .map(|c| -> Result<Jet> {
                match c {
                    '<' => Ok(Jet::Left),
                    '>' => Ok(Jet::Right),
                    _ => Err(anyhow!("not a valid jet")),
                }
            })
            .collect::<Result<_, _>>()?;
        Ok(Input { jets, rocks })
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(run_sim(input, 2022))
    }

    fn solve_part2(&self, _input: &Input) -> Result<Output> {
        // Could do this programmatically
        // tbh I just looked at the floor heights in excel, and did some maths

        // For real input,
        // tower appears to loop every ***1740*** rocks, height + 2724

        // 1000000000000 / 1740 = 574712643
        // 574712642 * 1740 = 999999997080
        // height at (1000000000000 - 999999997080), which is 2920 = 4574
        // 4574 + 2724 * 574712642 = 1565517241382

        Ok(0)
    }
}

fn run_sim(input: &Input, num_rocks: u64) -> u64 {
    let mut floor = 0;
    let mut jet_index = 0;
    let mut rock_index = 0;
    let mut settled = HashSet::new();
    for _ in 0..num_rocks {
        // Setup rock
        let rock = &input.rocks[rock_index];
        rock_index += 1;
        if rock_index >= input.rocks.len() {
            rock_index = 0;
        }
        let mut pos = Pos::new(2, floor + 3);

        loop {
            // Get next jet
            let jet = &input.jets[jet_index];
            jet_index += 1;
            if jet_index >= input.jets.len() {
                jet_index = 0;
            }
            // Apply jet
            let old_pos = pos;
            match jet {
                Jet::Left => {
                    if pos.x > 0 {
                        pos.x -= 1;
                    }
                }
                Jet::Right => {
                    if pos.x + rock.size.x < 7 {
                        pos.x += 1;
                    }
                }
            }
            // Check collision
            if old_pos != pos && is_colliding(pos, rock, &settled) {
                pos = old_pos;
            }
            // Apply gravity
            let old_pos = pos;
            if pos.y > 0 {
                pos.y -= 1;
            }
            // Check collision
            if old_pos.y == 0 || is_colliding(pos, rock, &settled) {
                pos = old_pos;
                settle(pos, rock, &mut settled);
                floor = floor.max(pos.y + rock.size.y);
                // println!("{}", floor);
                break;
            }
        }
    }
    floor
}

fn is_colliding(pos: Pos, rock: &Rock, settled: &HashSet<Pos>) -> bool {
    for x in 0..rock.size.x {
        for y in 0..rock.size.y {
            let cell = Pos::new(x, y);
            if !rock.open_cells.contains(&cell) && settled.contains(&(pos + cell)) {
                return true;
            }
        }
    }
    false
}

fn settle(pos: Pos, rock: &Rock, settled: &mut HashSet<Pos>) {
    // println!("Settle at {:?}", pos);
    for x in 0..rock.size.x {
        for y in 0..rock.size.y {
            let cell = Pos::new(x, y);
            if !rock.open_cells.contains(&cell) {
                let p = pos + cell;
                // println!("Blocking cell: {:?}", p);
                settled.insert(p);
            }
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
    fn test_part1() {
        assert_eq!(test_solution(&Day17, SolutionPart::One), 3068);
    }

    #[test]
    fn test_part2() {
        // For test input,
        // tower appears to loop every ***490*** rocks, height + 742

        // 1000000000000 / 490 = 2040816326
        // 2040816326 * 490 = 999999999740
        // height at (1000000000000 - 999999999740), which is 260 = 396
        // 396 + 742 * 2040816326 = 1514285714288
        assert_eq!(test_solution(&Day17, SolutionPart::Two), 0);
    }
}

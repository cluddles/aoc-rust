extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::data::{Grid, GridChar, GridPos};
use aoc_lib::harness::*;
use std::collections::HashSet;

pub struct Day24;

type Input = Valley;
type Output = usize;

type Pos = GridPos;

const DIR_LEFT: Pos = Pos::new(-1, 0);
const DIR_UP: Pos = Pos::new(0, -1);
const DIR_RIGHT: Pos = Pos::new(1, 0);
const DIR_DOWN: Pos = Pos::new(0, 1);
const POTENTIAL_MOVES: [Pos; 5] = [Pos::new(0, 0), DIR_RIGHT, DIR_DOWN, DIR_UP, DIR_LEFT];

#[derive(Debug)]
struct Valley {
    w: i32,
    h: i32,
    layers: Vec<Layer>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Cell {
    #[default]
    Floor,
    Wall,
    Blizzard,
}

impl GridChar for Cell {
    fn to_grid_char(&self) -> char {
        match self {
            Cell::Floor => '.',
            Cell::Wall => '#',
            Cell::Blizzard => '&',
        }
    }
}

impl Valley {
    fn start(&self) -> Pos {
        Pos::new(1, 0)
    }

    fn end(&self) -> Pos {
        Pos::new(self.w - 2, self.h - 1)
    }

    fn layer(&self, tick: usize) -> &Layer {
        &self.layers[tick % self.layers.len()]
    }
}

type Layer = Grid<Cell>;

struct Blizzard {
    pos: Pos,
    dir: Pos,
}

impl Blizzard {
    fn new(x: i32, y: i32, dir: Pos) -> Blizzard {
        Blizzard { pos: Pos::new(x, y), dir }
    }
}

impl Solution<Input, Output> for Day24 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Blizzard Basin", 2022, 24)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let source = resource.as_u8_grid(std::convert::identity)?;
        // Lowest common multiple is the number of valley layers we need
        let w = source.dim().x;
        let h = source.dim().y;
        let lcm = lcm(w as usize - 2, h as usize - 2);
        let mut layers = Vec::with_capacity(lcm);
        // Init blizzards
        let mut bliz = Vec::new();
        for x in 0..w {
            for y in 0..h {
                if let Some(b) = match source.get(x, y) {
                    b'>' => Some(Blizzard::new(x, y, DIR_RIGHT)),
                    b'<' => Some(Blizzard::new(x, y, DIR_LEFT)),
                    b'^' => Some(Blizzard::new(x, y, DIR_UP)),
                    b'v' => Some(Blizzard::new(x, y, DIR_DOWN)),
                    _ => None,
                } {
                    bliz.push(b);
                }
            }
        }
        // Init layers
        for _ in 0..lcm {
            let mut layer = Grid::new_default(w as usize, h as usize);
            // Walls
            for x in 0..w {
                if x != 1 {
                    layer.set(x, 0, Cell::Wall);
                }
                if x != w - 2 {
                    layer.set(x, h - 1, Cell::Wall);
                }
            }
            for y in 0..h {
                layer.set(0, y, Cell::Wall);
                layer.set(w - 1, y, Cell::Wall);
            }
            // Bliz
            bliz.iter().for_each(|b| layer.set(b.pos.x, b.pos.y, Cell::Blizzard));
            //println!("{}", layer);
            layers.push(layer);
            // Move bliz for next layer
            bliz.iter_mut().for_each(|b| {
                b.pos += b.dir;
                if b.pos.x == w - 1 {
                    b.pos.x = 1;
                } else if b.pos.x == 0 {
                    b.pos.x = w - 2;
                }
                if b.pos.y == h - 1 {
                    b.pos.y = 1;
                } else if b.pos.y == 0 {
                    b.pos.y = h - 2;
                }
            });
        }
        Ok(Valley { w, h, layers })
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(part1(input).ok_or_else(|| anyhow!("No result"))?)
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(part2(input).ok_or_else(|| anyhow!("No result"))?)
    }
}

/// Lower common multiple
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// Greatest common divisor - recursive Euclidean algorithm
fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn part1(valley: &Valley) -> Option<usize> {
    path(valley, &PosTime::new(valley.start(), 0), &valley.end())
}

fn part2(valley: &Valley) -> Option<usize> {
    let p1 = path(valley, &PosTime::new(valley.start(), 0), &valley.end())?;
    let p2 = path(valley, &PosTime::new(valley.end(), p1), &valley.start())?;
    let p3 = path(valley, &PosTime::new(valley.start(), p2), &valley.end())?;
    Some(p3)
}

fn path(valley: &Valley, from: &PosTime, to: &Pos) -> Option<usize> {
    // Sort-of-A*
    let mut open = HashSet::new();
    open.insert(*from);

    let mut closed = HashSet::new();

    while !open.is_empty() {
        let current = open.iter().min_by(|a, b| f(a, to).cmp(&f(b, to))).copied()?;
        // println!("{:?}", current);
        if &current.pos == to {
            return Some(current.tick);
        }
        open.remove(&current);
        closed.insert(current);
        // Expand neighbours
        for n in moves(valley, &current) {
            // println!("+ {:?}", n);
            if !closed.contains(&n) {
                open.insert(n);
            }
        }
    }
    None
}

fn f(n: &PosTime, goal: &Pos) -> usize {
    n.tick + heuristic(n, goal)
}

/// Allowed moves
fn moves(valley: &Valley, current: &PosTime) -> Vec<PosTime> {
    let next_tick = current.tick + 1;
    let layer = valley.layer(next_tick);
    POTENTIAL_MOVES
        .iter()
        .map(|m| current.pos + *m)
        .filter(|p| layer.is_in_bounds(p.x, p.y) && layer.get(p.x, p.y) == &Cell::Floor)
        .map(|p| PosTime::new(p, next_tick))
        .collect()
}

fn heuristic(n: &PosTime, goal: &Pos) -> usize {
    // This is just manhattan distance to exit - will never underestimate
    ((goal.x - n.pos.x).abs() + (goal.y - n.pos.y).abs()) as usize

    // A* with no heuristic = Uniform Cost Search; and in this case is faster, apparently
    //0
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct PosTime {
    pos: Pos,
    tick: usize,
}

impl PosTime {
    fn new(pos: Pos, tick: usize) -> PosTime {
        PosTime { pos, tick }
    }
}

fn main() -> Result<()> {
    run_solution(&Day24)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day24, SolutionPart::One), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day24, SolutionPart::Two), 54);
    }
}

extern crate aoc_lib;

use aoc_lib::data::{Grid, GridPos};
use aoc_lib::harness::*;
use std::collections::HashSet;

pub struct Day24;

type Input = Grid<Cell>;
type Valley = Input;
type Output = usize;

type Pos = GridPos;

// Pre-compute map to work out when tiles are blizzard free/blocked
// (times will loop for x- and y-axes)
// Cost to move into tile can include delay for it to be blizzard free.
// Remember that you cannot wait in a tile that's about to get blizzarded. Presumably model this
// by disallowing any moves that would blizzard your current tile before the target was clear.
// Heuristic is just the distance to goal, since that will never underestimate.
// Moves are up, down, left, right only

impl Solution<Input, Output> for Day24 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Blizzard Basin", 2022, 24)
    }

    fn parse_input(&self, resource: &dyn Resource) -> DynResult<Input> {
        let source = resource.as_u8_grid(std::convert::identity)?;
        // Both test and real input have start at min X, end at max X
        // Discard the outer walls
        let mut grid = Grid::new_default(source.dim().x as usize - 2, source.dim().y as usize - 2);
        for x in 0..grid.dim().x {
            for y in 0..grid.dim().y {
                grid.set(x, y, gen_cell(x, y, &source));
            }
        }
        // println!("{:?}", grid);
        Ok(grid)
    }

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        Ok(part1(input).ok_or_else(|| SimpleError::new_dyn("No result"))?)
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        Ok(part2(input).ok_or_else(|| SimpleError::new_dyn("No result"))?)
    }
}

const START: Pos = Pos::new(0, -1);

fn part1(valley: &Valley) -> Option<usize> {
    Some(path(valley, &Node::new(START, 0), &Pos::new(valley.dim().x - 1, valley.dim().y - 1))? + 1)
}

fn part2(valley: &Valley) -> Option<usize> {
    let w = valley.dim().x;
    let h = valley.dim().y;
    let p1 = path(valley, &Node::new(START, 0), &Pos::new(w - 1, h - 1))? + 1;
    let p2 = path(valley, &Node::new(Pos::new(w - 1, h), p1), &Pos::new(0, 0))? + 1;
    let p3 = path(valley, &Node::new(START, p2), &Pos::new(w - 1, h - 1))? + 1;
    Some(p3)
}

fn path(valley: &Valley, from: &Node, to: &Pos) -> Option<usize> {
    // This is sort of A*
    // But we're not really overwriting nodes when we find a quicker path to them, because
    // backtracking is required - so it's not very efficient at all.
    let mut open = HashSet::new();
    open.insert(*from);

    while !open.is_empty() {
        let current = open.iter().min_by(|a, b| f(valley, a).cmp(&f(valley, b))).copied()?;
        //println!("{:?}", current);
        if &current.pos == to {
            return Some(current.tick);
        }
        open.remove(&current);
        // Expand neighbours
        for n in neighbours(valley, &current) {
            //println!("+ {:?}", n);
            open.insert(n);
        }
    }
    None
}

fn f(valley: &Valley, node: &Node) -> usize {
    node.tick + heuristic(valley, node)
}

/// Adjacent cells - no diagonals
const ADJ: [Pos; 4] = [Pos::new(0, -1), Pos::new(0, 1), Pos::new(-1, 0), Pos::new(1, 0)];

/// Allowed adjacent cells, taking blockages into account
fn neighbours(valley: &Valley, current: &Node) -> Vec<Node> {
    let max = time_until_blocked(valley, current);
    //println!("Blocked in {:?}", max);
    let mut result = Vec::new();
    for a in ADJ {
        let pos = current.pos + a;
        if valley.is_in_bounds(pos.x, pos.y) {
            if let Some(cost) = move_cost(valley, &pos, current.tick, max) {
                result.push(Node::new(pos, current.tick + cost));
            }
        }
    }
    // Back to entry point(s)
    if current.pos.x == 0 && current.pos.y == 0 {
        result.push(Node::new(Pos::new(0, -1), current.tick + 1));
    }
    if current.pos.x == valley.dim().x - 1 && current.pos.y == valley.dim().y - 1 {
        result.push(Node::new(Pos::new(valley.dim().x - 1, valley.dim().y), current.tick + 1));
    }
    result
}

/// How long until the current (hopefully unblocked) cell becomes blocked
fn time_until_blocked(valley: &Valley, n: &Node) -> Option<usize> {
    // Entry point is never blocked
    if !valley.is_in_bounds(n.pos.x, n.pos.y) {
        return None;
    }
    // First wait_x or wait_y that indicates non-zero wait
    let c = valley.get(n.pos.x, n.pos.y);
    let max_dim = c.wait_x.len().max(c.wait_y.len());
    (1..max_dim).find(|&i| {
        c.wait_x[(n.tick + i) % c.wait_x.len()] != 0 || c.wait_y[(n.tick + i) % c.wait_y.len()] != 0
    })
}

/// Cost to move from assumed adjacent cell into the given one, taking blockage into account
fn move_cost(valley: &Valley, pos: &Pos, current_tick: usize, max: Option<usize>) -> Option<usize> {
    let cell = valley.get(pos.x, pos.y);
    let mut wait = 1;
    let mut tick = current_tick + wait;
    let max = max.unwrap_or(9999);
    loop {
        // Give up if the current position is blocked - we can't wait this long!
        if wait > max {
            return None;
        }
        // Check the next potentially free time
        let next = cell.wait_x[tick % cell.wait_x.len()].max(cell.wait_y[tick % cell.wait_y.len()]);
        if next == 0 {
            return Some(wait);
        }
        wait += next;
        tick += next;
    }
}

fn heuristic(valley: &Valley, n: &Node) -> usize {
    // This is just manhattan distance to exit - will never underestimate
    // (valley.dim().x - n.pos.x + valley.dim().y - n.pos.y + 1) as usize

    // A* with no heuristic = Uniform Cost Search, and in this case is faster, apparently
    0
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Node {
    pos: Pos,
    tick: usize,
}
impl Node {
    fn new(pos: Pos, tick: usize) -> Node {
        Node { pos, tick }
    }
}

// Refactor this?
fn gen_cell(x: i32, y: i32, source: &Grid<u8>) -> Cell {
    let w = source.dim().x - 2;
    let h = source.dim().y - 2;
    let mut cell = Cell { wait_x: vec![999; w as usize], wait_y: vec![999; h as usize] };
    // Scan L/R
    for i in 0..w {
        let x1 = (x - i).rem_euclid(w) + 1;
        let x2 = (x + i).rem_euclid(w) + 1;
        if source.get(x1, y + 1) != &b'>' && source.get(x2, y + 1) != &b'<' {
            cell.wait_x[i as usize] = 0;
        }
    }
    'outer: for i in 0..w {
        if cell.wait_x[i as usize] != 0 {
            for j in i + 1..w {
                if cell.wait_x[j as usize] == 0 {
                    cell.wait_x[i as usize] = (j - i) as usize;
                    continue 'outer;
                }
            }
            cell.wait_x[i as usize] = (w - i) as usize + cell.wait_x[0];
        }
    }
    // Scan U/D
    for i in 0..h {
        let y1 = (y - i).rem_euclid(h) + 1;
        let y2 = (y + i).rem_euclid(h) + 1;
        if source.get(x + 1, y1) != &b'v' && source.get(x + 1, y2) != &b'^' {
            cell.wait_y[i as usize] = 0;
        }
    }
    'outer: for i in 0..h {
        if cell.wait_y[i as usize] != 0 {
            for j in i + 1..h {
                if cell.wait_y[j as usize] == 0 {
                    cell.wait_y[i as usize] = (j - i) as usize;
                    continue 'outer;
                }
            }
            cell.wait_y[i as usize] = (h - i) as usize + cell.wait_y[0];
        }
    }
    cell
}

#[derive(Debug, Default, Clone)]
struct Cell {
    // Tick that the cell will next be open
    wait_x: Vec<usize>,
    wait_y: Vec<usize>,
}

fn main() -> DynResult<()> {
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

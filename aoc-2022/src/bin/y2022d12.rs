extern crate aoc_lib;

use aoc_lib::data::{Grid, Point2};
use aoc_lib::harness::*;
use std::collections::{HashMap, HashSet};

struct Year2022Day12;
type Input = Area;
type Output = usize;
impl Solution<Input, Output> for Year2022Day12 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Hill Climbing Algorithm", 2022, 12)
    }

    fn parse_input(&self, resource: &dyn Resource) -> DynResult<Input> {
        let grid = resource.as_u8_grid(std::convert::identity);
        Area::new(grid)
    }

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        Ok(path_find(input)
            .ok_or_else(|| SimpleError::new_dyn("No path found"))?
            .len())
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        Ok(path_find_var(input)
            .ok_or_else(|| SimpleError::new_dyn("No path found"))?
            .len())
    }
}

type GridPos = Point2<usize>;

/// Track the grid and related start/end points
struct Area {
    grid: Grid<u8>,
    start: GridPos,
    end: GridPos,
}

impl Area {
    const START: u8 = b'S';
    const END: u8 = b'E';

    fn new(grid: Grid<u8>) -> DynResult<Area> {
        let start = grid
            .find_pos(&Area::START)
            .ok_or_else(|| SimpleError::new_dyn("Could not find start"))?;
        let end = grid
            .find_pos(&Area::END)
            .ok_or_else(|| SimpleError::new_dyn("Could not find end"))?;
        Ok(Area { grid, start, end })
    }

    fn dim(&self) -> &Point2<usize> {
        self.grid.dim()
    }

    fn height_at(&self, pos: &GridPos) -> u8 {
        match *self.grid.get(pos.x, pos.y) {
            Area::START => b'a',
            Area::END => b'z',
            v => v,
        }
    }
}

/// Node in open set with the lowest F
fn select_best_from_open(open: &mut HashSet<GridPos>, nodes: &HashMap<GridPos, NodeData>) -> Option<GridPos> {
    open.iter()
        .map(|x| (x, nodes.get(x).map(|x| x.f).unwrap_or(9999)))
        .min_by(|(_, s1), (_, s2)| s1.cmp(s2))
        .map(|(x, _)| *x)
}

/// Heuristic, used to calculate F
fn h(area: &Area, pos: &GridPos) -> usize {
    // Return 0 to behave like Dijkstra - breadth first search
    0

    // Because of the evil spiral input, this heuristic is actually really bad!
    //((area.end.x as i32 - pos.x as i32).abs() + (area.end.y as i32 - pos.y as i32).abs()) as usize
}

/// All (valid) neighbours for given position
fn neighbours(area: &Area, p: &GridPos) -> Vec<GridPos> {
    let mut result = Vec::with_capacity(4);
    if p.x > 0 { neighbour_one(&mut result, area, p, GridPos::new(p.x - 1, p.y)); }
    if p.y > 0 { neighbour_one(&mut result, area, p, GridPos::new(p.x, p.y - 1)); }
    if p.x < area.dim().x - 1 { neighbour_one(&mut result, area, p, GridPos::new(p.x + 1, p.y)); }
    if p.y < area.dim().y - 1 { neighbour_one(&mut result, area, p, GridPos::new(p.x, p.y + 1)); }
    result
}

fn neighbour_one(result: &mut Vec<GridPos>, area: &Area, p: &Point2<usize>, n: Point2<usize>) {
    // Remember that we're traversing from end to start, so the rules are backwards
    let (h1, h2) = (area.height_at(&n), area.height_at(p));
    if h2 < h1 || h2 - h1 <= 1 {
        result.push(n);
    }
}

/// Track A* node data
struct NodeData {
    f: usize,
    g: usize,
    came_from: GridPos,
}

/// Find shortest path
fn path_find_inner(area: &Area, start: GridPos, ends: Vec<GridPos>) -> Option<Vec<GridPos>> {
    let mut open = HashSet::new();
    let mut nodes: HashMap<GridPos, NodeData> = HashMap::new();

    open.insert(start);
    nodes.insert(start, NodeData { f: h(area, &start), g: 0, came_from: start });

    while !open.is_empty() {
        let current = select_best_from_open(&mut open, &nodes)?;
        if ends.contains(&current) {
            // reconstruct path
            let mut p = current;
            let mut result = Vec::new();
            loop {
                if let Some(v) = nodes.get(&p).map(|x| x.came_from) {
                    if v == p { return Some(result); } else {
                        // println!("{:?}", v);
                        result.push(v);
                        p = v;
                    }
                }
            }
        }
        open.remove(&current);
        // Check neighbours
        let node = nodes.get(&current);
        let current_g = node.map(|x| x.g).unwrap_or(9999);
        for n in neighbours(area, &current) {
            let tentative_g = current_g + 1;
            let neighbour = nodes.get(&n);
            if tentative_g < neighbour.map(|x| x.g).unwrap_or(9999) {
                nodes.insert(n, NodeData { f: tentative_g + h(area, &n), g: tentative_g, came_from: current });
                open.insert(n);
            }
        }
    }
    None
}

/// Find shortest path from fixed start
fn path_find(area: &Area) -> Option<Vec<GridPos>> {
    // Flip start and end just so we can use the same rules as part 2...
    path_find_inner(area, area.end, vec![area.start])
}

/// Find shortest path from any starting position at height 'a'
fn path_find_var(area: &Area) -> Option<Vec<GridPos>> {
    // Multiple "starts", which we actually use as endpoints
    let mut starts = Vec::new();
    for x in 0..area.dim().x {
        for y in 0..area.dim().y {
            let p = Point2 { x, y };
            if area.height_at(&p) == b'a' {
                starts.push(p);
            }
        }
    }
    path_find_inner(area, area.end, starts)
}

fn main() -> DynResult<()> {
    run_solution(&Year2022Day12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Year2022Day12, SolutionPart::One), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Year2022Day12, SolutionPart::Two), 29);
    }
}

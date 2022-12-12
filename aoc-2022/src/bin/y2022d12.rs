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
        Ok(path_find(input, input.start)
            .ok_or_else(|| SimpleError::new_dyn("No path found"))?
            .len())
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        Ok(path_find_var(input).ok_or_else(|| SimpleError::new_dyn("No path found"))?)
    }
}

type GridPos = Point2<usize>;

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

fn best_f(open: &mut HashSet<GridPos>, f_scores: &HashMap<GridPos, usize>) -> Option<GridPos> {
    open.iter()
        .map(|x| (x, f_scores.get(x).unwrap_or(&9999)))
        .min_by(|(_, s1), (_, s2)| s1.cmp(s2))
        .map(|(x, _)| *x)
}

fn h(area: &Area, pos: &GridPos) -> usize {
    ((area.end.x as i32 - pos.x as i32).abs() + (area.end.y as i32 - pos.y as i32).abs()) as usize
}

fn neighbours(area: &Area, p: &GridPos) -> Vec<GridPos> {
    let mut result = Vec::new();
    if p.x > 0 { neighbour_one(&mut result, area, p, GridPos::new(p.x - 1, p.y)); }
    if p.y > 0 { neighbour_one(&mut result, area, p, GridPos::new(p.x, p.y - 1)); }
    if p.x < area.dim().x - 1 { neighbour_one(&mut result, area, p, GridPos::new(p.x + 1, p.y)); }
    if p.y < area.dim().y - 1 { neighbour_one(&mut result, area, p, GridPos::new(p.x, p.y + 1)); }
    result
}

fn neighbour_one(result: &mut Vec<GridPos>, area: &Area, p: &Point2<usize>, n: Point2<usize>) {
    let (hn, hp) = (area.height_at(&n), area.height_at(p));
    if hn < hp || hn - hp <= 1 {
        result.push(n);
    }
}

/// Find shortest path
fn path_find(area: &Area, start: GridPos) -> Option<Vec<GridPos>> {
    let mut open = HashSet::new();
    open.insert(start);
    let mut f_scores: HashMap<GridPos, usize> = HashMap::new();
    let mut g_scores: HashMap<GridPos, usize> = HashMap::new();
    g_scores.insert(start, 0);
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    f_scores.insert(start, h(area, &start));

    while !open.is_empty() {
        let current = best_f(&mut open, &f_scores)?;
        if current == area.end {
            // reconstruct path
            let mut p = current;
            let mut result = Vec::new();
            loop {
                match came_from.get(&p) {
                    Some(&v) => {
                        result.push(p);
                        p = v;
                    }
                    None => {
                        return Some(result);
                    }
                }
            }
        }
        open.remove(&current);
        // Check neighbours
        let current_g = *g_scores.get(&current).unwrap_or(&9999);
        for n in neighbours(area, &current) {
            let tentative_g = current_g + 1;
            if tentative_g < *g_scores.get(&n).unwrap_or(&9999) {
                came_from.insert(n, current);
                g_scores.insert(n, tentative_g);
                f_scores.insert(n, tentative_g + h(area, &n));
                open.insert(n);
            }
        }
    }
    None
}

/// Find shortest path from any starting position at height 'a'
fn path_find_var(area: &Area) -> Option<usize> {
    // This is incredibly slow
    // Suspect a better way would be to floodfill from the end and stop as soon as you hit an 'a'
    let mut best_len = 9999;
    for x in 0..area.dim().x {
        for y in 0..area.dim().y {
            let pos = Point2::new(x, y);
            if area.height_at(&pos) != b'a' {
                continue;
            }
            let h = h(area, &pos);
            // No point checking anything further from end than our current best
            if h >= best_len {
                continue;
            }
            let best = path_find(area, pos);
            if let Some(v) = best {
                let l = v.len();
                if l < best_len {
                    best_len = l;
                }
            }
        }
    }
    Some(best_len)
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

extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::data::{Dir4, Grid, GridPos};
use aoc_lib::harness::*;

type Treemap = Grid<u8>;

pub struct Day08;
type Input = Treemap;
type Output = u32;
impl Solution<Input, Output> for Day08 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Treetop Tree House", 2022, 8)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_u8_grid(|c| c - b'0')
    }

    fn solve_part1(&self, treemap: &Input) -> Result<Output> {
        let (w, h) = treemap.dim().to_tuple();
        let mut vis: Grid<bool> = Grid::new_default(w as usize, h as usize);
        for y in 0..h {
            scan_vis(treemap, &mut vis, &GridPos::new(0, y), &Dir4::Right);
            scan_vis(treemap, &mut vis, &GridPos::new(w - 1, y), &Dir4::Left);
        }
        for x in 0..w {
            scan_vis(treemap, &mut vis, &GridPos::new(x, 0), &Dir4::Down);
            scan_vis(treemap, &mut vis, &GridPos::new(x, h - 1), &Dir4::Up);
        }
        Ok(vis.vec().iter().filter(|&&x| x).count() as u32)
    }

    fn solve_part2(&self, treemap: &Input) -> Result<Output> {
        let mut best = 0;
        for x in 0..treemap.dim().x {
            for y in 0..treemap.dim().y {
                let score = scenic_score(treemap, x, y);
                if score > best {
                    best = score;
                }
            }
        }
        Ok(best)
    }
}

fn move_in_dir4(p: &mut GridPos, dir: &Dir4) {
    // Fine for these to over/underflow because those values are (way) out of bounds
    match dir {
        Dir4::Up => p.y = p.y.wrapping_sub(1),
        Dir4::Down => p.y = p.y.wrapping_add(1),
        Dir4::Left => p.x = p.x.wrapping_sub(1),
        Dir4::Right => p.x = p.x.wrapping_add(1),
    }
}

/// Scan row/column and update visibility map
fn scan_vis(treemap: &Treemap, vis: &mut Grid<bool>, start: &GridPos, dir: &Dir4) {
    let mut max: Option<u8> = None;
    let mut pos = start.to_owned();
    while treemap.is_in_bounds(pos.x, pos.y) && max != Some(9) {
        let tree = *treemap.get(pos.x, pos.y);
        if match max {
            None => true,
            Some(x) => tree > x,
        } {
            max = Some(tree);
            vis.set(pos.x, pos.y, true);
        }
        move_in_dir4(&mut pos, dir);
    }
}

/// Scan row/column until view blocked by taller tree
fn scan_scenic(treemap: &Treemap, start: &GridPos, dir: &Dir4) -> u32 {
    let mut score = 0;
    let start_tree = treemap.get(start.x, start.y);
    let mut pos = start.to_owned();
    move_in_dir4(&mut pos, dir);
    while treemap.is_in_bounds(pos.x, pos.y) {
        score += 1;
        if treemap.get(pos.x, pos.y) >= start_tree {
            break;
        }
        move_in_dir4(&mut pos, dir);
    }
    score
}

/// Product of scenic scores in each dir
fn scenic_score(treemap: &Treemap, x: i32, y: i32) -> u32 {
    let p = GridPos::new(x, y);
    Dir4::VALUES.iter().map(|x| scan_scenic(treemap, &p, x)).product()
}

fn main() -> Result<()> {
    run_solution(&Day08)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day08, SolutionPart::One), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day08, SolutionPart::Two), 8);
    }
}

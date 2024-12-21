extern crate aoc_lib;

use anyhow::{anyhow, Result};

use aoc_lib::data::{Grid, GridPos};
use aoc_lib::harness::*;
use aoc_lib::path;

pub struct Day15;

type Input = Grid<u8>;
type Output = u64;

const ADJACENTS: &[GridPos; 4] =
    &[GridPos::new(0, -1), GridPos::new(1, 0), GridPos::new(0, 1), GridPos::new(-1, 0)];

impl Solution<Input, Output> for Day15 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Chiton", 2021, 15)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_u8_grid(|x| x - b'0')
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        path_cost(input)
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let (w, h) = input.dim().to_tuple();
        let mut grid = Grid::new_default((w * 5) as usize, (h * 5) as usize);
        for x in 0..grid.dim().x {
            for y in 0..grid.dim().y {
                let d = ((x / w) + (y / h)) as u8;
                let sx = x % w;
                let sy = y % h;
                // values clamped between 1-9
                grid.set(x, y, (input.get(sx, sy) + d - 1) % 9 + 1);
            }
        }
        path_cost(&grid)
    }
}

fn path_cost(input: &Input) -> Result<u64> {
    // A* from top-left (0, 0) to bottom right
    let start = GridPos::new(0, 0);
    let end = *input.dim() - GridPos::new(1, 1);

    let path = path::a_star(&(input, start, end), &start, neighbours, heuristic, is_end)
        .ok_or_else(|| anyhow!("No path"))?;
    // println!("{:?}", path);
    Ok(path.into_iter().skip(1).map(|pos| *input.get(pos.x, pos.y) as u64).sum())
}

fn neighbours(ctx: &(&Input, GridPos, GridPos), n: &GridPos) -> Vec<(GridPos, u64)> {
    let (input, _, _) = ctx;
    ADJACENTS
        .iter()
        .map(|dir| *dir + *n)
        .filter(|pos| input.is_in_bounds(pos.x, pos.y))
        .map(|pos| (pos, *input.get(pos.x, pos.y) as u64))
        .collect()
}

fn heuristic(ctx: &(&Input, GridPos, GridPos), n: &GridPos) -> u64 {
    let (_, _, end) = ctx;
    (*end - *n).manhattan() as u64
}

fn is_end(ctx: &(&Input, GridPos, GridPos), n: &GridPos) -> bool {
    let (_, _, end) = ctx;
    n == end
}

fn main() -> Result<()> {
    run_solution(&Day15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day15, SolutionPart::One), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day15, SolutionPart::Two), 315);
    }
}

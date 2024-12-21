extern crate aoc_lib;

use anyhow::Result;

use aoc_lib::data::{Grid, GridPos, Point2};
use aoc_lib::harness::*;

pub struct Day09;

type Input = Grid<u8>;
type Output = u32;

impl Solution<Input, Output> for Day09 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Smoke Basin", 2021, 9)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_u8_grid(|c| c - b'0')
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(part1(input))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(part2(input))
    }
}

/// Find lowpoints, which are points lower than all adjacent cells
fn lowpoints(heights: &Grid<u8>) -> Vec<GridPos> {
    let mut result = Vec::new();
    for y in 0..heights.dim().y {
        for x in 0..heights.dim().x {
            let h = heights.get(x, y);
            if (x == 0 || heights.get(x - 1, y) > h)
                && (y == 0 || heights.get(x, y - 1) > h)
                && (x == heights.dim().x - 1 || heights.get(x + 1, y) > h)
                && (y == heights.dim().y - 1 || heights.get(x, y + 1) > h)
            {
                result.push(Point2 { x, y });
            }
        }
    }
    result
}

/// "Risk" for all lowpoints
fn part1(heights: &Grid<u8>) -> u32 {
    lowpoints(heights).iter().map(|x| (heights.get(x.x, x.y) + 1) as u32).sum()
}

/// Calculate basin size, using mutable grid to track visited cells
fn basin_iter(heights: &Grid<u8>, basins: &mut Grid<u8>, x: i32, y: i32) -> u32 {
    if basins.get(x, y) == &1 || heights.get(x, y) == &9 {
        return 0;
    }

    basins.set(x, y, 1);

    let mut result = 1;
    if x != 0 {
        result += basin_iter(heights, basins, x - 1, y);
    }
    if x != basins.dim().x - 1 {
        result += basin_iter(heights, basins, x + 1, y);
    }
    if y != 0 {
        result += basin_iter(heights, basins, x, y - 1);
    }
    if y != basins.dim().y - 1 {
        result += basin_iter(heights, basins, x, y + 1);
    }
    result
}

/// Product of largest three basin sizes
fn part2(heights: &Grid<u8>) -> u32 {
    let mut basins = Grid::new_default(heights.dim().x as usize, heights.dim().y as usize);
    let mut basin_sizes: Vec<u32> =
        lowpoints(heights).iter().map(|x| basin_iter(heights, &mut basins, x.x, x.y)).collect();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() -> Result<()> {
    run_solution(&Day09)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day09, SolutionPart::One), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day09, SolutionPart::Two), 1134);
    }
}

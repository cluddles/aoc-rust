extern crate aoc;

use aoc::shared;
use aoc::shared::{Grid, Point2};

const DAY: &str = "2021/09";

fn parse_heightmap(content: &str) -> Grid<u8> {
    let lines = shared::split_lines(content);
    Grid::from_2d(
        &lines
            .iter()
            .map(|x| x.chars().map(|c| c as u8 - b'0').collect())
            .collect(),
    )
}

/// Find lowpoints, which are points lower than all adjacent cells
fn lowpoints(heights: &Grid<u8>) -> Vec<Point2<usize>> {
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
    lowpoints(heights)
        .iter()
        .map(|x| (heights.get(x.x, x.y) + 1) as u32)
        .sum()
}

/// Calculate basin size, using mutable grid to track visited cells
fn basin_iter(heights: &Grid<u8>, basins: &mut Grid<u8>, x: usize, y: usize) -> u32 {
    if basins.get(x, y) == &1 || heights.get(x, y) == &9 { return 0; }

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
        result +=  basin_iter(heights, basins, x, y + 1);
    }
    result
}

/// Product of largest three basin sizes
fn part2(heights: &Grid<u8>) -> u32 {
    let mut basins = Grid::new(0, heights.dim().x, heights.dim().y);
    let mut basin_sizes: Vec<u32> = lowpoints(heights)
        .iter()
        .map(|x| basin_iter(heights, &mut basins, x.x, x.y))
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    let content = shared::read_res_day(DAY, "input");
    let heightmap = parse_heightmap(&content);
    println!("Part 1: {}", part1(&heightmap));
    println!("Part 2: {}", part2(&heightmap));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_heightmap(&shared::read_res_day(DAY, "input.test"));
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input = parse_heightmap(&shared::read_res_day(DAY, "input.test"));
        assert_eq!(part2(&input), 1134);
    }
}

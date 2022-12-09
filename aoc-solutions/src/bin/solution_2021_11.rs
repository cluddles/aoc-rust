extern crate aoc_lib;

use aoc_lib::common;
use aoc_lib::Grid;

const DAY: &str = "2021/11";

/// x, y deltas representing adjacent positions
const ADJACENTS: &[(i8, i8); 8] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn parse_input(input: &str) -> Grid<u8> {
    let lines = common::split_lines(input);
    Grid::from_2d(
        &lines
            .iter()
            .map(|x| x.chars().map(|y| y as u8 - b'0').collect())
            .collect::<Vec<Vec<u8>>>(),
    )
}

/// Increment the value of a single cell.
/// On flash, apply increment to all (valid) neighbours.
fn inc_one(grid: &mut Grid<u8>, ix: i8, iy: i8) {
    if ix < 0 || iy < 0 {
        return;
    }
    let (x, y) = (ix as usize, iy as usize);
    if x >= grid.dim().x || y >= grid.dim().y {
        return;
    }
    let prev = *grid.get(x, y);
    grid.set(x, y, prev + 1);
    if prev == 9 {
        ADJACENTS
            .iter()
            .for_each(|d| inc_one(grid, ix + d.0, iy + d.1));
    }
}

/// Run a single step of the simulation. Returns number of flashes that occurred.
fn step(grid: &mut Grid<u8>) -> u32 {
    // Increment all cells; flashes will modify neighbours
    for i in 0..grid.dim().x {
        for j in 0..grid.dim().y {
            inc_one(grid, i as i8, j as i8);
        }
    }
    // Reset all flashing cells to 0
    let mut flashes = 0;
    for i in 0..grid.dim().x {
        for j in 0..grid.dim().y {
            if grid.get(i, j) > &9 {
                grid.set(i, j, 0);
                flashes += 1;
            }
        }
    }
    flashes
}

/// Run 100 iterations, count flashes
fn part1(input: &Grid<u8>) -> u32 {
    let mut grid = input.to_owned();
    (0..100).map(|_| step(&mut grid)).sum()
}

/// Iterate until ALL cells flash
fn part2(input: &Grid<u8>) -> u32 {
    let mut grid = input.to_owned();
    let mut tick = 0;
    loop {
        tick += 1;
        if step(&mut grid) == (grid.dim().x * grid.dim().y) as u32 {
            return tick;
        }
    }
}

fn main() {
    let input = parse_input(&common::input_as_str(DAY, "input"));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_input() -> Grid<u8> {
        parse_input(&common::input_as_str(DAY, "input.test"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_test_input()), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_test_input()), 195);
    }
}

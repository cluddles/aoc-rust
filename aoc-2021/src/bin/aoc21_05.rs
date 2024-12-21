extern crate aoc_lib;

use std::collections::HashMap;

use anyhow::Result;

use aoc_lib::common;
use aoc_lib::data::Point2;
use aoc_lib::harness::*;

pub struct Day05;

type Input = Vec<Line>;
type Output = u32;

impl Solution<Input, Output> for Day05 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Hydrothermal Venture", 2021, 5)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()?.iter().map(|x| parse_line(x)).collect::<Result<_, _>>()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(draw_all(input, false))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(draw_all(input, true))
    }
}

/// Line, consisting of start and end position (inclusive)
pub struct Line {
    start: Pos,
    end: Pos,
}

/// Simple 2D position
type Pos = Point2<i32>;

/// 2D grid to track number of lines per point
pub struct Grid {
    map: HashMap<Pos, i32>,
}

fn parse_pos(pos: &str) -> Result<Pos> {
    let coords: Vec<i32> = common::tokenize(pos, ',')?;
    Ok(Pos { x: coords[0], y: coords[1] })
}

fn parse_line(line: &str) -> Result<Line> {
    let points: Vec<&str> = line.split(" -> ").collect();
    Ok(Line { start: parse_pos(points[0])?, end: parse_pos(points[1])? })
}

/// Yield all points along line
fn line_points(line: &Line) -> Vec<Pos> {
    let dx = (line.end.x - line.start.x).signum();
    let dy = (line.end.y - line.start.y).signum();
    let mut x = line.start.x;
    let mut y = line.start.y;
    let mut result = Vec::new();
    loop {
        result.push(Pos { x, y });
        if x == line.end.x && y == line.end.y {
            return result;
        }
        x += dx;
        y += dy;
    }
}

/// All lines are either horizontal, vertical, or diagonal (exactly 45 degrees)
fn is_diagonal(line: &Line) -> bool {
    line.start.x != line.end.x && line.start.y != line.end.y
}

fn draw_line(grid: &mut Grid, line: &Line) {
    for &pos in line_points(line).iter() {
        *(grid.map.entry(pos).or_insert(0)) += 1;
    }
}

fn draw_all(lines: &[Line], inc_diagonals: bool) -> u32 {
    let mut grid = Grid { map: HashMap::new() };
    for line in lines.iter() {
        if !is_diagonal(line) || inc_diagonals {
            draw_line(&mut grid, line);
        }
    }
    grid.map.values().filter(|x| **x > 1).count() as u32
}

fn main() -> Result<()> {
    run_solution(&Day05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day05, SolutionPart::One), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day05, SolutionPart::Two), 12);
    }
}

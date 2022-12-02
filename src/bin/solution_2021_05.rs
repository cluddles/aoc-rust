extern crate aoc;

use aoc::shared;
use hashbrown::HashMap;

/// Line, consisting of start and end position (inclusive)
struct Line {
    start: Pos,
    end: Pos,
}

/// Simple 2D position
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

/// 2D grid to track number of lines per point
struct Grid {
    map: HashMap<Pos, i32>,
}

fn parse_pos(pos: &str) -> Pos {
    let coords: Vec<i32> = shared::tokenize(pos, ',');
    Pos {
        x: coords[0],
        y: coords[1],
    }
}

fn parse_line(line: &str) -> Line {
    let points: Vec<&str> = line.split(" -> ").collect();
    Line {
        start: parse_pos(points[0]),
        end: parse_pos(points[1]),
    }
}

fn parse_lines(content: &str) -> Vec<Line> {
    let lines = shared::split_lines(content, false);
    lines.iter().map(|x| parse_line(x)).collect()
}

/// Yield all points along line
fn line_points(line: &Line) -> Vec<Pos> {
    let dx = num::signum(line.end.x - line.start.x);
    let dy = num::signum(line.end.y - line.start.y);
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
    let mut grid = Grid {
        map: HashMap::new(),
    };
    for line in lines.iter() {
        if !is_diagonal(line) || inc_diagonals {
            draw_line(&mut grid, line);
        }
    }
    grid.map.values().filter(|x| **x > 1).count() as u32
}

fn part1(lines: &[Line]) -> u32 {
    draw_all(lines, false)
}

fn part2(lines: &[Line]) -> u32 {
    draw_all(lines, true)
}

fn main() {
    let lines = parse_lines(&shared::read_resource("2021/05/input"));
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_input() -> Vec<Line> {
        parse_lines(&shared::read_resource("2021/05/input.test"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_test_input()), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_test_input()), 12);
    }
}

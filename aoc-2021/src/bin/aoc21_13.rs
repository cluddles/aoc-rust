extern crate aoc_lib;

use std::collections::HashSet;
use std::fmt::Formatter;

use anyhow::Result;

use aoc_lib::common;
use aoc_lib::data::Point2;
use aoc_lib::harness::*;

pub struct Day13;

type Output = usize;

impl Solution<Input, Output> for Day13 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Transparent Origami", 2021, 13)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        Ok(Input::parse(&resource.as_str()?))
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(part1(input))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(part2(input))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Paper {
    points: HashSet<Point2<usize>>,
}

impl Paper {
    /// Create new Paper by applying the fold predicate and op to all points
    fn apply_fold_inner(
        &self,
        fold_pos: usize,
        predicate: fn(Point2<usize>, usize) -> bool,
        op: fn(Point2<usize>, usize) -> Point2<usize>,
    ) -> Paper {
        let mut points = HashSet::new();
        self.points.iter().for_each(|&p| {
            if predicate(p, fold_pos) {
                points.insert(op(p, fold_pos));
            } else {
                points.insert(p);
            }
        });
        Paper { points }
    }

    /// Create new Paper by applying the given fold to it
    fn apply_fold(&self, fold: &Fold) -> Paper {
        match fold {
            Fold::X(val) => self.apply_fold_inner(
                *val,
                |p: Point2<usize>, v| p.x > v,
                |p: Point2<usize>, v| Point2 { x: v - (p.x - v), y: p.y },
            ),
            Fold::Y(val) => self.apply_fold_inner(
                *val,
                |p: Point2<usize>, v| p.y > v,
                |p: Point2<usize>, v| Point2 { x: p.x, y: v - (p.y - v) },
            ),
        }
    }
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // Determine w,h required (max in each dimension, +1)
        let (w, h) = (
            self.points.iter().map(|p| p.x).max().unwrap() + 1,
            self.points.iter().map(|p| p.y).max().unwrap() + 1,
        );
        // Render state (could use a Grid; probably not required)
        for y in 0..h {
            for x in 0..w {
                write!(f, "{}", if self.points.contains(&Point2 { x, y }) { "#" } else { "." })?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug)]
pub struct Input {
    paper: Paper,
    folds: Vec<Fold>,
}

impl Input {
    /// Parse input text as paper, folds
    fn parse(text: &str) -> Input {
        let parts: Vec<&str> = text.split("\n\n").collect();
        // Before the cut - paper points, comma delim
        let mut paper = Paper::default();
        common::split_lines(parts[0]).iter().for_each(|x| {
            let tokens = common::tokenize(x, ',').unwrap();
            paper.points.insert(Point2 { x: tokens[0], y: tokens[1] });
        });
        // After the cut - folds; axis and position, equals delim
        let mut folds = Vec::new();
        common::split_lines(parts[1]).iter().for_each(|x| {
            let tokens: Vec<&str> = x.split('=').collect();
            let num = tokens[1].parse::<usize>().unwrap();
            folds.push(if tokens[0].ends_with('x') { Fold::X(num) } else { Fold::Y(num) });
        });
        Input { paper, folds }
    }
}

fn part1(input: &Input) -> usize {
    let fold = input.folds.first().unwrap();
    input.paper.apply_fold(fold).points.len()
}

fn part2(input: &Input) -> usize {
    let paper = input.folds.iter().fold(input.paper.clone(), |p, f| p.apply_fold(f));
    println!("{}", paper);
    paper.points.len()
}

fn main() -> Result<()> {
    run_solution(&Day13)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day13, SolutionPart::One), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day13, SolutionPart::Two), 16);
    }
}

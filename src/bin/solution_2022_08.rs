extern crate aoc;

use aoc::shared;
use aoc::shared::{Grid, Point2};

const DAY: &str = "2022/08";

type Treemap = Grid<u8>;

pub enum Dir4 {
    Up, Down, Left, Right,
}

impl Dir4 {
    pub const VALUES: [Dir4; 4] = [Dir4::Up, Dir4::Down, Dir4::Left, Dir4::Right];
}

fn move_in_dir4(p: &mut Point2<usize>, dir: &Dir4) {
    // Fine for these to over/underflow because those values are (way) out of bounds
    match dir {
        Dir4::Up => p.y = p.y.wrapping_sub(1),
        Dir4::Down => p.y = p.y.wrapping_add(1),
        Dir4::Left => p.x = p.x.wrapping_sub(1),
        Dir4::Right => p.x = p.x.wrapping_add(1),
    }
}

fn parse_treemap(content: &str) -> Treemap {
    // TODO shared code (2021/09)
    let lines = shared::split_lines(content);
    Grid::from_2d(
        &lines
            .iter()
            .map(|x| x.chars().map(|c| c as u8 - b'0').collect())
            .collect(),
    )
}

/// Scan row/column and update visibility map
fn scan_vis(treemap: &Treemap, vis: &mut Grid<bool>, start: &Point2<usize>, dir: &Dir4) {
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
fn scan_scenic(treemap: &Treemap, start: &Point2<usize>, dir: &Dir4) -> u32 {
    let mut score = 0;
    let start_tree = treemap.get(start.x, start.y);
    let mut pos = start.to_owned();
    move_in_dir4(&mut pos, dir);
    while treemap.is_in_bounds(pos.x, pos.y) {
        score += 1;
        if treemap.get(pos.x, pos.y) >= start_tree { break; }
        move_in_dir4(&mut pos, dir);
    }
    score
}

/// Product of scenic scores in each dir
fn scenic_score(treemap: &Treemap, x: usize, y: usize) -> u32 {
    let p: Point2<usize> = Point2::new(x, y);
    Dir4::VALUES.iter().map(|x| scan_scenic(treemap, &p, x)).product()
}

fn part1(treemap: &Treemap) -> u32 {
    let (w, h) = treemap.dim().to_tuple();
    let mut vis: Grid<bool> = Grid::new_with_default(w, h);
    for y in 0..h {
        scan_vis(treemap, &mut vis, &Point2::new(0, y), &Dir4::Right);
        scan_vis(treemap, &mut vis, &Point2::new(w - 1, y), &Dir4::Left);
    }
    for x in 0..w {
        scan_vis(treemap, &mut vis, &Point2::new(x, 0), &Dir4::Down);
        scan_vis(treemap, &mut vis, &Point2::new(x, h - 1), &Dir4::Up);
    }
    vis.vec().iter().filter(|&&x| x).count() as u32
}

fn part2(treemap: &Treemap) -> u32 {
    let mut best = 0;
    for x in 0..treemap.dim().x {
        for y in 0..treemap.dim().y {
            let score = scenic_score(treemap, x, y);
            if score > best { best = score; }
        }
    }
    best
}

fn main() {
    let treemap = parse_treemap(&shared::input_as_str(DAY, "input"));
    println!("Part 1: {}", part1(&treemap));
    println!("Part 2: {}", part2(&treemap));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_treemap() -> Treemap {
        parse_treemap(&shared::input_as_str(DAY, "input.test"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_test_treemap()), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_test_treemap()), 8);
    }
}

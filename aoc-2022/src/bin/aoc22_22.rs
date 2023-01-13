extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::data::{Grid, GridPos};
use aoc_lib::harness::*;
use std::collections::{HashMap, VecDeque};

pub struct Day22;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Cell {
    Void,
    Open,
    Wall,
}

#[derive(Debug, Clone)]
enum Instruction {
    Forward(u8),
    Left,
    Right,
}

type Pos = GridPos;

/// Steps per facing: 0 = Right, 1 = Down, 2 = Left, 3 = Up
const DIRS: [Pos; NUM_DIRS] = [Pos::new(1, 0), Pos::new(0, 1), Pos::new(-1, 0), Pos::new(0, -1)];
const NUM_DIRS: usize = 4;
const DIR_RIGHT: usize = 0;
const DIR_DOWN: usize = 1;
const DIR_LEFT: usize = 2;
const DIR_UP: usize = 3;

struct Input {
    map: Grid<Cell>,
    instructions: Vec<Instruction>,
}
type Output = u32;

impl Solution<Input, Output> for Day22 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Monkey Map", 2022, 22)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let mut lines = resource.as_str_lines()?;
        let instr = lines.pop().expect("instruction line must be present");
        Ok(Input { map: parse_map(&lines), instructions: parse_instructions(&instr) })
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(part1(input))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(part2(input))
    }
}

fn parse_map(lines: &[String]) -> Grid<Cell> {
    let w = lines.iter().map(|x| x.len()).max().expect("must have max length");
    let mut result = Grid::new(Cell::Void, w, lines.len());
    lines.iter().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            result.set(
                x as i32,
                y as i32,
                match c {
                    '.' => Cell::Open,
                    '#' => Cell::Wall,
                    _ => Cell::Void,
                },
            )
        })
    });
    result
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut line = line.to_string();
    line = line.replace('L', " L ");
    line = line.replace('R', " R ");
    line.split_whitespace()
        .map(|i| match i {
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            x => Instruction::Forward(x.parse().expect("forward move must be valid")),
        })
        .collect()
}

fn calc_result(pos: &Pos, dir: usize) -> Output {
    // println!("calc_result: pos:{:?}, facing:{:?}", pos, dir);
    ((pos.y + 1) * 1000 + (pos.x + 1) * 4 + dir as i32) as Output
}

fn part1(input: &Input) -> Output {
    let (mut pos, mut dir) = init_walker(&input.map);
    input.instructions.iter().for_each(|i| proc_instruction(i, &input.map, &mut pos, &mut dir));
    calc_result(&pos, dir)
}

fn init_walker(map: &Grid<Cell>) -> (Pos, usize) {
    // Scan first row to find start position
    let (first_open, _) = map.find(|x| x == &Cell::Open).expect("");
    (Pos::new(first_open.x, first_open.y), DIR_RIGHT)
}

fn proc_instruction(instruction: &Instruction, map: &Grid<Cell>, pos: &mut Pos, dir: &mut usize) {
    match instruction {
        Instruction::Left => *dir = (*dir + NUM_DIRS - 1) % NUM_DIRS,
        Instruction::Right => *dir = (*dir + 1) % NUM_DIRS,
        Instruction::Forward(steps) => {
            // Take each step in turn
            for _ in 0..*steps {
                // Work out provisional new location, accounting for wrapping
                let mut next = *pos;
                loop {
                    next += DIRS[*dir];
                    // Wrap
                    next.x = next.x.rem_euclid(map.dim().x);
                    next.y = next.y.rem_euclid(map.dim().y);
                    // Skip blanks
                    match map.get(next.x, next.y) {
                        Cell::Void => (),
                        Cell::Open => break,
                        Cell::Wall => return,
                    }
                }
                // Apply
                *pos = next;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
enum Face {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}
const NUM_FACES: usize = 6;

/// Faces connectivity, assuming standardised orientation
/// (sides pointing upward, top and bottom have "up" pointing to the back)
///
/// Includes rotation FROM standardised when directly connected:
///   e.g. if you can go directly up from top to back, the back will be upside down...
const FACE_LINKS: [[(Face, usize); 4]; 6] = [
    // Everything is effectively defined twice here - e.g. Top > Right and Right > Top.
    // The rotations should be mirrored (so Top > Right = 3, Right > Top = 1)

    /* TOP    */
    [(Face::Right, 1), (Face::Front, 0), (Face::Left, 3), (Face::Back, 2)],
    /* BOTTOM */
    [(Face::Left, 3), (Face::Front, 2), (Face::Right, 1), (Face::Back, 0)],
    /* FRONT  */
    [(Face::Right, 0), (Face::Bottom, 2), (Face::Left, 0), (Face::Top, 0)],
    /* BACK   */
    [(Face::Left, 0), (Face::Bottom, 0), (Face::Right, 0), (Face::Top, 2)],
    /* LEFT   */
    [(Face::Front, 0), (Face::Bottom, 1), (Face::Back, 0), (Face::Top, 1)],
    /* RIGHT  */
    [(Face::Back, 0), (Face::Bottom, 3), (Face::Front, 0), (Face::Top, 3)],
];

/// Store face position and rotation
#[derive(Debug, Copy, Clone)]
struct FaceMapping {
    face: Face,
    pos: Pos,
    rotation: usize,
}

/// Process the given map input, folding to determine all cube faces areas and orientations.
fn cube_face_mapping(map: &Grid<Cell>) -> (Vec<FaceMapping>, i32) {
    // Figure out the size of the cube sides, in terms of grid cells
    let cell_count = map.vec().iter().filter(|c| c != &&Cell::Void).count();
    let side_size = ((cell_count / NUM_FACES) as f64).sqrt().round() as i32;
    // Find all sides
    let mut all_faces = VecDeque::new();
    for y in (0..map.dim().y).step_by(side_size as usize) {
        for x in (0..map.dim().x).step_by(side_size as usize) {
            if map.get(x, y) != &Cell::Void {
                all_faces.push_back(Pos::new(x / side_size, y / side_size));
            }
        }
    }

    // Do a quick BFS from starting node (TOP)
    let mut open = VecDeque::new();
    let mut faces = HashMap::new();

    let top_pos = all_faces.pop_front().expect("top must be present");
    faces.insert(top_pos, FaceMapping { face: Face::Top, pos: top_pos, rotation: 0 });
    open.push_back(top_pos);

    while !open.is_empty() {
        let current = open.pop_front().expect("open set cannot be empty here");
        for candidate in &all_faces {
            // Already processed?
            if faces.contains_key(candidate) {
                continue;
            }
            // Candidate is not yet resolved.
            // Check step in each direction from current; if that position gives this unknown face,
            // then we can resolve it.
            let delta = *candidate - current;
            for d in 0..NUM_DIRS {
                if delta == DIRS[d] {
                    let from = faces[&current];
                    // Standardise rotation
                    let rot = (from.rotation + d) % NUM_DIRS;
                    let (face, rot) = FACE_LINKS[from.face as usize][rot];
                    let face = FaceMapping {
                        face,
                        pos: *candidate,
                        rotation: (from.rotation + rot) % NUM_DIRS,
                    };
                    // println!("{:?} -> {:?}", from, face);
                    faces.insert(*candidate, face);
                    open.push_back(*candidate);
                }
            }
        }
    }
    let mut result: Vec<FaceMapping> = faces.values().copied().collect();
    result.sort_unstable_by(|a, b| (a.face as usize).cmp(&(b.face as usize)));
    // println!("{:?}", result);
    (result, side_size)
}

/// Apply an instruction to the current state.
///
/// Walking off the edge of known tiles should wrap around, as if folded to cube.
fn proc_instruction_cube(
    instruction: &Instruction,
    map: &Grid<Cell>,
    faces: &[FaceMapping],
    face_size: i32,
    pos: &mut Pos,
    dir: &mut usize,
) {
    //println!("{:?}", instruction);
    //println!("{:?}:{:?}", pos, dir);
    match instruction {
        Instruction::Left => *dir = (*dir + NUM_DIRS - 1) % NUM_DIRS,
        Instruction::Right => *dir = (*dir + 1) % NUM_DIRS,
        Instruction::Forward(steps) => {
            // Take each step in turn
            for _ in 0..*steps {
                // Work out provisional new location, accounting for wrapping
                let mut next = *pos;
                let mut next_dir = *dir;
                next += DIRS[*dir];
                // If we've walked off into the void, figure out where we re-enter
                if !map.is_in_bounds(next.x, next.y) || map.get(next.x, next.y) == &Cell::Void {
                    //println!("WRAP! {:?}:{:?}", pos, dir);
                    // Wrap around
                    // Figure out which face we're moving from and to
                    let old_face = faces
                        .iter()
                        .find(|x| x.pos == Pos::new(pos.x / face_size, pos.y / face_size))
                        .expect("old_face must exist");
                    let (link, rot) =
                        FACE_LINKS[old_face.face as usize][(old_face.rotation + *dir) % 4];
                    let next_face = faces[link as usize];
                    // Figure out our new facing
                    next_dir =
                        (next_dir + NUM_DIRS * 2 - next_face.rotation + old_face.rotation + rot)
                            % NUM_DIRS;
                    // Where are we going to emerge?
                    let mut offset =
                        if *dir % 2 == 1 { pos.x % face_size } else { pos.y % face_size };
                    //println!("offset:{}", offset);
                    if dir == &1 || dir == &2 {
                        offset = face_size - offset - 1;
                    }
                    next = Pos::new(next_face.pos.x * face_size, next_face.pos.y * face_size);
                    match next_dir {
                        DIR_RIGHT => next.y += offset,
                        DIR_DOWN => next.x += face_size - offset - 1,
                        DIR_LEFT => {
                            next.y += face_size - offset - 1;
                            next.x += face_size - 1;
                        }
                        DIR_UP => {
                            next.x += offset;
                            next.y += face_size - 1;
                        }
                        _ => (),
                    }

                    // Sanity check warps - so many bugs...
                    // println!("From {:?}: {:?}, facing {:?} -->  to {:?}: {:?}, facing {:?}", pos, old_face.face, dir, next, next_face.face, next_dir);
                }
                // Stop if we hit a wall
                if map.get(next.x, next.y) == &Cell::Wall {
                    return;
                }
                // Apply
                *pos = next;
                *dir = next_dir;
            }
        }
    }
}

fn part2(input: &Input) -> u32 {
    let (face_map, face_size) = cube_face_mapping(&input.map);
    let (mut pos, mut dir) = init_walker(&input.map);
    input.instructions.iter().for_each(|i| {
        proc_instruction_cube(i, &input.map, &face_map, face_size, &mut pos, &mut dir)
    });
    calc_result(&pos, dir)
}

fn main() -> Result<()> {
    run_solution(&Day22)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day22, SolutionPart::One), 6032);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day22, SolutionPart::Two), 5031);
    }
}

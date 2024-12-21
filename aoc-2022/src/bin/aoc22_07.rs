extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::harness::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Day07;
type Input = Rc<Dir>;
type Output = u32;
impl Solution<Input, Output> for Day07 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("No Space Left On Device", 2022, 7)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        build_fs(&resource.as_str_lines()?)
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(Dir::flatten(input).iter().map(|n| n.size()).filter(|x| x <= &100000).sum())
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let used = input.size();
        // Total 70000000, req 30000000: max usage=40000000
        let min_required = used - 40000000;
        let mut sorted: Vec<u32> = Dir::flatten(input).iter().map(|n| n.size()).collect();
        sorted.sort_unstable();
        Ok(*sorted.iter().find(|&&x| x >= min_required).ok_or_else(|| anyhow!("No result"))?)
    }
}

/// A directory in the filesystem
#[derive(Default, Debug)]
pub struct Dir {
    name: String,
    // Could use a hashmap for this instead; whatever
    dirs: RefCell<Vec<Rc<Dir>>>,
    files: RefCell<Vec<File>>,
}

impl Dir {
    /// Size of files in this directory and all subdirectories
    fn size(&self) -> u32 {
        self.files.borrow().iter().map(|x| x.size).sum::<u32>()
            + self.dirs.borrow().iter().map(|x| x.size()).sum::<u32>()
    }

    /// Flattened Vec of all subdirectories, and their subdirectories, etc. Includes the given dir.
    fn flatten(dir: &Rc<Dir>) -> Vec<Rc<Dir>> {
        let mut result: Vec<Rc<Dir>> = vec![Rc::clone(dir)];
        for i in dir.dirs.borrow().iter() {
            result.append(&mut Dir::flatten(i));
        }
        result
    }
}

/// A file in the filesystem
#[derive(Debug)]
pub struct File {
    size: u32,
}

/// Parse input into a meaningful filesystem
fn build_fs(lines: &Vec<String>) -> Result<Rc<Dir>> {
    let root = Rc::new(Dir { name: "".to_string(), ..Default::default() });
    let mut parents: Vec<Rc<Dir>> = vec![];
    let mut current = Rc::clone(&root);

    for line in lines {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        match (cmd[0], cmd[1]) {
            ("$", "ls") => { /* no-op */ }
            ("$", "cd") => match cmd[2] {
                ".." => {
                    current =
                        Rc::clone(&parents.pop().ok_or_else(|| anyhow!("Cannot traverse up"))?);
                }
                "/" => {
                    current = Rc::clone(&root);
                    parents.clear();
                }
                dir => {
                    parents.push(Rc::clone(&current));
                    let new_current = Rc::clone(
                        current
                            .borrow_mut()
                            .dirs
                            .borrow_mut()
                            .iter()
                            .find(|x| x.name == dir)
                            .ok_or_else(|| anyhow!("Dir not found"))?,
                    );
                    current = new_current;
                }
            },
            ("dir", dir) => {
                current
                    .dirs
                    .borrow_mut()
                    .push(Rc::new(Dir { name: dir.to_string(), ..Default::default() }));
            }
            (size, _) => current.files.borrow_mut().push(File {
                // name: file.to_string(),
                size: size.parse::<u32>()?,
            }),
        }
    }
    Ok(root)
}

fn main() -> Result<()> {
    run_solution(&Day07)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day07, SolutionPart::One), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day07, SolutionPart::Two), 24933642);
    }
}

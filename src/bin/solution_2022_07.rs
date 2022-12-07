extern crate aoc;

use aoc::shared;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

const DAY: &str = "2022/07";

/// A directory in the filesystem
#[derive(Default, Debug)]
struct Dir {
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
struct File {
    size: u32,
}

/// Parse input into a meaningful filesystem
fn build_fs(lines: &[&str]) -> Rc<Dir> {
    let root = Rc::new(Dir {
        name: "".to_string(),
        ..Default::default()
    });
    let mut parents: Vec<Rc<Dir>> = vec![];
    let mut current = Rc::clone(&root);

    for line in lines {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        match (cmd[0], cmd[1]) {
            ("$", "ls") => { /* no-op */ }
            ("$", "cd") => match cmd[2] {
                ".." => {
                    current = Rc::clone(&parents.pop().unwrap());
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
                            .unwrap(),
                    );
                    current = new_current;
                }
            },
            ("dir", dir) => {
                current.dirs.borrow_mut().push(Rc::new(Dir {
                    name: dir.to_string(),
                    ..Default::default()
                }));
            }
            (size, _) => current.files.borrow_mut().push(File {
                // name: file.to_string(),
                size: size.parse::<u32>().unwrap(),
            }),
        }
    }
    root
}

/// Sum of all directories that have size <= 100000
fn part1(fs: &Rc<Dir>) -> u32 {
    Dir::flatten(fs)
        .iter()
        .map(|n| n.size())
        .filter(|x| x <= &100000)
        .sum()
}

/// Smallest deletion to reach 30000000 bytes free
fn part2(fs: &Rc<Dir>) -> u32 {
    let used = fs.size();
    // Total 70000000, req 30000000: max usage=40000000
    let min_required = used - 40000000;
    let mut sorted: Vec<u32> = Dir::flatten(fs).iter().map(|n| n.size()).collect();
    sorted.sort_unstable();
    *sorted.iter().find(|&&x| x >= min_required).unwrap()
}

fn main() {
    let input = shared::input_as_str(DAY, "input");
    let lines = shared::split_lines(&input);
    let fs = build_fs(&lines);
    println!("Part 1: {}", part1(&fs));
    println!("Part 2: {}", part2(&fs));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_fs() -> Rc<Dir> {
        build_fs(&shared::split_lines(&shared::input_as_str(
            DAY,
            "input.test",
        )))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_test_fs()), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_test_fs()), 24933642);
    }
}

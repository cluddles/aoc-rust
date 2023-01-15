extern crate aoc_lib;

use std::collections::HashMap;

use anyhow::Result;

use aoc_lib::common;
use aoc_lib::harness::*;

/// All cave data
#[derive(Default, Debug)]
pub struct CaveSystem {
    caves: Vec<Cave>,
    cave_lookup: HashMap<String, usize>,
}

impl CaveSystem {
    const START: usize = 0;
    const END: usize = 1;

    /// Parse input text as CaveSystem object
    fn parse(input: &str) -> CaveSystem {
        let mut result = CaveSystem::default();
        result.get_or_create_cave("start");
        result.get_or_create_cave("end");
        let lines = common::split_lines(input);
        for line in lines {
            let parts: Vec<&str> = line.split('-').collect();
            let (a, b) = (parts[0], parts[1]);
            let c1 = result.get_or_create_cave(a);
            let c2 = result.get_or_create_cave(b);
            result.caves[c1].connections.push(c2);
            result.caves[c2].connections.push(c1);
        }
        result
    }

    /// Lookup cave with given name, creating it if it doesn't exist.
    fn get_or_create_cave(&mut self, name: &str) -> usize {
        if let Some(&x) = self.cave_lookup.get(name) {
            return x;
        }
        self.caves.push(Cave { name: name.to_string(), ..Default::default() });
        let index = self.caves.len() - 1;
        self.cave_lookup.insert(name.to_string(), self.caves.len() - 1);
        index
    }

    /// Determine whether the given node can be visited, based on previous visits
    fn can_visit(&self, history: &[usize], node: usize, dupes_allowed: u8) -> Visit {
        if node != CaveSystem::START {
            if self.caves[node].is_big() || !history.contains(&node) {
                return Visit::Allowed(false);
            }
            if dupes_allowed > 0 {
                return Visit::Allowed(true);
            }
        }
        Visit::Denied
    }

    /// Calculate all successful routes for the given history and current node
    fn traverse(&self, path: &[usize], current: usize, dupes_allowed: u8) -> Vec<Vec<usize>> {
        // Add current node to history
        let mut path = path.to_owned();
        path.push(current);
        // If we've reached the end then this route was successful - return it!
        if current == CaveSystem::END {
            return vec![path];
        }
        // Otherwise, traverse all valid connected nodes
        let mut result: Vec<Vec<usize>> = Vec::new();
        let nodes = &self.caves[current].connections;
        for &node in nodes {
            // Big caves can be revisited; everything else cannot
            if let Visit::Allowed(dupe) = self.can_visit(&path, node, dupes_allowed) {
                let d = if dupe { dupes_allowed - 1 } else { dupes_allowed };
                self.traverse(&path, node, d).into_iter().for_each(|path| result.push(path));
            }
        }
        result
    }
}

/// A single cave: name and connections (as indexes)
#[derive(Default, Debug)]
pub struct Cave {
    name: String,
    connections: Vec<usize>,
}

impl Cave {
    /// True if this cave is "big" (has an uppercase name)
    fn is_big(&self) -> bool {
        self.name.starts_with(|c: char| c.is_uppercase())
    }
}

/// Whether node can be visited, and whether it is a valid duplicate visit (part 2)
#[derive(Debug)]
enum Visit {
    Allowed(bool),
    Denied,
}

pub struct Day12;

impl Solution<CaveSystem, usize> for Day12 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Passage Pathing", 2021, 12)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<CaveSystem> {
        Ok(CaveSystem::parse(&resource.as_str()?))
    }

    fn solve_part1(&self, input: &CaveSystem) -> Result<usize> {
        Ok(input.traverse(&[], CaveSystem::START, 0).len())
    }

    fn solve_part2(&self, input: &CaveSystem) -> Result<usize> {
        Ok(input.traverse(&[], CaveSystem::START, 1).len())
    }
}

fn main() -> Result<()> {
    run_solution(&Day12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(test_ext(&Day12, SolutionPart::One, "input.test.1"), 10);
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(test_ext(&Day12, SolutionPart::One, "input.test.2"), 19);
    }
    #[test]
    fn test_part1_3() {
        assert_eq!(test_ext(&Day12, SolutionPart::One, "input.test.3"), 226);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(test_ext(&Day12, SolutionPart::Two, "input.test.1"), 36);
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(test_ext(&Day12, SolutionPart::Two, "input.test.2"), 103);
    }
    #[test]
    fn test_part2_3() {
        assert_eq!(test_ext(&Day12, SolutionPart::Two, "input.test.3"), 3509);
    }
}

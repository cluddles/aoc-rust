extern crate aoc;

use std::collections::HashMap;
use aoc::shared;

const DAY: &str = "2021/12";

// Potential improvement: stop using strings everywhere; cave lookup by usize or something?

/// Contains all permitted node -> node moves
struct Rulebook {
    connections: HashMap<String, Vec<String>>
}

/// Whether node can be visited, and whether it is a valid duplicate visit (part 2)
enum Visit {
    Allowed(bool),
    Denied,
}

fn parse_rulebook(input: &str) -> Rulebook {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let lines = shared::split_lines(input);
    for line in lines {
        let parts: Vec<&str> = line.split('-').collect();
        let (a, b) = (parts[0], parts[1]);
        connections.entry(a.to_string()).or_default().push(b.to_string());
        connections.entry(b.to_string()).or_default().push(a.to_string());
    }
    Rulebook { connections }
}

/// Determine whether the given node can be visited, based on previous visits
fn can_visit(history: &[String], node: &String, dupes_allowed: u8) -> Visit {
    if node != "start" {
        if node.starts_with(|c: char| c.is_uppercase()) || !history.contains(node) { return Visit::Allowed(false); }
        if dupes_allowed > 0 { return Visit::Allowed(true); }
    }
    Visit::Denied
}

/// Calculate all successful routes for the given history and current node
fn traverse(path: &[String], current: &str, rulebook: &Rulebook, dupes_allowed: u8) -> Vec<Vec<String>> {
    // Add current node to history
    let mut path = path.to_owned();
    path.push(current.to_string());
    // If we've reached the end then this route was successful - return it!
    if current == "end" {
        return vec![path];
    }
    // Otherwise, traverse all valid connected nodes
    let mut result: Vec<Vec<String>> = Vec::new();
    let nodes = rulebook.connections.get(current).unwrap();
    for node in nodes {
        // Big caves can be revisited; everything else cannot
        if let Visit::Allowed(dupe) = can_visit(&path, node, dupes_allowed) {
            let d = if dupe { dupes_allowed - 1 } else { dupes_allowed };
            traverse(&path, node, rulebook, d).into_iter().for_each(|path| result.push(path));
        }
    }
    result
}

fn part1(rulebook: &Rulebook) -> u32 {
    traverse(&[], "start", rulebook, 0).len() as u32
}

fn part2(rulebook: &Rulebook) -> u32 {
    traverse(&[], "start", rulebook, 1).len() as u32
}

fn main() {
    let rulebook = parse_rulebook(&shared::input_as_str(DAY, "input"));
    println!("Part 1: {}", part1(&rulebook));
    println!("Part 2: {}", part2(&rulebook));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rulebook(filename: &str) -> Rulebook {
        parse_rulebook(&shared::input_as_str(DAY, filename))
    }

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&rulebook("input.test.1")), 10);
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&rulebook("input.test.2")), 19);
    }
    #[test]
    fn test_part1_3() {
        assert_eq!(part1(&rulebook("input.test.3")), 226);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&rulebook("input.test.1")), 36);
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&rulebook("input.test.2")), 103);
    }
    #[test]
    fn test_part2_3() {
        assert_eq!(part2(&rulebook("input.test.3")), 3509);
    }

}

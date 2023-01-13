extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::harness::*;
use std::collections::{HashMap, VecDeque};

pub struct Day16;

type Input = Network;
type Output = u32;

impl Solution<Input, Output> for Day16 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Proboscidea Volcanium", 2022, 16)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        let lines = resource.as_str_lines()?;
        // Get all the valves, make a graph
        let valves: HashMap<String, Valve> = lines
            .into_iter()
            .map(|s| {
                let valve = parse_valve(&s).expect("valve must be valid");
                (valve.name.to_string(), valve)
            })
            .collect();
        let graph = build_graph(&valves);
        // Create graph of direct links between non-zero flow rate valves
        let valves: HashMap<String, Valve> =
            valves.into_iter().filter(|(n, x)| n == "AA" || x.flow_rate > 0).collect();
        let graph = reduce_graph(&graph, valves.values().map(|x| &x.name).collect());
        Ok(Network { valves, graph })
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(part1(input))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(part2(input))
    }
}

fn parse_valve(line: &str) -> Result<Valve> {
    // "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
    let parts: Vec<&str> = line.split(';').collect();
    let name = &(parts[0])[6..=7];
    let flow_rate = parts[0].split('=').collect::<Vec<&str>>()[1].parse()?;
    let parts: Vec<&str> = parts[1].split(' ').collect();
    let tunnels: Vec<String> = parts[5..].iter().map(|x| x[0..2].to_string()).collect();
    Ok(Valve { name: name.to_string(), flow_rate, tunnels })
}

fn build_graph(valves: &HashMap<String, Valve>) -> Graph {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    for (name, valve) in valves {
        let mut links = Vec::new();
        for tunnel in &valve.tunnels {
            links.push((tunnel.to_string(), 1));
        }
        nodes.insert(name.to_string(), Node { links });
    }
    Graph { nodes }
}

/// Reduce the graph, discarding all nodes except those given.
fn reduce_graph(graph: &Graph, nodes_to_keep: Vec<&String>) -> Graph {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    for i in 0..nodes_to_keep.len() {
        for j in i + 1..nodes_to_keep.len() {
            let from = &nodes_to_keep[i];
            let to = &nodes_to_keep[j];
            // +1 to include cost of activating the valve
            let cost = bfs_len(graph, from, to).expect("path must exist") + 1;
            nodes
                .entry(from.to_string())
                .or_insert_with(Node::default)
                .links
                .push((to.to_string(), cost));
            nodes
                .entry(to.to_string())
                .or_insert_with(Node::default)
                .links
                .push((from.to_string(), cost));
        }
    }
    Graph { nodes }
}

/// Find a path using breadth first search. Ignores cost.
pub fn bfs_len(graph: &Graph, start: &String, end: &String) -> Option<u32> {
    // Empty queue - "open". Add the start node.
    let mut open = VecDeque::new();
    open.push_back(start);
    // Visited nodes, track where they were visited from
    let mut prev: HashMap<&String, &String> = HashMap::new();
    prev.insert(start, start);
    // Dequeue from open
    while !open.is_empty() {
        let current = open.pop_front().expect("Queue cannot be empty");
        for (link, _) in &graph.nodes[current].links {
            // If we're connected to the end then that'll do
            if link == end {
                prev.insert(link, current);
                // Could unfold the path properly here, but we only care about length
                let mut at = link;
                let mut result = 0;
                loop {
                    let a = prev[at];
                    if a == at {
                        return Some(result);
                    }
                    at = a;
                    result += 1;
                }
            }
            // And connected node to open, remember prev
            prev.entry(link).or_insert_with(|| {
                open.push_back(link);
                current
            });
        }
    }
    None
}

/// Find the single best path
fn part1(network: &Network) -> u32 {
    eval(network, &String::from("AA"), &[], 0, 30, 0, 0, &mut 0)
}

/// Find the best pair of paths working in tandem
fn part2(network: &Network) -> u32 {
    // Our part1 algorithm is just about quick enough to brute force this
    let num_evals = &mut 0;
    let mut best = 0;
    let all: Vec<&String> = network.valves.keys().filter(|x| x != &"AA").collect();
    // Figure out all the ways we can split the nodes into two lists to tackle
    // We only need to consider half the space (because it doesn't matter whether it's us or the
    // elephant visiting the nodes)
    let count = (1 << all.len()) / 2;
    // println!("{:?}", all);
    for i in 0..count {
        let mut split1 = Vec::new();
        for j in 0..all.len() {
            if i & (1 << j) >= 1 {
                split1.push(all[j]);
            }
        }
        let mut split2 = all.to_vec();
        split2.retain(|x| !split1.contains(x));
        // println!("({}): s1 {:?}, s2 {:?}", num_evals, split1, split2);
        best = (eval(network, &String::from("AA"), &split1, 0, 26, 0, 0, num_evals)
            + eval(network, &String::from("AA"), &split2, 0, 26, 0, 0, num_evals))
        .max(best);
    }
    best
}

/// Depth-first scan for the "best" score.
///
/// Reject any traversals reaching a state that cannot possibly exceed the best score, even under
/// (impossibly) ideal conditions.
///
/// (For "real" input, this gets the search space down from 1+ trillion to about 80k)
fn eval(
    network: &Network,
    current: &String,
    visited: &[&String],
    tick: u32,
    max_tick: u32,
    score: u32,
    best: u32,
    num_evals: &mut u32,
) -> u32 {
    *num_evals += 1;
    // println!("(({}) {:?} => {} = ticks:{} score:{} best:{}", num_evals, visited, current, tick, score, best);
    // Bail out if we have no more time for meaningful action
    if tick >= max_tick - 2 {
        return score;
    }
    // Bail out of any branches where current + remaining score could not reach best
    if best > 0 {
        let mut remain = 0;
        let mut min_cost = u32::MAX;
        for (node, cost) in &network.graph.nodes[current].links {
            if visited.contains(&node) {
                continue;
            }
            remain += network.valves[node].flow_rate;
            min_cost = min_cost.min(*cost);
        }
        if min_cost == u32::MAX
            || tick + min_cost >= max_tick
            || score + (max_tick - (tick + min_cost)) * remain <= best
        {
            return score;
        }
    }
    let prev = &mut visited.to_vec();
    prev.push(current);
    let mut my_best = best.max(score);
    for (node, cost) in &network.graph.nodes[current].links {
        // Only hit each node once
        if prev.contains(&node) {
            continue;
        }
        // Out of time?
        let tick = tick + cost;
        if tick >= max_tick {
            continue;
        }
        // Score for this node
        let score = score + (max_tick - tick) * network.valves[node].flow_rate;
        my_best = my_best.max(eval(network, node, prev, tick, max_tick, score, my_best, num_evals));
    }
    my_best
}

#[derive(Debug)]
pub struct Network {
    valves: HashMap<String, Valve>,
    graph: Graph,
}

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<String, Node>,
}

#[derive(Debug, Default)]
pub struct Node {
    links: Vec<(String, u32)>,
}

fn main() -> Result<()> {
    run_solution(&Day16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day16, SolutionPart::One), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day16, SolutionPart::Two), 1707);
    }
}

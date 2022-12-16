extern crate aoc_lib;

use aoc_lib::harness::*;
use std::collections::{HashMap, HashSet};
use aoc_lib::path::bfs;

pub struct Day16;

type Input = Network;
type Output = usize;

impl Solution<Input, Output> for Day16 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Proboscidea Volcanium", 2022, 16)
    }

    fn parse_input(&self, resource: &dyn Resource) -> DynResult<Input> {
        let lines = resource.as_str_lines()?;
        let valves: Vec<Valve> = lines
            .iter()
            .map(|x| parse_valve(x).expect("valve must be valid"))
            .collect();
        let mut valve_lookup = HashMap::new();
        for (i, valve) in valves.iter().enumerate() {
            valve_lookup.insert(valve.name.to_string(), i);
        }
        let graph = build_graph(&valves, &valve_lookup);
        Ok(Network {
            valves,
            valve_lookup,
            graph,
        })
    }

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        // part1(input)
        Ok(0)
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        Ok(0)
    }
}

fn part1(network: &Network) -> SolutionResult<Output> {
    let mut meaningful_valves: HashSet<&str> = network
        .valves
        .iter()
        .filter(|x| x.flow_rate > 0)
        .map(|x| &x.name[..])
        .collect();
    // Calculate distance to each meaningful valve
    // Score: (30 - current_tick - cost - 1) * flow_rate
    // Repeat until all valves visited
    let mut ticks = 0;
    let mut score = 0;
    let mut current = network.valve_lookup["AA"];
    while !meaningful_valves.is_empty() {
        let (valve, _, v_cost, v_index) = meaningful_valves
            .iter()
            .map(|&v| {
                let index = network.valve_lookup[v];
                let valve = &network.valves[index];
                let cost =
                    breadth_first_search(&network.graph, current, index).expect("valid path");
                let score = ((30 - ticks - cost - 1) * valve.flow_rate).max(0);
                println!("{} = {}, {}", valve.name, cost, valve.flow_rate);
                (valve, score, cost, index)
            })
            .max_by(|a, b| a.1.cmp(&b.1))
            .expect("answer must exist");
        ticks += v_cost + 1;
        if ticks > 30 {
            break;
        }
        score += (30 - ticks) * valve.flow_rate;
        current = v_index;
        println!(">> {} ({}, {})", valve.name, v_cost, valve.flow_rate);
        meaningful_valves.remove(&valve.name[..]);
    }
    Ok(score)
}

fn parse_valve(line: &str) -> DynResult<Valve> {
    // "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
    let parts: Vec<&str> = line.split(';').collect();
    let name = &(parts[0])[6..=7];
    let flow_rate = parts[0].split('=').collect::<Vec<&str>>()[1].parse()?;
    let parts: Vec<&str> = parts[1].split(' ').collect();
    let tunnels: Vec<String> = parts[5..].iter().map(|x| x[0..2].to_string()).collect();
    Ok(Valve {
        name: name.to_string(),
        flow_rate,
        tunnels,
    })
}

fn build_graph(valves: &Vec<Valve>, valve_lookup: &HashMap<String, usize>) -> Graph {
    let mut result = vec![Vec::new(); valves.len()];
    for valve in valves {
        let node = &mut result[*valve_lookup
            .get(&valve.name)
            .expect("valve name must be mapped")];
        for tunnel in &valve.tunnels {
            node.push(
                *valve_lookup
                    .get(tunnel)
                    .expect("tunnel name must be mapped"),
            )
        }
    }
    result
}

#[derive(Debug)]
pub struct Network {
    valves: Vec<Valve>,
    valve_lookup: HashMap<String, usize>,
    graph: Graph,
}

#[derive(Debug)]
pub struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

type Link = Vec<usize>;
type Graph = Vec<Link>;

fn breadth_first_search(graph: &Graph, start_node: usize, end_node: usize) -> DynResult<usize> {
    let path = bfs(&start_node, &end_node, &graph, |ctx, n| ctx[*n].iter());
    Ok(path.expect("path not found").len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day16, SolutionPart::One), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day16, SolutionPart::Two), 0);
    }
}

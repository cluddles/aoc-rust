extern crate aoc_lib;

use std::collections::{HashMap, HashSet, VecDeque};
use aoc_lib::harness::*;

pub struct Day19;

type Input = Vec<Blueprint>;
type Output = u32;

impl Solution<Input, Output> for Day19 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Not Enough Minerals", 2022, 19)
    }

    fn parse_input(&self, resource: &dyn Resource) -> DynResult<Input> {
        resource
            .as_str_lines()?
            .iter()
            .map(|i| parse_blueprint(i))
            .collect()
    }

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        println!("{:?}", input);
        Ok(0)
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        Ok(0)
    }
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

const NUM_RESOURCES: usize = 4;
const NUM_COST_RESOURCES: usize = 3;

/// ore, clay, obsidian
type BotCost = [u32; NUM_COST_RESOURCES];

#[derive(Debug)]
struct Blueprint {
    id: u32,
    bots: [BotCost; NUM_RESOURCES],
    max: BotCost
}

fn parse_blueprint(line: &str) -> DynResult<Blueprint> {
    use regex::Regex;
    let re = Regex::new(
        r"Blueprint (\d+): .*costs (\d+) ore.*costs (\d+) ore.* costs (\d+) ore and (\d+) clay.* costs (\d+) ore and (\d+) obsidian",
    )?;
    if let Some(cap) = re.captures_iter(line).next() {
        let bot_costs = [ [cap[2].parse()?, 0, 0],
            [cap[3].parse()?, 0, 0],
            [cap[4].parse()?, cap[5].parse()?, 0],
            [cap[6].parse()?, 0, cap[7].parse()?], ];
        let max = bot_costs.iter().fold([0; NUM_COST_RESOURCES], |mut s, v| {
            for i in 0..NUM_COST_RESOURCES {
                s[i] = s[i].max(v[i]);
            }
            s
        });
        return Ok(Blueprint {
            id: cap[1].parse()?,
            bots: bot_costs,
            max,
        });
    }
    Err(SimpleError::new_dyn("Nothing to parse"))
}

#[derive(Debug, Clone, Default)]
struct Node {
    tick: u32,
    skip_bots: [bool; NUM_COST_RESOURCES],
    bots: [u32; NUM_RESOURCES],
    resources: [u32; NUM_COST_RESOURCES],
    geodes: u32,
}

impl Node {
    fn new() -> Node {
        let mut state = Node::default();
        state.bots[ORE] = 1;
        state
    }
}

fn max_geodes(bp: &Blueprint, num_ticks: u32) -> u32 {
    println!("{:?}", bp);
    let mut open = VecDeque::from([Node::new()]);
    let mut best = 0;
    while !open.is_empty() {
        let next = open.pop_front().expect("open cannot be empty");
        best = best.max(next.resources[GEODE]);
        expand_node(bp, num_ticks, &next, best).into_iter().for_each(|i| open.push_front(i));
    }
    best
}

fn expand_node(bp: &Blueprint, num_ticks: u32, node: &Node, best: u32) -> Vec<Node> {
    // Strats:
    // 1. Cull decisions you can't afford
    // 2. Cull decisions that make no sense (don't build a bot if you deliberately didn't in the previous step)
    // 3. Always build geode bots (cull everything else)
    // 4. Always build obsidian bots (cull everything else)
    // 5. Cull bot builds if you're maxed on a resource already: maxed = getting the biggest cost of any robot
    // 6. Best?

    println!("Expand: {:?}", node);

    // Out of time
    if node.tick + 1 >= num_ticks { return vec![]; }
    // Cannot hit best?
    let rem = num_ticks - node.tick;
    let potential = (node.resources[GEODE] * rem) + ((rem * (rem - 1)) / 2);
    if potential < best { return vec![]; }

    // 5 potential choices: 1 for each bot, 1 for nothing
    let mut result = Vec::with_capacity(5);
    // This is the do nothing state, where everything ticks up.
    // We can use this as a basis for everything else.
    let mut nop = node.clone();
    nop.tick += 1;
    for i in 0..NUM_RESOURCES {
        nop.resources[i] += nop.bots[i];
    }

    // Which bots can we build?
    // These checks are based on the resource levels BEFORE ticking up
    let mut affords = [false; 4];
    for i in 0..NUM_RESOURCES {
        if !node.skip_bots[i] && node.resources[ORE] >= bp.bots[i][ORE] && node.resources[CLAY] >= bp.bots[i][CLAY] && node.resources[OBSIDIAN] >= bp.bots[i][OBSIDIAN] {
            affords[i] = true;
        }
    }
    if affords[GEODE] {
        result.push(buy_bot(bp, &nop, GEODE));
    } else {
        if affords[OBSIDIAN] {
            nop.skip_bots[CLAY] = false;
            nop.skip_bots[ORE] = false;
            if check_max(bp, &nop, OBSIDIAN) { result.push(buy_bot(bp, &nop, OBSIDIAN)); }
            nop.skip_bots[OBSIDIAN] = true;
        }
        if affords[CLAY] {
            if check_max(bp, &nop, CLAY) { result.push(buy_bot(bp, &nop, CLAY)); }
            nop.skip_bots[CLAY] = true;
        }
        if affords[ORE] {
            if check_max(bp, &nop, ORE) { result.push(buy_bot(bp, &nop, ORE)); }
            nop.skip_bots[ORE] = true;
        }
        result.push(nop);
    }
    result
}

fn check_max(bp: &Blueprint, node: &Node, bot_res: usize) -> bool {
    node.bots[bot_res] < bp.max[bot_res]
}

fn buy_bot(bp: &Blueprint, node: &Node, bot_res: usize) -> Node {
    println!("buy bot: {}", bot_res);
    let mut n = node.clone();
    for i in 0..NUM_COST_RESOURCES {
        n.resources[i] -= bp.bots[bot_res][i];
    }
    n.bots[bot_res] += 1;
    n
}

fn main() -> DynResult<()> {
    run_solution(&Day19)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_geodes_blueprint1() -> DynResult<()> {
        let bp = parse_blueprint(
            FileResource::new("input.test", 2022, 19)
                .as_str_lines()?
                .first()
                .unwrap(),
        )?;
        assert_eq!(max_geodes(&bp, 24), 9);
        Ok(())
    }

    #[test]
    fn test_max_geodes_blueprint2() -> DynResult<()> {
        let bp = parse_blueprint(
            FileResource::new("input.test", 2022, 19)
                .as_str_lines()?
                .last()
                .unwrap(),
        )?;
        assert_eq!(max_geodes(&bp, 24), 12);
        Ok(())
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day19, SolutionPart::One), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day19, SolutionPart::Two), 0);
    }
}

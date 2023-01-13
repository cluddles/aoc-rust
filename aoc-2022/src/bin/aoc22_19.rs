extern crate aoc_lib;

use anyhow::{anyhow, Result};
use aoc_lib::harness::*;
use std::collections::VecDeque;

pub struct Day19;

type Input = Vec<Blueprint>;
type Output = u32;

impl Solution<Input, Output> for Day19 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Not Enough Minerals", 2022, 19)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        resource.as_str_lines()?.iter().map(|i| parse_blueprint(i)).collect()
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().map(|bp| bp.id * max_geodes(bp, 24)).sum())
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(input.iter().take(3).map(|bp| max_geodes(bp, 32)).product())
    }
}

const ORE: usize = 0;
// const CLAY: usize = 1;
// const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

/// Total number of resources, including geodes (which aren't used for costs)
const NUM_RESOURCES: usize = 4;
/// Total number of resources that are used for costs: ore, clay, obsidian
const NUM_COST_RESOURCES: usize = 3;

/// ore, clay, obsidian
type BotCost = [u32; NUM_COST_RESOURCES];

#[derive(Debug)]
struct Blueprint {
    id: u32,
    bots: [BotCost; NUM_RESOURCES],
    max: BotCost,
}

fn parse_blueprint(line: &str) -> Result<Blueprint> {
    use regex::Regex;
    let re = Regex::new(
        r"Blueprint (\d+): .*costs (\d+) ore.*costs (\d+) ore.* costs (\d+) ore and (\d+) clay.* costs (\d+) ore and (\d+) obsidian",
    )?;
    if let Some(cap) = re.captures_iter(line).next() {
        let bot_costs = [
            [cap[2].parse()?, 0, 0],
            [cap[3].parse()?, 0, 0],
            [cap[4].parse()?, cap[5].parse()?, 0],
            [cap[6].parse()?, 0, cap[7].parse()?],
        ];
        let max = bot_costs.iter().fold([0; NUM_COST_RESOURCES], |mut s, v| {
            for i in 0..NUM_COST_RESOURCES {
                s[i] = s[i].max(v[i]);
            }
            s
        });
        return Ok(Blueprint { id: cap[1].parse()?, bots: bot_costs, max });
    }
    Err(anyhow!("Nothing to parse"))
}

/// A single node in the decision process
#[derive(Debug, Clone, Default)]
struct Node {
    tick: u32,
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

/// Work out the max possible number of mined geodes for the given blueprint
fn max_geodes(bp: &Blueprint, num_ticks: u32) -> u32 {
    let mut open = VecDeque::from([Node::new()]);
    let mut best = 0;
    let mut _iterations = 0;
    while !open.is_empty() {
        _iterations += 1;
        let next = open.pop_front().expect("open cannot be empty");
        best = best.max(next.geodes);
        expand_node(bp, num_ticks, &next, best).into_iter().for_each(|i| open.push_back(i));
    }
    // println!("Done in {} iterations", _iterations);
    best
}

/// Expand a single node in the decision process, producing further nodes to consider.
fn expand_node(bp: &Blueprint, num_ticks: u32, node: &Node, best: u32) -> Vec<Node> {
    //    println!("Expand: {:?}", node);

    // Max geodes you could ever get from this position, if you did nothing but buy geode bots
    // every turn. Bail if this is worse than previous best. Totally unrealistic, but reduces
    // decision space by a considerable amount.
    let rem = num_ticks - node.tick - 1;
    let max_geodes = node.geodes + (rem * (rem + 1) / 2);
    if max_geodes <= best {
        return vec![];
    }

    // 4 potential bots we can build
    // (We could also do nothing, but I don't think that's ever desired)
    let mut result = Vec::with_capacity(4);
    'outer: for b in 0..NUM_RESOURCES {
        // Calculate the wait involved for this bot
        let mut wait = 0;
        for r in 0..NUM_COST_RESOURCES {
            let r_cost = bp.bots[b][r];
            let r_income = node.bots[r];
            if r_cost > 0 && r_income == 0 {
                // This bot requires resource that we have no income for!
                continue 'outer;
            }
            let r_current = node.resources[r];
            // The wait needs to be rounded up
            let r_wait = if r_cost <= r_current {
                0
            } else {
                (r_cost + r_income - r_current - 1) / r_income
            };
            wait = wait.max(r_wait);
        }
        // Time until bot is completed = wait + 1
        // Will this be too late to make any difference?
        wait += 1;
        if node.tick + wait >= num_ticks {
            continue;
        }
        // Wait for the appropriate duration
        let mut current = node.clone();
        apply_ticks(&mut current, wait);
        // Buy bot, but only if it makes sense to do so
        if check_max(bp, &current, b, num_ticks) {
            buy_bot(&mut current, bp, b, num_ticks);
            result.push(current);
        }
    }
    result
}

/// Passes time for the given node, ticking up resources from bot-mining income.
fn apply_ticks(node: &mut Node, num_ticks: u32) {
    node.tick += num_ticks;
    for i in 0..NUM_COST_RESOURCES {
        node.resources[i] += node.bots[i] * num_ticks;
    }
}

/// Determines whether purchasing the given bot type is permitted.
fn check_max(bp: &Blueprint, node: &Node, bot_type: usize, num_ticks: u32) -> bool {
    if bot_type == GEODE {
        return true;
    }
    // Don't exceed income required to buy most expensive bot every turn
    if node.bots[bot_type] >= bp.max[bot_type] {
        return false;
    }
    // Suppose you wanted to buy the most expensive bot every tick. If you've already got enough
    // resource and income to do this, don't get any more bots!
    let rem = num_ticks - node.tick - 1;
    // I don't know why this +1 is required, but otherwise the answer is wrong (as per bp24 test)
    let total_cost = bp.max[bot_type] * (rem + 1);
    let stockpile = node.resources[bot_type] + node.bots[bot_type] * rem;
    stockpile < total_cost
}

/// Purchases the given bot type, updating state.
fn buy_bot(node: &mut Node, bp: &Blueprint, bot_type: usize, num_ticks: u32) {
    //    println!("Tick: {}, Buy bot {}", node.tick, bot_res);
    // Reduce resources by cost
    for i in 0..NUM_COST_RESOURCES {
        node.resources[i] -= bp.bots[bot_type][i];
    }
    // Update bot count; for geodes, just apply the score immediately
    if bot_type == GEODE {
        let val = num_ticks - node.tick;
        node.geodes += val;
    } else {
        node.bots[bot_type] += 1;
    }
}

fn main() -> Result<()> {
    run_solution(&Day19)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_geodes_test_bp1() -> Result<()> {
        let bp = parse_blueprint("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.")?;
        assert_eq!(max_geodes(&bp, 24), 9);
        Ok(())
    }

    #[test]
    fn test_max_geodes_test_bp2() -> Result<()> {
        let bp = parse_blueprint("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.")?;
        assert_eq!(max_geodes(&bp, 24), 12);
        Ok(())
    }

    #[test]
    fn test_max_geodes_bp24() -> Result<()> {
        // This one was off in my initial part 1 attempts...
        let bp = parse_blueprint("Blueprint 24: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 10 clay. Each geode robot costs 2 ore and 11 obsidian.")?;
        assert_eq!(max_geodes(&bp, 24), 14);
        Ok(())
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day19, SolutionPart::One), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day19, SolutionPart::Two), 56 * 62);
    }
}

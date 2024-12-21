extern crate aoc_lib;

use anyhow::Result;

use aoc_lib::common;
use aoc_lib::harness::*;

pub struct Day06;

type Input = Vec<u32>;
type Output = u64;

impl Solution<Input, Output> for Day06 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Lanternfish", 2021, 6)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        common::tokenize_first_line(&resource.as_str()?, ',')
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(run_ticks(input, 80))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        Ok(run_ticks(input, 256))
    }
}

/// Track simulation state
pub struct State {
    /// Fish due to trigger per tick
    ticks: Vec<u64>,
}

/// Run a single tick on the supplied state
fn tick(state: &mut State, tick: usize) {
    let ticked = state.ticks[tick];
    // Ticking fish reset to tick+7
    state.ticks[tick + 7] += ticked;
    // Ticking fish also spawn new fish with tick+9
    state.ticks[tick + 9] += ticked;
}

/// Run the given number of ticks on the supplied input, returning the number of fish in play
fn run_ticks(input: &[u32], ticks: usize) -> u64 {
    // Pre-fill the state vec to be big enough to hold all the ticks we need
    let mut state = State { ticks: vec![0; ticks + 9] };
    for i in input {
        state.ticks[*i as usize] += 1;
    }
    // Run the requested number of simulation ticks
    for i in 0..ticks {
        tick(&mut state, i);
    }
    // Sum everything due to tick on or after the requested tick
    (ticks..ticks + 9).map(|x| state.ticks[x]).sum()
}

fn main() -> Result<()> {
    run_solution(&Day06)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_input() -> Vec<u32> {
        Day06.parse_input(&FileResource::new("input.test", 2021, 6)).unwrap()
    }

    #[test]
    fn test_18_ticks() {
        assert_eq!(run_ticks(&gen_input(), 18), 26);
    }

    #[test]
    fn test_80_ticks() {
        assert_eq!(run_ticks(&gen_input(), 80), 5934);
    }

    #[test]
    fn test_256_ticks() {
        assert_eq!(run_ticks(&gen_input(), 256), 26984457539);
    }
}

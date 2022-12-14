extern crate aoc_lib;

use aoc_lib::common;

const DAY: &str = "2021/06";

/// Track simulation state
struct State {
    /// Fish due to trigger per tick
    ticks: Vec<u64>,
}

/// Read input as a simple list of first ticks
fn parse_input(content: &str) -> Vec<u32> {
    common::tokenize_first_line(content, ',').unwrap()
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
    let mut state = State {
        ticks: vec![0; ticks + 9],
    };
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

fn part1(input: &[u32]) -> u64 {
    run_ticks(input, 80)
}

fn part2(input: &[u32]) -> u64 {
    run_ticks(input, 256)
}

fn main() {
    let input = parse_input(&common::input_as_str(DAY, "input"));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_input() -> Vec<u32> {
        parse_input(&common::input_as_str(DAY, "input.test"))
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

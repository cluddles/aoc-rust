extern crate aoc_lib;

use aoc_lib::harness::*;

#[derive(Default)]
struct State {
    x: isize,
    cycle: usize,
    pending_add: Option<isize>,
    line_index: usize,
    signal_sum: isize,
}

impl State {
    fn new() -> State { State { x: 1, ..Default::default() } }
}

fn cpu_tick(state: &mut State, program: &Input) {
    state.cycle += 1;
    let cycle = state.cycle as isize;
    if (cycle - 20) % 40 == 0 {
        state.signal_sum += cycle * state.x;
        // println!("{} * {}", cycle, state.x);
    }

    if let Some(v) = state.pending_add {
        state.x += v;
        state.pending_add = None;

    } else {
        let line = if state.line_index < program.len() { &program[state.line_index] } else { "noop" };
        state.line_index += 1;

        // Start
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "addx" {
            state.pending_add = Some(parts[1].parse::<isize>().unwrap());
        }
    }
}

fn run_program(program: &Input) -> isize {
    let mut state = State::new();
    loop {
        cpu_tick(&mut state, program);
        if state.cycle == 220 { return state.signal_sum; }
    }
}

fn render_tick(state: &mut State, output: &mut [String]) {
    let tick = state.cycle as isize;
    let col = tick % 40;
    let row = (tick / 40) as usize;
    output[row].push(if (state.x - col).abs() <= 1 { '#' } else { '.' });
}

fn run_and_render(program: &Input) -> String {
    let mut state = State::new();
    let mut result = vec!["".to_string(); 6];
    loop {
        render_tick(&mut state, &mut result);
        cpu_tick(&mut state, program);
        if state.cycle == 240 {
            return format!("\n{}", result.join("\n"));
        }
    }
}

struct Year2022Day10;
type Input = Vec<String>;
type Output = String;
impl Solution<Input, Output> for Year2022Day10 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Cathode-Ray Tube", 2022, 10)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Input {
        resource.as_str_lines()
    }

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        Ok(run_program(input).to_string())
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        Ok(run_and_render(input))
    }
}

fn main() {
    run_solution(&Year2022Day10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre() {
        let program = vec!["noop", "addx 3", "addx -5"].iter().map(|x| x.to_string()).collect();
        let mut state = State::new();
        cpu_tick(&mut state, &program);
        assert_eq!(state.x, 1);
        cpu_tick(&mut state, &program);
        assert_eq!(state.x, 1);
        cpu_tick(&mut state, &program);
        assert_eq!(state.x, 4);
        cpu_tick(&mut state, &program);
        assert_eq!(state.x, 4);
        cpu_tick(&mut state, &program);
        assert_eq!(state.x, -1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Year2022Day10, SolutionPart::One), "13140".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Year2022Day10, SolutionPart::Two), vec![
            "\n##..##..##..##..##..##..##..##..##..##..".to_string(),
            "\n###...###...###...###...###...###...###.".to_string(),
            "\n####....####....####....####....####....".to_string(),
            "\n#####.....#####.....#####.....#####.....".to_string(),
            "\n######......######......######......####".to_string(),
            "\n#######.......#######.......#######.....".to_string(),
        ].join(""));
    }
}

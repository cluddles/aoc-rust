extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::harness::*;
use std::collections::HashMap;

pub struct Day21;

type Input = HashMap<String, Op>;
type Output = i64;

#[derive(Clone)]
enum Op {
    Val(i64),
    Math(char, String, String),
}

const ROOT: &str = "root";
const HUMN: &str = "humn";

impl Solution<Input, Output> for Day21 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Monkey Math", 2022, 21)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        Ok(parse_monkeys(resource.as_str_lines()?))
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        Ok(eval(input, ROOT) as Output)
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let mut input = input.clone();
        let root = match &input[ROOT] {
            Op::Math(_, l, r) => [l.to_string(), r.to_string()],
            _ => panic!("Root is not a math op"),
        };

        let init_result = [eval(&input, &root[0]), eval(&input, &root[1])];

        // println!("{:?}", init_result);
        // println!("{:?}", eval_with_humn(&mut input, &root[0], 3560324848168));

        // Work out which side of the equation is changing
        let min = i64::MIN / 65536;
        let test_result = [
            eval_with_humn(&mut input, &root[0], min).1,
            eval_with_humn(&mut input, &root[1], min).1,
        ];
        let eval_pos = usize::from(test_result[0] == init_result[0]);
        let const_pos = 1 - eval_pos;

        // Do a binary search:
        //
        //         <- between.0 -------------------- between.1 ->
        //
        // TEST
        //         <- r1.0 ------- r2.0 ->
        // WITHIN?
        //         <------ between ------>
        //         <-- test -->
        //         etc...
        // NOT?
        //                                <------ between ------>
        //                                <-- test -->

        // Start with "ludicrous" values and home in
        // This may not be the greatest implementation - I was winging it.
        let mut r1 = (min, test_result[eval_pos]);
        let mut r2 = eval_with_humn(&mut input, &root[eval_pos], i64::MAX / 65536);
        let mut between = (r1.0, r2.0);
        let mut d1 = r1.1 - init_result[const_pos];
        let mut d2 = r2.1 - init_result[const_pos];
        loop {
            if d1 == 0.0 {
                return Ok(r1.0);
            }
            if d2 == 0.0 {
                return Ok(r2.0);
            }
            if d1.signum() != d2.signum() {
                // Result lies between previous test positions
                between = (r1.0, r2.0);
                r2 = eval_with_humn(&mut input, &root[eval_pos], r1.0 + (r2.0 - r1.0) / 2);
                d2 = r2.1 - init_result[const_pos];
            } else {
                // Result lies outside the previous test range
                between = (r2.0, between.1);
                let tmp = r2;
                r2 = eval_with_humn(&mut input, &root[eval_pos], r2.0 + (between.1 - r2.0) / 2);
                r1 = tmp;
                d1 = d2;
                d2 = r2.1 - init_result[const_pos];
            }
            //println!("{:?}, {:?}", r1, r2);
            //std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

/// Convert all lines of input into a map of Ops ("Monkeys")
fn parse_monkeys(lines: Vec<String>) -> HashMap<String, Op> {
    lines.iter().map(|l| parse_monkey(l)).collect()
}

/// Parse a single line
fn parse_monkey(line: &str) -> (String, Op) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    (parts[0].replace(':', ""), parse_op(&parts[1..]))
}

/// Parse an Op, which is either a literal value or some simple maths.
fn parse_op(parts: &[&str]) -> Op {
    if parts.len() == 1 {
        return Op::Val(parts[0].parse().expect("must be a valid int"));
    }
    let left = parts[0].to_string();
    let right = parts[2].to_string();
    let op = parts[1].chars().next().expect("need an opcode");
    Op::Math(op, left, right)
}

/// Completely eval the given node.
///
/// Most of this puzzle works fine with integer math, BUT it has the annoying side-effect of the
/// solution being non-unique (and of course I found the "wrong" one first).
fn eval(input: &Input, node: &str) -> f64 {
    match &input[node] {
        Op::Val(x) => *x as f64,
        Op::Math(op, l, r) => match op {
            '+' => eval(input, l) + eval(input, r),
            '-' => eval(input, l) - eval(input, r),
            '*' => eval(input, l) * eval(input, r),
            '/' => eval(input, l) / eval(input, r),
            _ => panic!("Unrecognised operator: {}", op),
        },
    }
}

/// Completely eval the given node, AFTER setting the "humn" node value.
///
/// Returns (humn value, eval result)
fn eval_with_humn(input: &mut Input, node: &str, humn: i64) -> (i64, f64) {
    input.insert(HUMN.to_string(), Op::Val(humn));
    (humn, eval(input, node))
}

fn main() -> Result<()> {
    run_solution(&Day21)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day21, SolutionPart::One), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day21, SolutionPart::Two), 301);
    }
}

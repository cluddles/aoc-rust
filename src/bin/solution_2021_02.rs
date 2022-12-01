extern crate aoc;
use aoc::shared;

struct Instruction {
    name: String,
    amount: u32,
}

/// Parse a single instruction from a line of text
fn parse_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.trim().split(' ').collect();
    return Instruction {
        name: parts[0].to_string(),
        amount: parts[1].parse().unwrap(),
    };
}

/// Convert input text to Vec of instructions
fn generate_instructions(content: &String) -> Vec<Instruction> {
    content.split('\n').map(parse_instruction).collect()
}

/// Horizontal * depth after running instructions
fn part1(instructions: &Vec<Instruction>) -> u32 {
    let mut horiz: u32 = 0;
    let mut depth: u32 = 0;
    for instruction in instructions {
        match instruction.name.as_str() {
            "forward" => horiz += instruction.amount,
            "up" => depth -= instruction.amount,
            "down" => depth += instruction.amount,
            _ => (),
        }
    }
    horiz * depth
}

/// With additional "aim"
fn part2(instructions: &Vec<Instruction>) -> u32 {
    let mut horiz: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    for instruction in instructions {
        match instruction.name.as_str() {
            "forward" => {
                horiz += instruction.amount;
                depth += aim * instruction.amount
            }
            "up" => aim -= instruction.amount,
            "down" => aim += instruction.amount,
            _ => (),
        }
    }
    horiz * depth
}

fn main() {
    let content = shared::read_resource("2021/02/input");
    let instructions = generate_instructions(&content);
    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let content = shared::read_resource("2021/02/input.test");
        let instructions = generate_instructions(&content);
        assert_eq!(part1(&instructions), 150);
    }

    #[test]
    fn test_part2() {
        let content = shared::read_resource("2021/02/input.test");
        let instructions = generate_instructions(&content);
        assert_eq!(part2(&instructions), 900);
    }
}

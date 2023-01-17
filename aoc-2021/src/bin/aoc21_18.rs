extern crate aoc_lib;

use anyhow::Result;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use aoc_lib::harness::*;

pub struct Day18;

type Input = Vec<Snailfish>;
type Output = u64;

impl Solution<Input, Output> for Day18 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Snailfish", 2021, 18)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        Ok(resource.as_str_lines()?.iter().map(|x| Snailfish::new(x)).collect())
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        let mut result = input.iter().next().unwrap().clone();
        for sf in input.iter().skip(1) {
            result = result.add(sf);
        }
        Ok(result.magnitude())
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let mut max = 0;
        for i in 0..input.len()-1 {
            for j in i+1..input.len() {
                if i == j {
                    continue;
                }
                max = max.max(input[i].add(&input[j]).magnitude());
                max = max.max(input[j].add(&input[i]).magnitude());
            }
        }
        Ok(max)
    }
}

#[derive(Debug, Copy, Clone)]
enum Symbol {
    Value(u64),
    Sigil(char),
}

#[derive(Debug, Clone)]
struct Snailfish {
    symbols: VecDeque<Symbol>,
}

impl Snailfish {
    fn new(s: &str) -> Self {
        Self { symbols: Self::str_to_symbols(s) }
    }

    /// Break string up into symbols: values (1, 23, 456, etc) or sigils ('[', ']', ',')
    fn str_to_symbols(s: &str) -> VecDeque<Symbol> {
        let mut last = 0;
        let mut result: VecDeque<Symbol> = VecDeque::new();
        for (i, c) in s.chars().enumerate() {
            if c == '[' || c == ',' || c == ']' {
                if i != last {
                    result.push_back(Symbol::Value(s[last..i].parse::<u64>().unwrap()));
                }
                result.push_back(Symbol::Sigil(c));
                last = i + 1;
            }
        }
        if last < s.len() {
            result.push_back(Symbol::Value(s[last..].parse::<u64>().unwrap()));
        }
        result
    }

    fn magnitude(&self) -> u64 {
        // Factor at current depth, position
        let mut f = vec![1];
        let mut result = 0;
        for c in &self.symbols {
            match c {
                Symbol::Value(v) => result += v * f.last().unwrap(),
                Symbol::Sigil(c) => match c {
                    '[' => {
                        f.push(f.last().unwrap() * 3);
                    }
                    ',' => {
                        f.pop();
                        f.push(f.last().unwrap() * 2);
                    }
                    ']' => {
                        f.pop();
                    }
                    _ => panic!("Unrecognised sigil: {}", c),
                },
            };
        }
        result
    }

    fn add(&self, other: &Self) -> Self {
        let mut s = self.symbols.to_owned();
        s.push_front(Symbol::Sigil('['));
        s.push_back(Symbol::Sigil(','));
        other.symbols.iter().for_each(|x| s.push_back(*x));
        s.push_back(Symbol::Sigil(']'));
        let mut result = Self { symbols: s };
        result.reduce();
        result
    }

    fn reduce(&mut self) {
        loop {
            if self.explode_once() {
                continue;
            }
            if self.split_once() {
                continue;
            }
            break;
        }
    }

    fn explode_once(&mut self) -> bool {
        let mut pos = None;
        let mut depth = 0;
        for i in 0..self.symbols.len() - 3 {
            match self.symbols[i] {
                Symbol::Sigil('[') => depth += 1,
                Symbol::Sigil(']') => depth -= 1,
                Symbol::Value(_) => {
                    if depth > 4
                        && matches!(self.symbols[i + 1], Symbol::Sigil(','))
                        && matches!(self.symbols[i + 2], Symbol::Value(_))
                    {
                        pos = Some(i);
                        break;
                    }
                }
                _ => (),
            }
        }
        if let Some(pos) = pos {
            if let Symbol::Value(v1) = self.symbols[pos] {
                if let Symbol::Value(v2) = self.symbols[pos + 2] {
                    self.push_val(v1, pos, -1);
                    self.push_val(v2, pos + 2, 1);
                    self.symbols.remove(pos);
                    self.symbols.remove(pos);
                    self.symbols[pos] = Symbol::Value(0);
                    self.symbols.remove(pos + 1);
                    self.symbols.remove(pos - 1);
                    return true;
                }
            }
        }
        false
    }

    fn push_val(&mut self, val: u64, pos: usize, dir: isize) {
        let mut pos = pos as isize;
        loop {
            pos += dir;
            if pos < 0 {
                return;
            }
            let upos = pos as usize;
            if upos >= self.symbols.len() {
                return;
            }
            if let Symbol::Value(v) = self.symbols[upos] {
                self.symbols[upos] = Symbol::Value(v + val);
                return;
            }
        }
    }

    fn split_once(&mut self) -> bool {
        let to_split = self
            .symbols
            .iter()
            .enumerate()
            .find(|(_, x)| matches!(x, Symbol::Value(x) if x >= &10));
        if let Some((pos, Symbol::Value(v))) = to_split {
            let v = *v;
            self.symbols.remove(pos);
            self.symbols.insert(pos, Symbol::Sigil('['));
            self.symbols.insert(pos + 1, Symbol::Value(v / 2));
            self.symbols.insert(pos + 2, Symbol::Sigil(','));
            self.symbols.insert(pos + 3, Symbol::Value(v / 2 + v.rem_euclid(2)));
            self.symbols.insert(pos + 4, Symbol::Sigil(']'));
            return true;
        }
        false
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for s in &self.symbols {
            match s {
                Symbol::Value(v) => write!(f, "{}", v)?,
                Symbol::Sigil(c) => write!(f, "{}", c)?,
            };
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    run_solution(&Day18)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sf(s: &str) -> Snailfish {
        Snailfish::new(s)
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(sf("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(sf("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
        assert_eq!(sf("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(sf("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(sf("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
        assert_eq!(sf("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
    }

    #[test]
    fn test_explode() {
        let mut sf = sf("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        assert!(sf.explode_once());
        assert_eq!(sf.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    }

    #[test]
    fn test_split() {
        let mut sf = sf("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert!(sf.split_once());
        assert_eq!(sf.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    }

    #[test]
    fn test_add() {
        assert_eq!(sf("[1,2]").add(&sf("[[3,4],5]")).to_string(), "[[1,2],[[3,4],5]]");
        assert_eq!(
            sf("[[[[4,3],4],4],[7,[[8,4],9]]]").add(&sf("[1,1]")).to_string(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day18, SolutionPart::One), 4140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day18, SolutionPart::Two), 3993);
    }
}

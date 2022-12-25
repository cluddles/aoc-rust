extern crate aoc_lib;

use aoc_lib::harness::*;

pub struct Day24;

type Input = ();
type Output = u32;

// Pre-compute map to work out when tiles are blizzard free/blocked
// (times will loop for x- and y-axes)
// Cost to move into tile can include delay for it to be blizzard free.
// Remember that you cannot wait in a tile that's about to get blizzarded. Presumably model this
// by disallowing any moves that would blizzard your current tile before the target was clear.
// Heuristic is just the distance to goal, since that will never underestimate.
// Moves are up, down, left, right only

impl Solution<Input, Output> for Day24 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Blizzard Basin", 2022, 24)
    }

    fn parse_input(&self, resource: &dyn Resource) -> DynResult<Input> {
        todo!()
    }

    fn solve_part1(&self, input: &Input) -> SolutionResult<Output> {
        todo!()
    }

    fn solve_part2(&self, input: &Input) -> SolutionResult<Output> {
        todo!()
    }
}

fn main() -> DynResult<()> {
    run_solution(&Day24)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day24, SolutionPart::One), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day24, SolutionPart::Two), 0);
    }
}

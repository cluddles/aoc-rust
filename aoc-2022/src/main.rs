use aoc_2022::day01::Day01;
use aoc_2022::day11::Day11;
use aoc_2022::day12::Day12;
use aoc_2022::day13::Day13;
use aoc_2022::day14::Day14;
use aoc_lib::harness::*;

fn main() -> DynResult<()> {
    run_solution(&Day01)?;
    run_solution(&Day11)?;
    run_solution(&Day12)?;
    run_solution(&Day13)?;
    run_solution(&Day14)?;
    Ok(())
}

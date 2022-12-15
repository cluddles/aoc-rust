use std::time::SystemTime;
use aoc_2021::day01::Day01;
use aoc_2021::day02::Day02;
use aoc_2021::day03::Day03;
use aoc_2021::day04::Day04;
use aoc_2021::day05::Day05;
use aoc_2021::day06::Day06;
use aoc_2021::day12::Day12;
use aoc_lib::harness::*;

fn main() -> DynResult<()> {
    let time = SystemTime::now();

    run_solution(&Day01)?;
    run_solution(&Day02)?;
    run_solution(&Day03)?;
    run_solution(&Day04)?;
    run_solution(&Day05)?;
    run_solution(&Day06)?;
    run_solution(&Day12)?;

    println!("\nTotal time: {:?}", time.elapsed()?);
    Ok(())
}

use std::time::SystemTime;
use aoc_2022::day01::Day01;
use aoc_2022::day02::Day02;
use aoc_2022::day03::Day03;
use aoc_2022::day04::Day04;
use aoc_2022::day05::Day05;
use aoc_2022::day06::Day06;
use aoc_2022::day07::Day07;
use aoc_2022::day08::Day08;
use aoc_2022::day09::Day09;
use aoc_2022::day10::Day10;
use aoc_2022::day11::Day11;
use aoc_2022::day12::Day12;
use aoc_2022::day13::Day13;
use aoc_2022::day14::Day14;
use aoc_lib::harness::*;

fn main() -> DynResult<()> {
    let time = SystemTime::now();

    run_solution(&Day01)?;
    run_solution(&Day02)?;
    run_solution(&Day03)?;
    run_solution(&Day04)?;
    run_solution(&Day05)?;
    run_solution(&Day06)?;
    run_solution(&Day07)?;
    run_solution(&Day08)?;
    run_solution(&Day09)?;
    run_solution(&Day10)?;
    run_solution(&Day11)?;
    run_solution(&Day12)?;
    run_solution(&Day13)?;
    run_solution(&Day14)?;

    println!("\nTotal time: {:?}", time.elapsed()?);
    Ok(())
}

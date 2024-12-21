use crate::common::*;
use crate::data::Grid;
use anyhow::{Context, Result};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Solution for a day's puzzle
pub trait Solution<I, O> {
    /// Solution metadata
    fn info(&self) -> SolutionInfo;
    /// Get puzzle input from given resource
    fn parse_input(&self, resource: &dyn Resource) -> Result<I>;
    /// Solution to puzzle part 1
    fn solve_part1(&self, input: &I) -> Result<O>;
    /// Solution to puzzle part 2
    fn solve_part2(&self, input: &I) -> Result<O>;
}

/// Run the solution for a day and output part 1 and 2 results
pub fn run_solution<S: Solution<I, O>, I, O: Display>(solution: &S) -> Result<()> {
    // Get info from solution
    let info = solution.info();
    println!("\n--- [{}] Day {}: {} ---", info.year, info.day, info.title);
    // Create resource using year/day from info
    let resource = FileResource::new("", info.year, info.day);
    // Call proc on solution to parse input into relevant part1/2 input type
    let time = SystemTime::now();
    println!("\nParse input");
    let input = solution.parse_input(&resource)?;
    println!("[{:?}]", time.elapsed()?);
    // Solve part 1
    let time = SystemTime::now();
    println!("\nPart 1:");
    println!("{}", solution.solve_part1(&input)?);
    println!("[{:?}]", time.elapsed()?);
    // Solve part 2
    let time = SystemTime::now();
    println!("\nPart 2:");
    println!("{}", solution.solve_part2(&input)?);
    println!("[{:?}]", time.elapsed()?);

    Ok(())
}

pub enum SolutionPart {
    One,
    Two,
}

/// Test-run solution on default test input
pub fn test_solution<S: Solution<I, O>, I, O>(solution: &S, part: SolutionPart) -> O {
    let info = solution.info();
    test_solution_inner(solution, part, &FileResource::new("test", info.year, info.day))
}

/// Test-run solution on specific (presumably non-default) test input
pub fn test_ext<S: Solution<I, O>, I, O>(
    solution: &S,
    part: SolutionPart,
    filename: &'static str,
) -> O {
    let info = solution.info();
    test_solution_inner(solution, part, &FileResource::new(filename, info.year, info.day))
}

/// Test-run solution on inline input text
pub fn test_inline<S: Solution<I, O>, I, O>(
    solution: &S,
    part: SolutionPart,
    text: &'static str,
) -> O {
    test_solution_inner(solution, part, &InlineResource { text })
}

fn test_solution_inner<S: Solution<I, O>, I, O>(
    solution: &S,
    part: SolutionPart,
    resource: &dyn Resource,
) -> O {
    // We're "just" testing, so panics are probably okay here
    let input = solution.parse_input(resource).unwrap();
    match part {
        SolutionPart::One => solution.solve_part1(&input).unwrap(),
        SolutionPart::Two => solution.solve_part2(&input).unwrap(),
    }
}

/// Resource to pull solution input from
pub trait Resource {
    /// Read string from resource
    fn as_str(&self) -> Result<String>;

    /// Read u8 vec from resource
    fn as_u8(&self) -> Result<Vec<u8>>;

    /// Read string lines from resource. Filters out empty lines.
    fn as_str_lines(&self) -> Result<Vec<String>> {
        let lines = self.as_str()?;
        Ok(lines.split('\n').filter(|x| !x.is_empty()).map(|x| x.to_owned()).collect())
    }

    /// Read grid of u8 from resource
    fn as_u8_grid(&self, converter: fn(u8) -> u8) -> Result<Grid<u8>> {
        let input = self.as_u8()?;
        let w = input.iter().enumerate().find(|(_, &x)| x < 32).unwrap_or((input.len(), &0)).0;
        let grid_raw = input.iter().filter(|&&x| x >= 32).map(|&x| converter(x)).collect();
        Ok(Grid::from_1d(grid_raw, w))
    }
}

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn resource_path(suffix: &str, year: u32, day: u8) -> Result<PathBuf> {
    // Look in ../aoc-yyyy/resource/dayxx.suffix
    let mut path = Path::new(MANIFEST_DIR)
        .join("..")
        .join(format!("aoc-{}", year))
        .join("resource")
        .join(format!("day{:02}.{}", day, suffix));
    if !path.exists() {
        // Look in ../aoc-secret/yyyy/dayxx.suffix
        path = Path::new(MANIFEST_DIR)
            .join("..")
            .join("aoc-secret")
            .join(format!("{}", year))
            .join(format!("day{:02}.{}", day, suffix));
    }
    path.canonicalize().with_context(|| format!("Failed to canonicalize path: {}", path.display()))
}

fn file_res_as_str(filename: &str, year: u32, day: u8) -> Result<String> {
    let path = resource_path(filename, year, day)?;
    std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read from {}", path.display()))
}

fn file_res_as_u8(filename: &str, year: u32, day: u8) -> Result<Vec<u8>> {
    let path = resource_path(filename, year, day)?;
    std::fs::read(&path).with_context(|| format!("Failed to read from {}", path.display()))
}

/// Resource corresponding to a file on disk
///
/// Puzzle inputs are expected to be named according to the day. Suffix can be used to differentiate
/// between real and test inputs, multiple test inputs, etc
pub struct FileResource {
    suffix: &'static str,
    year: u32,
    day: u8,
}

impl FileResource {
    pub fn new(suffix: &'static str, year: u32, day: u8) -> Self {
        Self { suffix, year, day }
    }
}

impl Resource for FileResource {
    fn as_str(&self) -> Result<String> {
        file_res_as_str(self.suffix, self.year, self.day)
    }

    fn as_u8(&self) -> Result<Vec<u8>> {
        file_res_as_u8(self.suffix, self.year, self.day)
    }
}

/// Resource corresponding to inline text
pub struct InlineResource {
    text: &'static str,
}

impl InlineResource {
    pub fn new(text: &'static str) -> Self {
        Self { text }
    }
}

impl Resource for InlineResource {
    fn as_str(&self) -> Result<String> {
        Ok(self.text.to_string())
    }

    fn as_u8(&self) -> Result<Vec<u8>> {
        Ok(str_to_u8(self.text))
    }
}

/// Simple solution metadata: title, date
pub struct SolutionInfo {
    title: &'static str,
    year: u32,
    day: u8,
}

impl SolutionInfo {
    pub fn new(title: &'static str, year: u32, day: u8) -> SolutionInfo {
        SolutionInfo { title, year, day }
    }
}

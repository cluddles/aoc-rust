use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::time::SystemTime;
use crate::common;
use crate::data::Grid;

pub type DynResult<O> = Result<O, Box<dyn Error>>;
pub type SolutionResult<O> = DynResult<O>;

/// Error that displays "something"
#[derive(Debug, Clone)]
pub struct SimpleError<T: Debug + Display + Clone> {
    to_display: T,
}
impl <T: Debug + Display + Clone> SimpleError<T> {
    pub fn new(v: T) -> SimpleError<T> { SimpleError { to_display: v } }
    pub fn new_dyn(v: T) -> Box<SimpleError<T>> { Self::new(v).into() }
}

impl<T: Debug + Display + Clone> Error for SimpleError<T> {}

impl<T: Debug + Display + Clone> Display for SimpleError<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.to_display)
    }
}

/// Solution for a day's puzzle
pub trait Solution<I, O> {
    fn info(&self) -> SolutionInfo;
    fn parse_input(&self, resource: &dyn Resource) -> DynResult<I>;
    fn solve_part1(&self, input: &I) -> SolutionResult<O>;
    fn solve_part2(&self, input: &I) -> SolutionResult<O>;
}

/// Run the solution for a day and output part 1 and 2 results
pub fn run_solution<S: Solution<I, O>, I, O: Display> (solution: &S) -> DynResult<()> {
    // Get info from solution
    let info = solution.info();
    println!();
    println!("--- [{}] Day {}: {} ---", info.year, info.day, info.title);
    // Create resource using year/day from info
    let resource = FileResource { filename: "input", year: info.year, day: info.day };
    // Call proc on solution to parse input into relevant part1/2 input type
    let input = solution.parse_input(&resource)?;

    let time = SystemTime::now();
    println!();
    println!("Part 1: {}", solution.solve_part1(&input)?);
    println!("[in {:?}]", time.elapsed()?);

    let time = SystemTime::now();
    println!();
    println!("Part 2: {}", solution.solve_part2(&input)?);
    println!("[in {:?}]", time.elapsed()?);

    Ok(())
}

pub enum SolutionPart {
    One,
    Two,
}

/// Test-run solution on default test input
pub fn test_solution<S: Solution<I, O>, I, O> (solution: &S, part: SolutionPart) -> O {
    let info = solution.info();
    test_solution_inner(solution, part, &FileResource { filename: "input.test", year: info.year, day: info.day })
}

/// Test-run solution on specific (presumably non-default) test input
pub fn test_solution_ext<S: Solution<I, O>, I, O> (solution: &S, part: SolutionPart, filename: &'static str) -> O {
    let info = solution.info();
    test_solution_inner(solution, part, &FileResource { filename, year: info.year, day: info.day })
}

/// Test-run solution on inline input text
pub fn test_solution_inline<S: Solution<I, O>, I, O> (solution: &S, part: SolutionPart, text: &'static str) -> O {
    test_solution_inner(solution, part, &InlineResource { text })
}

fn test_solution_inner<S: Solution<I, O>, I, O> (solution: &S, part: SolutionPart, resource: &dyn Resource) -> O {
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
    fn as_str(&self) -> String;

    /// Read u8 vec from resource
    fn as_u8(&self) -> Vec<u8>;

    /// Read string lines from resource
    fn as_str_lines(&self) -> Vec<String> {
        let lines = self.as_str();
        lines.split('\n').filter(|x| !x.is_empty()).map(|x| x.to_owned()).collect()
    }

    /// Read grid of u8 from resource
    fn as_u8_grid(&self, converter: fn(u8) -> u8) -> Grid<u8> {
        let input = self.as_u8();
        let w = input.iter().enumerate().find(|(_, &x)| x < 32).unwrap_or((input.len(), &0)).0;
        let grid_raw = input.iter().filter(|&&x| x >= 32).map(|&x| converter(x)).collect();
        Grid::from_1d(grid_raw, w)
    }
}

/// Resource corresponding to a file on disk
pub struct FileResource {
    filename: &'static str,
    year: u32,
    day: u8,
}

impl Resource for FileResource {
    fn as_str(&self) -> String {
        common::input_as_str(&format!("{}/{:02}", self.year, self.day), self.filename)
    }

    fn as_u8(&self) -> Vec<u8> {
        common::input_as_u8(&format!("{}/{:02}", self.year, self.day), self.filename)
    }
}

/// Resource corresponding to inline text
pub struct InlineResource {
    text: &'static str,
}

impl Resource for InlineResource {
    fn as_str(&self) -> String {
        self.text.to_string()
    }

    fn as_u8(&self) -> Vec<u8> {
        common::str_to_u8(self.text)
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


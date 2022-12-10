use crate::common;

pub fn run_solution<S: Solution<I>, I> (solution: &S) {
    // Get info from solution
    let info = solution.info();
    println!("--- [{}] Day {}: {} ---", info.year, info.day, info.title);
    // Create resource using year/day from info
    let resource = FileResource { filename: "input", year: info.year, day: info.day };
    // Call proc on solution to parse input into relevant part1/2 input type
    let input = solution.parse_input(&resource);
    // Call part 1
    if let Ok(v) = solution.solve_part1(&input) { println!("Part 1: {}", v) }
    // Call part 2
    if let Ok(v) = solution.solve_part2(&input) { println!("Part 1: {}", v) }
}

// TODO this needs refactoring lol
pub fn test_exec_part1<S: Solution<I>, I> (solution: &S) -> String {
    let info = solution.info();
    let resource = FileResource { filename: "input.test", year: info.year, day: info.day };
    let input = solution.parse_input(&resource);
    match solution.solve_part1(&input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}

pub fn test_exec_part2<S: Solution<I>, I> (solution: &S) -> String {
    let info = solution.info();
    let resource = FileResource { filename: "input.test", year: info.year, day: info.day };
    let input = solution.parse_input(&resource);
    match solution.solve_part2(&input) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}

pub trait Resource {
    // Given year, day (and maybe extra test functionality?)
    // input_as_str, input_as_u8
    fn as_str(&self) -> String;
    fn as_u8(&self) -> Vec<u8>;
}

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

pub type SolutionResult = Result<String, &'static str>;

pub trait Solution<T> {
    fn info(&self) -> SolutionInfo;

    fn parse_input(&self, resource: &dyn Resource) -> T;

    fn solve_part1(&self, input: &T) -> SolutionResult;
    fn solve_part2(&self, input: &T) -> SolutionResult;
}

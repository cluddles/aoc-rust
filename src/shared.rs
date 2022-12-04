use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

const RESOURCE_PREFIX: &str = "resource/";

/// Read resource file as String.
pub fn read_resource(f: &str) -> String {
    // Create a path to the desired file
    let full_loc = format!("{}{}", RESOURCE_PREFIX, f);
    let path = Path::new(&full_loc);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }

    s
}

/// Convenience function to read resource for a particular day.
pub fn read_res_day(day: &str, filename: &str) -> String {
    read_resource(&format!("{}/{}", day, filename))
}

/// Split string on newlines, optionally keeping empty lines.
pub fn split_lines(content: &str, keep_empty: bool) -> Vec<&str> {
    content
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| keep_empty || !x.is_empty())
        .collect()
}

/// Parse a value, panicking on error (without relying on Debug)
fn parse<T: FromStr>(val: &str) -> T {
    match val.trim().parse::<T>() {
        Ok(v) => v,
        Err(_) => panic!("Could not parse '{}'", val),
    }
}

/// Split text on given delim, converting tokens with parse()
///
/// Empty tokens will be ignored.
pub fn tokenize<T: FromStr>(text: &str, delim: char) -> Vec<T> {
    text.split(delim)
        .filter(|x| !x.is_empty())
        .map(parse)
        .collect()
}

/// Split the first line of given text, converting tokens with parse()
pub fn tokenize_first_line<T: FromStr>(content: &str, delim: char) -> Vec<T> {
    tokenize(split_lines(content, true).first().unwrap(), delim)
}

/// Basic 2d point
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

/// Basic 2d grid
pub struct Grid<T> {
    dim: Point2<usize>,
    vals: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Create 2d grid
    pub fn new(vals: Vec<Vec<T>>) -> Grid<T> {
        Grid {
            dim: Point2 {
                x: vals[0].len(),
                y: vals.len(),
            },
            vals,
        }
    }
    /// Get single value from grid
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.vals[y][x]
    }
    /// Set single value in grid
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.vals[y][x] = val;
    }
    /// Get grid dimensions
    pub fn dim(&self) -> &Point2<usize> {
        &self.dim
    }
}

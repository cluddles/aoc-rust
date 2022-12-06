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
    std::fs::read_to_string(&format!("{}{}/{}", RESOURCE_PREFIX, day, filename)).unwrap()
}

/// Convenience function to read resource for a particular day, as Vec of u8.
pub fn read_res_day_u8(day: &str, filename: &str) -> Vec<u8> {
    std::fs::read(&format!("{}{}/{}", RESOURCE_PREFIX, day, filename)).unwrap()
}

/// Split string on newlines, optionally keeping empty lines.
fn split_lines_ext(content: &str, keep_empty: bool) -> Vec<&str> {
    content
        .split('\n')
        .filter(|x| keep_empty || !x.is_empty())
        .collect()
}

/// Split string on newlines, discarding empty lines.
pub fn split_lines(content: &str) -> Vec<&str> {
    split_lines_ext(content, false)
}

/// Split string on newlines, keeping empty lines.
pub fn split_lines_keep_empty(content: &str) -> Vec<&str> {
    split_lines_ext(content, true)
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
    tokenize(split_lines(content).first().unwrap(), delim)
}

/// Basic 2d point
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

/// Basic 2d grid
pub struct Grid<T: Clone> {
    dim: Point2<usize>,
    vals: Vec<T>,
}

impl<T: Clone> Grid<T> {

    /// Create grid filled with val
    pub fn new(val: T, x: usize, y: usize) -> Grid<T> {
        Grid {
            dim: Point2 { x, y },
            vals: vec![val; x * y],
        }
    }

    /// Create grid, copying values from source
    pub fn from_2d(source: &Vec<Vec<T>>) -> Grid<T> {
        let dim = Point2 {
            x: source[0].len(),
            y: source.len(),
        };
        let vals = source
            .iter()
            .flat_map(|x| {
                if x.len() != dim.x {
                    panic!("row lengths vary")
                }
                x.to_vec()
            })
            .collect();
        Grid { vals, dim }
    }

    /// Get single value from grid
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.vals[y * self.dim.x + x]
    }

    /// Set single value in grid
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.vals[y * self.dim.x + x] = val;
    }

    /// Get grid dimensions
    pub fn dim(&self) -> &Point2<usize> {
        &self.dim
    }
}

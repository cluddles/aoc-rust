use std::str::FromStr;

const RESOURCE_PREFIX: &str = "resource/";

/// Convenience function to read resource for a particular day.
pub fn input_as_str(day: &str, filename: &str) -> String {
    std::fs::read_to_string(&format!("{}{}/{}", RESOURCE_PREFIX, day, filename)).unwrap()
}

/// Convenience function to read resource for a particular day, as Vec of u8.
pub fn input_as_u8(day: &str, filename: &str) -> Vec<u8> {
    std::fs::read(&format!("{}{}/{}", RESOURCE_PREFIX, day, filename)).unwrap()
}

/// Convert string to Vec of u8
pub fn str_to_u8(text: &str) -> Vec<u8> {
    text.chars().map(|x| x as u8).collect()
}

/// Convert Vec of u8 to string
pub fn u8_to_str(input: &[u8]) -> String {
    input.iter().map(|&x| x as char).collect()
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

/// Basic 2d point.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2<T: Copy> {
    pub x: T,
    pub y: T,
}

impl <T: Copy> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }

    pub fn to_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
}

/// Thin wrapper for a vector, to treat it as a 2d grid of values
#[derive(Debug, Clone)]
pub struct Grid<T: Clone> {
    dim: Point2<usize>,
    vals: Vec<T>,
}

impl<T: Clone> Grid<T> {
    /// Create grid filled with val
    pub fn new(val: T, w: usize, h: usize) -> Self {
        Grid {
            dim: Point2 { x: w, y: h },
            vals: vec![val; w * h],
        }
    }

    /// Create grid, copying values from source
    pub fn from_2d(source: &Vec<Vec<T>>) -> Grid<T> {
        let dim = Point2::new(source[0].len(), source.len());
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

    /// True if given position is an edge
    pub fn is_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == self.dim.x - 1 || y == self.dim.y - 1
    }

    /// True if given position is in bounds
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.dim.x && y < self.dim.y
    }

    /// Provides immutable access to the underlying vector, mostly for iteration
    pub fn vec(&self) -> &Vec<T> {
        &self.vals
    }
}

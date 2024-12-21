use crate::data::Point2;
use std::fmt::{Display, Formatter};

pub type GridPos = Point2<i32>;
pub type GridDim = Point2<i32>;

/// Thin wrapper for a vector, to treat it as a 2d grid of values
#[derive(Debug, Clone)]
pub struct Grid<T> {
    dim: GridDim,
    vals: Vec<T>,
}

impl<T> Grid<T> {
    /// Create grid filled with val
    pub fn new(val: T, w: usize, h: usize) -> Self
    where
        T: Clone,
    {
        Grid { dim: GridDim::new(w as i32, h as i32), vals: vec![val; w * h] }
    }

    /// Create grid filled with default
    pub fn new_default(w: usize, h: usize) -> Self
    where
        T: Default + Clone,
    {
        Grid::new(T::default(), w, h)
    }

    /// Create grid, copying values from source
    pub fn from_2d(source: &Vec<Vec<T>>) -> Grid<T>
    where
        T: Clone,
    {
        let dim = GridDim::new(source[0].len() as i32, source.len() as i32);
        let vals = source
            .iter()
            .flat_map(|x| {
                if x.len() as i32 != dim.x {
                    panic!("row lengths vary")
                }
                x.to_vec()
            })
            .collect();
        Grid { vals, dim }
    }

    /// Create grid, copying values from source
    pub fn from_1d(source: Vec<T>, w: usize) -> Grid<T> {
        let h = source.len() / w;
        let dim = GridDim::new(w as i32, h as i32);
        Grid { vals: source, dim }
    }

    /// Get single value from grid
    pub fn get(&self, x: i32, y: i32) -> &T {
        &self.vals[(y * self.dim.x + x) as usize]
    }

    /// Set single value in grid
    pub fn set(&mut self, x: i32, y: i32, val: T) {
        self.vals[(y * self.dim.x + x) as usize] = val;
    }

    /// Get grid dimensions
    pub fn dim(&self) -> &GridDim {
        &self.dim
    }

    /// True if given position is in bounds
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.dim.x && y < self.dim.y
    }

    /// Provides immutable access to the underlying vector, mostly for iteration
    pub fn vec(&self) -> &Vec<T> {
        &self.vals
    }

    /// Scans (left-right, top-to-bottom) for the first matching cell's position
    pub fn find(&self, predicate: fn(&T) -> bool) -> Option<(GridPos, &T)>
    where
        T: PartialEq,
    {
        let (i, v) = self.vals.iter().enumerate().find(|(_, x)| predicate(x))?;
        Some((GridPos::new(i as i32 % self.dim.x, i as i32 / self.dim.x), v))
    }
}

impl<T: GridChar> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut x = 0;
        for v in self.vals.iter() {
            write!(f, "{}", v.to_grid_char())?;
            x += 1;
            if x == self.dim.x {
                writeln!(f)?;
                x = 0;
            }
        }
        Ok(())
    }
}

pub trait GridChar {
    fn to_grid_char(&self) -> char;
}

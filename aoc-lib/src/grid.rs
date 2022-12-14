use crate::data::Point2;
use std::fmt::{Display, Formatter};

/// Thin wrapper for a vector, to treat it as a 2d grid of values
#[derive(Debug, Clone)]
pub struct Grid<T> {
    dim: Point2<usize>,
    vals: Vec<T>,
}

impl<T> Grid<T> {
    /// Create grid filled with val
    pub fn new(val: T, w: usize, h: usize) -> Self
    where
        T: Clone,
    {
        Grid {
            dim: Point2 { x: w, y: h },
            vals: vec![val; w * h],
        }
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

    /// Create grid, copying values from source
    pub fn from_1d(source: Vec<T>, w: usize) -> Grid<T> {
        let h = source.len() / w;
        let dim = Point2::new(w, h);
        Grid { vals: source, dim }
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

    pub fn find_pos(&self, value: &T) -> Option<Point2<usize>>
    where
        T: PartialEq,
    {
        let (i, _) = self.vals.iter().enumerate().find(|(_, x)| x == &value)?;
        Some(Point2::new(i % self.dim.x, i / self.dim.x))
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

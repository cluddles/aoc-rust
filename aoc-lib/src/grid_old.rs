use crate::data::Point2;
use std::fmt::{Display, Formatter};

pub type GridOldPos = Point2<usize>;

/// Thin wrapper for a vector, to treat it as a 2d grid of values
#[derive(Debug, Clone)]
pub struct GridOld<T> {
    dim: GridOldPos,
    vals: Vec<T>,
}

impl<T> GridOld<T> {
    /// Create grid filled with val
    pub fn new(val: T, w: usize, h: usize) -> Self
    where
        T: Clone,
    {
        GridOld {
            dim: GridOldPos::new(w, h),
            vals: vec![val; w * h],
        }
    }

    /// Create grid filled with default
    pub fn new_default(w: usize, h: usize) -> Self
    where
        T: Default + Clone,
    {
        GridOld::new(T::default(), w, h)
    }

    /// Create grid, copying values from source
    pub fn from_2d(source: &Vec<Vec<T>>) -> GridOld<T>
    where
        T: Clone,
    {
        let dim = GridOldPos::new(source[0].len(), source.len());
        let vals = source
            .iter()
            .flat_map(|x| {
                if x.len() != dim.x {
                    panic!("row lengths vary")
                }
                x.to_vec()
            })
            .collect();
        GridOld { vals, dim }
    }

    /// Create grid, copying values from source
    pub fn from_1d(source: Vec<T>, w: usize) -> GridOld<T> {
        let h = source.len() / w;
        let dim = GridOldPos::new(w, h);
        GridOld { vals: source, dim }
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
    pub fn dim(&self) -> &GridOldPos {
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

    /// Scans (left-right, top-to-bottom) for the first matching cell's position
    pub fn find_pos(&self, value: &T) -> Option<GridOldPos>
    where
        T: PartialEq,
    {
        let (i, _) = self.vals.iter().enumerate().find(|(_, x)| x == &value)?;
        Some(GridOldPos::new(i % self.dim.x, i / self.dim.x))
    }
}

impl<T: GridOldChar> Display for GridOld<T> {
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

pub trait GridOldChar {
    fn to_grid_char(&self) -> char;
}

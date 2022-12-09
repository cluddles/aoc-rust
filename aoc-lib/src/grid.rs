use crate::Point2;

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

impl<T: Default + Clone> Grid<T> {
    /// Create grid filled with default
    pub fn new_default(w: usize, h: usize) -> Self {
        Grid::new(T::default(), w, h)
    }
}

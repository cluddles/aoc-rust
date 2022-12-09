use std::ops::{Add, AddAssign, Sub, SubAssign};
use num_traits::{Num, NumAssign};

/// Basic 2d point.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2<T: Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
    pub fn to_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Default + Copy> Point2<T> {
    pub fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T: Copy + Num> Add for Point2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: Copy + Num> Sub for Point2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: Copy + NumAssign> AddAssign for Point2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Copy + NumAssign> SubAssign for Point2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

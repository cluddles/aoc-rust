use crate::common;
use num_traits::{Num, NumAssign, Signed};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;
use crate::harness::{DynError, DynResult, SimpleError};

/// Basic 2d point.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2<T: Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Point2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
    pub fn to_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Copy> Point2<T> {
    pub fn new_default() -> Self where T: Default {
        Self::new(T::default(), T::default())
    }

    /// Manhattan distance of 0,0 to this point
    pub fn manhattan(&self) -> T where T: Num + Signed {
        self.x.abs() + self.y.abs()
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

impl<T: Copy + FromStr> FromStr for Point2<T> {
    type Err = DynError;
    fn from_str(s: &str) -> DynResult<Self> {
        let p: Vec<T> = common::tokenize(s, ',')?;
        let mut i = p.into_iter();
        Ok(Point2 {
            x: i.next().ok_or_else(|| SimpleError::new_dyn("No x"))?,
            y: i.next().ok_or_else(|| SimpleError::new_dyn("No y"))?,
        })
    }
}

use crate::common;
use anyhow::{anyhow, Error, Result};
use num_traits::{Num, NumAssign, Signed};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

/// Basic 3d point.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> Point3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Point3 { x, y, z }
    }
    pub fn to_tuple(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T: Copy> Point3<T> {
    /// Manhattan distance of 0,0 to this point
    pub fn manhattan(&self) -> T
    where
        T: Num + Signed,
    {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl<T: Copy + Num> Add for Point3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: Copy + Num> Sub for Point3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: Copy + NumAssign> AddAssign for Point3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Copy + NumAssign> SubAssign for Point3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Copy + FromStr> FromStr for Point3<T> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let p: Vec<T> = common::tokenize(s, ',')?;
        let mut i = p.into_iter();
        Ok(Point3 {
            x: i.next().ok_or_else(|| anyhow!("No x"))?,
            y: i.next().ok_or_else(|| anyhow!("No y"))?,
            z: i.next().ok_or_else(|| anyhow!("No z"))?,
        })
    }
}

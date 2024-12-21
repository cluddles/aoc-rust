use anyhow::{bail, Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

impl Dir4 {
    pub const VALUES: [Dir4; 4] = [Dir4::Up, Dir4::Down, Dir4::Left, Dir4::Right];
}

impl FromStr for Dir4 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "U" => Dir4::Up,
            "D" => Dir4::Down,
            "L" => Dir4::Left,
            "R" => Dir4::Right,
            _ => bail!("Unrecognised dir4: {}", s),
        };
        Ok(result)
    }
}

use std::{num::ParseIntError, str::FromStr};

pub type Ordinate = u8;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point {
    pub x: Ordinate,
    pub y: Ordinate,
    pub z: Ordinate,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x
            .cmp(&other.x)
            .then(self.y.cmp(&other.y))
            .then(self.z.cmp(&other.z))
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');

        Ok(Point {
            x: split.next().unwrap().parse::<Ordinate>()?,
            y: split.next().unwrap().parse::<Ordinate>()?,
            z: split.next().unwrap().parse::<Ordinate>()?,
        })
    }
}

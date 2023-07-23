use std::cmp::Ordering;

pub type Ordinate = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: Ordinate,
    pub y: Ordinate,
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut ordering = self.y.cmp(&other.y);
        if ordering == Ordering::Equal {
            ordering = self.x.cmp(&other.x);
        }
        Some(ordering)
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

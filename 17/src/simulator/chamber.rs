use std::{cmp::Ordering, collections::HashSet, fmt::Display};

use crate::coordinate::{Coordinate, Ordinate};

pub struct Chamber {
    pub width: u64,
    pub rocks: HashSet<Coordinate>,
    pub cycle_height: u64,
}

impl Chamber {
    pub fn get_height(&self) -> Ordinate {
        match self.rocks.iter().map(|coor| coor.y).max() {
            Some(height) => height + 1,
            None => 0,
        }
    }

    pub fn get_adjusted_height(&self) -> Ordinate {
        self.get_height() + self.cycle_height - 1
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted_rocks: Vec<Coordinate> = self.rocks.iter().cloned().collect();
        sorted_rocks.sort_by(|a, b| {
            let mut ordering = b.y.cmp(&a.y);
            if ordering == Ordering::Equal {
                ordering = a.x.cmp(&b.x);
            }
            ordering
        });

        let mut rocks = sorted_rocks.iter();
        let max_x = self.width;
        let max_y = &self.rocks.iter().map(|coor| coor.y).max().unwrap_or(0);

        let mut output: String = String::new();

        let mut maybe_rock = rocks.next();

        for y in (0..max_y + 1).rev() {
            output.push('|');

            for x in 0..max_x {
                if let Some(current_rock) = maybe_rock {
                    if (current_rock.x == x) && (current_rock.y == y) {
                        output.push('#');
                        maybe_rock = rocks.next();
                        continue;
                    }
                }

                output.push('.');
            }

            output.push_str("|\n");
        }

        output.push_str("+-------+\n");
        write!(f, "{}", output)
    }
}

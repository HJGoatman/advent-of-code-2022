use crate::coordinate::{Coordinate, Ordinate};

#[derive(Debug)]
pub enum RockType {
    Minus,
    Plus,
    L,
    I,
    Square,
}

pub struct RockIterator {
    pub start_index: u8,
}

impl Iterator for RockIterator {
    type Item = RockType;

    fn next(&mut self) -> Option<Self::Item> {
        let rock_type = match self.start_index {
            0 => RockType::Minus,
            1 => RockType::Plus,
            2 => RockType::L,
            3 => RockType::I,
            4 => RockType::Square,
            _ => unreachable!(),
        };

        self.start_index = (self.start_index + 1) % 5;

        Some(rock_type)
    }
}

pub fn create_rock(rock_type: &RockType, current_height: Ordinate) -> Vec<Coordinate> {
    let min_x = 2;
    let min_y = current_height + 3;

    match rock_type {
        RockType::Minus => vec![
            Coordinate { x: min_x, y: min_y },
            Coordinate {
                x: min_x + 1,
                y: min_y,
            },
            Coordinate {
                x: min_x + 2,
                y: min_y,
            },
            Coordinate {
                x: min_x + 3,
                y: min_y,
            },
        ],
        RockType::Plus => vec![
            Coordinate {
                x: min_x + 1,
                y: min_y,
            },
            Coordinate {
                x: min_x,
                y: min_y + 1,
            },
            Coordinate {
                x: min_x + 1,
                y: min_y + 1,
            },
            Coordinate {
                x: min_x + 2,
                y: min_y + 1,
            },
            Coordinate {
                x: min_x + 1,
                y: min_y + 2,
            },
        ],
        RockType::L => vec![
            Coordinate { x: min_x, y: min_y },
            Coordinate {
                x: min_x + 1,
                y: min_y,
            },
            Coordinate {
                x: min_x + 2,
                y: min_y,
            },
            Coordinate {
                x: min_x + 2,
                y: min_y + 1,
            },
            Coordinate {
                x: min_x + 2,
                y: min_y + 2,
            },
        ],
        RockType::I => vec![
            Coordinate { x: min_x, y: min_y },
            Coordinate {
                x: min_x,
                y: min_y + 1,
            },
            Coordinate {
                x: min_x,
                y: min_y + 2,
            },
            Coordinate {
                x: min_x,
                y: min_y + 3,
            },
        ],
        RockType::Square => vec![
            Coordinate { x: min_x, y: min_y },
            Coordinate {
                x: min_x + 1,
                y: min_y,
            },
            Coordinate {
                x: min_x,
                y: min_y + 1,
            },
            Coordinate {
                x: min_x + 1,
                y: min_y + 1,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        coordinate::Coordinate,
        simulator::rock::{create_rock, RockType},
    };

    #[test]
    fn test_create_rock() {
        let input = RockType::Minus;
        let expected = vec![
            Coordinate { x: 2, y: 3 },
            Coordinate { x: 3, y: 3 },
            Coordinate { x: 4, y: 3 },
            Coordinate { x: 5, y: 3 },
        ];
        let actual = create_rock(&input, 0);
        assert_eq!(expected, actual);
    }
}

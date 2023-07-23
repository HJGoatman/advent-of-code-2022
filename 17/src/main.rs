use std::{cmp::Ordering, collections::HashSet, env, fmt::Display, fs, hash::Hash};

#[derive(Debug, PartialEq, Eq)]
enum JetDirection {
    Left,
    Right,
}

type JetPattern = Vec<JetDirection>;

#[derive(Debug)]
enum RockType {
    Minus,
    Plus,
    L,
    I,
    Square,
}

struct RockIterator {
    start_index: u8,
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

type Ordinate = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: Ordinate,
    y: Ordinate,
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

struct Chamber {
    width: u16,
    rocks: HashSet<Coordinate>,
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

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_jet_pattern(input: &str) -> JetPattern {
    input
        .chars()
        .map(|c| match c {
            '<' => JetDirection::Left,
            '>' => JetDirection::Right,
            _ => unreachable!(),
        })
        .collect()
}

fn create_rock(rock_type: &RockType, current_height: Ordinate) -> Vec<Coordinate> {
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

fn get_tower_height(rocks: &HashSet<Coordinate>) -> Ordinate {
    match rocks.iter().map(|coor| coor.y).max() {
        Some(height) => height + 1,
        None => 0,
    }
}

fn is_blocked_below(rock: &[Coordinate], rocks: &HashSet<Coordinate>) -> bool {
    let is_at_floor = rock.get(0).unwrap().y == 0;

    if is_at_floor {
        return true;
    }

    let has_rock_below = rock.iter().any(|part| {
        rocks.contains(&Coordinate {
            x: part.x,
            y: part.y - 1,
        })
    });
    is_at_floor || has_rock_below
}

fn is_blocked_left(rock: &[Coordinate], rocks: &HashSet<Coordinate>) -> bool {
    let is_at_wall = rock.iter().any(|part| part.x == 0);

    if is_at_wall {
        return true;
    }

    let has_rock_left = rock.iter().any(|part| {
        rocks.contains(&Coordinate {
            x: part.x - 1,
            y: part.y,
        })
    });
    is_at_wall || has_rock_left
}

fn is_blocked_right(rock: &[Coordinate], rocks: &HashSet<Coordinate>) -> bool {
    let is_at_wall = rock.iter().any(|part| part.x == 6);
    let has_rock_left = rock.iter().any(|part| {
        rocks.contains(&Coordinate {
            x: part.x + 1,
            y: part.y,
        })
    });
    is_at_wall || has_rock_left
}

fn run_simulation(pattern: &JetPattern, simulation_length: u16) -> Chamber {
    let mut rock_iterator = RockIterator { start_index: 0 };
    let mut chamber = Chamber {
        width: 7,
        rocks: HashSet::new(),
    };

    let mut pattern_iter = pattern.iter().cycle();
    for _ in 0..simulation_length {
        let rock_type = rock_iterator.next().unwrap();

        log::debug!("A new rock ({:?}) begins falling:", rock_type);

        let current_height = get_tower_height(&chamber.rocks);

        let mut rock = create_rock(&rock_type, current_height);

        loop {
            let this_pattern = pattern_iter.next().unwrap();

            if let JetDirection::Left = this_pattern {
                if is_blocked_left(&rock, &chamber.rocks) {
                    log::debug!("Jet of gas pushes rock left, but nothing happens:");
                } else {
                    log::debug!("Jet of gas pushes rock left");
                    rock.iter_mut().for_each(|part| part.x -= 1);
                }
            } else if is_blocked_right(&rock, &chamber.rocks) {
                log::debug!("Jet of gas pushes rock right, but nothing happens:");
            } else {
                log::debug!("Jet of gas pushes rock right:");
                rock.iter_mut().for_each(|part| part.x += 1);
            }

            if is_blocked_below(&rock, &chamber.rocks) {
                log::debug!("Rock falls 1 unit, causing it to come to rest:");
                break;
            } else {
                log::debug!("Rock falls 1 unit:");
                rock.iter_mut().for_each(|part| part.y -= 1);
            }
        }

        // Add rock
        rock.into_iter().for_each(|part| {
            chamber.rocks.insert(part);
        });

        chamber.rocks = boundry_trace(&chamber.rocks, chamber.width);

        log::debug!("\n{}", chamber);
    }

    chamber
}

struct MooreNeighbourhoodIterator {
    iterator: Box<dyn Iterator<Item = Coordinate>>,
}

impl MooreNeighbourhoodIterator {
    fn new(centre_point: Coordinate, start: Option<Coordinate>) -> MooreNeighbourhoodIterator {
        let mut neighbours = Vec::new();

        neighbours.push(Coordinate {
            x: centre_point.x,
            y: centre_point.y + 1,
        });
        neighbours.push(Coordinate {
            x: centre_point.x + 1,
            y: centre_point.y + 1,
        });
        neighbours.push(Coordinate {
            x: centre_point.x + 1,
            y: centre_point.y,
        });

        if centre_point.y > 0 {
            neighbours.push(Coordinate {
                x: centre_point.x + 1,
                y: centre_point.y - 1,
            });
        }

        if centre_point.y > 0 {
            neighbours.push(Coordinate {
                x: centre_point.x,
                y: centre_point.y - 1,
            });
        }

        if centre_point.x > 0 && centre_point.y > 0 {
            neighbours.push(Coordinate {
                x: centre_point.x - 1,
                y: centre_point.y - 1,
            });
        }

        if centre_point.x > 0 {
            neighbours.push(Coordinate {
                x: centre_point.x - 1,
                y: centre_point.y,
            });
        }

        if centre_point.x > 0 {
            neighbours.push(Coordinate {
                x: centre_point.x - 1,
                y: centre_point.y + 1,
            });
        }

        if let Some(start) = start {
            // log::debug!("{:?}", start);
            let index_of_start = neighbours.iter().position(|&r| r == start).unwrap();
            neighbours.rotate_left(index_of_start);
            // log::debug!("{:?}", neighbours);
        }

        MooreNeighbourhoodIterator {
            iterator: Box::new(neighbours.into_iter()),
        }
    }
}

impl Iterator for MooreNeighbourhoodIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

fn main() {
    env_logger::init();
    let input = load_input();
    let jet_pattern = parse_jet_pattern(&input);
    let chamber = run_simulation(&jet_pattern, 2022);
    let tower_height = get_tower_height(&chamber.rocks);
    println!("{}", tower_height);
}

fn boundry_trace(chamber: &HashSet<Coordinate>, width: Ordinate) -> HashSet<Coordinate> {
    let mut chamber: HashSet<Coordinate> = chamber.iter().cloned().collect();

    log::trace!("Original: {:#?}", chamber);

    const SHIFT_AMOUNT: Ordinate = 2;

    // Shift all points
    chamber = chamber
        .into_iter()
        .map(|coord| Coordinate {
            x: coord.x + SHIFT_AMOUNT,
            y: coord.y + SHIFT_AMOUNT,
        })
        .collect();

    log::trace!("Shifted: {:#?}", chamber);

    let right_wall_x = width + 2;

    // Add Walls
    let max_y = chamber.iter().map(|c| c.y).max().unwrap();
    let left_wall = (1..max_y + 1).map(|y| Coordinate { x: 1, y }).collect();
    let right_wall = (1..max_y + 1)
        .map(|y| Coordinate { x: right_wall_x, y })
        .collect();

    // Add Floor
    let floor = (2..right_wall_x).map(|x| Coordinate { x, y: 1 }).collect();

    chamber = chamber
        .union(&left_wall)
        .cloned()
        .collect::<HashSet<Coordinate>>()
        .union(&right_wall)
        .cloned()
        .collect::<HashSet<Coordinate>>()
        .union(&floor)
        .cloned()
        .collect();

    log::trace!("Walls and Floor: {:#?}", chamber);

    // Boundary Trace
    chamber = _boundry_trace(&chamber);

    log::trace!("After Trace: {:#?}", chamber);

    // Remove Floor
    // Remove Walls
    chamber.retain(|coord| !(coord.x == 1 || coord.x == right_wall_x || coord.y == 1));

    log::trace!("Removed Floor: {:#?}", chamber);

    // Shift points back
    chamber = chamber
        .into_iter()
        .map(|coord| Coordinate {
            x: coord.x - SHIFT_AMOUNT,
            y: coord.y - SHIFT_AMOUNT,
        })
        .collect();

    log::trace!("Shifted Back: {:#?}", chamber);

    chamber
}

fn _boundry_trace(chamber: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    // Add the walls and floor as coordinates

    // Get the smallest x and largest y coordinate as start.
    // The Bottom should be a boundry.

    let start = *chamber
        .iter()
        .max_by(|a, b| a.x.cmp(&b.x).reverse().then(a.y.cmp(&b.y)))
        .unwrap();

    let mut boundary: HashSet<Coordinate> = HashSet::new();

    boundary.insert(start);

    let mut neighbourhood_iterator = MooreNeighbourhoodIterator::new(start, None);

    let mut previous_point = None;
    let mut current_point = neighbourhood_iterator.next().unwrap();

    while current_point != start {
        if chamber.contains(&current_point) {
            boundary.insert(current_point);
            neighbourhood_iterator = MooreNeighbourhoodIterator::new(current_point, previous_point);
        }

        previous_point = Some(current_point);
        current_point = neighbourhood_iterator.next().unwrap();
    }

    boundary.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::JetDirection::{Left, Right};
    use crate::{
        create_rock, get_tower_height, parse_jet_pattern, run_simulation, Coordinate, JetPattern,
        RockType,
    };

    fn get_test_jet_pattern() -> JetPattern {
        vec![
            Right, Right, Right, Left, Left, Right, Left, Right, Right, Left, Left, Left, Right,
            Right, Left, Right, Right, Right, Left, Left, Left, Right, Right, Right, Left, Left,
            Left, Right, Left, Left, Left, Right, Right, Left, Right, Right, Left, Left, Right,
            Right,
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let expected = get_test_jet_pattern();
        let actual = parse_jet_pattern(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_tower_height() {
        let input = get_test_jet_pattern();
        let expected = 3068;
        let actual_chamber = run_simulation(&input, 2022);
        let actual = get_tower_height(&actual_chamber.rocks);
        assert_eq!(expected, actual);
    }

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

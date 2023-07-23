mod chamber;
mod rock;

use std::collections::{HashMap, HashSet};

use crate::coordinate::{Coordinate, Ordinate};
use crate::parser::JetDirection;
use crate::{parser::JetPattern, simulator::rock::create_rock};

use self::rock::RockType;
use self::{chamber::Chamber, rock::RockIterator};

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

#[derive(Debug, Hash, Eq, PartialEq)]
struct State {
    rock_type: RockType,
    pattern_num: usize,
    boundary: Vec<Coordinate>,
}

pub fn run_simulation(pattern: &JetPattern, simulation_length: u64) -> Chamber {
    let mut rock_iterator = RockIterator { start_index: 0 };
    let mut chamber = Chamber {
        width: 7,
        rocks: HashSet::new(),
        cycle_height: 0,
    };
    let mut states = HashMap::new();
    let mut has_used_repeat = false;

    let mut pattern_iter = pattern.iter().enumerate().cycle();

    let mut i = 0;

    while i < simulation_length {
        let rock_type = rock_iterator.next().unwrap();

        log::debug!("A new rock ({:?}) begins falling:", rock_type);

        let current_height = chamber.get_height();

        let mut rock = create_rock(&rock_type, current_height);

        let (mut pattern_num, mut this_pattern) = pattern_iter.next().unwrap();

        loop {
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

            (pattern_num, this_pattern) = pattern_iter.next().unwrap();
        }

        // Add rock
        rock.into_iter().for_each(|part| {
            chamber.rocks.insert(part);
        });

        log::debug!("\n{}", chamber);

        if !has_used_repeat {
            chamber.rocks = boundry_trace(&chamber.rocks, chamber.width);

            let boundary = normalise_chamber(&chamber.rocks);

            let state = State {
                rock_type,
                pattern_num,
                boundary,
            };

            if let Some((previous_i, previous_height)) = states.get(&state) {
                log::info!("Found Cycle!");

                let current_height = chamber.get_height();

                let change_in_increment = i - previous_i;
                let change_in_height = current_height - previous_height;

                let remaining_increments = simulation_length - i;

                let repeats = remaining_increments / change_in_increment;
                let remaining = remaining_increments % change_in_increment;

                let new_height = change_in_height * repeats;
                chamber.cycle_height = new_height;

                i = simulation_length - remaining;

                log::info!("New i = {}", i);

                log::info!(
                    "There are {} many repeats remaining, then there's a remaining {} iterations",
                    repeats,
                    remaining
                );
                log::info!("The change of height per cycle is {}, this means that the height will increase by {}.", change_in_height, new_height);

                has_used_repeat = true;
                continue;
            }

            states.insert(state, (i, chamber.get_height()));
        }

        i += 1;
    }

    chamber
}

fn normalise_chamber(rocks: &HashSet<Coordinate>) -> Vec<Coordinate> {
    let min_x = rocks.iter().map(|coord| coord.x).min().unwrap();
    let min_y = rocks.iter().map(|coord| coord.y).min().unwrap();

    rocks
        .iter()
        .map(|coord| Coordinate {
            x: coord.x - min_x,
            y: coord.y - min_y,
        })
        .collect()
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
    use crate::parser::JetDirection::{Left, Right};
    use crate::{parser::JetPattern, simulator::run_simulation};

    fn get_test_jet_pattern() -> JetPattern {
        vec![
            Right, Right, Right, Left, Left, Right, Left, Right, Right, Left, Left, Left, Right,
            Right, Left, Right, Right, Right, Left, Left, Left, Right, Right, Right, Left, Left,
            Left, Right, Left, Left, Left, Right, Right, Left, Right, Right, Left, Left, Right,
            Right,
        ]
    }

    #[test]
    fn test_get_tower_height() {
        let input = get_test_jet_pattern();
        let expected = 3068;
        let actual_chamber = run_simulation(&input, 2022);
        let actual = actual_chamber.get_height();
        assert_eq!(expected, actual);
    }
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    env,
    fmt::{Display, Write},
    fs, process,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValleyPart {
    Wall,
    ClearGround,
    Blizzard(u16, Direction),
    Overlap(
        Option<(u16, Direction)>,
        Option<(u16, Direction)>,
        Option<(u16, Direction)>,
        Option<(u16, Direction)>,
    ),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valley {
    data: Vec<ValleyPart>,
    width: usize,
    height: usize,
}

impl Valley {
    fn get_part(&self, coordinate: Coordinate) -> ValleyPart {
        let index = coordinate.y * self.width + coordinate.x;
        self.data[index]
    }

    fn set_part(&mut self, coordinate: Coordinate, part: ValleyPart) {
        let index = coordinate.y * self.width + coordinate.x;
        self.data[index] = part;
    }

    fn get_coordinate(&self, index: usize) -> Coordinate {
        let (quotiant, remainder) = (index / self.width, index % self.width);
        Coordinate {
            x: remainder,
            y: quotiant,
        }
    }

    fn get_blizzards(&self) -> Vec<(ValleyPart, Coordinate)> {
        let blizzards = self
            .data
            .iter()
            .enumerate()
            .filter(|(_, part)| match part {
                ValleyPart::Blizzard(_, _) => true,
                _ => false,
            })
            .map(|(i, part)| (*part, self.get_coordinate(i)))
            .collect();

        let overlaps: Vec<(ValleyPart, Coordinate)> = self
            .data
            .iter()
            .enumerate()
            .map(|(i, part)| match part {
                ValleyPart::Overlap(a, b, c, d) => [a, b, c, d]
                    .iter()
                    .filter(|part| part.is_some())
                    .map(|part| {
                        (
                            ValleyPart::Blizzard(part.unwrap().0, part.unwrap().1),
                            self.get_coordinate(i),
                        )
                    })
                    .collect(),
                _ => vec![],
            })
            .flatten()
            .collect();

        [blizzards, overlaps].concat()
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn remove_part(&mut self, position: Coordinate, part: ValleyPart) {
        let part_at_position = self.get_part(position);

        if part_at_position == part {
            self.set_part(position, ValleyPart::ClearGround);
        }

        if let ValleyPart::Overlap(a, b, c, d) = part_at_position {
            if let ValleyPart::Blizzard(id, dir) = part {
                let x = Some((id, dir));
                let mut new_part = if x == a {
                    ValleyPart::Overlap(None, b, c, d)
                } else if x == b {
                    ValleyPart::Overlap(a, None, c, d)
                } else if x == c {
                    ValleyPart::Overlap(a, b, None, d)
                } else if x == d {
                    ValleyPart::Overlap(a, b, c, None)
                } else {
                    ValleyPart::Overlap(a, b, c, d)
                };

                if let ValleyPart::Overlap(a, b, c, d) = new_part {
                    if a.is_none() && b.is_none() && c.is_none() && d.is_none() {
                        new_part = ValleyPart::ClearGround;
                    }
                }

                self.set_part(position, new_part);
            }
        }
    }

    fn set_blizzards(&mut self, blizzards: Vec<(ValleyPart, Coordinate, Coordinate)>) {
        let mut map = HashMap::new();

        for (part, old_position, new_position) in blizzards {
            map.entry(new_position)
                .and_modify(|parts: &mut Vec<(ValleyPart, Coordinate)>| {
                    parts.push((part, old_position))
                })
                .or_insert(vec![(part, old_position)]);
        }

        for (new_position, parts) in map {
            if parts.len() == 1 {
                let (part, old_position) = parts[0];
                self.set_part(new_position, part);
                self.remove_part(old_position, part);
            } else {
                let part = ValleyPart::Overlap(
                    parts.get(0).map(|(part, _)| match part {
                        ValleyPart::Blizzard(id, dir) => (*id, *dir),
                        _ => panic!(),
                    }),
                    parts.get(1).map(|(part, _)| match part {
                        ValleyPart::Blizzard(id, dir) => (*id, *dir),
                        _ => panic!(),
                    }),
                    parts.get(2).map(|(part, _)| match part {
                        ValleyPart::Blizzard(id, dir) => (*id, *dir),
                        _ => panic!(),
                    }),
                    parts.get(3).map(|(part, _)| match part {
                        ValleyPart::Blizzard(id, dir) => (*id, *dir),
                        _ => panic!(),
                    }),
                );

                self.set_part(new_position, part);

                for (part, old_position) in parts {
                    self.remove_part(old_position, part)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseValleyError;

impl FromStr for Valley {
    type Err = ParseValleyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blizzard_count = 0..;

        let parsed_rows: Result<Vec<Vec<ValleyPart>>, ParseValleyError> = s
            .split('\n')
            .filter(|val| val != &"")
            .map(|row| {
                row.chars()
                    .map(|char| match char {
                        '#' => Ok(ValleyPart::Wall),
                        '.' => Ok(ValleyPart::ClearGround),
                        '^' => Ok(ValleyPart::Blizzard(
                            blizzard_count.next().unwrap(),
                            Direction::Up,
                        )),
                        'v' => Ok(ValleyPart::Blizzard(
                            blizzard_count.next().unwrap(),
                            Direction::Down,
                        )),
                        '<' => Ok(ValleyPart::Blizzard(
                            blizzard_count.next().unwrap(),
                            Direction::Left,
                        )),
                        '>' => Ok(ValleyPart::Blizzard(
                            blizzard_count.next().unwrap(),
                            Direction::Right,
                        )),
                        _ => Err(ParseValleyError),
                    })
                    .collect()
            })
            .collect();

        let parsed_rows: Vec<Vec<ValleyPart>> = parsed_rows?;

        let width = *&parsed_rows[0].len();
        let height = *&parsed_rows.len();
        let data = parsed_rows.into_iter().flatten().collect();

        Ok(Valley {
            data,
            width,
            height,
        })
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;

        for (i, part) in self.data.iter().enumerate() {
            let c = match part {
                ValleyPart::Wall => '#',
                ValleyPart::ClearGround => '.',
                ValleyPart::Blizzard(_, direction) => match direction {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                },
                ValleyPart::Overlap(a, b, c, d) => {
                    let count = [a, b, c, d].into_iter().filter(|v| v.is_some()).count();
                    char::from_digit(count as u32, 10).unwrap()
                }
            };

            f.write_char(c)?;

            if (i + 1) % (self.width) == 0 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

// Iterates the valley's state by a minute.
struct ValleyIterator {
    valley: Valley,
}

impl Iterator for ValleyIterator {
    type Item = Valley;

    fn next(&mut self) -> Option<Self::Item> {
        let blizzards = self.valley.get_blizzards();

        let first_open_y = 1;
        let last_open_y = self.valley.get_height() - 2;

        let first_open_x = 1;
        let last_open_x = self.valley.get_width() - 2;

        let new_positions: Vec<(ValleyPart, Coordinate, Coordinate)> = blizzards
            .iter()
            .map(|(blizzard, position)| {
                (
                    *blizzard,
                    *position,
                    match blizzard {
                        ValleyPart::Blizzard(_, Direction::Up) => {
                            if position.y == first_open_y {
                                Coordinate {
                                    x: position.x,
                                    y: last_open_y,
                                }
                            } else {
                                Coordinate {
                                    x: position.x,
                                    y: position.y - 1,
                                }
                            }
                        }
                        ValleyPart::Blizzard(_, Direction::Down) => {
                            if position.y == last_open_y {
                                Coordinate {
                                    x: position.x,
                                    y: first_open_y,
                                }
                            } else {
                                Coordinate {
                                    x: position.x,
                                    y: position.y + 1,
                                }
                            }
                        }
                        ValleyPart::Blizzard(_, Direction::Left) => {
                            if position.x == first_open_x {
                                Coordinate {
                                    x: last_open_x,
                                    y: position.y,
                                }
                            } else {
                                Coordinate {
                                    x: position.x - 1,
                                    y: position.y,
                                }
                            }
                        }
                        ValleyPart::Blizzard(_, Direction::Right) => {
                            if position.x == last_open_x {
                                Coordinate {
                                    x: first_open_x,
                                    y: position.y,
                                }
                            } else {
                                Coordinate {
                                    x: position.x + 1,
                                    y: position.y,
                                }
                            }
                        }
                        _ => todo!(),
                    },
                )
            })
            .collect();

        self.valley.set_blizzards(new_positions);

        Some(self.valley.clone())
    }
}

#[derive(Eq, PartialEq, Debug)]
struct State {
    position: Coordinate,
    predicted_time: u32,
    current_time: u32,
    valley: Valley,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.predicted_time.cmp(&other.predicted_time))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    env_logger::init();

    let input = load_input();
    let valley = Valley::from_str(&input).unwrap_or_else(|_| {
        eprintln!("Problem parsing valley!");
        process::exit(1);
    });

    let time = find_quickest_time_to_goal(valley);
    println!("{}", time.unwrap());
}

fn find_quickest_time_to_goal(valley: Valley) -> Option<u32> {
    log::debug!("{}", valley);

    let start = Coordinate { x: 1, y: 0 };
    let end = Coordinate {
        x: &valley.get_width() - 2,
        y: &valley.get_height() - 1,
    };

    let mut queue = BinaryHeap::new();

    fn h(pos: Coordinate, end: Coordinate) -> u32 {
        ((end.x - pos.x) + (end.y - pos.y)).try_into().unwrap()
    }

    queue.push(Reverse(State {
        position: start,
        predicted_time: 0 + h(start, end),
        current_time: 0,
        valley,
    }));

    while let Some(Reverse(state)) = queue.pop() {
        let State {
            position: current_position,
            current_time,
            predicted_time: _,
            valley,
        } = state;

        if current_position == end {
            return Some(current_time);
        }

        let next_valley = ValleyIterator { valley }.next().unwrap();

        for next_position in get_next_positions(current_position, &next_valley) {
            let new_state = Reverse(State {
                position: next_position,
                predicted_time: current_time + 1 + h(next_position, end),
                current_time: current_time + 1,
                valley: next_valley.clone(),
            });

            if !queue.iter().any(|state| state == &new_state) {
                queue.push(new_state);
            }
        }
    }

    None
}

fn get_next_positions(current_position: Coordinate, valley: &Valley) -> Vec<Coordinate> {
    let mut next_positions = Vec::new();

    if current_position.x > 0 {
        let left = Coordinate {
            x: current_position.x - 1,
            y: current_position.y,
        };

        next_positions.push(left);
    }

    if current_position.y > 0 {
        let up = Coordinate {
            x: current_position.x,
            y: current_position.y - 1,
        };
        next_positions.push(up);
    }

    if current_position.x < valley.get_width() - 1 {
        let right = Coordinate {
            x: current_position.x + 1,
            y: current_position.y,
        };
        next_positions.push(right);
    }

    if current_position.y < valley.get_height() - 1 {
        let down = Coordinate {
            x: current_position.x,
            y: current_position.y + 1,
        };
        next_positions.push(down);
    }

    let wait = current_position;
    next_positions.push(wait);

    next_positions
        .into_iter()
        .filter(|position| match valley.get_part(*position) {
            ValleyPart::ClearGround => true,
            _ => false,
        })
        .collect()
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

use nalgebra::DMatrix;
use std::{env, fmt::Display, fs};

type Ordinate = u32;
type Distance = u32;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: Ordinate,
    y: Ordinate,
}

trait Rectangular {
    fn get_start(&self) -> Coordinate;
    fn get_width(&self) -> Ordinate;
    fn get_height(&self) -> Ordinate;
    fn has_wall_at_point(&self, coordinate: Coordinate) -> bool;
}

#[derive(Debug, PartialEq, Clone)]
struct MapPart {
    start: Coordinate,
    map: DMatrix<Token>,
}

struct Map {
    parts: Vec<MapPart>,
    connections: Vec<MapPartConnection>,
}
impl Map {
    fn move_up_part(&self, state: State) -> State {
        let potential_map_part_id = self.connections[state.current_map_part_id].up;
        let potential_new_board = &self.parts[potential_map_part_id];
        let potential_new_y =
            potential_new_board.get_start().y + potential_new_board.get_height() - 1;
        State {
            current_map_part_id: potential_map_part_id,
            position: Coordinate {
                y: potential_new_y,
                ..state.position
            },
            ..state
        }
    }

    fn move_right_part(&self, state: State) -> State {
        let potential_map_part_id = self.connections[state.current_map_part_id].right;
        let potential_new_board = &self.parts[potential_map_part_id];
        let potential_new_x = potential_new_board.get_start().x;
        State {
            current_map_part_id: potential_map_part_id,
            position: Coordinate {
                x: potential_new_x,
                ..state.position
            },
            ..state
        }
    }

    fn move_down_part(&self, state: State) -> State {
        let potential_map_part_id = self.connections[state.current_map_part_id].down;
        let potential_new_board = &self.parts[potential_map_part_id];
        let potential_new_y = potential_new_board.get_start().y;
        State {
            current_map_part_id: potential_map_part_id,
            position: Coordinate {
                y: potential_new_y,
                ..state.position
            },
            ..state
        }
    }

    fn move_left_part(&self, state: State) -> State {
        let potential_map_part_id = self.connections[state.current_map_part_id].left;
        let potential_new_board = &self.parts[potential_map_part_id];
        let potential_new_x =
            potential_new_board.get_start().x + potential_new_board.get_width() - 1;

        State {
            current_map_part_id: potential_map_part_id,
            position: Coordinate {
                x: potential_new_x,
                ..state.position
            },
            ..state
        }
    }
}

impl Rectangular for MapPart {
    fn get_start(&self) -> Coordinate {
        self.start
    }

    fn get_width(&self) -> Ordinate {
        self.map.ncols() as Ordinate
    }

    fn get_height(&self) -> Ordinate {
        self.map.nrows() as Ordinate
    }

    fn has_wall_at_point(&self, coordinate: Coordinate) -> bool {
        let start = self.start;

        log::trace!("Start: {:?}", start);
        log::trace!("Coordinate: {:?}", coordinate);

        let x = coordinate.x - start.x;
        let y = coordinate.y - start.y;

        log::trace!("Rect positions: ({}, {})", x, y);

        self.map[(y as usize, x as usize)] == Token::SolidWall
    }
}

#[derive(Debug, PartialEq, Clone)]
struct MapPartConnection {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

#[derive(Debug, PartialEq)]
enum Turn {
    AntiClockwise,
    Clockwise,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, PartialEq)]
enum Step {
    Turn(Turn),
    Move(Distance),
}

type Path = Vec<Step>;

#[derive(Debug, PartialEq, Copy, Clone)]
struct State {
    position: Coordinate,
    facing: Facing,
    current_map_part_id: usize,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Blank,
    OpenTile,
    SolidWall,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Blank => f.write_str(" "),
            Token::OpenTile => f.write_str("."),
            Token::SolidWall => f.write_str("#"),
        }
    }
}

fn parse_map(input: &str) -> Vec<MapPart> {
    let mut tokenised: Vec<Vec<Token>> = input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|char| match char {
                    ' ' => Token::Blank,
                    '.' => Token::OpenTile,
                    '#' => Token::SolidWall,
                    _ => todo!(),
                })
                .collect()
        })
        .collect();

    let ncols = &tokenised.iter().map(|row| row.len()).max().unwrap();
    let nrows = &tokenised.len();

    tokenised = tokenised
        .into_iter()
        .map(|mut row| {
            let diff = ncols - row.len();
            let padding = std::iter::repeat(Token::Blank).take(diff);
            row.extend(padding);
            row
        })
        .collect();

    // All cube nets have a single square with at some point, this is our part length.
    let part_length = tokenised
        .iter()
        .map(|row| {
            row.into_iter()
                .filter(|&&token| token != Token::Blank)
                .count()
        })
        .min()
        .unwrap();

    let flat: Vec<Token> = tokenised.into_iter().flatten().collect();

    let matrix = DMatrix::from_iterator(*ncols, *nrows, flat).transpose();

    log::debug!("{}", matrix);

    let matrix_shape = matrix.shape();

    let mut parts = Vec::new();
    let mut starts = Vec::new();

    for y in 0..matrix_shape.0 / part_length {
        for x in 0..matrix_shape.1 / part_length {
            log::debug!("{}, {}", x, y);

            let start = (y * part_length, x * part_length);
            let part = matrix.view(start, (part_length, part_length));

            if part.iter().all(|val| val == &Token::Blank) {
                continue;
            }

            parts.push(part);
            starts.push(start);
            // log::debug!("{}", part);
        }
    }

    let map_parts = starts
        .iter()
        .zip(parts.iter())
        .map(|(start, part)| MapPart {
            start: Coordinate {
                x: (start.1 + 1) as u32,
                y: (start.0 + 1) as u32,
            },
            map: part.clone_owned(),
        })
        .collect();

    map_parts
}

fn parse_path(input: &str) -> Path {
    let mut path = Vec::new();

    let chars = input.chars();
    let mut buffer: Vec<char> = Vec::new();

    fn collect_buffer(buffer: &mut Vec<char>, path: &mut Vec<Step>) {
        if !buffer.is_empty() {
            let distance = buffer
                .iter()
                .collect::<String>()
                .parse::<Distance>()
                .unwrap();
            buffer.clear();
            path.push(Step::Move(distance));
        }
    }

    for char in chars {
        match char {
            'L' => {
                collect_buffer(&mut buffer, &mut path);
                path.push(Step::Turn(Turn::AntiClockwise));
            }
            'R' => {
                collect_buffer(&mut buffer, &mut path);
                path.push(Step::Turn(Turn::Clockwise));
            }
            digit => buffer.push(digit),
        }
    }

    collect_buffer(&mut buffer, &mut path);

    path
}

fn parse_board_map(input: &str) -> (Vec<MapPart>, Path) {
    let mut split = input.split("\n\n");
    let map_parts = parse_map(split.next().unwrap());
    let path = parse_path(split.next().unwrap());
    (map_parts, path)
}

fn create_flat_map(parts: &[MapPart]) -> Map {
    let starts: Vec<Coordinate> = parts
        .iter()
        .map(|part| {
            let start = part.get_start();

            Coordinate {
                x: (start.x - 1) / part.get_width(),
                y: (start.y - 1) / part.get_height(),
            }
        })
        .collect();
    log::debug!("Starts: {:#?}", starts);

    let mut i = 0;
    let net = DMatrix::from_fn(
        (starts.iter().map(|a| a.x).max().unwrap() + 1)
            .try_into()
            .unwrap(),
        (starts.iter().map(|a| a.y).max().unwrap() + 1)
            .try_into()
            .unwrap(),
        |r, c| {
            if starts.contains(&Coordinate {
                x: r as Ordinate,
                y: c as Ordinate,
            }) {
                let v = i;
                i += 1;
                Some(v)
            } else {
                None
            }
        },
    )
    .transpose();

    log::debug!(
        "net: {}",
        net.map(|v| match v {
            Some(a) => a.to_string(),
            None => " ".to_string(),
        })
    );

    let mut connections = Vec::new();
    for Coordinate { x, y } in starts {
        let col: Vec<u8> = net
            .column(x as usize)
            .into_iter()
            .cloned()
            .flatten()
            .collect();
        let row: Vec<u8> = net.row(y as usize).into_iter().cloned().flatten().collect();

        log::debug!("Row: {:?}, Col: {:?}", row, col);

        fn modulo(a: i8, b: i8) -> i8 {
            ((a % b) + b) % b
        }

        let get_wrap_part = |ord, vec: &[u8]| vec[(modulo(ord, vec.len() as i8)) as usize] as usize;

        let val = net[(y as usize, x as usize)].unwrap();
        let col_index = col.iter().position(|&r| r == val).unwrap() as i8;
        let row_index = row.iter().position(|&r| r == val).unwrap() as i8;

        let up = get_wrap_part(col_index - 1, &col);
        let down = get_wrap_part(col_index + 1, &col);
        let right = get_wrap_part(row_index + 1, &row);
        let left = get_wrap_part(row_index - 1, &row);

        connections.push(MapPartConnection {
            up,
            right,
            down,
            left,
        });
    }
    //
    log::debug!("Connections: {:#?}", connections);

    Map {
        parts: parts.to_vec(),
        connections,
    }
}

fn move_across_map(map: &Map, state: State, distance: Distance) -> State {
    log::debug!(
        "At {:?}, going {:?}, {} places.",
        state.position,
        state.facing,
        distance
    );

    let mut state = state;

    for _ in 0..distance {
        let current_map_part = &map.parts[state.current_map_part_id];
        let facing = state.facing;
        let position = state.position;

        let current_start = current_map_part.get_start();

        let potential_state = match facing {
            Facing::Up => {
                let is_at_top_edge = position.y == current_start.y;
                if is_at_top_edge {
                    map.move_up_part(state)
                } else {
                    State {
                        position: Coordinate {
                            y: position.y - 1,
                            ..position
                        },
                        ..state
                    }
                }
            }
            Facing::Right => {
                let is_at_right_edge =
                    position.x == current_start.x + current_map_part.get_width() - 1;
                if is_at_right_edge {
                    map.move_right_part(state)
                } else {
                    State {
                        position: Coordinate {
                            x: position.x + 1,
                            ..position
                        },
                        ..state
                    }
                }
            }
            Facing::Down => {
                let is_at_bottom_edge =
                    position.y == current_start.y + current_map_part.get_height() - 1;
                if is_at_bottom_edge {
                    map.move_down_part(state)
                } else {
                    State {
                        position: Coordinate {
                            y: position.y + 1,
                            ..position
                        },
                        ..state
                    }
                }
            }
            Facing::Left => {
                let is_at_left_edge = position.x == current_start.x;
                if is_at_left_edge {
                    map.move_left_part(state)
                } else {
                    State {
                        position: Coordinate {
                            x: position.x - 1,
                            ..position
                        },
                        ..state
                    }
                }
            }
        };

        let current_part = &map.parts[potential_state.current_map_part_id];
        if current_part.has_wall_at_point(potential_state.position) {
            log::debug!("Has wall at point! {:?}", potential_state.position);
            break;
        }

        state = potential_state;
    }

    state
}

fn turn_clockwise(facing: Facing) -> Facing {
    match facing {
        Facing::Up => Facing::Right,
        Facing::Right => Facing::Down,
        Facing::Down => Facing::Left,
        Facing::Left => Facing::Up,
    }
}

fn turn_anticlockwise(facing: Facing) -> Facing {
    match facing {
        Facing::Up => Facing::Left,
        Facing::Right => Facing::Up,
        Facing::Down => Facing::Right,
        Facing::Left => Facing::Down,
    }
}

fn follow_step(board_map: &Map, state: State, step: &Step) -> State {
    match step {
        Step::Move(distance) => move_across_map(board_map, state, *distance),
        Step::Turn(Turn::Clockwise) => State {
            facing: turn_clockwise(state.facing),
            ..state
        },
        Step::Turn(Turn::AntiClockwise) => State {
            facing: turn_anticlockwise(state.facing),
            ..state
        },
    }
}

fn walk(board_map: &Map, path: &[Step]) -> State {
    for map_part in board_map.parts.iter() {
        log::debug!("{:?}", map_part.start);
        log::debug!("{}", map_part.map);
    }

    let initial_state = State {
        position: board_map.parts.get(0).unwrap().start,
        facing: Facing::Right,
        current_map_part_id: 0,
    };

    let mut current_state = initial_state;
    for step in path.iter() {
        log::trace!("{:?}, {:?}", current_state, step);
        current_state = follow_step(board_map, current_state, step);
    }

    current_state
}

fn determine_password(state: &State) -> Ordinate {
    1000 * state.position.y + 4 * state.position.x + state.facing as Ordinate
}

fn main() {
    env_logger::init();
    let input = load_input();
    let (map_parts, path) = parse_board_map(&input);
    let flat_map = create_flat_map(&map_parts);
    let final_state = walk(&flat_map, &path);
    let final_password = determine_password(&final_state);
    println!("{}", final_password);
}

#[cfg(test)]
mod tests {
    use nalgebra::dmatrix;

    use crate::Turn::{AntiClockwise, Clockwise};
    use crate::{
        determine_password, parse_board_map, walk, Coordinate, Facing, Map, MapPart,
        MapPartConnection, Path, State, Step, Token,
    };

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_test_input() -> (Map, Path) {
        let parts = vec![
            MapPart {
                start: Coordinate { x: 9, y: 1 },
                map: dmatrix![
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::SolidWall;
                    Token::OpenTile, Token::SolidWall, Token::OpenTile, Token::OpenTile;
                    Token::SolidWall, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                ],
            },
            MapPart {
                start: Coordinate { x: 1, y: 5 },
                map: dmatrix![
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::SolidWall;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                    Token::OpenTile, Token::OpenTile, Token::SolidWall, Token::OpenTile;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                ],
            },
            MapPart {
                start: Coordinate { x: 5, y: 5 },
                map: dmatrix![
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::SolidWall;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                ],
            },
            MapPart {
                start: Coordinate { x: 9, y: 5 },
                map: dmatrix![
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::SolidWall;
                    Token::SolidWall, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                    Token::OpenTile, Token::OpenTile, Token::SolidWall, Token::OpenTile;
                ],
            },
            MapPart {
                start: Coordinate { x: 9, y: 9 },
                map: dmatrix![
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::SolidWall;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                    Token::OpenTile, Token::SolidWall, Token::OpenTile, Token::OpenTile;
                    Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                ],
            },
            MapPart {
                start: Coordinate { x: 13, y: 9 },
                map: dmatrix![
                     Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                     Token::OpenTile, Token::SolidWall, Token::OpenTile, Token::OpenTile;
                     Token::OpenTile, Token::OpenTile, Token::OpenTile, Token::OpenTile;
                     Token::OpenTile, Token::OpenTile, Token::SolidWall, Token::OpenTile;
                ],
            },
        ];

        let connections = vec![
            MapPartConnection {
                up: 4,
                right: 0,
                down: 3,
                left: 0,
            },
            MapPartConnection {
                up: 1,
                right: 2,
                down: 1,
                left: 3,
            },
            MapPartConnection {
                up: 2,
                right: 3,
                down: 2,
                left: 1,
            },
            MapPartConnection {
                up: 0,
                right: 1,
                down: 4,
                left: 2,
            },
            MapPartConnection {
                up: 3,
                right: 5,
                down: 0,
                left: 5,
            },
            MapPartConnection {
                up: 5,
                down: 5,
                left: 4,
                right: 4,
            },
        ];

        let map = Map { parts, connections };

        let path = vec![
            Step::Move(10),
            Step::Turn(Clockwise),
            Step::Move(5),
            Step::Turn(AntiClockwise),
            Step::Move(5),
            Step::Turn(Clockwise),
            Step::Move(10),
            Step::Turn(AntiClockwise),
            Step::Move(4),
            Step::Turn(Clockwise),
            Step::Move(5),
            Step::Turn(AntiClockwise),
            Step::Move(5),
        ];

        (map, path)
    }

    #[test]
    fn test_parse_board_map() {
        init();
        let input = include_str!("../test.txt");
        let (expected_map, expected_path) = get_test_input();
        let (actual_map, actual_path) = parse_board_map(input);
        for (expected_part, actual_part) in expected_map.parts.iter().zip(actual_map.iter()) {
            assert_eq!(expected_part.start, actual_part.start);
            assert_eq!(expected_part.map, actual_part.map);
        }
        assert_eq!(expected_path, actual_path);
    }

    #[test]
    fn test_walk() {
        init();
        let (map, path) = get_test_input();
        let expected = State {
            position: Coordinate { x: 8, y: 6 },
            facing: Facing::Right,
            current_map_part_id: 2,
        };
        let actual = walk(&map, &path);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_determine_password() {
        let input = State {
            position: Coordinate { x: 8, y: 6 },
            facing: Facing::Right,
            current_map_part_id: 0,
        };
        let expected = 6032;
        let actual = determine_password(&input);
        assert_eq!(expected, actual);
    }
}

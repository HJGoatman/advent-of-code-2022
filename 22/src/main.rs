use nalgebra::DMatrix;
use petgraph::algo::isomorphism::is_isomorphic;
use petgraph::graph::UnGraph;
use std::{collections::HashMap, env, fmt::Display, fs};

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

#[derive(Debug)]
enum Rotation {
    Rotate90,
    Rotate180,
    Rotate270,
    None,
}

fn single_90_clockwise_rotation(
    coordinate: Coordinate,
    length: Ordinate,
    start: Coordinate,
) -> Coordinate {
    let relative_x = coordinate.x - start.x;
    let relative_y = coordinate.y - start.y;

    let y = relative_x + start.y;
    let x = length - 1 - relative_y + start.x;

    Coordinate { x, y }
}

fn rotate(
    coordinate: Coordinate,
    start: Coordinate,
    width: Ordinate,
    turns: Rotation,
) -> Coordinate {
    match turns {
        Rotation::Rotate90 => single_90_clockwise_rotation(coordinate, width, start),
        Rotation::Rotate180 => (0..2).fold(coordinate, |c: Coordinate, _| {
            single_90_clockwise_rotation(c, width, start)
        }),
        Rotation::Rotate270 => (0..3).fold(coordinate, |c: Coordinate, _| {
            single_90_clockwise_rotation(c, width, start)
        }),
        Rotation::None => coordinate,
    }
}

struct Map {
    parts: Vec<MapPart>,
    connections: Vec<MapPartConnection>,
}

enum Flip {
    Vertical,
    Horizontal,
}

impl Map {
    fn get_part(&self, part_id: usize) -> MapPart {
        self.parts[part_id].clone()
    }

    fn move_part(&self, state: State) -> State {
        // Rotate
        //   Depends on starting edge and ending edge.
        //
        // Flip
        //   depends on rotations and starting facing
        //
        // Translate
        //   Move the distance of the starts.

        let connection = &self.connections[state.current_map_part_id]
            .get(&state.facing)
            .unwrap();

        let new_edge = &connection.edge;
        let new_map_part_id = connection.part_id;

        let new_board = &self.parts[new_map_part_id];

        let rotation = match state.facing {
            Facing::Right => match new_edge {
                Edge::Top => Rotation::Rotate270,
                Edge::Right => Rotation::Rotate180,
                Edge::Bottom => Rotation::Rotate90,
                Edge::Left => Rotation::None,
            },
            Facing::Down => match new_edge {
                Edge::Top => Rotation::None,
                Edge::Right => Rotation::Rotate270,
                Edge::Bottom => Rotation::Rotate180,
                Edge::Left => Rotation::Rotate90,
            },
            Facing::Left => match new_edge {
                Edge::Top => Rotation::Rotate90,
                Edge::Right => Rotation::None,
                Edge::Bottom => Rotation::Rotate270,
                Edge::Left => Rotation::Rotate180,
            },
            Facing::Up => match new_edge {
                Edge::Top => Rotation::Rotate180,
                Edge::Right => Rotation::Rotate90,
                Edge::Bottom => Rotation::None,
                Edge::Left => Rotation::Rotate270,
            },
        };

        let current_board = &self.parts[state.current_map_part_id];

        log::trace!("Rotate: {:?}", &rotation);
        let rotated = rotate(
            state.position,
            current_board.get_start(),
            current_board.get_width(),
            rotation,
        );

        log::trace!("Rotated: {:?}", rotated);

        let flip = match state.facing {
            Facing::Up | Facing::Down => Flip::Horizontal,
            Facing::Right | Facing::Left => Flip::Vertical,
        };

        let flipped = match flip {
            Flip::Vertical => {
                let distance = current_board.get_width() - 1;
                let start_x = current_board.get_start().x;

                let x = (start_x + distance) - (rotated.x - start_x);

                Coordinate { x, y: rotated.y }
            }
            Flip::Horizontal => {
                let distance = current_board.get_height() - 1;

                let start_y = current_board.get_start().y;
                let y = (start_y + distance) - (rotated.y - start_y);

                Coordinate { x: rotated.x, y }
            }
        };

        let new_start = new_board.get_start();
        let current_start = current_board.get_start();

        let (dx, dy): (i32, i32) = (
            new_start.x as i32 - current_start.x as i32,
            new_start.y as i32 - current_start.y as i32,
        );

        log::trace!("Flipped: {:?}", flipped);
        log::trace!("(dx, dy): ({}, {})", dx, dy);

        let transformed = Coordinate {
            x: (flipped.x as i32 + dx as i32).try_into().unwrap(),
            y: (flipped.y as i32 + dy as i32).try_into().unwrap(),
        };

        State {
            position: transformed,
            facing: match new_edge {
                Edge::Top => Facing::Down,
                Edge::Right => Facing::Left,
                Edge::Bottom => Facing::Up,
                Edge::Left => Facing::Right,
            },
            current_map_part_id: new_map_part_id,
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

#[derive(Debug, Clone, PartialEq)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, PartialEq, Clone)]
struct Connection {
    part_id: usize,
    edge: Edge,
}

type MapPartConnection = HashMap<Facing, Connection>;

#[derive(Debug, PartialEq)]
enum Turn {
    AntiClockwise,
    Clockwise,
}

#[derive(Hash, Eq, Debug, PartialEq, Copy, Clone)]
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
        .map(|row| row.iter().filter(|&&token| token != Token::Blank).count())
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

type Net = nalgebra::Matrix<
    Option<u8>,
    nalgebra::Dyn,
    nalgebra::Dyn,
    nalgebra::VecStorage<Option<u8>, nalgebra::Dyn, nalgebra::Dyn>,
>;

fn create_net(parts: &[MapPart]) -> (Vec<Coordinate>, Net) {
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

    (starts, net)
}

fn create_flat_map(parts: &[MapPart]) -> Map {
    let (starts, net) = create_net(parts);

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

        let up = Connection {
            part_id: get_wrap_part(col_index - 1, &col),
            edge: Edge::Bottom,
        };
        let down = Connection {
            part_id: get_wrap_part(col_index + 1, &col),
            edge: Edge::Top,
        };
        let right = Connection {
            part_id: get_wrap_part(row_index + 1, &row),
            edge: Edge::Left,
        };
        let left = Connection {
            part_id: get_wrap_part(row_index - 1, &row),
            edge: Edge::Right,
        };

        connections.push(HashMap::from([
            (Facing::Up, up),
            (Facing::Right, right),
            (Facing::Down, down),
            (Facing::Left, left),
        ]));
    }
    //
    log::debug!("Connections: {:#?}", connections);

    Map {
        parts: parts.to_vec(),
        connections,
    }
}

fn move_across_map(map: &Map, state: State, distance: Distance) -> State {
    log::debug!("State : {:#?}, distance: {}", state, distance);

    let mut state = state;

    for _ in 0..distance {
        let current_map_part = map.get_part(state.current_map_part_id);
        let facing = state.facing;
        let position = state.position;

        let current_start = current_map_part.get_start();

        let potential_state = match facing {
            Facing::Up => {
                let is_at_top_edge = position.y == current_start.y;
                if is_at_top_edge {
                    map.move_part(state)
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
                    map.move_part(state)
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
                    map.move_part(state)
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
                    map.move_part(state)
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

        let current_part = map.get_part(potential_state.current_map_part_id);
        if current_part.has_wall_at_point(potential_state.position) {
            log::debug!("Has wall at point! {:?}", potential_state.position);
            break;
        }

        state = potential_state;

        log::trace!("State : {:#?}", state);
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
    let initial_state = State {
        position: board_map.get_part(0).start,
        facing: Facing::Right,
        current_map_part_id: 0,
    };

    log::debug!("Initial State: {:?}", initial_state);

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

    let cube_map = create_cube_map(&map_parts);
    let part_2_state = walk(&cube_map, &path);
    let part_2_password = determine_password(&part_2_state);
    println!("{}", part_2_password);
}

fn create_graph(starts: &[Coordinate], net: Net) -> UnGraph<(), (), u8> {
    let mut edges = Vec::new();

    for start in starts {
        let (x, y) = (start.x.try_into().unwrap(), start.y.try_into().unwrap());
        let face_id = net[(y, x)].unwrap();

        const NODES_PER_FACE: u8 = 4;

        const TOP_EDGE_INDEX: u8 = 0;
        const RIGHT_EDGE_INDEX: u8 = 1;
        const BOTTOM_EDGE_INDEX: u8 = 2;
        const LEFT_EDGE_INDEX: u8 = 3;

        let top_node = NODES_PER_FACE * face_id + TOP_EDGE_INDEX;
        let right_node = NODES_PER_FACE * face_id + RIGHT_EDGE_INDEX;
        let bottom_node = NODES_PER_FACE * face_id + BOTTOM_EDGE_INDEX;
        let left_node = NODES_PER_FACE * face_id + LEFT_EDGE_INDEX;

        edges.push((top_node, right_node));
        edges.push((right_node, bottom_node));
        edges.push((bottom_node, left_node));
        edges.push((left_node, top_node));

        if y > 0 {
            if let Some(above_face) = net[(y - 1, x)] {
                let above_face_bottom_edge = NODES_PER_FACE * above_face + BOTTOM_EDGE_INDEX;
                edges.push((top_node, above_face_bottom_edge));
            }
        }

        if x < net.shape().1 - 2 {
            if let Some(right_face) = net[(y, x + 1)] {
                let right_face_left_edge = NODES_PER_FACE * right_face + LEFT_EDGE_INDEX;
                edges.push((right_node, right_face_left_edge));
            }
        }

        if y < net.shape().0 - 2 {
            if let Some(bottom_face) = net[(y + 1, x)] {
                let bottom_face_top_edge = NODES_PER_FACE * bottom_face + TOP_EDGE_INDEX;
                edges.push((bottom_node, bottom_face_top_edge));
            }
        }

        if x > 0 {
            if let Some(left_face) = net[(y, x - 1)] {
                let left_face_right_edge = NODES_PER_FACE * left_face + RIGHT_EDGE_INDEX;
                edges.push((left_node, left_face_right_edge));
            }
        }
    }

    let graph = UnGraph::<(), (), u8>::from_edges(&edges);

    log::debug!("{:?}", graph);

    graph
}

fn create_cube_map(map_parts: &[MapPart]) -> Map {
    for part in map_parts {
        log::debug!("Start: {:?}", part.start);
        log::debug!("{}", part.map);
    }

    let (starts, net) = create_net(map_parts);
    let graph = create_graph(&starts, net);
    let connections = get_connections(graph);

    Map {
        parts: map_parts.to_vec(),
        connections,
    }
}

fn get_connections(graph: UnGraph<(), (), u8>) -> Vec<MapPartConnection> {
    let graph_to_connections_map: Vec<(UnGraph<(), (), u8>, Vec<MapPartConnection>)> = vec![
        (
            UnGraph::<(), (), u8>::from_edges(&[
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0),
                (2, 8),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 4),
                (7, 1),
                (8, 9),
                (9, 10),
                (10, 11),
                (11, 8),
                (8, 2),
                (10, 16),
                (12, 13),
                (13, 14),
                (14, 15),
                (15, 12),
                (13, 19),
                (16, 17),
                (17, 18),
                (18, 19),
                (19, 16),
                (16, 10),
                (19, 13),
                (20, 21),
                (21, 22),
                (22, 23),
                (23, 20),
                (20, 14),
            ]),
            vec![
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 5,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 1,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 2,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 3,
                            edge: Edge::Left,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 5,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 4,
                            edge: Edge::Right,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 2,
                            edge: Edge::Right,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 0,
                            edge: Edge::Right,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 0,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 1,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 4,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 3,
                            edge: Edge::Top,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 2,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 4,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 5,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 0,
                            edge: Edge::Left,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 2,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 1,
                            edge: Edge::Right,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 5,
                            edge: Edge::Right,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 3,
                            edge: Edge::Right,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 3,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 4,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 1,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 0,
                            edge: Edge::Top,
                        },
                    ),
                ]),
            ],
        ),
        (
            UnGraph::<(), (), u8>::from_edges(&[
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0),
                (2, 12),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 4),
                (5, 11),
                (8, 9),
                (9, 10),
                (10, 11),
                (11, 8),
                (9, 15),
                (11, 5),
                (12, 13),
                (13, 14),
                (14, 15),
                (15, 12),
                (12, 2),
                (15, 9),
                (16, 17),
                (17, 18),
                (18, 19),
                (19, 16),
                (16, 14),
                (20, 21),
                (21, 22),
                (22, 23),
                (23, 20),
                (23, 17),
            ]),
            vec![
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 1,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 5,
                            edge: Edge::Right,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 3,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 2,
                            edge: Edge::Top,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 0,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 2,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 4,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 5,
                            edge: Edge::Bottom,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 0,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 3,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 4,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 1,
                            edge: Edge::Right,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 0,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 5,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 4,
                            edge: Edge::Top,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 2,
                            edge: Edge::Right,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 3,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 5,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 1,
                            edge: Edge::Bottom,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 2,
                            edge: Edge::Bottom,
                        },
                    ),
                ]),
                HashMap::from([
                    (
                        Facing::Up,
                        Connection {
                            part_id: 3,
                            edge: Edge::Right,
                        },
                    ),
                    (
                        Facing::Down,
                        Connection {
                            part_id: 1,
                            edge: Edge::Left,
                        },
                    ),
                    (
                        Facing::Left,
                        Connection {
                            part_id: 4,
                            edge: Edge::Right,
                        },
                    ),
                    (
                        Facing::Right,
                        Connection {
                            part_id: 0,
                            edge: Edge::Right,
                        },
                    ),
                ]),
            ],
        ),
    ];

    match graph_to_connections_map
        .into_iter()
        .find(|(other_graph, _)| is_isomorphic(&graph, other_graph))
    {
        Some((_, connections)) => connections.to_vec(),
        None => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use nalgebra::dmatrix;

    use crate::Turn::{AntiClockwise, Clockwise};
    use crate::{
        determine_password, parse_board_map, single_90_clockwise_rotation, walk, Connection,
        Coordinate, Edge, Facing, Map, MapPart, Path, State, Step, Token,
    };

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_test_parts() -> (Vec<MapPart>, Vec<Step>) {
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

        (parts, path)
    }

    fn get_part_1_test_input() -> (Map, Path) {
        let (parts, path) = get_test_parts();

        let connections = vec![
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 4,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 0,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 3,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 0,
                        edge: Edge::Right,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 1,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 2,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 1,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 3,
                        edge: Edge::Right,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 2,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 3,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 2,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 1,
                        edge: Edge::Right,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 0,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 1,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 4,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 2,
                        edge: Edge::Right,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 3,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 5,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 0,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 5,
                        edge: Edge::Right,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 5,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 5,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 4,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 4,
                        edge: Edge::Right,
                    },
                ),
            ]),
        ];

        let map = Map { parts, connections };

        (map, path)
    }

    fn get_part_2_test_input() -> (Map, Path) {
        let (parts, path) = get_test_parts();

        let connections = vec![
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 1,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 5,
                        edge: Edge::Right,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 3,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 2,
                        edge: Edge::Top,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 0,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 2,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 4,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 5,
                        edge: Edge::Bottom,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 0,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 3,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 4,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 1,
                        edge: Edge::Right,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 0,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 5,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 4,
                        edge: Edge::Top,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 2,
                        edge: Edge::Right,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 3,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 5,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 1,
                        edge: Edge::Bottom,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 2,
                        edge: Edge::Bottom,
                    },
                ),
            ]),
            HashMap::from([
                (
                    Facing::Up,
                    Connection {
                        part_id: 3,
                        edge: Edge::Right,
                    },
                ),
                (
                    Facing::Down,
                    Connection {
                        part_id: 1,
                        edge: Edge::Left,
                    },
                ),
                (
                    Facing::Left,
                    Connection {
                        part_id: 4,
                        edge: Edge::Right,
                    },
                ),
                (
                    Facing::Right,
                    Connection {
                        part_id: 0,
                        edge: Edge::Right,
                    },
                ),
            ]),
        ];

        let map = Map { parts, connections };

        (map, path)
    }

    #[test]
    fn test_parse_board_map() {
        init();
        let input = include_str!("../test.txt");
        let (expected_map, expected_path) = get_part_1_test_input();
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
        let (map, path) = get_part_1_test_input();
        let expected = State {
            position: Coordinate { x: 8, y: 6 },
            facing: Facing::Right,
            current_map_part_id: 2,
        };
        let actual = walk(&map, &path);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_walk_part_2() {
        init();
        let (map, path) = get_part_2_test_input();
        let expected = Coordinate { x: 7, y: 5 };
        let actual = walk(&map, &path).position;
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

    #[test]
    fn test_rotate() {
        let input = Coordinate { x: 12, y: 6 };
        let start = Coordinate { x: 9, y: 5 };
        let expected_1 = Coordinate { x: 11, y: 8 };
        let actual_1 = single_90_clockwise_rotation(input, 4, start);

        assert_eq!(expected_1, actual_1);

        let expected_2 = Coordinate { x: 9, y: 7 };
        let actual_2 = single_90_clockwise_rotation(actual_1, 4, start);

        assert_eq!(expected_2, actual_2);

        let expected_3 = Coordinate { x: 10, y: 5 };
        let actual_3 = single_90_clockwise_rotation(actual_2, 4, start);

        assert_eq!(expected_3, actual_3);
    }
}

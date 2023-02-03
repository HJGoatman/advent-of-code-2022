use nalgebra::DMatrix;
use std::{env, fs};

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

#[derive(Debug, PartialEq)]
struct MapPart {
    start: Coordinate,
    map: DMatrix<u8>,
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
        let x = coordinate.x - start.x;
        let y = coordinate.y - start.y;

        log::trace!("Rect positions: ({}, {})", x, y);

        self.map[(y as usize, x as usize)] == 1
    }
}

type BoardMap = Vec<MapPart>;

#[derive(Debug, PartialEq)]
enum Turn {
    AntiClockwise,
    Clockwise,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl TryFrom<Ordinate> for Facing {
    type Error = ();

    fn try_from(v: Ordinate) -> Result<Self, Self::Error> {
        match v {
            x if x == Facing::Right as Ordinate => Ok(Facing::Right),
            x if x == Facing::Down as Ordinate => Ok(Facing::Down),
            x if x == Facing::Left as Ordinate => Ok(Facing::Left),
            x if x == Facing::Up as Ordinate => Ok(Facing::Up),
            _ => Err(()),
        }
    }
}

type Path = Vec<(Turn, Distance)>;

#[derive(Debug, PartialEq, Copy, Clone)]
struct State {
    position: Coordinate,
    facing: Facing,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn build_map_part(ncols: usize, matrix: Vec<u8>, x: Ordinate, y: Ordinate) -> MapPart {
    log::debug!("Columns: {}, Items: {}", ncols, matrix.len());
    let map = DMatrix::from_vec(ncols, matrix.len() / ncols, matrix).transpose();

    MapPart {
        start: Coordinate { x, y },
        map,
    }
}

fn parse_map(input: &str) -> BoardMap {
    let mut parts = Vec::new();

    let mut current_line: Option<(Ordinate, Ordinate, usize)> = None;
    let mut current_values = Vec::new();

    for (col, row_str) in input.split('\n').enumerate() {
        let mut row_start = 0;

        let mut row_values: Vec<u8> = row_str
            .chars()
            .skip_while(|&c| {
                row_start += 1;
                c == ' '
            })
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => unreachable!(),
            })
            .collect();

        if let Some((line_start, line_number, line_length)) = current_line {
            if row_start != line_start || row_values.len() != line_length {
                // New Rectangle
                let mut map_values = Vec::new();
                map_values.append(&mut current_values);
                let map_part = build_map_part(line_length, map_values, line_start, line_number);
                parts.push(map_part);
            }
        }

        if current_values.is_empty() {
            current_line = Some((row_start, col as Ordinate + 1, row_values.len()));
        }

        current_values.append(&mut row_values);
    }

    if let Some((line_start, line_number, line_length)) = current_line {
        let map_part = build_map_part(line_length, current_values, line_start, line_number);
        parts.push(map_part);
    }

    parts
}

fn parse_path(input: &str) -> Path {
    let mut path = Vec::new();
    let mut current_direction = Turn::Clockwise;
    let mut distance_str: Vec<char> = Vec::new();
    for c in input.chars() {
        if c != 'R' && c != 'L' {
            distance_str.push(c);
            continue;
        }

        let distance = distance_str
            .iter()
            .collect::<String>()
            .parse::<Ordinate>()
            .unwrap();

        path.push((current_direction, distance));
        distance_str.clear();

        current_direction = match c {
            'L' => Turn::AntiClockwise,
            'R' => Turn::Clockwise,
            _ => unreachable!(),
        };
    }

    path.push((
        current_direction,
        distance_str
            .iter()
            .collect::<String>()
            .parse::<Ordinate>()
            .unwrap(),
    ));

    path
}

fn parse_board_map(input: &str) -> (BoardMap, Path) {
    let mut split = input.split("\n\n");
    let map = parse_map(split.next().unwrap());
    let path = parse_path(split.next().unwrap());
    (map, path)
}

fn contains_point(rect: &impl Rectangular, point: Coordinate) -> bool {
    let rect_start = rect.get_start();
    point.x >= rect_start.x
        && point.x < rect_start.x + rect.get_width()
        && point.y >= rect_start.y
        && point.y < rect_start.y + rect.get_height()
}

type MoveFunction = Box<dyn Fn(Ordinate, Ordinate) -> Ordinate>;

fn move_in_rectangle(
    rects: &[impl Rectangular],
    start: Coordinate,
    direction: Facing,
    distance: Ordinate,
) -> Coordinate {
    log::debug!("At {:?}, going {:?} {} places.", start, direction, distance);
    let (mut ordinate, set_ordinate): (Ordinate, Box<dyn Fn(Coordinate, Ordinate) -> Coordinate>) =
        match direction {
            Facing::Left | Facing::Right => (
                start.x,
                Box::new(|coordinate, val| Coordinate {
                    x: val,
                    y: coordinate.y,
                }),
            ),
            _ => (
                start.y,
                Box::new(|coordinate, val| Coordinate {
                    x: coordinate.x,
                    y: val,
                }),
            ),
        };

    let (direction_func, reverse_func): (MoveFunction, MoveFunction) = match direction {
        Facing::Left | Facing::Up => (Box::new(|a, b| a - b), Box::new(|a, b| a + b)),
        _ => (Box::new(|a, b| a + b), Box::new(|a, b| a - b)),
    };

    for _ in 0..distance {
        let rect = rects
            .iter()
            .find(|rect| contains_point(*rect, start))
            .unwrap();

        let edge: Ordinate = match direction {
            Facing::Up => rect.get_start().y,
            Facing::Down => rect.get_start().y + rect.get_height() - 1,
            Facing::Left => rect.get_start().x,
            Facing::Right => rect.get_start().x + rect.get_width() - 1,
        };

        let mut can_move_rectangles = false;
        if ordinate > 0 {
            let potential_ord = direction_func(ordinate, 1);

            can_move_rectangles = rects
                .iter()
                .any(|rec| contains_point(rec, set_ordinate(start, potential_ord)));
        }

        let mut next_ordinate = ordinate;

        if ordinate == edge && !can_move_rectangles {
            // TODO: More Optimised Code that doesn't move one space at a time!
            while rects
                .iter()
                .any(|rec| contains_point(rec, set_ordinate(start, reverse_func(next_ordinate, 1))))
            {
                next_ordinate = reverse_func(next_ordinate, 1);

                if next_ordinate == 0 {
                    break;
                }
            }
        } else {
            next_ordinate = direction_func(ordinate, 1);
        }

        let next_coordinate = set_ordinate(start, next_ordinate);
        let next_rect = rects
            .iter()
            .find(|&rec| contains_point(rec, next_coordinate))
            .unwrap();

        if next_rect.has_wall_at_point(next_coordinate) {
            log::debug!("Hit a wall :(");
            break;
        }

        ordinate = next_ordinate;
        log::trace!("Ordinate: {}", ordinate);
    }

    set_ordinate(start, ordinate)
}

fn take_step(board_map: &[MapPart], state: &State, (turn, distance): &(Turn, Distance)) -> State {
    let facing = match turn {
        Turn::Clockwise => (((state.facing as Ordinate) + 1) % 4).try_into().unwrap(),
        Turn::AntiClockwise => {
            if state.facing == Facing::Right {
                Facing::Up
            } else {
                ((state.facing as Ordinate) - 1).try_into().unwrap()
            }
        }
    };

    let new_position = move_in_rectangle(board_map, state.position, facing, *distance);

    State {
        position: new_position,
        facing,
    }
}

fn walk(board_map: &[MapPart], path: &[(Turn, Distance)]) -> State {
    for map_part in board_map {
        log::debug!("{:?}", map_part.start);
        log::debug!("{}", map_part.map);
    }

    let initial_state = State {
        position: board_map.get(0).unwrap().start,
        facing: Facing::Up,
    };

    let mut current_state = initial_state;
    for step in path.iter() {
        log::trace!("{:?}, {:?}", current_state, step);
        current_state = take_step(board_map, &current_state, step);
    }

    current_state
}

fn determine_password(state: &State) -> Ordinate {
    1000 * state.position.y + 4 * state.position.x + state.facing as Ordinate
}

fn main() {
    env_logger::init();
    let input = load_input();
    let (board_map, path) = parse_board_map(&input);
    let final_state = walk(&board_map, &path);
    let final_password = determine_password(&final_state);
    println!("{}", final_password);
}

#[cfg(test)]
mod tests {
    use nalgebra::dmatrix;

    use crate::Turn::{AntiClockwise, Clockwise};
    use crate::{
        determine_password, move_in_rectangle, parse_board_map, walk, BoardMap, Coordinate, Facing,
        MapPart, Ordinate, Path, Rectangular, State,
    };

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_test_input() -> (BoardMap, Path) {
        let map = vec![
            MapPart {
                start: Coordinate { x: 9, y: 1 },
                map: dmatrix![
                    0, 0, 0, 1;
                    0, 1, 0, 0;
                    1, 0, 0, 0;
                    0, 0, 0, 0;
                ],
            },
            MapPart {
                start: Coordinate { x: 1, y: 5 },
                map: dmatrix![
                    0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1;
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0;
                    0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0;
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0;
                ],
            },
            MapPart {
                start: Coordinate { x: 9, y: 9 },
                map: dmatrix![
                    0, 0, 0, 1, 0, 0, 0, 0;
                    0, 0, 0, 0, 0, 1, 0, 0;
                    0, 1, 0, 0, 0, 0, 0, 0;
                    0, 0, 0, 0, 0, 0, 1, 0;
                ],
            },
        ];
        let path = vec![
            (Clockwise, 10),
            (Clockwise, 5),
            (AntiClockwise, 5),
            (Clockwise, 10),
            (AntiClockwise, 4),
            (Clockwise, 5),
            (AntiClockwise, 5),
        ];

        (map, path)
    }

    #[test]
    fn test_parse_board_map() {
        init();
        let input = include_str!("../test.txt");
        let (expected_map, expected_path) = get_test_input();
        let (actual_map, actual_path) = parse_board_map(input);
        for (expected_part, actual_part) in expected_map.iter().zip(actual_map.iter()) {
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
        };
        let actual = walk(&map, &path);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_determine_password() {
        let input = State {
            position: Coordinate { x: 8, y: 6 },
            facing: Facing::Right,
        };
        let expected = 6032;
        let actual = determine_password(&input);
        assert_eq!(expected, actual);
    }

    #[derive(Debug, Clone, Copy)]
    struct TestRectangle {
        start: Coordinate,
        width: Ordinate,
        height: Ordinate,
    }

    impl Rectangular for TestRectangle {
        fn get_start(&self) -> Coordinate {
            self.start
        }

        fn get_width(&self) -> crate::Ordinate {
            self.width
        }

        fn get_height(&self) -> crate::Ordinate {
            self.height
        }

        fn has_wall_at_point(&self, _: Coordinate) -> bool {
            false
        }
    }

    #[test]
    fn test_wrap_around_rectangle() {
        let test_rectangle = TestRectangle {
            start: Coordinate { x: 0, y: 0 },
            width: 4,
            height: 6,
        };

        let actual_1 =
            move_in_rectangle(&[test_rectangle], Coordinate { x: 1, y: 1 }, Facing::Up, 8);
        let expected_1 = Coordinate { x: 1, y: 5 };
        assert_eq!(expected_1, actual_1);

        let actual_2 = move_in_rectangle(
            &[test_rectangle],
            Coordinate { x: 3, y: 3 },
            Facing::Right,
            1,
        );
        let expected_2 = Coordinate { x: 0, y: 3 };
        assert_eq!(expected_2, actual_2);

        let actual_3 = move_in_rectangle(
            &[test_rectangle],
            Coordinate { x: 2, y: 5 },
            Facing::Down,
            3,
        );
        let expected_3 = Coordinate { x: 2, y: 2 };
        assert_eq!(expected_3, actual_3);

        let actual_4 = move_in_rectangle(
            &[test_rectangle],
            Coordinate { x: 3, y: 5 },
            Facing::Left,
            4,
        );
        let expected_4 = Coordinate { x: 3, y: 5 };
        assert_eq!(expected_4, actual_4);
    }

    #[test]
    fn test_moving_between_rectangles() {
        let rectangle_1 = TestRectangle {
            start: Coordinate { x: 0, y: 0 },
            width: 5,
            height: 5,
        };
        let rectangle_2 = TestRectangle {
            start: Coordinate { x: 2, y: 5 },
            width: 5,
            height: 5,
        };

        let rectangles = [rectangle_1, rectangle_2];

        let actual_1 = move_in_rectangle(&rectangles, Coordinate { x: 3, y: 3 }, Facing::Down, 4);
        let expected_1 = Coordinate { x: 3, y: 7 };
        assert_eq!(expected_1, actual_1);

        let actual_2 = move_in_rectangle(&rectangles, Coordinate { x: 3, y: 3 }, Facing::Up, 4);
        let expected_2 = Coordinate { x: 3, y: 9 };
        assert_eq!(expected_2, actual_2);
    }
}

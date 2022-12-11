use std::env;
use std::{collections::HashSet, fs};

type Distance = i16;

type Position = (Distance, Distance);

#[derive(Debug, PartialEq)]
enum Motion {
    Right(Distance),
    Up(Distance),
    Left(Distance),
    Down(Distance),
}

#[derive(Debug, PartialEq)]
struct State {
    visited: HashSet<Position>,
    head: Position,
    tail: Position,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_motion(input: &str) -> Motion {
    let mut split = input.split(" ");
    let (direction, distance) = (split.next().unwrap(), split.next().unwrap());
    let distance_value = distance.parse::<Distance>().unwrap();

    match direction {
        "R" => Motion::Right(distance_value),
        "U" => Motion::Up(distance_value),
        "L" => Motion::Left(distance_value),
        "D" => Motion::Down(distance_value),
        _ => panic!("Unknown Motion"),
    }
}

fn parse_motions(input: &str) -> Vec<Motion> {
    input
        .split("\n")
        .filter(|a| a != &"")
        .map(parse_motion)
        .collect()
}

fn get_initial_state() -> State {
    State {
        visited: HashSet::new(),
        head: (0, 0),
        tail: (0, 0),
    }
}

fn move_tail(tail: Position, head: Position) -> Position {
    let x_diff = tail.0 - head.0;
    let y_diff = tail.1 - head.1;

    let move_x = if x_diff < 0 { 1 } else { -1 };

    let move_y = if y_diff < 0 { 1 } else { -1 };

    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        return tail;
    }

    if (x_diff.abs() == 0) && (y_diff.abs() == 2) {
        return (tail.0, tail.1 + move_y);
    }
    if (x_diff.abs() == 2) && (y_diff.abs() == 0) {
        return (tail.0 + move_x, tail.1);
    }

    (tail.0 + move_x, tail.1 + move_y)
}

fn move_head<'a>(
    state: &'a mut State,
    distance: &Distance,
    movement: &dyn Fn(Position) -> Position,
) -> &'a mut State {
    state.visited.insert(state.tail.clone());

    for _ in 0..*distance {
        state.head = movement(state.head);
        state.tail = move_tail(state.tail, state.head);
        state.visited.insert(state.tail.clone());
    }

    state
}

fn run_motion<'a>(state: &'a mut State, motion: &Motion) -> &'a mut State {
    match motion {
        Motion::Right(dist) => move_head(state, dist, &|(x, y)| (x + 1, y)),
        Motion::Up(dist) => move_head(state, dist, &|(x, y)| (x, y + 1)),
        Motion::Left(dist) => move_head(state, dist, &|(x, y)| (x - 1, y)),
        Motion::Down(dist) => move_head(state, dist, &|(x, y)| (x, y - 1)),
    }
}

fn main() {
    let input = load_input();
    let motions = parse_motions(&input);
    let mut initial_state = get_initial_state();
    let final_state = motions.iter().fold(&mut initial_state, run_motion);
    let number_tail_visited = final_state.visited.len();
    println!("{}", number_tail_visited);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{parse_motions, run_motion, Motion, State};

    #[test]
    fn test_parse_motions() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
        let expected = vec![
            Motion::Right(4),
            Motion::Up(4),
            Motion::Left(3),
            Motion::Down(1),
            Motion::Right(4),
            Motion::Down(1),
            Motion::Left(5),
            Motion::Right(2),
        ];
        let actual = parse_motions(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_direct_motions() {
        let mut input_state_right = State {
            visited: HashSet::new(),
            head: (1, 0),
            tail: (0, 0),
        };
        let input_motion_right = Motion::Right(1);
        let expected_state_right = State {
            visited: HashSet::from([(0, 0), (1, 0)]),
            head: (2, 0),
            tail: (1, 0),
        };
        let actual_state_right = run_motion(&mut input_state_right, &input_motion_right);
        assert_eq!(&expected_state_right, actual_state_right);

        let mut input_state_left = State {
            visited: HashSet::new(),
            head: (0, -1),
            tail: (0, 0),
        };
        let input_motion_left = Motion::Down(1);
        let expected_state_left = State {
            visited: HashSet::from([(0, 0), (0, -1)]),
            head: (0, -2),
            tail: (0, -1),
        };

        let actual_state_left = run_motion(&mut input_state_left, &input_motion_left);
        assert_eq!(&expected_state_left, actual_state_left);
    }

    #[test]
    fn test_diagonal_motions() {
        let mut input_state_diag_up = State {
            visited: HashSet::new(),
            head: (1, 1),
            tail: (0, 0),
        };
        let input_motion_diag_up = Motion::Up(1);
        let expected_state_diag_up = State {
            visited: HashSet::from([(0, 0), (1, 1)]),
            head: (1, 2),
            tail: (1, 1),
        };

        let actual_state_diag_up = run_motion(&mut input_state_diag_up, &input_motion_diag_up);
        assert_eq!(&expected_state_diag_up, actual_state_diag_up);

        let mut input_state_diag_right = State {
            visited: HashSet::new(),
            head: (1, 1),
            tail: (0, 0),
        };
        let input_motion_diag_right = Motion::Right(1);
        let expected_state_diag_right = State {
            visited: HashSet::from([(0, 0), (1, 1)]),
            head: (2, 1),
            tail: (1, 1),
        };
        let actual_state_diag_right =
            run_motion(&mut input_state_diag_right, &input_motion_diag_right);
        assert_eq!(&expected_state_diag_right, actual_state_diag_right);
    }
}

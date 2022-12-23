use std::{collections::HashSet, env, fs};

type Ordinate = u8;

#[derive(Debug, PartialEq)]
struct Position {
    x: Ordinate,
    y: Ordinate,
    z: Ordinate,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_position(input: &str) -> Position {
    let mut split = input.split(',');

    Position {
        x: split.next().unwrap().parse::<Ordinate>().unwrap(),
        y: split.next().unwrap().parse::<Ordinate>().unwrap(),
        z: split.next().unwrap().parse::<Ordinate>().unwrap(),
    }
}

fn parse_input(input: &str) -> Vec<Position> {
    input.split('\n').map(parse_position).collect()
}

fn count_open_facing_sides(positions: &[Position]) -> usize {
    let mut combinations = Vec::new();
    for x in [0, 1] {
        for y in [0, 1] {
            for z in [0, 1] {
                combinations.push([x, y, z]);
            }
        }
    }

    let mut faces_diff: Vec<Vec<[Ordinate; 3]>> = Vec::new();
    for part in [0, 1, 2] {
        for val in [0, 1] {
            let diff: Vec<[Ordinate; 3]> = combinations
                .iter()
                .cloned()
                .filter(|combs| combs[part] == val)
                .collect();

            faces_diff.push(diff);
        }
    }

    let mut number_of_duplicate_faces = 0;

    let mut faces = Vec::new();
    for face_diff in faces_diff {
        for position in positions {
            let mut face = HashSet::new();

            for vertex_diff in &face_diff {
                let x = vertex_diff[0];
                let y = vertex_diff[1];
                let z = vertex_diff[2];

                face.insert((position.x + x, position.y + y, position.z + z));
            }

            faces.push(face);
        }
    }

    for i in 0..(faces.len() - 1) {
        for j in (i + 1)..faces.len() {
            if faces.get(i).unwrap() == faces.get(j).unwrap() {
                number_of_duplicate_faces += 2;
            }
        }
    }

    positions.len() * 6 - number_of_duplicate_faces
}

fn main() {
    let input = load_input();
    let positions = parse_input(&input);
    let num_open_facing_sides = count_open_facing_sides(&positions);
    println!("{}", num_open_facing_sides);
}

#[cfg(test)]
mod tests {
    use crate::{count_open_facing_sides, parse_input, Position};

    fn get_test_positions() -> Vec<Position> {
        vec![
            Position { x: 2, y: 2, z: 2 },
            Position { x: 1, y: 2, z: 2 },
            Position { x: 3, y: 2, z: 2 },
            Position { x: 2, y: 1, z: 2 },
            Position { x: 2, y: 3, z: 2 },
            Position { x: 2, y: 2, z: 1 },
            Position { x: 2, y: 2, z: 3 },
            Position { x: 2, y: 2, z: 4 },
            Position { x: 2, y: 2, z: 6 },
            Position { x: 1, y: 2, z: 5 },
            Position { x: 3, y: 2, z: 5 },
            Position { x: 2, y: 1, z: 5 },
            Position { x: 2, y: 3, z: 5 },
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../test.txt");
        let expected = get_test_positions();
        let actual = parse_input(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_open_facing_sides_small() {
        let input = vec![Position { x: 1, y: 1, z: 1 }, Position { x: 2, y: 1, z: 1 }];
        let expected = 10;
        let actual = count_open_facing_sides(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_open_facing_sides_large() {
        let input = get_test_positions();
        let expected = 64;
        let actual = count_open_facing_sides(&input);
        assert_eq!(expected, actual);
    }
}

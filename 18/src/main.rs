mod octree;
mod point;

use std::{collections::HashSet, env, fs, num::ParseIntError};

use octree::{OctreeError, Tree};
use point::{Ordinate, Point};

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_input(input: &str) -> Result<Vec<Point>, ParseIntError> {
    input
        .split('\n')
        .map(|s| s.parse::<Point>())
        .collect::<Result<Vec<Point>, ParseIntError>>()
}

fn get_adjacent_positions(position: Point, upper_bound: Ordinate) -> Vec<Point> {
    let mut adjacent_positions = Vec::new();

    if position.x < upper_bound {
        adjacent_positions.push(Point {
            x: position.x + 1,
            ..position
        })
    }

    if position.y < upper_bound {
        adjacent_positions.push(Point {
            y: position.y + 1,
            ..position
        })
    }

    if position.z < upper_bound {
        adjacent_positions.push(Point {
            z: position.z + 1,
            ..position
        })
    }

    if position.x > 0 {
        adjacent_positions.push(Point {
            x: position.x - 1,
            ..position
        })
    }

    if position.y > 0 {
        adjacent_positions.push(Point {
            y: position.y - 1,
            ..position
        })
    }

    if position.z > 0 {
        adjacent_positions.push(Point {
            z: position.z - 1,
            ..position
        })
    }

    adjacent_positions
}

fn count_open_facing_sides(positions: &[Point], tree: &Tree) -> usize {
    let mut total_connections = 0;

    for position in positions {
        let position = *position;
        let adjacent_positions = get_adjacent_positions(position, 100);

        for adjacent_position in adjacent_positions {
            if let Ok(true) = tree.find(adjacent_position) {
                total_connections += 1
            }
        }
    }

    6 * positions.len() - total_connections
}

fn count_outside_faces(start: Point, tree: &Tree) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut stack = Vec::new();
    let mut total_connections = 0;

    stack.push(start);

    let max = tree.get_max_point().x + 5;

    while !stack.is_empty() {
        let current = stack.pop().unwrap();

        if !visited.contains(&current) {
            match tree.find(current) {
                Ok(true) => {
                    total_connections += 1;
                }
                _ => {
                    visited.insert(current);
                    for position in get_adjacent_positions(current, max) {
                        stack.push(position);
                    }
                }
            }
        }
    }

    total_connections
}

fn main() {
    env_logger::init();

    let input = load_input();
    let positions = parse_input(&input).expect("Unable to Parse Input!");
    let positions: Vec<Point> = positions
        .iter()
        .map(|pos| Point {
            x: pos.x + 1,
            y: pos.y + 1,
            z: pos.z + 1,
        })
        .collect();
    let tree = create_tree(positions.clone()).expect("Unable to Create Tree!");
    log::debug!("{:?}", tree);
    let num_open_facing_sides = count_open_facing_sides(&positions, &tree);
    println!("{}", num_open_facing_sides);

    let start = Point { x: 0, y: 0, z: 0 };
    let num_outside_faces = count_outside_faces(start, &tree);
    println!("{}", num_outside_faces);
}

fn create_tree(positions: Vec<Point>) -> Result<Tree, OctreeError> {
    let min_x = positions.iter().map(|p| p.x).min().unwrap();
    let min_y = positions.iter().map(|p| p.y).min().unwrap();
    let min_z = positions.iter().map(|p| p.z).min().unwrap();

    let min = *[min_x, min_y, min_z].iter().min().unwrap();

    let min_point = Point {
        x: min,
        y: min,
        z: min,
    };

    let max_x = positions.iter().map(|p| p.x).max().unwrap();
    let max_y = positions.iter().map(|p| p.y).max().unwrap();
    let max_z = positions.iter().map(|p| p.z).max().unwrap();

    let max = *[max_x, max_y, max_z].iter().max().unwrap();

    const TWO: Ordinate = 2;
    let power_of_two = (0..).map(|exp| TWO.pow(exp)).find(|v| *v >= max).unwrap();

    let max = min + power_of_two;

    let max_point = Point {
        x: max,
        y: max,
        z: max,
    };

    let mut tree = Tree::new(min_point, max_point);

    for position in positions {
        tree.insert(position)?;
    }

    Ok(tree)
}

#[cfg(test)]
mod tests {
    use crate::{count_open_facing_sides, create_tree, parse_input, Point};

    fn get_test_positions() -> Vec<Point> {
        vec![
            Point { x: 2, y: 2, z: 2 },
            Point { x: 1, y: 2, z: 2 },
            Point { x: 3, y: 2, z: 2 },
            Point { x: 2, y: 1, z: 2 },
            Point { x: 2, y: 3, z: 2 },
            Point { x: 2, y: 2, z: 1 },
            Point { x: 2, y: 2, z: 3 },
            Point { x: 2, y: 2, z: 4 },
            Point { x: 2, y: 2, z: 6 },
            Point { x: 1, y: 2, z: 5 },
            Point { x: 3, y: 2, z: 5 },
            Point { x: 2, y: 1, z: 5 },
            Point { x: 2, y: 3, z: 5 },
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../test.txt");
        let expected = get_test_positions();
        let actual = parse_input(&input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_open_facing_sides_small() {
        let input = vec![Point { x: 1, y: 1, z: 1 }, Point { x: 2, y: 1, z: 1 }];
        let expected = 10;
        let tree = create_tree(input.clone()).unwrap();
        let actual = count_open_facing_sides(&input, &tree);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_open_facing_sides_large() {
        let input = get_test_positions();
        let expected = 64;
        let tree = create_tree(input.clone()).unwrap();
        let actual = count_open_facing_sides(&input, &tree);
        assert_eq!(expected, actual);
    }
}

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    env, fs,
};

use nalgebra::DMatrix;

type Elevation = i8;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct HeightMap {
    map: DMatrix<Elevation>,
    start: Point,
    end: Point,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn convert_to_elevation(c: &char) -> Elevation {
    let char_pairs: HashMap<char, Elevation> = ('a'..='z').zip(0..26).collect();
    match c {
        'a'..='z' => *char_pairs.get(c).unwrap(),
        'S' => 0,
        'E' => 25,
        _ => panic!("Unknown char"),
    }
}

fn index_to_point(index: usize, rows: usize) -> Point {
    let x = index / rows;
    let y = index % rows;
    Point { x, y }
}

fn to_char_array(lines: &[String]) -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();

    for i in 0..lines[0].len() {
        for line in lines.iter() {
            chars.push(line.chars().nth(i).unwrap());
        }
    }

    chars
}

fn parse_height_map(input: &str) -> HeightMap {
    let split: Vec<String> = input
        .split('\n')
        .filter(|a| a != &"")
        .map(String::from)
        .collect();

    let rows = &split.len();
    let columns = &split[0].len();

    let values: Vec<char> = to_char_array(&split);

    let start_index = values.iter().position(|c| c == &'S').unwrap();
    let end_index = values.iter().position(|c| c == &'E').unwrap();
    let start = index_to_point(start_index, *rows);
    let end = index_to_point(end_index, *rows);

    let elevations: Vec<Elevation> = values.iter().map(convert_to_elevation).collect();

    let map = DMatrix::from_vec(*rows, *columns, elevations);

    log::debug!("Map: {}", map);

    HeightMap { map, start, end }
}

fn get_next_steps(map: &DMatrix<Elevation>, position: &Point) -> Vec<Point> {
    let mut points = Vec::new();

    let x_i = position.x as i32;
    let y_i = position.y as i32;

    for (x, y) in [
        (x_i, y_i - 1),
        (x_i, y_i + 1),
        (x_i - 1, y_i),
        (x_i + 1, y_i),
    ] {
        if x >= map.ncols().try_into().unwrap() || x < 0 {
            continue;
        }

        if y >= map.nrows().try_into().unwrap() || y < 0 {
            continue;
        }

        let current_elevation = map[(position.y, position.x)] as i16;
        let next_elevation = map[(y as usize, x as usize)] as i16;

        if current_elevation - next_elevation < -1 {
            continue;
        }

        points.push(Point {
            x: x as usize,
            y: y as usize,
        })
    }

    points
}

fn get_all_points(cols: usize, rows: usize) -> HashMap<Point, u32> {
    let mut points = HashMap::new();

    for x in 0..cols {
        for y in 0..rows {
            points.insert(Point { x, y }, u32::MAX);
        }
    }

    points
}

fn dijkstras(map: &DMatrix<Elevation>, start: Point, end: Option<Point>) -> HashMap<Point, u32> {
    let mut dist: HashMap<Point, u32> = get_all_points(map.ncols(), map.nrows());

    let mut heap = BinaryHeap::new();

    *dist.get_mut(&start).unwrap() = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    log::debug!("Starting");

    while let Some(State { cost, position }) = heap.pop() {
        log::debug!("Position {:?}, Cost: {}", position, cost);

        if end.is_some() && position == end.unwrap() {
            return dist;
        }

        if cost > dist[&position] {
            continue;
        }

        for edge in get_next_steps(&map, &position) {
            log::debug!("Edge: {:?}", edge);
            let next = State {
                cost: cost + 1,
                position: edge,
            };

            if next.cost < *dist.get(&next.position).unwrap() {
                heap.push(next);

                *dist.get_mut(&next.position).unwrap() = next.cost;
            }
        }
    }

    dist
}

fn find_shortest_path(height_map: &HeightMap) -> Option<u32> {
    let dist: HashMap<Point, u32> =
        dijkstras(&height_map.map, height_map.start, Some(height_map.end));
    let cost = dist.get(&height_map.end)?;
    Some(*cost)
}

fn get_start_points(map: &DMatrix<Elevation>) -> Vec<Point> {
    let mut points = Vec::new();

    for (x, column) in map.column_iter().enumerate() {
        for (y, val) in column.iter().enumerate() {
            if *val == 1 {
                points.push(Point { x, y });
            }
        }
    }

    points
}

fn find_single_destination_shortest_path(height_map: &HeightMap) -> Option<u32> {
    let start_points = get_start_points(&height_map.map);
    let destinations = dijkstras(&height_map.map.map(|a| a * -1), height_map.end, None);
    let distances = start_points
        .iter()
        .map(|point| *destinations.get(point).unwrap() + 1);
    distances.min()
}

fn main() {
    env_logger::init();

    let input = load_input();
    let height_map = parse_height_map(&input);
    let steps = find_shortest_path(&height_map).unwrap();

    println!("{}", steps);

    let single_dest_steps = find_single_destination_shortest_path(&height_map).unwrap();
    println!("{}", single_dest_steps);
}

#[cfg(test)]
mod tests {
    use nalgebra::DMatrix;

    use crate::{
        find_shortest_path, find_single_destination_shortest_path, parse_height_map, HeightMap,
        Point,
    };

    fn get_test_height_map() -> HeightMap {
        HeightMap {
            map: DMatrix::from_vec(
                5,
                8,
                vec![
                    0, 0, 0, 0, 0, 0, 1, 2, 2, 1, 1, 2, 2, 2, 3, 16, 17, 18, 19, 4, 15, 24, 25, 20,
                    5, 14, 23, 25, 21, 6, 13, 23, 23, 22, 7, 12, 11, 10, 9, 8,
                ],
            ),
            start: Point { x: 0, y: 0 },
            end: Point { x: 5, y: 2 },
        }
    }

    #[test]
    fn test_parse_height_map() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
        let expected = get_test_height_map();
        let actual = parse_height_map(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_shortest_path() {
        env_logger::init();
        let input = get_test_height_map();
        let expected = 31;
        let actual = find_shortest_path(&input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_single_destination_shortest_path() {
        let input = get_test_height_map();
        let expected = 29;
        let actual = find_single_destination_shortest_path(&input).unwrap();
        assert_eq!(expected, actual);
    }
}

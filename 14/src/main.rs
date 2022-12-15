use std::{collections::HashMap, env, fs};

type Coordinate = u32;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: Coordinate,
    y: Coordinate,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Tile {
    Sand,
    Rock,
}

type Cave = HashMap<Point, Tile>;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn create_rock_wall(a: Point, b: Point) -> Cave {
    let mut set: Cave = HashMap::new();

    let mut xs = vec![a.x, b.x];
    let mut ys = vec![a.y, b.y];

    xs.sort();
    ys.sort();

    for x in xs[0]..(xs[1] + 1) {
        for y in ys[0]..(ys[1] + 1) {
            set.insert(Point { x, y }, Tile::Rock);
        }
    }

    set
}

fn parse_point(input: &str) -> Point {
    let mut split = input.split(',');
    let x_str = split.next().unwrap();
    let y_str = split.next().unwrap();
    Point {
        x: x_str.parse::<Coordinate>().unwrap(),
        y: y_str.parse::<Coordinate>().unwrap(),
    }
}

fn parse_rock_path(input: &str) -> Cave {
    let points: Vec<Point> = input.split(" -> ").map(parse_point).collect();

    let mut rock_path: Cave = HashMap::new();
    for i in 0..(points.len() - 1) {
        for (point, t) in create_rock_wall(points[i], points[i + 1]) {
            rock_path.insert(point, t);
        }
    }

    rock_path
}

fn parse_cave(input: &str) -> Cave {
    let mut cave: Cave = HashMap::new();

    for split in input.split('\n') {
        cave.extend(parse_rock_path(split));
    }

    cave
}

fn start_sand_fall(cave: &mut Cave, start: Point) -> &mut Cave {
    let cave_x_min = cave.iter().map(|(k, _)| k.x).min().unwrap();
    let cave_x_max = cave.iter().map(|(k, _)| k.x).max().unwrap();
    let cave_y_max = cave.iter().map(|(k, _)| k.y).max().unwrap();

    loop {
        let mut falling_grain = start.clone();

        loop {
            let mut potential_grain = Point {
                x: falling_grain.x,
                y: falling_grain.y + 1,
            };

            if (potential_grain.x < cave_x_min)
                || (potential_grain.x > cave_x_max)
                || (potential_grain.y > cave_y_max)
            {
                return cave;
            }

            if !cave.contains_key(&potential_grain) {
                // Keep falling
                falling_grain = potential_grain;
                continue;
            }

            potential_grain = Point {
                x: falling_grain.x - 1,
                y: falling_grain.y + 1,
            };
            if !cave.contains_key(&potential_grain) {
                falling_grain = potential_grain;
                continue;
            }

            potential_grain = Point {
                x: falling_grain.x + 1,
                y: falling_grain.y + 1,
            };
            if !cave.contains_key(&potential_grain) {
                falling_grain = potential_grain;
                continue;
            }

            cave.insert(falling_grain, Tile::Sand);
            break;
        }
    }
}

fn count_sand(cave: &Cave) -> usize {
    cave.iter()
        .filter(|(_, v)| match v {
            Tile::Sand => true,
            Tile::Rock => false,
        })
        .count()
}

fn main() {
    let input = load_input();
    let mut cave = parse_cave(&input);
    start_sand_fall(&mut cave, Point { x: 500, y: 0 });
    let num_sand_at_rest = count_sand(&cave);
    println!("{}", num_sand_at_rest);
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{parse_cave, Point, Tile::Rock};

    #[test]
    fn test_parse_cave() {
        let input = include_str!("../test.txt");
        let expected = HashMap::from([
            (Point { x: 498, y: 4 }, Rock),
            (Point { x: 498, y: 5 }, Rock),
            (Point { x: 498, y: 6 }, Rock),
            (Point { x: 497, y: 6 }, Rock),
            (Point { x: 496, y: 6 }, Rock),
            (Point { x: 503, y: 4 }, Rock),
            (Point { x: 502, y: 4 }, Rock),
            (Point { x: 502, y: 5 }, Rock),
            (Point { x: 502, y: 6 }, Rock),
            (Point { x: 502, y: 7 }, Rock),
            (Point { x: 502, y: 8 }, Rock),
            (Point { x: 502, y: 9 }, Rock),
            (Point { x: 501, y: 9 }, Rock),
            (Point { x: 500, y: 9 }, Rock),
            (Point { x: 499, y: 9 }, Rock),
            (Point { x: 498, y: 9 }, Rock),
            (Point { x: 497, y: 9 }, Rock),
            (Point { x: 496, y: 9 }, Rock),
            (Point { x: 495, y: 9 }, Rock),
            (Point { x: 494, y: 9 }, Rock),
        ]);
        let actual = parse_cave(&input);
        assert_eq!(expected, actual);
    }
}

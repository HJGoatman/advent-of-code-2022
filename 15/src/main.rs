use std::{collections::HashSet, env, fs};

type Coordinate = i32;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: Coordinate,
    y: Coordinate,
}

#[derive(Debug, PartialEq)]
struct SensorReport {
    sensor_position: Point,
    beacon_position: Point,
}

#[derive(Ord, PartialEq, PartialOrd, Eq)]
enum Range {
    Min(Coordinate),
    Max(Coordinate),
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_point(input: &str) -> Point {
    let mut split = input.split(", ");

    let x_str = split.next().unwrap();
    let y_str = split.next().unwrap();

    let x = x_str[2..].parse::<Coordinate>().unwrap();
    let y = y_str[2..].parse::<Coordinate>().unwrap();

    Point { x, y }
}

fn parse_report(input: &str) -> SensorReport {
    let mut split = input.split(": ");

    let sensor_position = parse_point(&split.next().unwrap()[10..]);
    let beacon_position = parse_point(&split.next().unwrap()[21..]);

    SensorReport {
        sensor_position,
        beacon_position,
    }
}

fn parse_input(input: &str) -> Vec<SensorReport> {
    input
        .split('\n')
        .filter(|a| a != &"")
        .map(parse_report)
        .collect()
}

fn manhatten_distance(a: &Point, b: &Point) -> Coordinate {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn get_non_overlapping_ranges(ranges: &mut Vec<Range>) -> Vec<(Coordinate, Coordinate)> {
    ranges.sort();

    let mut stack = Vec::new();
    let mut non_overlapping_ranges = Vec::new();

    for range in ranges {
        match range {
            Range::Min(min) => {
                stack.push(min);
            }
            Range::Max(max) => {
                let min = stack.pop().unwrap();

                if stack.is_empty() {
                    non_overlapping_ranges.push((*min, *max));
                }
            }
        };
    }

    log::debug!("{:?}, {:?}", stack, non_overlapping_ranges);

    non_overlapping_ranges
}

fn get_num_no_beacons(reports: &[SensorReport], row: Coordinate) -> Coordinate {
    // For there not to be a beacon:
    // 1. There is not currently a beacon there from the report.
    // 2. The manhatten distance from x, y must be less than the manhatten distance from sensor to
    //    reported beacon.
    let mut ranges = Vec::new();

    for report in reports {
        let manhatten_distance =
            manhatten_distance(&report.sensor_position, &report.beacon_position);
        let rhs = manhatten_distance - (report.sensor_position.y - row).abs();

        let x_max = rhs + report.sensor_position.x;
        let x_min = report.sensor_position.x - rhs;

        if x_min >= x_max {
            continue;
        }

        log::debug!(
            "Sensor: ({}, {}), Beacon: ({}, {}), Distance:{}, Min: {}, Max: {}",
            report.sensor_position.x,
            report.sensor_position.y,
            report.beacon_position.x,
            report.beacon_position.y,
            manhatten_distance,
            x_min,
            x_max
        );

        ranges.push(Range::Min(x_min));
        ranges.push(Range::Max(x_max));
    }

    let non_overlapping_ranges = get_non_overlapping_ranges(&mut ranges);
    let within_sensor_sum: Coordinate = non_overlapping_ranges
        .iter()
        .map(|(min, max)| max + 1 - min)
        .sum();

    let beacons: HashSet<Point> = reports
        .iter()
        .map(|report| report.beacon_position)
        .filter(|point| point.y == row)
        .collect();

    within_sensor_sum - beacons.len() as Coordinate
}

fn main() {
    env_logger::init();
    let input = load_input();
    let sensors = parse_input(&input);
    let num_no_beacons = get_num_no_beacons(&sensors, 2000000);
    println!("{}", num_no_beacons);
}

#[cfg(test)]
mod tests {
    use crate::{get_num_no_beacons, parse_input, Point, SensorReport};

    fn get_test_report() -> Vec<SensorReport> {
        vec![
            SensorReport {
                sensor_position: Point { x: 2, y: 18 },
                beacon_position: Point { x: -2, y: 15 },
            },
            SensorReport {
                sensor_position: Point { x: 9, y: 16 },
                beacon_position: Point { x: 10, y: 16 },
            },
            SensorReport {
                sensor_position: Point { x: 13, y: 2 },
                beacon_position: Point { x: 15, y: 3 },
            },
            SensorReport {
                sensor_position: Point { x: 12, y: 14 },
                beacon_position: Point { x: 10, y: 16 },
            },
            SensorReport {
                sensor_position: Point { x: 10, y: 20 },
                beacon_position: Point { x: 10, y: 16 },
            },
            SensorReport {
                sensor_position: Point { x: 14, y: 17 },
                beacon_position: Point { x: 10, y: 16 },
            },
            SensorReport {
                sensor_position: Point { x: 8, y: 7 },
                beacon_position: Point { x: 2, y: 10 },
            },
            SensorReport {
                sensor_position: Point { x: 2, y: 0 },
                beacon_position: Point { x: 2, y: 10 },
            },
            SensorReport {
                sensor_position: Point { x: 0, y: 11 },
                beacon_position: Point { x: 2, y: 10 },
            },
            SensorReport {
                sensor_position: Point { x: 20, y: 14 },
                beacon_position: Point { x: 25, y: 17 },
            },
            SensorReport {
                sensor_position: Point { x: 17, y: 20 },
                beacon_position: Point { x: 21, y: 22 },
            },
            SensorReport {
                sensor_position: Point { x: 16, y: 7 },
                beacon_position: Point { x: 15, y: 3 },
            },
            SensorReport {
                sensor_position: Point { x: 14, y: 3 },
                beacon_position: Point { x: 15, y: 3 },
            },
            SensorReport {
                sensor_position: Point { x: 20, y: 1 },
                beacon_position: Point { x: 15, y: 3 },
            },
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../test.txt");
        let expected = get_test_report();
        let actual = parse_input(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_get_num_no_beacons() {
        env_logger::init();
        let input = get_test_report();
        let expected = 26;
        let actual = get_num_no_beacons(&input, 10);
        assert_eq!(expected, actual);
    }
}

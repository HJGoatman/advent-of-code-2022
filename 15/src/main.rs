use std::{cmp::Ordering, collections::HashSet, env, fs};

type Coordinate = i64;

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

#[derive(Debug, PartialEq, Eq)]
enum Range {
    Min(Coordinate),
    Max(Coordinate),
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut ordering = match (self, other) {
            (Range::Min(a), Range::Min(b)) => a.partial_cmp(b),
            (Range::Min(a), Range::Max(b)) => a.partial_cmp(b),
            (Range::Max(a), Range::Min(b)) => a.partial_cmp(b),
            (Range::Max(a), Range::Max(b)) => a.partial_cmp(b),
        };

        if ordering == Some(Ordering::Equal) {
            ordering = match (self, other) {
                (Range::Min(_), Range::Min(_)) => Some(Ordering::Equal),
                (Range::Min(_), Range::Max(_)) => Some(Ordering::Less),
                (Range::Max(_), Range::Min(_)) => Some(Ordering::Greater),
                (Range::Max(_), Range::Max(_)) => Some(Ordering::Equal),
            }
        }

        ordering
    }
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

    log::debug!("{:?}", ranges);

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
    let non_overlapping_ranges = build_non_overlapping_ranges(reports, row);

    let within_sensor_sum: Coordinate = non_overlapping_ranges
        .iter()
        .map(|(min, max)| max - min)
        .sum();

    let beacons: HashSet<Point> = reports
        .iter()
        .map(|report| report.beacon_position)
        .filter(|point| point.y == row)
        .collect();

    within_sensor_sum - beacons.len() as Coordinate
}

fn build_non_overlapping_ranges(
    reports: &[SensorReport],
    row: Coordinate,
) -> Vec<(Coordinate, Coordinate)> {
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
        ranges.push(Range::Max(x_max + 1));
    }

    get_non_overlapping_ranges(&mut ranges)
}

fn get_distress_beacon(reports: &[SensorReport], max_range: Coordinate) -> Point {
    // |s_x - x| + |s_y - y| > |s_x - b_x| + |s_y - b_y|
    // |s_x - x| + |s_y - y| > rhs
    //
    // (+ +) 1.  s_x - x + s_y - y > rhs
    //                      -x - y > rhs - s_x - s_y
    // (+ -) 2.  s_x - x - s_y + y > rhs
    //                      -x + y > rhs - s_x + s_y
    // (- +) 3. -s_x + x + s_y - y > rhs
    //                       x - y > rhs + s_x - s_y
    // (- -) 4. -s_x + x - s_y + y > rhs
    //                       x + y > rhs + s_x + s_y

    // let manhatten_distance = manhatten_distance(&report.sensor_position, &report.beacon_position);

    // let p_p = |x, y| - x - y > manhatten_distance - report.sensor_position.x - report.sensor_position.y;
    // let p_m = |x, y|

    for y in 0..max_range {
        let non_overlapping_ranges = build_non_overlapping_ranges(reports, y);
        if non_overlapping_ranges.len() > 1 {
            return Point {
                x: non_overlapping_ranges[0].1,
                y,
            };
        }
    }

    Point { x: 0, y: 0 }
}

fn get_tuning_signal(point: &Point) -> Coordinate {
    point.x * 4000000 + point.y
}

fn main() {
    env_logger::init();
    let input = load_input();
    let sensors = parse_input(&input);
    let num_no_beacons = get_num_no_beacons(&sensors, 2000000);
    println!("{}", num_no_beacons);

    let distress_beacon: Point = get_distress_beacon(&sensors, 4000000);
    let tuning_signal = get_tuning_signal(&distress_beacon);
    println!("{}", tuning_signal);
}

#[cfg(test)]
mod tests {
    use crate::{get_distress_beacon, get_num_no_beacons, parse_input, Point, SensorReport};

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

    #[test]
    fn test_get_distress_beacon() {
        let input = get_test_report();
        let expected = Point { x: 14, y: 11 };
        let actual = get_distress_beacon(&input, 20);
        assert_eq!(expected, actual);
    }
}

use itertools::iproduct;
use std::{
    collections::HashMap,
    env,
    fmt::{Display, Write},
    fs,
    ops::Add,
};

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

type Ordinate = i8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: Ordinate,
    y: Ordinate,
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, PartialEq)]
struct Elves {
    positions: Vec<Position>,
}

impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.positions.iter().map(|pos| pos.x).min().unwrap();
        let min_y = self.positions.iter().map(|pos| pos.y).min().unwrap();

        let max_x = self.positions.iter().map(|pos| pos.x).max().unwrap();
        let max_y = self.positions.iter().map(|pos| pos.y).max().unwrap();

        f.write_char('\n')?;

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                if self.positions.contains(&Position { x, y }) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

struct ElvesIterator {
    order: [Direction; 4],
    elves: Elves,
}

impl ElvesIterator {
    pub fn new(elves: &Elves) -> Self {
        let order = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];

        let elves = elves.clone();

        ElvesIterator { order, elves }
    }
}

impl Iterator for ElvesIterator {
    type Item = Elves;

    fn next(&mut self) -> Option<Self::Item> {
        log::debug!("{:?}", self.order);

        let proposals: Vec<Option<Position>> = self
            .elves
            .positions
            .iter()
            .map(|pos| get_proposal(pos, &self.elves.positions, &self.order))
            .collect();

        let proposal_counts = get_proposal_counts(&proposals);

        let proposals: Vec<Option<Position>> = proposals
            .iter()
            .cloned()
            .map(|proposal| match proposal_counts.get(&proposal) {
                Some(count) => {
                    if *count > 1 {
                        None
                    } else {
                        proposal
                    }
                }
                None => None,
            })
            .collect();

        let positions = self
            .elves
            .positions
            .iter()
            .zip(proposals)
            .map(|(current_position, new_position)| match new_position {
                Some(position) => position,
                None => current_position.clone(),
            })
            .collect();

        self.elves = Elves { positions };
        self.order.rotate_left(1);

        log::debug!("{}", self.elves);

        return Some(self.elves.clone());
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn parse_elves(input: &str) -> Elves {
    let mut elves = Vec::new();

    for (y, row) in input.split('\n').enumerate() {
        for (x, val) in row.chars().enumerate() {
            if val == '.' {
                continue;
            }

            elves.push(Position {
                x: x as Ordinate,
                y: y as Ordinate,
            });
        }
    }

    Elves { positions: elves }
}

fn has_any_elf_around(pos: &Position, elves: &[Position]) -> bool {
    iproduct!([pos.x - 1, pos.x, pos.x + 1], [pos.y - 1, pos.y, pos.y + 1])
        .filter(|(x, y)| !(*x == pos.x && *y == pos.y))
        .map(|(x, y)| Position { x, y })
        .any(|elf| elves.contains(&elf))
}

fn get_proposal(pos: &Position, elves: &[Position], order: &[Direction]) -> Option<Position> {
    if !has_any_elf_around(pos, elves) {
        return None;
    }

    const NORTH: (bool, Ordinate) = (true, -1);
    const WEST: (bool, Ordinate) = (false, -1);
    const EAST: (bool, Ordinate) = (false, 1);
    const SOUTH: (bool, Ordinate) = (true, 1);

    let order: Vec<(bool, Ordinate)> = order
        .iter()
        .map(|direction| match direction {
            Direction::North => NORTH,
            Direction::South => SOUTH,
            Direction::West => WEST,
            Direction::East => EAST,
        })
        .collect();

    for (is_vertical, displacement) in order {
        let create_position = if is_vertical {
            |a, b| Position { x: a, y: b }
        } else {
            |a, b| Position { x: b, y: a }
        };

        if [0, 1, -1]
            .into_iter()
            .map(|disp| *pos + create_position(disp, displacement))
            .all(|pos| !elves.contains(&pos))
        {
            return Some(*pos + create_position(0, displacement));
        }
    }

    None
}

fn run_process(elves: &Elves, num_rounds: i8) -> Elves {
    let mut elves_iter = ElvesIterator::new(elves);

    let mut elves = elves.clone();

    for _ in 0..num_rounds {
        elves = elves_iter.next().unwrap();
    }

    elves
}

fn get_proposal_counts(proposals: &[Option<Position>]) -> HashMap<&Option<Position>, u8> {
    let mut counts = HashMap::new();

    for proposal in proposals {
        if proposal.is_some() {
            counts
                .entry(proposal)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    counts
}

fn main() {
    env_logger::init();

    let input = load_input();

    let elves = parse_elves(&input);
    log::debug!("{}", &elves);

    const NUM_ROUNDS: i8 = 10;
    let elves_1 = run_process(&elves, NUM_ROUNDS);
    log::debug!("{}", &elves);

    let empty_ground_tiles = count_empty_ground_tiles(&elves_1);
    println!("{}", empty_ground_tiles);

    let round = first_round_no_elves_move(&elves);
    println!("{}", round);
}

fn first_round_no_elves_move(elves: &Elves) -> u32 {
    let mut elves_iter = ElvesIterator::new(elves);

    let mut previous_elves = elves.clone();

    let mut round = 1;

    loop {
        let new_elves = elves_iter.next().unwrap();

        log::debug!("{}\n{}", previous_elves, new_elves);

        if previous_elves == new_elves {
            break;
        }

        previous_elves = new_elves;
        round += 1;
    }

    round
}

fn count_empty_ground_tiles(elves: &Elves) -> u32 {
    let positions = &elves.positions;

    let min_x = positions.iter().map(|pos| pos.x).min().unwrap();
    let min_y = positions.iter().map(|pos| pos.y).min().unwrap();
    let max_x = positions.iter().map(|pos| pos.x).max().unwrap() + 1;
    let max_y = positions.iter().map(|pos| pos.y).max().unwrap() + 1;

    let area = (max_x - min_x) as u32 * (max_y - min_y) as u32;

    area - positions.len() as u32
}

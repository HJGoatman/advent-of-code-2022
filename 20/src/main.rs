use std::{env, fs};

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_input(input: &str) -> Vec<i16> {
    input
        .split('\n')
        .map(|val| val.parse::<i16>().unwrap())
        .collect()
}

fn mix(coordinates: &[i16]) -> i16 {
    let len = coordinates.len() as i16;
    let mut positions: Vec<i16> = (0..len).collect();

    for i in 0..coordinates.len() {
        let old_position = positions[i];
        let mut new_position = old_position + coordinates[i];

        log::trace!("Pre-cycle new position: {}", new_position);

        if new_position <= 0 {
            new_position = (new_position).rem_euclid(len - 1);
        }

        if new_position >= len {
            new_position = (new_position).rem_euclid(len - 1);
        }

        log::trace!(
            "Old Position: {}. New Position: {}",
            old_position,
            new_position
        );

        let is_forward = new_position > old_position;

        for j in 0..positions.len() {
            if j == i {
                continue;
            }
            // log::trace!("Current Position: {}", positions[j]);
            if is_forward {
                if positions[j] >= old_position && positions[j] <= new_position {
                    positions[j] -= 1;
                }
            } else {
                if positions[j] >= new_position && positions[j] <= old_position {
                    positions[j] += 1;
                }
            }
        }

        positions[i] = new_position;

        log::debug!("{:?}", positions);
    }

    let zeroth_position = coordinates.iter().position(|a| *a == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|coordinate_position| (positions[zeroth_position] + coordinate_position) % len)
        .map(|position| positions.iter().position(|a| *a == position).unwrap())
        .map(|position| coordinates[position])
        .map(|p| {
            log::debug!("{}", p);
            p
        })
        .sum()
}

fn main() {
    env_logger::init();
    let input = load_input();
    let coordinates = parse_input(&input);
    let mixed = mix(&coordinates);
    println!("{:?}", mixed);
}

#[cfg(test)]
mod tests {
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}

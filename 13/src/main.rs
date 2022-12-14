use std::{cmp::Ordering, env, fs};

type Integer = u16;

#[derive(Debug, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Integer(Integer),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn partial_cmp_list(a: &[Packet], b: &[Packet]) -> Option<std::cmp::Ordering> {
            let mut a_iter = a.iter();
            let mut b_iter = b.iter();

            while let (Some(packet_a), Some(packet_b)) = (a_iter.next(), b_iter.next()) {
                let ordering = packet_a.partial_cmp(packet_b).unwrap();

                if let Ordering::Equal = ordering {
                    continue;
                }

                log::debug!("    - Left side is {:?}", ordering);

                return Some(ordering);
            }

            let num_remaining_a = a.len();
            let num_remaining_b = b.len();

            log::debug!(
                "Left has {} length and Right has {} length.",
                num_remaining_a,
                num_remaining_b
            );

            let ordering = if num_remaining_a > num_remaining_b {
                Ordering::Greater
            } else if num_remaining_a < num_remaining_b {
                Ordering::Less
            } else {
                Ordering::Equal
            };

            log::debug!("    - Left side is {:?}", ordering);
            Some(ordering)
        }

        log::debug!("  - Compare {:?} vs {:?}", self, other);

        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => Some(a.cmp(b)),
            (Packet::List(a), Packet::List(b)) => partial_cmp_list(a, b),
            (a, Packet::Integer(v)) => a.partial_cmp(&Packet::List(vec![Packet::Integer(*v)])),
            (Packet::Integer(v), b) => Packet::List(vec![Packet::Integer(*v)]).partial_cmp(b),
        }
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_packet(input: &str) -> Packet {
    if let Ok(i) = input.parse::<Integer>() {
        return Packet::Integer(i);
    }

    if input == "[]" {
        return Packet::List(vec![]);
    }

    // Goal is to find the indexes where there's commas that are at the same level as the list.
    let mut split_indices: Vec<usize> = Vec::new();
    split_indices.push(0);

    let mut level: u8 = 0;
    for (i, c) in input.chars().enumerate() {
        // log::debug!("Index: {} | Char: {} | Level: {}", i, c, level);
        if c == '[' {
            level += 1;
        }

        if c == ']' {
            level -= 1;
        }

        if (c == ',') && (level == 1) {
            split_indices.push(i);
        }
    }

    split_indices.push(input.len() - 1);

    let mut items: Vec<String> = Vec::new();
    for i in 0..(split_indices.len() - 1) {
        let j = i + 1;

        let split_i = split_indices[i] + 1;
        let split_j = split_indices[j];

        // log::debug!(
        //     "Attempting to Split {} between {} and {}",
        //     &input,
        //     split_i,
        //     split_j,
        // );

        let list_item_str = &input[split_i..split_j];
        items.push(String::from(list_item_str));
    }

    let inner_packets: Vec<Packet> = items.iter().map(|a| parse_packet(&a)).collect();
    Packet::List(inner_packets)
}

fn parse_packet_pair(input: &str) -> (Packet, Packet) {
    let mut split = input.split('\n').map(parse_packet);
    (split.next().unwrap(), split.next().unwrap())
}

fn parse_packets(input: &str) -> Vec<(Packet, Packet)> {
    input.split("\n\n").map(parse_packet_pair).collect()
}

fn get_number_of_correct_pairs(packet_pairs: &[(Packet, Packet)]) -> u16 {
    let mut number_of_correct_pairs = 0;

    for (i, (left, right)) in packet_pairs.iter().enumerate() {
        log::debug!("== Pair {} ==", i + 1);
        log::debug!("- Compare {:?} vs {:?}", left, right);
        if left < right {
            number_of_correct_pairs += i as u16 + 1;
        }
    }

    number_of_correct_pairs
}

fn main() {
    let input = load_input();
    let packet_pairs = parse_packets(&input);
    let num_correct_pairs = get_number_of_correct_pairs(&packet_pairs);
    println!("{}", num_correct_pairs);
}

#[cfg(test)]
mod tests {

    use crate::{
        get_number_of_correct_pairs, parse_packets, Packet,
        Packet::{Integer, List},
    };

    fn get_test_packet_pairs() -> Vec<(Packet, Packet)> {
        vec![
            (
                List(vec![
                    Integer(1),
                    Integer(1),
                    Integer(3),
                    Integer(1),
                    Integer(1),
                ]),
                List(vec![
                    Integer(1),
                    Integer(1),
                    Integer(5),
                    Integer(1),
                    Integer(1),
                ]),
            ),
            (
                List(vec![
                    List(vec![Integer(1)]),
                    List(vec![Integer(2), Integer(3), Integer(4)]),
                ]),
                List(vec![List(vec![Integer(1)]), Integer(4)]),
            ),
            (
                List(vec![Integer(9)]),
                List(vec![List(vec![Integer(8), Integer(7), Integer(6)])]),
            ),
            (
                List(vec![
                    List(vec![Integer(4), Integer(4)]),
                    Integer(4),
                    Integer(4),
                ]),
                List(vec![
                    List(vec![Integer(4), Integer(4)]),
                    Integer(4),
                    Integer(4),
                    Integer(4),
                ]),
            ),
            (
                List(vec![Integer(7), Integer(7), Integer(7), Integer(7)]),
                List(vec![Integer(7), Integer(7), Integer(7)]),
            ),
            (List(vec![]), List(vec![Integer(3)])),
            (
                List(vec![List(vec![List(vec![])])]),
                List(vec![List(vec![])]),
            ),
            (
                List(vec![
                    Integer(1),
                    List(vec![
                        Integer(2),
                        List(vec![
                            Integer(3),
                            List(vec![
                                Integer(4),
                                List(vec![Integer(5), Integer(6), Integer(7)]),
                            ]),
                        ]),
                    ]),
                    Integer(8),
                    Integer(9),
                ]),
                List(vec![
                    Integer(1),
                    List(vec![
                        Integer(2),
                        List(vec![
                            Integer(3),
                            List(vec![
                                Integer(4),
                                List(vec![Integer(5), Integer(6), Integer(0)]),
                            ]),
                        ]),
                    ]),
                    Integer(8),
                    Integer(9),
                ]),
            ),
        ]
    }

    #[test]
    fn test_parse_packets() {
        let input = include_str!("../test.txt");
        let expected = get_test_packet_pairs();
        let actual = parse_packets(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_number_of_correct_pairs() {
        env_logger::init();
        let input = get_test_packet_pairs();
        let expected = 13;
        let actual = get_number_of_correct_pairs(&input);
        assert_eq!(expected, actual);
    }
}

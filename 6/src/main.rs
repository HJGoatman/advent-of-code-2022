use std::{collections::VecDeque, fs};

fn load_input() -> String {
    fs::read_to_string("input.txt").expect("Should have been able to read the file")
}

fn find_start(stream: &str, distinct_characters_amount: usize) -> Option<u32> {
    let mut iterator = stream.chars().enumerate();
    let mut deque: VecDeque<char> = VecDeque::with_capacity(distinct_characters_amount);

    'outer: while let Some((start_int, letter)) = iterator.next() {
        deque.push_back(letter);

        let deque_length = deque.len();

        if deque_length < distinct_characters_amount {
            continue;
        }

        for i in 0..deque_length {
            for j in i + 1..deque_length {
                if deque[i] == deque[j] {
                    deque.pop_front();
                    continue 'outer;
                }
            }
        }

        let marker_apperance: u32 = start_int.try_into().unwrap();

        return Some(marker_apperance + 1);
    }

    return None;
}

fn main() {
    let input = load_input();
    let start_packet_number = find_start(&input, 4).unwrap();
    println!("{:?}", start_packet_number);

    let start_message_marker = find_start(&input, 14).unwrap();
    println!("{:?}", start_message_marker);
}

#[cfg(test)]
mod tests {
    use crate::find_start;

    fn test_part(input: &str, expected: Option<u32>, number_distinct: usize) {
        let actual = find_start(&input, number_distinct);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_start() {
        let part_1_distinct = 4;
        test_part("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(7), part_1_distinct);

        test_part("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(5), part_1_distinct);

        test_part("nppdvjthqldpwncqszvftbrmjlhg", Some(6), part_1_distinct);

        test_part(
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            Some(10),
            part_1_distinct,
        );

        test_part(
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
            Some(11),
            part_1_distinct,
        );
    }

    #[test]
    fn test_find_message() {
        let part_2_distinct = 14;

        test_part("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(19), part_2_distinct);

        test_part("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(23), part_2_distinct);

        test_part("nppdvjthqldpwncqszvftbrmjlhg", Some(23), part_2_distinct);

        test_part(
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            Some(29),
            part_2_distinct,
        );

        test_part(
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
            Some(26),
            part_2_distinct,
        );
    }
}

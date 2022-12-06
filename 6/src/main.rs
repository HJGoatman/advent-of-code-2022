use std::{collections::VecDeque, fs};

fn load_input() -> String {
    fs::read_to_string("input.txt").expect("Should have been able to read the file")
}

fn find_start(stream: &str) -> Option<u32> {
    let mut iterator = stream.chars().enumerate();
    let mut deque: VecDeque<char> = VecDeque::with_capacity(4);

    'outer: while let Some((start_int, letter)) = iterator.next() {
        deque.push_back(letter);

        let deque_length = deque.len();

        if deque_length < 4 {
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
    let start_packet_number = find_start(&input).unwrap();
    println!("{:?}", start_packet_number);
}

#[cfg(test)]
mod tests {
    use crate::find_start;

    fn test_part_1(input: &str, expected: Option<u32>) {
        let actual = find_start(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_start() {
        test_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(7));

        test_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(5));

        test_part_1("nppdvjthqldpwncqszvftbrmjlhg", Some(6));

        test_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(10));

        test_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(11));
    }
}

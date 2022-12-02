#[derive(Debug, PartialEq, Eq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
struct Round(HandShape, HandShape);

fn parse_handshape(val: char) -> HandShape {
    match val {
        'A' | 'X' => HandShape::Rock,
        'B' | 'Y' => HandShape::Paper,
        'C' | 'Z' => HandShape::Scissors,
        _ => panic!("Not a letter."),
    }
}

fn parse_round(round_str: &str) -> Round {
    let (player_1, player_2) = (
        round_str.chars().next().unwrap(),
        round_str.chars().rev().next().unwrap(),
    );

    let (hand_1, hand_2) = (parse_handshape(player_1), parse_handshape(player_2));

    Round(hand_1, hand_2)
}

fn parse_encrypted_strategy_guide(guide: String) -> Vec<Round> {
    let round_strings = guide.split('\n');

    round_strings
        .filter(|val| val != &"")
        .map(parse_round)
        .collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{parse_encrypted_strategy_guide, HandShape, Round};

    #[test]
    fn test_parse_encrypted_strategy_guide() {
        let puzzle_string = String::from("A Y\nB X\nC Z\n\n");
        let parse_result = parse_encrypted_strategy_guide(puzzle_string);
        let expected_result = vec![
            Round(HandShape::Rock, HandShape::Paper),
            Round(HandShape::Paper, HandShape::Rock),
            Round(HandShape::Scissors, HandShape::Scissors),
        ];
        assert_eq!(parse_result, expected_result);
    }
}

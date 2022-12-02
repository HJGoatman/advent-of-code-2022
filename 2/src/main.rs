use std::{cmp::Ordering, fs};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

fn get_weight(shape: &HandShape) -> i8 {
    match shape {
        HandShape::Rock => 0,
        HandShape::Paper => 1,
        HandShape::Scissors => 2,
    }
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let difference: i8 = get_weight(self) - get_weight(other);
        if difference == 0 {
            return Some(Ordering::Equal);
        }

        if difference == 1 || difference < -1 {
            return Some(Ordering::Greater);
        }

        if difference == -1 || difference > 1 {
            return Some(Ordering::Less);
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct Round(HandShape, HandShape);

#[derive(Debug)]
struct Score(u32, u32);

enum Outcome {
    Lost,
    Draw,
    Won,
}

fn load_input() -> String {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    String::from(contents)
}

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

fn get_shape_score(shape: &HandShape) -> u32 {
    match shape {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    }
}

fn get_outcome_score(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Lost => 0,
        Outcome::Draw => 3,
        Outcome::Won => 6,
    }
}

fn get_outcome(hand_1: &HandShape, hand_2: &HandShape) -> (Outcome, Outcome) {
    if hand_1 > hand_2 {
        return (Outcome::Won, Outcome::Lost);
    }

    if hand_1 == hand_2 {
        return (Outcome::Draw, Outcome::Draw);
    }

    (Outcome::Lost, Outcome::Won)
}

fn get_round_score(round: &Round) -> Score {
    let (hand_1, hand_2) = (round.0, round.1);
    let (outcome_1, outcome_2) = get_outcome(&hand_1, &hand_2);
    let (shape_score_1, shape_score_2) = (get_shape_score(&hand_1), get_shape_score(&hand_2));
    let (outcome_score_1, outcome_score_2) =
        (get_outcome_score(&outcome_1), get_outcome_score(&outcome_2));
    Score(
        shape_score_1 + outcome_score_1,
        shape_score_2 + outcome_score_2,
    )
}

fn calculate_outcome(current_score: Score, this_round: &Round) -> Score {
    let outcome = get_round_score(this_round);
    Score(current_score.0 + outcome.0, current_score.1 + outcome.1)
}

fn play_tournament(guide: Vec<Round>) -> Score {
    guide.iter().fold(Score(0, 0), calculate_outcome)
}

fn main() {
    let input_str = load_input();
    let guide = parse_encrypted_strategy_guide(input_str);
    let final_score = play_tournament(guide);

    println!("{:?}", final_score);
}

#[cfg(test)]
mod tests {
    use crate::{parse_encrypted_strategy_guide, play_tournament, HandShape, Round};

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

    #[test]
    fn test_final_score() {
        let guide = vec![
            Round(HandShape::Rock, HandShape::Paper),
            Round(HandShape::Paper, HandShape::Rock),
            Round(HandShape::Scissors, HandShape::Scissors),
        ];

        let scores = play_tournament(guide);
        assert_eq!(15, scores.1);
    }
}

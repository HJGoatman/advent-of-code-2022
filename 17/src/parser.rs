#[derive(Debug, PartialEq, Eq)]
pub enum JetDirection {
    Left,
    Right,
}

pub type JetPattern = Vec<JetDirection>;

pub fn parse_jet_pattern(input: &str) -> JetPattern {
    input
        .chars()
        .map(|c| match c {
            '<' => JetDirection::Left,
            '>' => JetDirection::Right,
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::parse_jet_pattern,
        JetDirection::{Left, Right},
    };

    use super::JetPattern;

    fn get_test_jet_pattern() -> JetPattern {
        vec![
            Right, Right, Right, Left, Left, Right, Left, Right, Right, Left, Left, Left, Right,
            Right, Left, Right, Right, Right, Left, Left, Left, Right, Right, Right, Left, Left,
            Left, Right, Left, Left, Left, Right, Right, Left, Right, Right, Left, Left, Right,
            Right,
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let expected = get_test_jet_pattern();
        let actual = parse_jet_pattern(&input);
        assert_eq!(expected, actual);
    }
}

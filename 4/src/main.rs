use std::fs;

type SectionID = u32;
type RangePair = (Range, Range);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    start: SectionID,
    end: SectionID,
}

fn load_input() -> String {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    contents
}

fn parse_range(input: &str) -> Range {
    let ranges: Vec<SectionID> = input
        .split('-')
        .map(|val| val.parse::<SectionID>().unwrap())
        .collect();
    Range {
        start: ranges[0],
        end: ranges[1],
    }
}

fn parse_pair(input: &str) -> RangePair {
    let pair: Vec<Range> = input.split(',').map(parse_range).collect();
    (pair[0], pair[1])
}

fn parse_input(input: &String) -> Vec<RangePair> {
    input
        .split("\n")
        .filter(|a| a != &"")
        .map(parse_pair)
        .collect()
}

fn has_fully_contained_pair(range_pair: &&RangePair) -> bool {
    let (a, b) = range_pair;
    (a.start >= b.start && a.end <= b.end) || (b.start >= a.start && b.end <= a.end)
}

fn get_fully_contained_pairs(pairs: &Vec<RangePair>) -> Vec<&RangePair> {
    pairs
        .into_iter()
        .filter(&has_fully_contained_pair)
        .collect()
}

fn main() {
    let input = load_input();
    let pairs = parse_input(&input);
    let fully_contained_pairs = get_fully_contained_pairs(&pairs);
    let number_of_contained_pairs = fully_contained_pairs.len();
    println!("{:?}", number_of_contained_pairs);
}

#[cfg(test)]
mod tests {
    use crate::{get_fully_contained_pairs, parse_input, Range, RangePair};

    fn get_ranges() -> Vec<RangePair> {
        vec![
            (Range { start: 2, end: 4 }, Range { start: 6, end: 8 }),
            (Range { start: 2, end: 3 }, Range { start: 4, end: 5 }),
            (Range { start: 5, end: 7 }, Range { start: 7, end: 9 }),
            (Range { start: 2, end: 8 }, Range { start: 3, end: 7 }),
            (Range { start: 6, end: 6 }, Range { start: 4, end: 6 }),
            (Range { start: 2, end: 6 }, Range { start: 4, end: 8 }),
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = String::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n");
        let expected = get_ranges();
        let actual = parse_input(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_fully_contained_pairs() {
        let input = get_ranges();
        let expected = vec![
            &(Range { start: 2, end: 8 }, Range { start: 3, end: 7 }),
            &(Range { start: 6, end: 6 }, Range { start: 4, end: 6 }),
        ];
        let actual = get_fully_contained_pairs(&input);
        assert_eq!(actual, expected);
    }
}

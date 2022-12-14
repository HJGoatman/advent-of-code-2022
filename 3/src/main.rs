use std::{
    char::ParseCharError,
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Item(char);

impl FromStr for Item {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item_value = s.parse::<char>()?;

        Ok(Item(item_value))
    }
}

type Compartment = Vec<Item>;

#[derive(Debug, PartialEq)]
struct Rucksack(Compartment, Compartment);

type Priority = u32;

fn load_input() -> String {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    contents
}

fn parse_compartment(s: &str) -> Result<Compartment, ParseCharError> {
    let items = s
        .split("")
        .filter(|v| v != &"")
        .map(Item::from_str)
        .map(|val| val.unwrap())
        .collect();
    Ok(items)
}

impl FromStr for Rucksack {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len() % 2, 0);
        let midpoint = s.len() / 2;
        let (part_1, part_2) = (&s[..midpoint], &s[midpoint..]);
        let (compartment_1, compartment_2) = (
            parse_compartment(part_1).unwrap(),
            parse_compartment(part_2).unwrap(),
        );
        Ok(Rucksack(compartment_1, compartment_2))
    }
}

fn parse_rucksacks(s: &str) -> Vec<Rucksack> {
    s.split('\n')
        .filter(|val| val != &"")
        .map(Rucksack::from_str)
        .map(|rucksack| rucksack.unwrap())
        .collect()
}

fn get_duplicate_items(rucksack: &Rucksack) -> HashSet<Item> {
    let (compartment_1, compartment_2) = (rucksack.0.clone(), rucksack.1.clone());
    let hash_1: HashSet<Item> = compartment_1.into_iter().collect();
    let hash_2: HashSet<Item> = compartment_2.into_iter().collect();

    hash_1.intersection(&hash_2).map(Item::clone).collect()
}

fn next_char(c: char) -> char {
    char::from_u32((c as u32) + 1).unwrap()
}

fn get_priority_map() -> HashMap<Item, Priority> {
    let priority_iterator = ('a'..next_char('z'))
        .chain('A'..next_char('Z'))
        .zip(1..52 + 1);

    let mut priority_map = HashMap::new();

    for (char, priority) in priority_iterator {
        priority_map.insert(Item(char), priority);
    }

    priority_map
}

fn get_priorities(
    items: Vec<HashSet<Item>>,
    priority_map: &HashMap<Item, Priority>,
) -> Vec<Priority> {
    items
        .iter()
        .flat_map(|hm| {
            hm.iter()
                .map(|item| priority_map.get(item).unwrap().clone())
        })
        .collect()
}

fn get_common_items(item_sets: Vec<HashSet<&Item>>) -> Vec<&Item> {
    let mut item_set_iter = item_sets.into_iter();
    let mut common_items = item_set_iter.next().unwrap();

    for _ in 0..2 {
        common_items = common_items
            .intersection(&item_set_iter.next().unwrap())
            .cloned()
            .collect();
    }

    common_items.into_iter().collect()
}

fn get_badge(group: &[Rucksack]) -> Item {
    let elves_items: Vec<HashSet<&Item>> = group
        .iter()
        .map(|rucksack| rucksack.0.iter().chain(rucksack.1.iter()).collect())
        .collect();

    let badges: Vec<&Item> = get_common_items(elves_items);
    badges.get(0).unwrap().clone().clone()
}

fn get_badges(rucksacks: &Vec<Rucksack>) -> Vec<Item> {
    rucksacks.chunks(3).map(get_badge).collect()
}

fn main() {
    let input = load_input();
    let rucksacks = parse_rucksacks(&input);
    let duplicate_items: Vec<HashSet<Item>> = rucksacks.iter().map(get_duplicate_items).collect();
    let priority_map = get_priority_map();
    let priorities: Vec<Priority> = get_priorities(duplicate_items, &priority_map);
    let sum: u32 = priorities.iter().sum();
    println!("{:?}", sum);

    let badges: Vec<Item> = get_badges(&rucksacks);
    let part_2_sum: u32 = badges
        .iter()
        .map(|badge| priority_map.get(badge).unwrap().clone())
        .sum();
    println!("{:?}", part_2_sum);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{get_badge, parse_rucksacks, Item, Rucksack};

    #[test]
    fn test_parse_rucksack() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let expected = Rucksack(
            vec![
                Item('v'),
                Item('J'),
                Item('r'),
                Item('w'),
                Item('p'),
                Item('W'),
                Item('t'),
                Item('w'),
                Item('J'),
                Item('g'),
                Item('W'),
                Item('r'),
            ],
            vec![
                Item('h'),
                Item('c'),
                Item('s'),
                Item('F'),
                Item('M'),
                Item('M'),
                Item('f'),
                Item('F'),
                Item('F'),
                Item('h'),
                Item('F'),
                Item('p'),
            ],
        );
        assert_eq!(Rucksack::from_str(input).unwrap(), expected);
    }

    #[test]
    fn test_get_badge() {
        let input =
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg";
        let input_rucksacks = parse_rucksacks(&input);
        let group = &input_rucksacks[..];
        assert_eq!(get_badge(group), Item('r'))
    }
}

use std::fs;

type Calorie = u32;

#[derive(Debug, PartialEq, Eq)]
struct Elf {
    inventory: Vec<Calorie>,
}

fn load_puzzle() -> String {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    String::from(contents)
}

fn parse_elf(elf_string: &str) -> Elf {
    let inventory = elf_string
        .split("\n")
        .filter(|val| val != &"")
        .map(|val| val.parse::<Calorie>().unwrap())
        .collect();
    Elf { inventory }
}

fn parse_puzzle(puzzle_string: String) -> Vec<Elf> {
    puzzle_string.split("\n\n").map(parse_elf).collect()
}

fn get_total_calories(elves: Vec<Elf>) -> Vec<Calorie> {
    elves.iter().map(|elf| elf.inventory.iter().sum()).collect()
}

fn get_most_calories(calories: &Vec<Calorie>) -> Calorie {
    calories.iter().max().unwrap().clone()
}

fn main() {
    let puzzle_string = load_puzzle();
    let elves = parse_puzzle(puzzle_string);
    let mut total_calories = get_total_calories(elves);
    let most_calories = get_most_calories(&total_calories);

    println!("{:?}", most_calories);

    total_calories.sort();
    let top_3 = total_calories.iter().rev().take(3);
    let top_3_sum: Calorie = top_3.sum();

    println!("{:?}", top_3_sum);
}

#[cfg(test)]
mod tests {
    use crate::{get_total_calories, parse_puzzle, Elf};

    fn assert_veq_eq(a: Vec<Elf>, b: Vec<Elf>) {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b.iter())
            .map(|(elf_a, elf_b)| assert_eq!(elf_a, elf_b));
    }

    fn get_test_elves() -> Vec<Elf> {
        vec![
            Elf {
                inventory: vec![1000, 2000, 3000],
            },
            Elf {
                inventory: vec![4000],
            },
            Elf {
                inventory: vec![5000, 6000],
            },
            Elf {
                inventory: vec![7000, 8000, 9000],
            },
            Elf {
                inventory: vec![10000],
            },
        ]
    }

    #[test]
    fn test_parse_puzzle() {
        let puzzle_string =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n");
        assert_veq_eq(parse_puzzle(puzzle_string), get_test_elves());
    }

    #[test]
    fn test_get_total_calories() {
        assert_eq!(
            get_total_calories(get_test_elves()),
            vec![6000, 4000, 11000, 24000, 10000],
        );
    }
}

use env_logger;
use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

type WorryLevel = u64;
type MonkeyID = u8;

#[derive(Debug, PartialEq)]
enum A {
    Value(WorryLevel),
    X,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add(A, A),
    Mul(A, A),
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: MonkeyID,
    items: VecDeque<WorryLevel>,
    operation: Operation,
    test_value: WorryLevel,
    true_monkey_id: MonkeyID,
    false_monkey_id: MonkeyID,
    inspections: u64,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_monkey_id(input: &str) -> MonkeyID {
    let id_str = &input[7..input.len() - 1];
    id_str.parse::<MonkeyID>().unwrap()
}

fn parse_starting_items(input: &str) -> VecDeque<WorryLevel> {
    let items_str = &input.trim()[16..];
    items_str
        .split(", ")
        .map(|item| item.parse::<WorryLevel>().unwrap())
        .collect()
}

fn parse_a(input: &str) -> A {
    match input {
        "old" => A::X,
        a => A::Value(a.parse::<WorryLevel>().unwrap()),
    }
}

fn parse_operation(input: &str) -> Operation {
    let operation_str = &input.trim()[17..];
    let mut operation_split = operation_str.split(" ");

    let (lhs_str, op_str, rhs_str) = (
        operation_split.next().unwrap(),
        operation_split.next().unwrap(),
        operation_split.next().unwrap(),
    );

    let lhs = parse_a(lhs_str);
    let rhs = parse_a(rhs_str);

    match op_str {
        "+" => Operation::Add(lhs, rhs),
        "*" => Operation::Mul(lhs, rhs),
        _ => panic!("Unsupported Operation!"),
    }
}

fn parse_test_value(input: &str) -> WorryLevel {
    let test_val_str = &input.trim()[19..];
    test_val_str.parse::<WorryLevel>().unwrap()
}

fn parse_to_monkey_id(input: &str) -> MonkeyID {
    input[16..].parse::<MonkeyID>().unwrap()
}

fn parse_monkey(input: &str) -> Monkey {
    let split: Vec<String> = input.split("\n").map(String::from).collect();

    let id = parse_monkey_id(&split[0]);
    let items = parse_starting_items(&split[1]);
    let operation = parse_operation(&split[2]);
    let test_value = parse_test_value(&split[3]);
    let true_monkey_id = parse_to_monkey_id(&split[4].trim()[9..]);
    let false_monkey_id = parse_to_monkey_id(&split[5].trim()[10..]);

    Monkey {
        id,
        items,
        operation,
        test_value,
        true_monkey_id,
        false_monkey_id,
        inspections: 0,
    }
}

fn parse_monkeys(input: &str) -> HashMap<MonkeyID, Monkey> {
    input
        .split("\n\n")
        .filter(|a| a != &"")
        .map(parse_monkey)
        .map(|monkey| (monkey.id, monkey))
        .collect()
}

fn calculation(
    a: &A,
    b: &A,
    item: WorryLevel,
    operation_func: &dyn Fn(WorryLevel, WorryLevel) -> WorryLevel,
) -> WorryLevel {
    let a_val = match a {
        A::X => item,
        A::Value(val) => *val,
    };

    let b_val = match b {
        A::X => item,
        A::Value(val) => *val,
    };

    operation_func(a_val, b_val)
}

fn run_operation(operation: &Operation, item: WorryLevel) -> WorryLevel {
    match operation {
        Operation::Add(a, b) => calculation(a, b, item, &|x, y| x + y),
        Operation::Mul(a, b) => calculation(a, b, item, &|x, y| x * y),
    }
}

fn take_turn(
    id: MonkeyID,
    monkeys: &mut HashMap<MonkeyID, Monkey>,
    is_part_1: bool,
    mod_value: u64,
) {
    let monkey = monkeys.get_mut(&id).unwrap();

    let mut deq = VecDeque::new();

    while let Some(item) = monkey.items.pop_front() {
        let mut worry_level = run_operation(&monkey.operation, item);

        monkey.inspections = monkey.inspections + 1;

        if is_part_1 {
            worry_level = worry_level / 3;
        } else {
            worry_level = worry_level % mod_value
        }

        let target_monkey_id = if worry_level % monkey.test_value == 0 {
            &monkey.true_monkey_id
        } else {
            &monkey.false_monkey_id
        };

        deq.push_back((target_monkey_id.clone(), worry_level.clone()))
    }

    while let Some((id, worry)) = deq.pop_front() {
        monkeys.get_mut(&id).unwrap().items.push_back(worry);
    }
}

fn run_round<'a>(
    monkeys: &'a mut HashMap<MonkeyID, Monkey>,
    is_part_1: bool,
    mod_value: u64,
) -> &'a mut HashMap<MonkeyID, Monkey> {
    for i in 0..monkeys.len() {
        take_turn(i.try_into().unwrap(), monkeys, is_part_1, mod_value);
    }
    monkeys
}

fn run_rounds(monkeys: &mut HashMap<MonkeyID, Monkey>, num_rounds: u32, is_part_1: bool) {
    let lcm = monkeys
        .iter()
        .map(|(_, monkey)| monkey.test_value)
        .product();

    for i in 1..num_rounds + 1 {
        run_round(monkeys, is_part_1, lcm);

        if i == 1 || i == 20 || i % 1000 == 0 {
            let inspections = get_inspections(&monkeys);
            log::debug!("Round: {}, Inspections: {:?}", i, &inspections);
        }
    }
}

fn get_inspections(monkeys: &HashMap<MonkeyID, Monkey>) -> Vec<u64> {
    let mut inspections: Vec<(MonkeyID, u64)> = monkeys
        .iter()
        .map(|(i, monkey)| (*i, monkey.inspections))
        .collect();

    inspections.sort_by(|a, b| a.0.cmp(&b.0));

    inspections.into_iter().map(|(_, b)| b).collect()
}

fn get_monkey_business(input: &str, num_rounds: u32, is_part_1: bool) -> u64 {
    let mut monkeys = parse_monkeys(&input);
    run_rounds(&mut monkeys, num_rounds, is_part_1);
    let mut inspections: Vec<u64> = get_inspections(&monkeys);
    inspections.sort();
    let monkey_business: u64 = inspections.iter().rev().take(2).product();
    monkey_business
}

fn main() {
    env_logger::init();
    let input = load_input();
    let monkey_business = get_monkey_business(&input, 20, true);
    println!("{:?}", monkey_business);

    let monkey_business_2 = get_monkey_business(&input, 10000, false);
    println!("{:?}", monkey_business_2);
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, VecDeque};

    use crate::{parse_monkeys, Monkey, Operation, A};

    #[test]
    fn test_parse_monkeys() {
        let input = include_str!("../test.txt");
        let expected = HashMap::from([
            (
                0,
                Monkey {
                    id: 0,
                    items: VecDeque::from(vec![79, 98]),
                    operation: Operation::Mul(A::X, A::Value(19)),
                    test_value: 23,
                    true_monkey_id: 2,
                    false_monkey_id: 3,
                    inspections: 0,
                },
            ),
            (
                1,
                Monkey {
                    id: 1,
                    items: VecDeque::from(vec![54, 65, 75, 74]),
                    operation: Operation::Add(A::X, A::Value(6)),
                    test_value: 19,
                    true_monkey_id: 2,
                    false_monkey_id: 0,
                    inspections: 0,
                },
            ),
            (
                2,
                Monkey {
                    id: 2,
                    items: VecDeque::from(vec![79, 60, 97]),
                    operation: Operation::Mul(A::X, A::X),
                    test_value: 13,
                    true_monkey_id: 1,
                    false_monkey_id: 3,
                    inspections: 0,
                },
            ),
            (
                3,
                Monkey {
                    id: 3,
                    items: VecDeque::from(vec![74]),
                    operation: Operation::Add(A::X, A::Value(3)),
                    test_value: 17,
                    true_monkey_id: 0,
                    false_monkey_id: 1,
                    inspections: 0,
                },
            ),
        ]);
        let actual = parse_monkeys(&input);
        assert_eq!(expected, actual);
    }
}

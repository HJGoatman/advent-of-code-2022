use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

type Value = i64;
type MonkeyId = String;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Add(MonkeyId, MonkeyId),
    Subtract(MonkeyId, MonkeyId),
    Divide(MonkeyId, MonkeyId),
    Multiply(MonkeyId, MonkeyId),
}

#[derive(Debug, PartialEq, Clone)]
enum Something {
    Number(Value),
    MathsOperation(Operation),
}

#[derive(Debug, PartialEq, Clone)]
struct Job {
    monkey_id: MonkeyId,
    yell: Something,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_operation(input: &str) -> Operation {
    let mut split = input.split(' ');
    let a = split.next().unwrap().to_string();
    let op = split.next().unwrap();
    let b = split.next().unwrap().to_string();

    match op {
        "+" => Operation::Add(a, b),
        "-" => Operation::Subtract(a, b),
        "*" => Operation::Multiply(a, b),
        "/" => Operation::Divide(a, b),
        _ => panic!("Unknown Operator whilst parsing!"),
    }
}

fn parse_job(input: &str) -> Job {
    let mut split = input.split(": ");
    let monkey_id = split.next().unwrap().to_string();

    let yell_str = split.next().unwrap();
    let determine_yell: char = yell_str.chars().next().unwrap();

    let yell = match determine_yell.is_ascii_digit() {
        true => Something::Number(yell_str.parse::<Value>().unwrap()),
        false => Something::MathsOperation(parse_operation(yell_str)),
    };

    Job { monkey_id, yell }
}

fn parse_jobs(input: &str) -> HashMap<MonkeyId, Something> {
    HashMap::from_iter(
        input
            .split('\n')
            .map(parse_job)
            .map(|job| (job.monkey_id, job.yell)),
    )
}

fn get_execution_order(job_map: &HashMap<MonkeyId, Something>, root: &str) -> Vec<MonkeyId> {
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    queue.push_front(job_map.get(root).unwrap());
    order.push(root.to_string());

    while !queue.is_empty() {
        let job = queue.pop_back().unwrap();

        let maybe_children = match job {
            Something::Number(_) => None,
            Something::MathsOperation(Operation::Add(a, b))
            | Something::MathsOperation(Operation::Subtract(a, b))
            | Something::MathsOperation(Operation::Divide(a, b))
            | Something::MathsOperation(Operation::Multiply(a, b)) => Some([a, b]),
        };

        if let Some(children) = maybe_children {
            for child in children.into_iter() {
                if order.contains(child) {
                    order.remove(order.iter().position(|a| a == child).unwrap());
                }

                order.push(child.to_string());
                queue.push_front(job_map.get(child).unwrap())
            }
        }
    }

    order.reverse();
    log::trace!("{:?}", order);

    order
}

fn unwrap_value(jobs: &HashMap<MonkeyId, Something>, monkey_id: &str) -> Value {
    let something = jobs.get(monkey_id).unwrap();

    if let Something::Number(v) = something {
        return *v;
    }

    panic!("Expected to unwrap a Number not an expression!");
}

fn get_yell(job_map_read: &HashMap<MonkeyId, Something>, root: &str) -> Value {
    let mut job_map: HashMap<MonkeyId, Something> = job_map_read.clone();

    let execution_order = get_execution_order(&job_map, root);

    log::trace!("{:#?}", execution_order);

    for monkey_id in execution_order {
        let something = job_map.get(&monkey_id).unwrap();

        if let Something::MathsOperation(operation) = something {
            let result = match operation {
                Operation::Add(a, b) => unwrap_value(&job_map, a) + unwrap_value(&job_map, b),
                Operation::Subtract(a, b) => unwrap_value(&job_map, a) - unwrap_value(&job_map, b),
                Operation::Multiply(a, b) => unwrap_value(&job_map, a) * unwrap_value(&job_map, b),
                Operation::Divide(a, b) => unwrap_value(&job_map, a) / unwrap_value(&job_map, b),
            };

            job_map.insert(monkey_id, Something::Number(result));
        }
    }

    unwrap_value(&job_map, root)
}

fn find_path(job_map: &HashMap<MonkeyId, Something>, start: &str, goal: &str) -> Vec<MonkeyId> {
    let mut stack = Vec::new();

    stack.push((start, vec![]));

    while !stack.is_empty() {
        let (id, path) = stack.pop().unwrap();

        if id == goal {
            let mut new_path = path;
            new_path.push(id.to_string());
            return new_path;
        }

        let job = job_map.get(id).unwrap();

        let maybe_children = match job {
            Something::Number(_) => None,
            Something::MathsOperation(Operation::Add(a, b))
            | Something::MathsOperation(Operation::Subtract(a, b))
            | Something::MathsOperation(Operation::Divide(a, b))
            | Something::MathsOperation(Operation::Multiply(a, b)) => Some([a, b]),
        };

        if let Some(children) = maybe_children {
            for child in children.into_iter() {
                let mut new_path = path.clone();
                new_path.push(id.to_string());
                stack.push((child, new_path));
            }
        }
    }

    panic!("No humn found!");
}

fn get_humn_yell(jobs: &HashMap<MonkeyId, Something>) -> Value {
    let path_to_humn = find_path(jobs, "root", "humn");
    log::debug!("{:?}", path_to_humn);

    let root_job = jobs.get("root").unwrap();
    let maybe_children = match root_job {
        Something::Number(_) => None,
        Something::MathsOperation(Operation::Add(a, b))
        | Something::MathsOperation(Operation::Subtract(a, b))
        | Something::MathsOperation(Operation::Divide(a, b))
        | Something::MathsOperation(Operation::Multiply(a, b)) => Some([a, b]),
    };

    let child = path_to_humn.get(1).unwrap();
    let other = maybe_children
        .unwrap()
        .into_iter()
        .find(|a| *a != child)
        .unwrap();
    let mut humn_yell = get_yell(jobs, other);
    log::debug!("Yell: {}", humn_yell);

    for i in 1..(path_to_humn.len() - 1) {
        let parent = path_to_humn.get(i).unwrap();
        let child = path_to_humn.get(i + 1).unwrap();

        log::debug!("Parent: {}, Child: {}", parent, child);

        if let Something::MathsOperation(parent_operation) = jobs.get(parent).unwrap() {
            log::debug!("{:?}", parent_operation);

            if let Operation::Add(a, b) = parent_operation {
                if a == child {
                    humn_yell -= get_yell(jobs, b);
                } else {
                    humn_yell -= get_yell(jobs, a);
                }
            }

            if let Operation::Subtract(a, b) = parent_operation {
                if a == child {
                    humn_yell += get_yell(jobs, b);
                } else {
                    humn_yell = get_yell(jobs, a) - humn_yell;
                }
            }

            if let Operation::Multiply(a, b) = parent_operation {
                if a == child {
                    humn_yell /= get_yell(jobs, b);
                } else {
                    humn_yell /= get_yell(jobs, a);
                }
            }

            if let Operation::Divide(a, b) = parent_operation {
                if a == child {
                    humn_yell *= get_yell(jobs, b);
                } else {
                    humn_yell = get_yell(jobs, a) / humn_yell;
                }
            }
        }
        log::debug!("Yell: {}", humn_yell);
    }

    humn_yell
}

fn main() {
    env_logger::init();
    let input = load_input();
    let jobs = parse_jobs(&input);

    let root_yell = get_yell(&jobs, "root");
    println!("{}", root_yell);

    let humn_yell = get_humn_yell(&jobs);
    println!("{}", humn_yell);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{get_humn_yell, get_yell, parse_jobs, Job, MonkeyId, Operation, Something};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_test_jobs() -> HashMap<MonkeyId, Something> {
        let job_list = vec![
            Job {
                monkey_id: String::from("root"),
                yell: Something::MathsOperation(Operation::Add(
                    String::from("pppw"),
                    String::from("sjmn"),
                )),
            },
            Job {
                monkey_id: String::from("dbpl"),
                yell: Something::Number(5),
            },
            Job {
                monkey_id: String::from("cczh"),
                yell: Something::MathsOperation(Operation::Add(
                    String::from("sllz"),
                    String::from("lgvd"),
                )),
            },
            Job {
                monkey_id: String::from("zczc"),
                yell: Something::Number(2),
            },
            Job {
                monkey_id: String::from("ptdq"),
                yell: Something::MathsOperation(Operation::Subtract(
                    String::from("humn"),
                    String::from("dvpt"),
                )),
            },
            Job {
                monkey_id: String::from("dvpt"),
                yell: Something::Number(3),
            },
            Job {
                monkey_id: String::from("lfqf"),
                yell: Something::Number(4),
            },
            Job {
                monkey_id: String::from("humn"),
                yell: Something::Number(5),
            },
            Job {
                monkey_id: String::from("ljgn"),
                yell: Something::Number(2),
            },
            Job {
                monkey_id: String::from("sjmn"),
                yell: Something::MathsOperation(Operation::Multiply(
                    String::from("drzm"),
                    String::from("dbpl"),
                )),
            },
            Job {
                monkey_id: String::from("sllz"),
                yell: Something::Number(4),
            },
            Job {
                monkey_id: String::from("pppw"),
                yell: Something::MathsOperation(Operation::Divide(
                    String::from("cczh"),
                    String::from("lfqf"),
                )),
            },
            Job {
                monkey_id: String::from("lgvd"),
                yell: Something::MathsOperation(Operation::Multiply(
                    String::from("ljgn"),
                    String::from("ptdq"),
                )),
            },
            Job {
                monkey_id: String::from("drzm"),
                yell: Something::MathsOperation(Operation::Subtract(
                    String::from("hmdt"),
                    String::from("zczc"),
                )),
            },
            Job {
                monkey_id: String::from("hmdt"),
                yell: Something::Number(32),
            },
        ];

        HashMap::from_iter(job_list.into_iter().map(|job| (job.monkey_id, job.yell)))
    }

    #[test]
    fn test_parse_input() {
        init();
        let input = include_str!("../test.txt");
        let expected = get_test_jobs();
        let actual = parse_jobs(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_root_yell() {
        init();
        let input = get_test_jobs();
        let expected = 152;
        let actual = get_yell(&input, "root");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_humn_yell() {
        init();
        let input = get_test_jobs();
        let expected = 301;
        let actual = get_humn_yell(&input);
        assert_eq!(expected, actual);
    }
}

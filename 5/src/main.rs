use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Crate(char);

type StackID = u8;

#[derive(Debug, PartialEq, Clone)]
struct Stack {
    id: StackID,
    stack: Vec<Crate>,
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    amount: u32,
    from: StackID,
    to: StackID,
}

type Procedure = Vec<Instruction>;

fn load_input() -> String {
    fs::read_to_string("input.txt").expect("Should have been able to read the file")
}

fn parse_crate(input: &str) -> Option<Crate> {
    assert_eq!(input.len(), 3);

    if input == "   " {
        return None;
    }

    Some(Crate(input.chars().nth(1).unwrap()))
}

fn parse_stack_line(input: &String) -> Vec<Option<Crate>> {
    let mut stack_line = Vec::new();
    let mut chars = input.chars();

    loop {
        let maybe_crate_str: String = (0..).take(3).map(|_| chars.next().unwrap()).collect();

        let maybe_crate = parse_crate(&maybe_crate_str);
        stack_line.push(maybe_crate);

        let space = chars.next();

        if space.is_none() {
            break;
        }
    }

    stack_line
}

fn parse_id_line(input: &str) -> Vec<StackID> {
    String::from(input)
        .trim()
        .split("   ")
        .map(|id| id.parse::<StackID>().unwrap())
        .collect()
}

fn create_stacks(rows: &[&[Option<Crate>]], ids: &[StackID]) -> Vec<Stack> {
    let mut stacks: Vec<Vec<Crate>> = Vec::new();

    for _ in ids {
        stacks.push(Vec::new())
    }

    for row in rows.iter().rev() {
        for (i, maybe_crate) in row.iter().enumerate() {
            match maybe_crate {
                Some(c) => stacks[i].push(c.clone()),
                None => continue,
            }
        }
    }

    ids.into_iter()
        .zip(stacks.into_iter())
        .map(|(id, stack)| Stack {
            id: id.clone(),
            stack,
        })
        .collect()
}

fn parse_stacks(input: &str) -> Vec<Stack> {
    let lines: Vec<String> = input.split('\n').map(String::from).collect();
    let last_index = lines.len() - 1;
    let stacks_lines = &lines[..last_index];
    let id_line = &lines[last_index];

    let stack_rows: Vec<Vec<Option<Crate>>> = stacks_lines.iter().map(parse_stack_line).collect();
    let ids: Vec<StackID> = parse_id_line(id_line);

    let slice_2d: Vec<&[Option<Crate>]> = stack_rows.iter().map(Vec::as_slice).collect();
    create_stacks(&slice_2d, &ids)
}

fn parse_instruction(input: &str) -> Instruction {
    let mut split = input.split(" ");
    Instruction {
        amount: split.nth(1).unwrap().parse::<u32>().unwrap(),
        from: split.nth(1).unwrap().parse::<StackID>().unwrap(),
        to: split.nth(1).unwrap().parse::<StackID>().unwrap(),
    }
}

fn parse_procedure(input: &str) -> Procedure {
    input.split('\n').map(parse_instruction).collect()
}

fn parse_input(input: &str) -> (Vec<Stack>, Procedure) {
    let (stacks_str, procedure_str) = input.split_once("\n\n").unwrap();
    (parse_stacks(&stacks_str), parse_procedure(&procedure_str))
}

fn run_instruction<'a>(
    stacks: &'a mut Vec<Stack>,
    instruction: &Instruction,
) -> &'a mut Vec<Stack> {
    for _ in 0..instruction.amount {
        let from = &mut stacks[(instruction.from - 1) as usize];
        let from_crate = from.stack.pop().unwrap();
        let to = &mut stacks[(instruction.to - 1) as usize];

        to.stack.push(from_crate);
    }

    stacks
}

fn run_instruction_9001<'a>(
    stacks: &'a mut Vec<Stack>,
    instruction: &Instruction,
) -> &'a mut Vec<Stack> {
    let mut queue = vec![];
    for _ in 0..instruction.amount {
        let from = &mut stacks[(instruction.from - 1) as usize];
        let from_crate = from.stack.pop().unwrap();
        queue.push(from_crate);
    }

    let to = &mut stacks[(instruction.to - 1) as usize];

    for _ in 0..instruction.amount {
        to.stack.push(queue.pop().unwrap())
    }

    stacks
}

fn run<'a>(
    stacks: &'a mut Vec<Stack>,
    procedure: &[Instruction],
    processor: &dyn Fn(&'a mut Vec<Stack>, &Instruction) -> &'a mut Vec<Stack>,
) -> &'a mut Vec<Stack> {
    procedure.to_vec().iter().fold(stacks, processor)
}

fn get_top_crates(stacks: &[Stack]) -> String {
    stacks
        .iter()
        .map(|stack| stack.stack.last().unwrap().0)
        .collect()
}

fn main() {
    let input = load_input();
    let (mut stacks, procedure) = parse_input(&input);
    let modified_stacks = run(&mut stacks, &procedure, &run_instruction);
    let top_crates = get_top_crates(&modified_stacks);
    println!("{}", top_crates);

    // Part 2
    let (mut stacks_2, _) = parse_input(&input);
    let modified_stacks_2 = run(&mut stacks_2, &procedure, &run_instruction_9001);
    let top_crates_2 = get_top_crates(&modified_stacks_2);
    println!("{}", top_crates_2);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, parse_stack_line, run, run_instruction, Crate, Instruction, Stack};

    #[test]
    fn test_parse_input() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        let expected_stacks = vec![
            Stack {
                id: 1,
                stack: vec![Crate('Z'), Crate('N')],
            },
            Stack {
                id: 2,
                stack: vec![Crate('M'), Crate('C'), Crate('D')],
            },
            Stack {
                id: 3,
                stack: vec![Crate('P')],
            },
        ];

        let expected_procedure = vec![
            Instruction {
                amount: 1,
                from: 2,
                to: 1,
            },
            Instruction {
                amount: 3,
                from: 1,
                to: 3,
            },
            Instruction {
                amount: 2,
                from: 2,
                to: 1,
            },
            Instruction {
                amount: 1,
                from: 1,
                to: 2,
            },
        ];

        let (stacks, procedure) = parse_input(&input);
        assert_eq!(stacks, expected_stacks);
        assert_eq!(expected_procedure, procedure);
    }

    #[test]
    fn test_parse_stack_line() {
        let input = String::from("    [D]    ");
        let expected = vec![None, Some(Crate('D')), None];
        let actual = parse_stack_line(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_run_instruction() {
        let mut stacks = vec![
            Stack {
                id: 1,
                stack: vec![Crate('Z'), Crate('N')],
            },
            Stack {
                id: 2,
                stack: vec![Crate('M'), Crate('C'), Crate('D')],
            },
            Stack {
                id: 3,
                stack: vec![Crate('P')],
            },
        ];

        let instruction = Instruction {
            amount: 1,
            from: 2,
            to: 1,
        };

        let mut expected = vec![
            Stack {
                id: 1,
                stack: vec![Crate('Z'), Crate('N'), Crate('D')],
            },
            Stack {
                id: 2,
                stack: vec![Crate('M'), Crate('C')],
            },
            Stack {
                id: 3,
                stack: vec![Crate('P')],
            },
        ];

        run_instruction(&mut stacks, &instruction);
        assert_eq!(expected, stacks);

        let instruction_2 = Instruction {
            amount: 3,
            from: 1,
            to: 3,
        };
        let mut expected_2 = vec![
            Stack {
                id: 1,
                stack: vec![],
            },
            Stack {
                id: 2,
                stack: vec![Crate('M'), Crate('C')],
            },
            Stack {
                id: 3,
                stack: vec![Crate('P'), Crate('D'), Crate('N'), Crate('Z')],
            },
        ];

        run_instruction(&mut expected, &instruction_2);
        assert_eq!(expected_2, expected);
    }

    #[test]
    fn test_run() {
        let mut input_stacks = vec![
            Stack {
                id: 1,
                stack: vec![Crate('Z'), Crate('N')],
            },
            Stack {
                id: 2,
                stack: vec![Crate('M'), Crate('C'), Crate('D')],
            },
            Stack {
                id: 3,
                stack: vec![Crate('P')],
            },
        ];

        let input_procedure = vec![
            Instruction {
                amount: 1,
                from: 2,
                to: 1,
            },
            Instruction {
                amount: 3,
                from: 1,
                to: 3,
            },
            Instruction {
                amount: 2,
                from: 2,
                to: 1,
            },
            Instruction {
                amount: 1,
                from: 1,
                to: 2,
            },
        ];

        let expected_stacks = vec![
            Stack {
                id: 1,
                stack: vec![Crate('C')],
            },
            Stack {
                id: 2,
                stack: vec![Crate('M')],
            },
            Stack {
                id: 3,
                stack: vec![Crate('P'), Crate('D'), Crate('N'), Crate('Z')],
            },
        ];

        let actual_stacks = run(&mut input_stacks, &input_procedure, &run_instruction);
        assert_eq!(expected_stacks, input_stacks);
    }
}

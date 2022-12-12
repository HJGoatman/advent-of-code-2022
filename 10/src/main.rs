use env_logger;
use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Instruction {
    NoOp,
    AddX(i32),
}

type Program = Vec<Instruction>;
type SignalStrength = i32;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_instruction(input: &str) -> Instruction {
    if input == "noop" {
        return Instruction::NoOp;
    }

    let val = input.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
    Instruction::AddX(val)
}

fn parse_program(input: &str) -> Program {
    input
        .split("\n")
        .filter(|line| line != &"")
        .map(parse_instruction)
        .collect()
}

fn increment_clock(
    clock: &mut dyn Iterator<Item = i32>,
    register: &i32,
    signal_strengths: &mut Vec<SignalStrength>,
    crt: &mut Vec<bool>,
) {
    let cycle_num = clock.next().unwrap();
    log::debug!("Cycle: {}", cycle_num);
    if (cycle_num == 20) || ((cycle_num - 20) % 40 == 0) {
        let signal_strength = cycle_num * register;
        signal_strengths.push(signal_strength);

        log::debug!("During the {}th cycle, register X has the value {}, so the signal strength is {} * {} = {}.", cycle_num, register, cycle_num, register, signal_strength);
    }

    let pixel = (cycle_num % 40) - 1;
    let sprite_start = register - 1;
    let sprite_end = register + 1;
    let is_pixel_lit = sprite_start <= pixel && pixel <= sprite_end;
    crt.push(is_pixel_lit);
}

fn run_program(program: &[Instruction]) -> (Vec<SignalStrength>, Vec<bool>) {
    let mut signal_strengths: Vec<SignalStrength> = vec![];
    let mut clock = 1..;
    let mut register = 1;
    let mut crt: Vec<bool> = vec![];

    for instruction in program {
        if let Instruction::AddX(val) = instruction {
            increment_clock(&mut clock, &register, &mut signal_strengths, &mut crt);
            increment_clock(&mut clock, &register, &mut signal_strengths, &mut crt);
            register = register + val;
        } else {
            increment_clock(&mut clock, &register, &mut signal_strengths, &mut crt);
        }

        log::debug!("Instruction: {:?}, X: {}", instruction, register);
        log::debug!("");
    }

    (signal_strengths, crt)
}

fn display_crt(crt: &[bool]) -> String {
    let chars: Vec<char> = crt
        .iter()
        .map(|a| match a {
            true => '#',
            _ => '.',
        })
        .collect();

    chars.chunks(40).fold(String::from(""), |x, y| {
        format!("{}\n{}", x, y.to_vec().iter().collect::<String>())
    })
}

fn main() {
    env_logger::init();

    let input = load_input();
    let program = parse_program(&input);
    let (signal_strengths, crt) = run_program(&program);
    let total_signal_strength: i32 = signal_strengths.iter().sum();

    println!("{:?}", total_signal_strength);

    let display = display_crt(&crt);
    print!("{}", display);
}

#[cfg(test)]
mod tests {
    use crate::{parse_program, run_program, Instruction};

    #[test]
    fn test_parse_program() {
        let input = "noop\naddx 3\naddx -5\n";
        let expected = vec![
            Instruction::NoOp,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ];
        let actual = parse_program(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_run() {
        env_logger::init();
        let input = include_str!("../test.txt");
        let input_program = parse_program(&input);
        let signal_strengths = run_program(&input_program);
        let expected_signal_strengths = vec![420, 1140, 1800, 2940, 2880, 3960];
        assert_eq!(expected_signal_strengths, signal_strengths);
    }
}

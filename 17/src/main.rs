use std::{env, fs};

mod coordinate;
mod parser;
mod simulator;

use parser::parse_jet_pattern;
use simulator::run_simulation;

fn main() {
    env_logger::init();
    let input = load_input();
    let jet_pattern = parse_jet_pattern(&input);
    let chamber = run_simulation(&jet_pattern, 1000000000000);
    let tower_height = &chamber.get_adjusted_height();
    println!("{}", tower_height);
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

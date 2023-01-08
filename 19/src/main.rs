use enum_map::{enum_map, Enum, EnumMap};

use std::{env, fs};

type Cost = u16;
type Count = u16;

#[derive(Debug, PartialEq, Copy, Clone)]
struct ObsidianRobotCost {
    ore: Cost,
    clay: Cost,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct GeodeRobotCost {
    ore: Cost,
    obsidian: Cost,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Blueprint {
    id: u8,
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: ObsidianRobotCost,
    geode_robot_cost: GeodeRobotCost,
}

#[derive(Debug, PartialEq, Eq, Enum, Copy, Clone)]
enum OreType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Copy, Clone)]
struct State {
    minute: u8,
    robots: EnumMap<OreType, Count>,
    ores: EnumMap<OreType, Count>,
}

impl State {
    fn iter(&self, blueprint: Blueprint) -> StateIter {
        StateIter {
            state: self,
            i: 0,
            blueprint,
        }
    }
}

struct StateIter<'a> {
    state: &'a State,
    i: usize,
    blueprint: Blueprint,
}

impl<'a> Iterator for StateIter<'a> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        fn a(b: &[(OreType, Cost)], robot: OreType, mut next: State) -> Option<State> {
            let is_affordable = b.iter().all(|(typ, cost)| next.ores[*typ] >= *cost);

            if !is_affordable {
                return None;
            }

            if is_affordable {
                for (typ, cost) in b.iter() {
                    next.ores[*typ] -= cost
                }
            }

            next = build_robots(next);

            if is_affordable {
                next.robots[robot] += 1;
            }

            Some(next)
        }

        fn build_robots(mut next: State) -> State {
            next.ores[OreType::Ore] += next.robots[OreType::Ore];
            next.ores[OreType::Clay] += next.robots[OreType::Clay];
            next.ores[OreType::Obsidian] += next.robots[OreType::Obsidian];
            next.ores[OreType::Geode] += next.robots[OreType::Geode];

            next
        }

        let blueprint = self.blueprint;

        let mut maybe_next_val = None;
        while maybe_next_val.is_none() {
            if self.i > 4 {
                break;
            }

            maybe_next_val = match self.i {
                0 => Some(build_robots(*self.state)),
                1 => a(
                    &[(OreType::Ore, blueprint.ore_robot_cost)],
                    OreType::Ore,
                    *self.state,
                ),
                2 => a(
                    &[(OreType::Ore, blueprint.clay_robot_cost)],
                    OreType::Clay,
                    *self.state,
                ),
                3 => a(
                    &[
                        (OreType::Clay, blueprint.obsidian_robot_cost.clay),
                        (OreType::Ore, blueprint.obsidian_robot_cost.ore),
                    ],
                    OreType::Obsidian,
                    *self.state,
                ),
                4 => a(
                    &[
                        (OreType::Ore, blueprint.geode_robot_cost.ore),
                        (OreType::Obsidian, blueprint.geode_robot_cost.obsidian),
                    ],
                    OreType::Geode,
                    *self.state,
                ),
                _ => None,
            };

            if maybe_next_val.is_none() {
                self.i += 1;
            }
        }

        self.i += 1;

        maybe_next_val = maybe_next_val.map(|mut a: State| {
            a.minute += 1;
            a
        });

        maybe_next_val
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_blueprint(input: &str) -> Blueprint {
    let mut split = input.split(": ");
    let id = split.next().unwrap()[10..].parse::<u8>().unwrap();
    let description = split.next().unwrap();

    let mut robot_requirements = description.split(". ");

    let ore_robot_cost = robot_requirements
        .next()
        .map(|val| val[21..val.len() - 4].parse::<Cost>().unwrap())
        .unwrap();

    let clay_robot_cost = robot_requirements
        .next()
        .map(|val| val[22..val.len() - 4].parse::<Cost>().unwrap())
        .unwrap();

    let (obsidian_robot_cost_ore, obsidian_robot_cost_clay) = robot_requirements
        .next()
        .map(|val| {
            let stripped = &val[26..&val.len() - 5];
            let mut splitted = stripped.split(" ore and ");
            (
                splitted.next().unwrap().parse::<Cost>().unwrap(),
                splitted.next().unwrap().parse::<Cost>().unwrap(),
            )
        })
        .unwrap();
    let obsidian_robot_cost = ObsidianRobotCost {
        ore: obsidian_robot_cost_ore,
        clay: obsidian_robot_cost_clay,
    };

    let (geode_robot_cost_ore, geode_robot_cost_obsidian) = robot_requirements
        .next()
        .map(|val| {
            let stripped = &val[23..&val.len() - 10];
            let mut splitted = stripped.split(" ore and ");
            (
                splitted.next().unwrap().parse::<Cost>().unwrap(),
                splitted.next().unwrap().parse::<Cost>().unwrap(),
            )
        })
        .unwrap();
    let geode_robot_cost = GeodeRobotCost {
        ore: geode_robot_cost_ore,
        obsidian: geode_robot_cost_obsidian,
    };

    Blueprint {
        id,
        ore_robot_cost,
        clay_robot_cost,
        obsidian_robot_cost,
        geode_robot_cost,
    }
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    input
        .split('\n')
        .filter(|a| a != &"")
        .map(parse_blueprint)
        .collect()
}

fn overestimate_maximum(state: State, max_time: u8) -> Count {
    let remaining_time = (max_time - state.minute) as u16;

    let guarenteed_geodes: Count = state.robots[OreType::Geode] * remaining_time;
    let estimated_geodes = (remaining_time * (remaining_time - 1)) / 2;
    let total_geodes = state.ores[OreType::Geode] + guarenteed_geodes + estimated_geodes;
    total_geodes
}

fn dfs(blueprint: &Blueprint, state: State, max_time: u8, current_max: Count) -> Count {
    log::debug!("{:?}", state);

    if state.minute == max_time {
        return state.ores[OreType::Geode];
    }

    let mut max = current_max;

    for new_state in state.iter(*blueprint) {
        if overestimate_maximum(state, max_time) < max {
            continue;
        }

        let maybe_new_max = dfs(blueprint, new_state, max_time, max);

        if maybe_new_max > max {
            max = maybe_new_max;
        }
    }

    max
}

fn maximise_geodes(blueprint: &Blueprint, minutes: u8) -> Count {
    let initial_state = State {
        minute: 0,
        robots: enum_map! {
        OreType::Ore => 1,
        OreType::Clay => 0,
        OreType::Obsidian => 0,
        OreType::Geode => 0,
        },
        ores: enum_map! {
        OreType::Ore => 0,
        OreType::Clay => 0,
        OreType::Obsidian => 0,
        OreType::Geode => 0,
        },
    };

    dfs(blueprint, initial_state, minutes, 0)
}

fn get_quality_level_sum(blueprints: &[Blueprint], minutes: u8) -> Count {
    blueprints
        .iter()
        .map(|blueprint| blueprint.id as Count * maximise_geodes(blueprint, minutes))
        .sum()
}

fn get_geode_product(blueprints: &[Blueprint], minutes: u8) -> Count {
    blueprints
        .iter()
        .map(|blueprint| maximise_geodes(blueprint, minutes))
        .product()
}

fn main() {
    env_logger::init();
    let input = load_input();
    let blueprints = parse_blueprints(&input);
    let quality_level_sum = get_quality_level_sum(&blueprints, 24);
    println!("{}", quality_level_sum);

    // Warning this takes 11 hours on input.
    let first_3: Vec<Blueprint> = blueprints.iter().take(3).cloned().collect();
    let geode_product = get_geode_product(&first_3, 32);
    println!("{}", geode_product);
}

#[cfg(test)]
mod tests {
    use crate::{
        get_quality_level_sum, maximise_geodes, parse_blueprints, Blueprint, GeodeRobotCost,
        ObsidianRobotCost,
    };

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_test_blueprints() -> Vec<Blueprint> {
        vec![
            Blueprint {
                id: 1,
                ore_robot_cost: 4,
                clay_robot_cost: 2,
                obsidian_robot_cost: ObsidianRobotCost { ore: 3, clay: 14 },
                geode_robot_cost: GeodeRobotCost {
                    ore: 2,
                    obsidian: 7,
                },
            },
            Blueprint {
                id: 2,
                ore_robot_cost: 2,
                clay_robot_cost: 3,
                obsidian_robot_cost: ObsidianRobotCost { ore: 3, clay: 8 },
                geode_robot_cost: GeodeRobotCost {
                    ore: 3,
                    obsidian: 12,
                },
            },
        ]
    }

    #[test]
    fn test_parse_blueprints() {
        init();
        let input = include_str!("../test.txt");
        let expected = get_test_blueprints();
        let actual = parse_blueprints(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_maximise_geodes() {
        init();
        let blueprints = get_test_blueprints();

        let expected_1 = 9;
        let actual_1 = maximise_geodes(blueprints.get(0).unwrap(), 24);
        assert_eq!(expected_1, actual_1);

        let expected_2 = 12;
        let actual_2 = maximise_geodes(blueprints.get(1).unwrap(), 24);
        assert_eq!(expected_2, actual_2);
    }

    #[test]
    fn test_get_quality_level_sum() {
        init();
        let blueprints = get_test_blueprints();

        let expected = 33;
        let actual = get_quality_level_sum(&blueprints, 24);
        assert_eq!(expected, actual);
    }
}

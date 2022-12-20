use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    env, fs,
};

type FlowRate = u32;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Valve(String);

type FlowRates = HashMap<Valve, FlowRate>;

type Tunnels = HashMap<Valve, Vec<Valve>>;

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_scan_item(input: &str) -> ((Valve, FlowRate), (Valve, Vec<Valve>)) {
    let mut split = input.split("; ");
    let part_1 = split.next().unwrap();
    let part_2 = split.next().unwrap();
    let name = String::from(&part_1[6..8]);
    let flow_rate = part_1[23..].parse::<FlowRate>().unwrap();
    let tunnels = part_2[22..]
        .trim()
        .split(", ")
        .map(String::from)
        .map(Valve)
        .collect();

    ((Valve(name.clone()), flow_rate), (Valve(name), tunnels))
}

fn parse_input(input: &str) -> (FlowRates, Tunnels) {
    let (flow_rates, tunnels): (FlowRates, Tunnels) =
        input.split('\n').map(parse_scan_item).unzip();
    (flow_rates, tunnels)
}

#[derive(PartialEq, Eq)]
struct QueueItem {
    valve: Valve,
    priority: u32,
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn dijkstra(tunnels: &Tunnels, source: &Valve) -> HashMap<Valve, u32> {
    let mut distances: HashMap<Valve, u32> = HashMap::new();
    distances.insert(source.clone(), 0);

    let mut heap = BinaryHeap::new();

    for tunnel in tunnels.keys() {
        if tunnel != source {
            distances.insert(tunnel.clone(), u32::MAX);
        }
    }

    heap.push(Reverse(QueueItem {
        valve: source.clone(),
        priority: 0,
    }));

    while let Some(Reverse(tunnel)) = heap.pop() {
        if tunnel.priority < *distances.get(&tunnel.valve).unwrap() {
            continue;
        }

        for neighbour in &tunnels[&tunnel.valve] {
            let alt = distances.get(&tunnel.valve).unwrap() + 1;

            if alt < *distances.get(neighbour).unwrap() {
                *distances.get_mut(neighbour).unwrap() = alt;

                heap.push(Reverse(QueueItem {
                    valve: neighbour.clone(),
                    priority: alt,
                }));
            }
        }
    }

    distances
}

fn visit<'a>(
    distances: &HashMap<Valve, HashMap<Valve, u32>>,
    bitmap: &HashMap<Valve, u16>,
    flow_rates: &FlowRates,
    start: &Valve,
    budget: FlowRate,
    state: u16,
    flow: FlowRate,
    answer: &'a mut HashMap<u16, FlowRate>,
) -> &'a mut HashMap<u16, FlowRate> {
    let new_current_answer = std::cmp::max(*answer.get(&state).unwrap_or(&0), flow);
    if let Some(current_answer) = answer.get_mut(&state) {
        *current_answer = new_current_answer;
    } else {
        answer.insert(state, new_current_answer);
    }

    for (u, distance_start_to_next) in distances.get(start).unwrap() {
        if bitmap.get(u).unwrap() & state != 0 || distance_start_to_next + 1 > budget {
            continue;
        }

        let new_budget = budget - distance_start_to_next - 1;

        visit(
            distances,
            bitmap,
            flow_rates,
            u,
            new_budget,
            bitmap.get(u).unwrap() | state,
            flow + new_budget * flow_rates.get(u).unwrap(),
            answer,
        );
    }

    answer
}

fn get_all_states(
    flow_rates: &FlowRates,
    tunnels: &Tunnels,
    start: &Valve,
    time: u32,
) -> HashMap<u16, FlowRate> {
    let positive_flow_rates: FlowRates = flow_rates
        .iter()
        .filter(|(k, v)| **v != 0 || k == &start)
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    log::debug!("{} -> {}", flow_rates.len(), positive_flow_rates.len());

    let pairwise_distances: HashMap<Valve, HashMap<Valve, u32>> = tunnels
        .keys()
        .map(|tunnel| (tunnel.clone(), dijkstra(tunnels, tunnel)))
        .collect();

    log::debug!("{:?}", pairwise_distances);

    let positive_distances = pairwise_distances
        .into_iter()
        .filter(|(k, _)| positive_flow_rates.contains_key(k))
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .filter(|(k, _)| positive_flow_rates.contains_key(k))
                    .collect(),
            )
        })
        .collect();

    let bitmap: HashMap<Valve, u16> = positive_flow_rates
        .keys()
        .enumerate()
        .map(|(i, k)| (k.clone(), 1 << i))
        .collect();

    let mut answer = HashMap::new();

    visit(
        &positive_distances,
        &bitmap,
        &positive_flow_rates,
        start,
        time,
        0,
        0,
        &mut answer,
    );

    answer
}

fn get_most_pressure_possible(
    flow_rates: &FlowRates,
    tunnels: &Tunnels,
    start: &Valve,
    time: u32,
) -> FlowRate {
    let all_states = get_all_states(flow_rates, tunnels, start, time);
    *all_states.values().max().unwrap()
}

fn get_most_pressure_possible_with_elephant(
    flow_rates: &FlowRates,
    tunnels: &Tunnels,
    start: &Valve,
    time: u32,
) -> FlowRate {
    let all_states = get_all_states(flow_rates, tunnels, start, time);

    let keys: Vec<u16> = all_states.keys().cloned().collect();
    let mut disjoint_pairs: Vec<(u16, u16)> = Vec::new();

    for i in 0..(keys.len() - 1) {
        for j in i + 1..keys.len() {
            if (keys[i] & keys[j]) != 0 {
                continue;
            }

            disjoint_pairs.push((keys[i], keys[j]));
        }
    }

    disjoint_pairs
        .iter()
        .map(|(left, right)| all_states.get(left).unwrap() + all_states.get(right).unwrap())
        .max()
        .unwrap()
}

fn main() {
    env_logger::init();
    let input = load_input();
    let (flow_rates, tunnels) = parse_input(&input);
    log::debug!("Flow Rates: {:?}", flow_rates);
    log::debug!("Tunnels: {:?}", tunnels);
    let most_pressure_state =
        get_most_pressure_possible(&flow_rates, &tunnels, &Valve(String::from("AA")), 30);
    println!("{}", most_pressure_state);

    let most_pressure_with_elephant = get_most_pressure_possible_with_elephant(
        &flow_rates,
        &tunnels,
        &Valve(String::from("AA")),
        26,
    );
    println!("{}", most_pressure_with_elephant);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        get_most_pressure_possible, get_most_pressure_possible_with_elephant, parse_input,
        FlowRates, Tunnels, Valve,
    };

    fn get_test_flow_rates() -> FlowRates {
        HashMap::from([
            (Valve(String::from("AA")), 0),
            (Valve(String::from("BB")), 13),
            (Valve(String::from("CC")), 2),
            (Valve(String::from("DD")), 20),
            (Valve(String::from("EE")), 3),
            (Valve(String::from("FF")), 0),
            (Valve(String::from("GG")), 0),
            (Valve(String::from("HH")), 22),
            (Valve(String::from("II")), 0),
            (Valve(String::from("JJ")), 21),
        ])
    }

    fn get_test_tunnels() -> Tunnels {
        HashMap::from([
            (
                Valve(String::from("AA")),
                vec![
                    Valve(String::from("DD")),
                    Valve(String::from("II")),
                    Valve(String::from("BB")),
                ],
            ),
            (
                Valve(String::from("BB")),
                vec![Valve(String::from("CC")), Valve(String::from("AA"))],
            ),
            (
                Valve(String::from("CC")),
                vec![Valve(String::from("DD")), Valve(String::from("BB"))],
            ),
            (
                Valve(String::from("DD")),
                vec![
                    Valve(String::from("CC")),
                    Valve(String::from("AA")),
                    Valve(String::from("EE")),
                ],
            ),
            (
                Valve(String::from("EE")),
                vec![Valve(String::from("FF")), Valve(String::from("DD"))],
            ),
            (
                Valve(String::from("FF")),
                vec![Valve(String::from("EE")), Valve(String::from("GG"))],
            ),
            (
                Valve(String::from("GG")),
                vec![Valve(String::from("FF")), Valve(String::from("HH"))],
            ),
            (Valve(String::from("HH")), vec![Valve(String::from("GG"))]),
            (
                Valve(String::from("II")),
                vec![Valve(String::from("AA")), Valve(String::from("JJ"))],
            ),
            (Valve(String::from("JJ")), vec![Valve(String::from("II"))]),
        ])
    }

    #[test]
    fn test_parse_valves() {
        let input = include_str!("../test.txt");
        let expected_flow_rates = get_test_flow_rates();
        let expected_tunnels = get_test_tunnels();
        let (actual_flow_rates, actual_tunnels) = parse_input(&input);
        assert_eq!(expected_flow_rates, actual_flow_rates);
        assert_eq!(expected_tunnels, actual_tunnels)
    }

    #[test]
    fn test_get_most_pressure() {
        env_logger::init();
        let input_flow_rates = get_test_flow_rates();
        let input_tunnels = get_test_tunnels();
        let expected = 1651;
        let actual = get_most_pressure_possible(
            &input_flow_rates,
            &input_tunnels,
            &Valve(String::from("AA")),
            30,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_most_pressure_with_elephant() {
        let input_flow_rates = get_test_flow_rates();
        let input_tunnels = get_test_tunnels();
        let expected = 1707;
        let actual = get_most_pressure_possible_with_elephant(
            &input_flow_rates,
            &input_tunnels,
            &Valve(String::from("AA")),
            26,
        );
        assert_eq!(expected, actual);
    }
}

use std::{
    collections::{HashMap, HashSet},
    ops::Not,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending, one_of, space0},
    combinator::{complete, opt},
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple, Tuple},
    IResult,
};

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy)]
enum Signal {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ModuleType {
    Broadcaster,
    Flipper(bool),
    Conjuction(u64),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Module {
    name: String,
    mod_type: ModuleType,
    inputs: Vec<String>,
    recievers: Vec<String>,
}

impl Module {
    fn new(
        name: String,
        mod_type: ModuleType,
        inputs: Vec<String>,
        recievers: Vec<String>,
    ) -> Self {
        Module {
            name,
            mod_type,
            inputs,
            recievers,
        }
    }

    fn recieve(&mut self, signal: Signal, input_name: &str) -> bool {
        match (self.mod_type, signal) {
            (ModuleType::Flipper(state), Signal::Low) => {
                self.mod_type = ModuleType::Flipper(!state);
                true
            }
            (ModuleType::Conjuction(state), signal) => {
                let idx = self.inputs.iter().position(|n| *n == input_name).unwrap();
                let new_state = match signal {
                    Signal::High => state & !(1 << idx),
                    Signal::Low => state | (1 << idx),
                };
                self.mod_type = ModuleType::Conjuction(new_state);
                true
            }
            (ModuleType::Broadcaster, Signal::Low) => true,
            _ => false,
        }
    }

    fn signal_to_send(&self) -> Signal {
        match self.mod_type {
            ModuleType::Broadcaster => Signal::Low,
            ModuleType::Flipper(false) => Signal::Low,
            ModuleType::Flipper(true) => Signal::High,
            ModuleType::Conjuction(0) => Signal::Low,
            ModuleType::Conjuction(_) => Signal::High,
            _ => unreachable!(),
        }
    }

    fn set_inputs(&mut self, inputs: Vec<String>) {
        if let ModuleType::Conjuction(_) = self.mod_type {
            self.mod_type = ModuleType::Conjuction(u64::MAX >> (64 - inputs.len()));
        }
        self.inputs = inputs;
    }
}

#[derive(Debug, Clone)]
struct Network(HashMap<String, Module>);
impl Network {
    fn new(modules: HashMap<String, Module>) -> Self {
        Network(modules)
    }

    fn push_button(&mut self) -> (u64, u64) {
        let (mut low_count, mut high_count, mut is_reached) = (1, 0, false);

        let mut targets = vec!["broadcaster".to_string()];

        while !targets.is_empty() {
            let source_name = targets.remove(0);
            let source = self.0.get(&source_name).expect("wrong source");
            let signal = source.signal_to_send();
            let recievers = source.recievers.clone();

            match signal {
                Signal::Low => low_count += recievers.len() as u64,
                Signal::High => high_count += recievers.len() as u64,
            }

            for reciever in recievers {
                let target = self.0.get_mut(&reciever).expect("wrong target");
                // println!("{}: {:?} -> {:?}", source_name, signal, target);

                if target.recieve(signal, &source_name) {
                    targets.push(reciever);
                }
            }
            // println!("{:?}", targets);
        }

        (low_count, high_count)
    }

    fn push_button_with_search(&mut self, nodes: &[String]) -> Vec<String> {
        let mut nodes_to_find: HashSet<String> = HashSet::from_iter(nodes.iter().cloned());
        let mut targets = vec!["broadcaster".to_string()];

        while !targets.is_empty() {
            let source_name = targets.remove(0);
            let source = self.0.get(&source_name).expect("wrong source");
            let signal = source.signal_to_send();
            let recievers = source.recievers.clone();

            for reciever in recievers {
                let target = self.0.get_mut(&reciever).expect("wrong target");
                // println!("{}: {:?} -> {:?}", source_name, signal, target);

                if target.recieve(signal, &source_name) {
                    targets.push(reciever);
                }

                if matches!(signal, Signal::Low) && nodes.contains(&target.name) {
                    nodes_to_find.remove(&target.name);
                }
            }
            // println!("{:?}", targets);
        }

        nodes_to_find.into_iter().collect()
    }

    fn state(&self) -> Vec<Module> {
        self.0
            .values()
            .cloned()
            .sorted_by_key(|m| m.name.to_string())
            .collect()
    }
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    let (input, (mod_type, name, recievers)) = tuple((
        opt(one_of("&%")),
        alpha1,
        preceded(
            tuple((space0, tag("->"), space0)),
            separated_list1(tuple((tag(","), space0)), alpha1),
        ),
    ))(input)?;

    let mod_type = match mod_type {
        Some('&') => ModuleType::Conjuction(0),
        Some('%') => ModuleType::Flipper(false),
        None if name == "broadcaster" => ModuleType::Broadcaster,
        _ => panic!("invalid module type"),
    };
    let name = name.to_string();

    let recievers = recievers.iter().map(|r| r.to_string()).collect();

    let inputs = vec![];

    Ok((input, Module::new(name, mod_type, inputs, recievers)))
}

fn parse(input: &str) -> IResult<&str, Network> {
    let (input, mut modules) = separated_list1(line_ending, parse_module)(input)?;

    let mut modules_set: HashSet<String> = modules.iter().map(|el| el.name.clone()).collect();
    let mut inputs_map =
        modules
            .clone()
            .iter()
            .fold(HashMap::<String, Vec<String>>::new(), |mut acc, el| {
                el.recievers.iter().for_each(|r| {
                    // no recievers for current module
                    if !modules_set.contains(r) {
                        modules.push(Module::new(r.clone(), ModuleType::None, vec![], vec![]));
                        modules_set.insert(r.clone());
                    }
                    acc.entry(r.clone()).or_default().push(el.name.clone());
                });
                acc
            });
    let modules = modules
        .iter_mut()
        .fold(HashMap::<String, Module>::new(), |mut acc, el| {
            el.set_inputs(inputs_map.remove(&el.name).unwrap_or_default());
            acc.insert(el.name.clone(), el.clone());
            acc
        });
    Ok((input, Network::new(modules)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, mut modules) = parse(input).unwrap();

    let (mut low_count, mut high_count) = (0, 0);
    for _ in 0..1000 {
        let (low, high) = modules.push_button();
        low_count += low;
        high_count += high;
    }

    Some(low_count * high_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut modules) = parse(input).unwrap();

    let fin = "rx".to_string();

    let node = modules
        .0
        .iter()
        .find_map(|(_, v)| v.recievers.contains(&fin).then_some(v.name.clone()))
        .expect("no node found");

    let mut nodes: Vec<String> = modules
        .0
        .iter()
        .filter_map(|(_, v)| v.recievers.contains(&node).then_some(v.name.clone()))
        .collect();

    println!("{:?}", nodes);

    let mut indexes = vec![];
    for i in 0.. {
        let len_before = nodes.len();
        nodes = modules.push_button_with_search(&nodes);
        if len_before != nodes.len() {
            indexes.push(i + 1);
        }
        if nodes.is_empty() {
            break;
        };
    }
    println!("{:?}", indexes);

    Some(lcm(&indexes))
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

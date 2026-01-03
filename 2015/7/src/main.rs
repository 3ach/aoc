use adventage::{day, part1demo};
use std::collections::HashMap;

day!(2015, 7);
part1demo!(
    "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
    114
);

#[derive(Debug, Clone)]
enum Signal {
    Value(u32),
    Pass(String),
    And(String, String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
    Not(String),
}

type TInput = HashMap<String, Signal>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|line| {
            let (signal, name) = line.split_once(" -> ").unwrap();
            if let Ok(value) = signal.parse() {
                (name.to_string(), Signal::Value(value))
            } else {
                let parts = signal.split(" ").collect::<Vec<&str>>();
                match parts.len() {
                    1 => (name.to_string(), Signal::Pass(parts[0].to_string())),
                    2 => (name.to_string(), Signal::Not(parts[1].to_string())),
                    3 if parts[1] == "AND" => (
                        name.to_string(),
                        Signal::And(parts[0].to_string(), parts[2].to_string()),
                    ),
                    3 if parts[1] == "OR" => (
                        name.to_string(),
                        Signal::Or(parts[0].to_string(), parts[2].to_string()),
                    ),
                    3 if parts[1] == "LSHIFT" => (
                        name.to_string(),
                        Signal::LShift(parts[0].to_string(), parts[2].parse().unwrap()),
                    ),
                    3 if parts[1] == "RSHIFT" => (
                        name.to_string(),
                        Signal::RShift(parts[0].to_string(), parts[2].parse().unwrap()),
                    ),
                    _ => panic!(),
                }
            }
        })
        .collect()
}

fn visit(node: &str, circuit: &TInput, topo: &mut Vec<String>) {
    if topo.contains(&node.to_string()) {
        return;
    }

    let next = if let Some(next) = circuit.get(node) {
        match next {
            Signal::Value(_) => vec![],
            Signal::And(a, b) => vec![a.clone(), b.clone()],
            Signal::Or(a, b) => vec![a.clone(), b.clone()],
            Signal::RShift(node, _) => vec![node.clone()],
            Signal::LShift(node, _) => vec![node.clone()],
            Signal::Not(node) => vec![node.clone()],
            Signal::Pass(node) => vec![node.clone()],
        }
    } else {
        return;
    };

    for next in next {
        visit(&next, circuit, topo);
    }

    topo.push(node.to_string());
}

fn topo(from: &str, circuit: &TInput) -> Vec<String> {
    let mut sorted = vec![];
    visit(from, circuit, &mut sorted);
    sorted
}

fn simulate(goal: &str, circuit: &TInput) -> u32 {
    let mut resolved = HashMap::new();
    let to_resolve = topo(goal, circuit);

    for signal in to_resolve {
        let read = |name: &str| {
            if let Some(sig) = resolved.get(name) {
                *sig
            } else {
                name.parse::<u32>().unwrap()
            }
        };

        let value: u32 = match circuit.get(&signal).unwrap() {
            Signal::Value(v) => *v,
            Signal::And(a, b) => 0xffff & (read(a) & read(b)),
            Signal::Or(a, b) => 0xffff & (read(a) | read(b)),
            Signal::RShift(s, c) => 0xffff & (read(s) >> *c),
            Signal::LShift(s, c) => 0xffff & (read(s) << *c),
            Signal::Not(s) => 0xffff & !read(s),
            Signal::Pass(s) => 0xffff & read(s),
        };

        resolved.insert(signal.clone(), value);
    }

    *resolved.get(goal).unwrap()
}

fn part1(circuit: &TInput) -> u32 {
    let goal = if circuit.contains_key("a") {
        "a"
    } else {
        "g"
    };

    simulate(goal, circuit)
}

fn part2(circuit: &TInput) -> u32 {
    let a = simulate("a", circuit);

    let mut modified = circuit.clone();
    modified.insert(String::from("b"), Signal::Value(a));

    simulate("a", &modified)
}

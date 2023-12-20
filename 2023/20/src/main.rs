use adventage::{day, part1demo};
use std::collections::HashMap;
use std::collections::VecDeque;

part1demo!(
    "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
    32000000
);
part1demo!(
    "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
    11687500
);

day!(2023, 20);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

#[derive(Clone, Debug)]
enum Module {
    Button,
    Output,
    Broadcaster(Vec<String>),
    FlipFlop(Signal, Vec<String>),
    Conjunction(HashMap<String, Signal>, Vec<String>),
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    let mut conjunctions = vec![];

    modules.insert("button".to_string(), Module::Button);
    modules.insert("output".to_string(), Module::Output);

    for line in input.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs
            .split(", ")
            .map(String::from)
            .collect::<Vec<String>>();
        if name == "broadcaster" {
            modules.insert(name.to_string(), Module::Broadcaster(outputs));
        } else if name.starts_with('%') {
            let name = name.trim_start_matches('%');
            modules.insert(name.to_string(), Module::FlipFlop(Signal::Low, outputs));
        } else if name.starts_with('&') {
            let name = name.trim_start_matches('&');
            conjunctions.push(name.to_string());
            modules.insert(
                name.to_string(),
                Module::Conjunction(HashMap::new(), outputs),
            );
        }
    }

    for conjunction in conjunctions {
        for (name, module) in modules.clone() {
            match module {
                Module::Broadcaster(outputs) | Module::FlipFlop(_, outputs) | Module::Conjunction(_, outputs) => {
                    if !outputs.contains(&conjunction) {
                        continue;
                    }

                    if let Some(Module::Conjunction(ref mut states, _)) = modules.get_mut(&conjunction) {
                        states.insert(name.to_string(), Signal::Low);
                    }
                }
                _ => {},
            }
        }
    }

    modules
}

fn press(state: &mut HashMap<String, Module>, observed: &[String]) -> (usize, usize, Vec<String>) {
    let mut unprocessed = VecDeque::from([("button".to_string(), "me".to_string(), Signal::High)]);
    let mut toggled = vec![];
    let mut low = 0;
    let mut high = 0;

    while let Some((name, sender, signal)) = unprocessed.pop_back() {
        if let Some(module) = state.get_mut(&name) {
            match (module, signal) {
                (Module::Button, _) => {
                    low += 1;
                    unprocessed.push_front((
                        "broadcaster".to_string(),
                        name.to_string(),
                        Signal::Low,
                    ));
                }
                (Module::Broadcaster(outputs), Signal::Low) => {
                    low += outputs
                        .iter()
                        .inspect(|out| {
                            unprocessed.push_front((out.to_string(), name.to_string(), Signal::Low))
                        })
                        .count();
                }
                (Module::Broadcaster(outputs), Signal::High) => {
                    high += outputs
                        .iter()
                        .inspect(|out| {
                            unprocessed.push_front((
                                out.to_string(),
                                name.to_string(),
                                Signal::High,
                            ))
                        })
                        .count();
                }
                (Module::FlipFlop(_, _), Signal::High) => {}
                (Module::FlipFlop(ref mut signal, outputs), Signal::Low) => {
                    *signal = match signal {
                        Signal::Low => {
                            high += outputs
                                .iter()
                                .inspect(|out| {
                                    unprocessed.push_front((
                                        out.to_string(),
                                        name.to_string(),
                                        Signal::High,
                                    ))
                                })
                                .count();
                            Signal::High
                        }
                        Signal::High => {
                            low += outputs
                                .iter()
                                .inspect(|out| {
                                    unprocessed.push_front((
                                        out.to_string(),
                                        name.to_string(),
                                        Signal::Low,
                                    ))
                                })
                                .count();
                            Signal::Low
                        }
                    };
                }
                (Module::Conjunction(ref mut state, outputs), Signal::High) => {
                    if observed.contains(&sender) {
                        toggled.push(sender.clone());
                    }
                    state.insert(sender, Signal::High);

                    if state.values().all(|k| *k == Signal::High) {
                        low += outputs
                            .iter()
                            .inspect(|out| {
                                unprocessed.push_front((
                                    out.to_string(),
                                    name.to_string(),
                                    Signal::Low,
                                ))
                            })
                            .count();
                    } else {
                        high += outputs
                            .iter()
                            .inspect(|out| {
                                unprocessed.push_front((
                                    out.to_string(),
                                    name.to_string(),
                                    Signal::High,
                                ))
                            })
                            .count();
                    }
                }
                (Module::Conjunction(ref mut state, outputs), Signal::Low) => {
                    state.insert(sender, Signal::Low);
                    high += outputs
                        .iter()
                        .inspect(|out| {
                            unprocessed.push_front((
                                out.to_string(),
                                name.to_string(),
                                Signal::High,
                            ))
                        })
                        .count();
                }
                (Module::Output, _) => {}
            }
        }
    }

    (low, high, toggled)
}

fn part1(state: &HashMap<String, Module>) -> usize {
    let mut low = 0;
    let mut high = 0;
    let mut state = state.clone();

    for _ in 0..1000 {
        let (d_low, d_high, _) = press(&mut state, &vec![]);
        low += d_low;
        high += d_high;
    }

    low * high
}

fn part2(state: &HashMap<String, Module>) -> u64 {
    let mut loops = 1;
    let mut state = state.clone();
    let direct_inputs = state.iter()
        .filter(|(_, module)| match module {
            Module::Conjunction(_, outputs) | Module::Broadcaster(outputs) | Module::FlipFlop(_, outputs) => outputs.contains(&"rx".to_string()),
            _ => false
        })
        .map(|(name, _)| name.clone())
        .collect::<Vec<String>>();

    let direct_input = &direct_inputs[0];
    let to_observe: Vec<String> = if let Module::Conjunction(inputs, _) = state.get(direct_input).unwrap() {
        inputs.keys().map(String::from).collect()
    } else {
        panic!();
    };

    let mut cycles = HashMap::new();

    loop {
        let (_, _, toggled) = press(&mut state, &to_observe);
        loops += 1;

        for signal in toggled {
            if !cycles.contains_key(&signal) {
                cycles.insert(signal.clone(), loops - 1);
            }
        }
        if cycles.len() == to_observe.len() {
            return cycles.values().product();
        }
    }
}

use adventage::{day, part1demo, part2demo};
use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

enum Test {
    Greater(String, u32),
    Lesser(String, u32),
}

struct Rule {
    test: Option<Test>,
    destination: String,
}

part1demo!(
    "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
    19114
);

part2demo!(
    "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
    167409079868000
);

day!(2023, 19);

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let (workflow_input, part_input) = input.split_once("\n\n").unwrap();

    let workflows = workflow_input
        .lines()
        .map(|line| {
            line.split(&['{', '}'])
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .map(|comps| {
            (
                comps[0].clone(),
                comps[1]
                    .split(',')
                    .map(|rule| {
                        if let Some((test, dest)) = rule.split_once(":") {
                            let op_idx = test.find(&['>', '<']).unwrap();
                            let (arg, param) = test.split_at(op_idx + 1);
                            let op = &arg[1..];
                            let arg = arg[0..1].to_string();
                            let param = param.parse::<u32>().unwrap();

                            Rule {
                                test: match op {
                                    ">" => Some(Test::Greater(arg, param)),
                                    "<" => Some(Test::Lesser(arg, param)),
                                    _ => panic!(),
                                },
                                destination: dest.to_string(),
                            }
                        } else {
                            Rule {
                                test: None,
                                destination: rule.to_string(),
                            }
                        }
                    })
                    .collect(),
            )
        })
        .collect();

    let parts = part_input
        .lines()
        .map(|line| {
            line.trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|p| p.split_once('=').unwrap())
                .map(|(name, value)| (name, value.parse::<u32>().unwrap()))
                .fold(
                    Part {
                        x: 0,
                        m: 0,
                        a: 0,
                        s: 0,
                    },
                    |mut part, x| {
                        match x.0 {
                            "x" => part.x = x.1,
                            "m" => part.m = x.1,
                            "a" => part.a = x.1,
                            "s" => part.s = x.1,
                            _ => panic!(),
                        };
                        part
                    },
                )
        })
        .collect();

    (workflows, parts)
}

fn grab(part: &Part, field: &str) -> u32 {
    match field {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => panic!(),
    }
}

fn step(part: &Part, rules: &Vec<Rule>) -> String {
    for rule in rules {
        if let Some(test) = &rule.test {
            let matches = match test {
                Test::Greater(field, value) => grab(part, field) > *value,
                Test::Lesser(field, value) => grab(part, field) < *value,
            };

            if matches {
                return rule.destination.clone();
            }
        } else {
            return rule.destination.clone();
        }
    }

    panic!();
}

fn run(part: &Part, rules: &HashMap<String, Vec<Rule>>) -> String {
    let mut current = String::from("in");

    while &current != "A" && &current != "R" {
        let rule = rules.get(&current).unwrap();
        current = step(part, rule);
    }

    current
}

fn part1(input: &(HashMap<String, Vec<Rule>>, Vec<Part>)) -> u32 {
    let (workflow, parts) = input;

    parts
        .iter()
        .filter_map(|part| {
            if &run(part, workflow) == "A" {
                Some(part.x + part.m + part.a + part.s)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &(HashMap<String, Vec<Rule>>, Vec<Part>)) -> usize {
    let (workflow, _) = input;
    let part = (1..=4000, 1..=4000, 1..=4000, 1..=4000);
    let mut states = vec![("in".to_string(), part)];
    let mut accepted = vec![];

    while let Some(state) = states.pop() {
        let rule = state.0;
        let mut parts = state.1;

        if &rule == "A" {
            accepted.push(parts);
            continue;
        } else if &rule == "R" {
            continue;
        }

        let rules = workflow.get(&rule).unwrap();
        for step in rules {
            if let Some(test) = &step.test {
                match test {
                    Test::Greater(field, value) => match field.as_str() {
                        "x" => {
                            if parts.0.start() > value {
                                states.push((step.destination.to_string(), parts.clone()));
                                break;
                            } else if parts.0.end() > value {
                                let mut matching = parts.clone();
                                matching.0 = value + 1..=*parts.0.end();
                                states.push((step.destination.to_string(), matching));

                                parts.0 = *parts.0.start()..=*value;
                            }
                        }
                        "m" => {
                            if parts.1.start() > value {
                                states.push((step.destination.to_string(), parts.clone()));
                                break;
                            } else if parts.1.end() > value {
                                let mut matching = parts.clone();
                                matching.1 = value + 1..=*parts.1.end();
                                states.push((step.destination.to_string(), matching));

                                parts.1 = *parts.1.start()..=*value;
                            }
                        }
                        "a" => {
                            if parts.2.start() > value {
                                states.push((step.destination.to_string(), parts.clone()));
                                break;
                            } else if parts.2.end() > value {
                                let mut matching = parts.clone();
                                matching.2 = value + 1..=*parts.2.end();
                                states.push((step.destination.to_string(), matching));

                                parts.2 = *parts.2.start()..=*value;
                            }
                        }
                        "s" => {
                            if parts.3.start() > value {
                                states.push((step.destination.to_string(), parts.clone()));
                                break;
                            } else if parts.3.end() > value {
                                let mut matching = parts.clone();
                                matching.3 = value + 1..=*parts.3.end();
                                states.push((step.destination.to_string(), matching));

                                parts.3 = *parts.3.start()..=*value;
                            }
                        }
                        _ => panic!(),
                    },
                    Test::Lesser(field, value) => match field.as_str() {
                        "x" => {
                            if parts.0.end() < value {
                                states.push((step.destination.to_string(), parts.clone()));
                                break;
                            } else if parts.0.start() < value {
                                let mut matching = parts.clone();
                                matching.0 = *parts.0.start()..=value - 1;
                                states.push((step.destination.to_string(), matching));

                                parts.0 = *value..=*parts.0.end();
                            }
                        }
                        "m" => {
                            if parts.1.end() < value {
                                states.push((step.destination.to_string(), parts.clone()));
                                break;
                            } else if parts.1.start() < value {
                                let mut matching = parts.clone();
                                matching.1 = *parts.1.start()..=value - 1;
                                states.push((step.destination.to_string(), matching));

                                parts.1 = *value..=*parts.1.end();
                            }
                        }
                        "a" => {
                            if parts.2.end() < value {
                                states.push((step.destination.to_string(), parts.clone()));
                                break;
                            } else if parts.2.start() < value {
                                let mut matching = parts.clone();
                                matching.2 = *parts.2.start()..=value - 1;
                                states.push((step.destination.to_string(), matching));

                                parts.2 = *value..=*parts.2.end();
                            }
                        }
                        "s" => {
                            if parts.3.end() < value {
                                states.push((step.destination.to_string(), parts.clone()));
                                break;
                            } else if parts.3.start() < value {
                                let mut matching = parts.clone();
                                matching.3 = *parts.3.start()..=value - 1;
                                states.push((step.destination.to_string(), matching));

                                parts.3 = *value..=*parts.3.end();
                            }
                        }
                        _ => panic!(),
                    },
                }
            } else {
                states.push((step.destination.clone(), parts));
                break;
            }
        }
    }

    accepted
        .iter()
        .map(|p| {
            p.0.clone().count() * p.1.clone().count() * p.2.clone().count() * p.3.clone().count()
        })
        .sum()
}

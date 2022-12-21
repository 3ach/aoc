use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbors: Vec<(String, u32)>,
}

#[derive(Debug, Clone)]
struct Step<'a> {
    current: &'a str,
    released: u32,
    minute: u32,
    opened: HashSet<String>,
    path: String,
}

type TInput = HashMap<String, Valve>;

fn relax(valves: &mut TInput) {
    let zeros = valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate == 0)
        .map(|(name, _)| name)
        .cloned()
        .collect::<Vec<String>>();

    for zero in zeros {
        if zero == "AA" {
            continue;
        }
        let valve = valves.remove(&zero).unwrap();
        for (neighbor_valve, distance) in &valve.neighbors {
            if *neighbor_valve == zero {
                continue;
            }
            let neighbor = valves.get_mut(neighbor_valve).unwrap();
            neighbor.neighbors = neighbor
                .neighbors
                .iter()
                .filter(|(n, _)| *n != zero)
                .cloned()
                .collect();

            neighbor.neighbors.append(
                &mut valve
                    .neighbors
                    .iter()
                    .filter(|(n, _)| *n != zero)
                    .map(|(n, d)| (n.clone(), *d + distance))
                    .collect(),
            );
        }
    }
}

fn shortests(valves: &mut TInput) {
    let mut shortests = HashMap::new();
    for (name, valve) in &*valves {
        for (neighbor, distance) in &valve.neighbors {
            shortests.insert((name.clone(), neighbor.clone()), *distance);
            shortests.insert((neighbor.clone(), name.clone()), *distance);
        }
    }

    for (first, _) in &*valves {
        for (second, _) in &*valves {
            for (third, _) in &*valves {
                if let Some(second_third) = shortests.get(&(second.clone(), third.clone())) {
                    if let (Some(first_second), Some(first_third)) = (
                        shortests.get(&(first.clone(), second.clone())),
                        shortests.get(&(first.clone(), third.clone())),
                    ) {
                        if *second_third > first_second + first_third {
                            shortests.insert(
                                (second.clone(), third.clone()),
                                first_second + first_third,
                            );
                        }
                    }
                } else {
                    if let (Some(first_second), Some(first_third)) = (
                        shortests.get(&(first.clone(), second.clone())),
                        shortests.get(&(first.clone(), third.clone())),
                    ) {
                        shortests
                            .insert((second.clone(), third.clone()), first_second + first_third);
                    }
                }
            }
        }
    }

    for (name, mut valve) in valves {
        valve.neighbors = shortests
            .iter()
            .filter(|((from, _), _)| from == name)
            .filter(|((from, to), _)| from != to)
            .map(|((_, to), distance)| (to.clone(), *distance))
            .collect();
    }
}

fn next<'a>(mut step: Step<'a>, valves: &'a TInput, time: u32) -> Vec<Step<'a>> {
    let mut next = vec![];
    let valve = &valves[step.current];
    if step.current != "AA" && !step.opened.insert(step.current.to_string()) {
        return vec![];
    }

    if valve.flow_rate > 0 {
        step.released += (time - step.minute) * valve.flow_rate;
        step.minute += 1;
        step.path.push('F');
        next.push(Step {
            current: step.current.clone(),
            released: step.released,
            minute: step.minute,
            opened: step.opened.clone(),
            path: step.path.clone(),
        });
    }

    for (neighbor, distance) in &valve.neighbors {
        if !step.opened.contains(neighbor) && step.minute + distance < time {
            let neighbor_valve = &valves[neighbor];
            let mut path = step.path.clone();
            path.push_str(&format!("{}{}", neighbor, step.minute));

            next.push(Step {
                current: neighbor,
                released: step.released,
                minute: step.minute + distance,
                opened: step.opened.clone(),
                path: path.clone(),
            });
        }
    }

    next
}

fn part1(input: &TInput) -> u32 {
    let mut stack = vec![];
    let mut max = 0;

    stack.push(Step {
        current: "AA",
        released: 0,
        minute: 1,
        opened: HashSet::new(),
        path: "".to_string(),
    });
    while let Some(step) = stack.pop() {
        if step.released >= max {
            max = step.released;
        }

        for next in next(step, input, 30) {
            stack.push(next);
        }
    }

    max
}

fn part2(input: &TInput) -> u32 {
    let mut stack = vec![];
    let mut max = 0;
    let mut c = 0;
    let mut s = 0;
    let mut explored = HashSet::new();

    stack.push((
        Step {
            current: "AA",
            released: 0,
            minute: 1,
            opened: HashSet::from(["AA".to_string()]),
            path: "H".to_string(),
        },
        Step {
            current: "AA",
            released: 0,
            minute: 1,
            opened: HashSet::from(["AA".to_string()]),
            path: "E".to_string(),
        },
    ));

    while let Some((human, elephant)) = stack.pop() {
        c += 1;

        if c % 1000000 == 0 {
            println!("Checked {} so far, and skipped {}.", c, s);
        }
        let released = elephant.released + human.released;
        if released >= max {
            max = elephant.released + human.released;
        }
        let unopened: HashSet<_> = input
            .keys()
            .filter(|k| !human.opened.contains(k.as_str()) & !elephant.opened.contains(k.as_str()))
            .collect();

        let potential: u32 = unopened
            .iter()
            .map(|v| {
                input[v.as_str()].flow_rate * (26 - cmp::min(human.minute, elephant.minute) - 1)
            })
            .sum();

        if potential + released < max {
            s += 1;
            continue;
        }

        let wholepath = format!("{}{}", human.path, elephant.path);
        if !explored.insert(wholepath) {
            s += 1;
            continue;
        }

        for next in next(human.clone(), input, 26) {
            let mut elephant = elephant.clone();
            elephant.opened = next.opened.clone();
            stack.push((next, elephant));
        }

        for next in next(elephant.clone(), input, 26) {
            let mut human = human.clone();
            human.opened = next.opened.clone();
            stack.push((human, next));
        }
    }
    println!("Checked {} possible solutions, skipped {}", c, s);

    max
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut input: TInput = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let (valve_part, neighbor_part) = line.split_once("; ").unwrap();
            let valve_part = valve_part
                .replace("=", " ")
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let neighbor_part = neighbor_part
                .split_once("valve")
                .unwrap()
                .1
                .trim_start_matches('s')
                .trim();
            (
                valve_part[1].clone(),
                valve_part[5].parse::<u32>().unwrap(),
                neighbor_part
                    .split(", ")
                    .map(|s| (s.to_string(), 1))
                    .collect::<Vec<(String, u32)>>(),
            )
        })
        .map(|(name, flow_rate, neighbors)| {
            (
                name,
                Valve {
                    flow_rate,
                    neighbors,
                },
            )
        })
        .collect();

    relax(&mut input);
    shortests(&mut input);

    let answer1 = part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = part2(&input);
    println!("Answer 2: {}", answer2);

    Ok(())
}

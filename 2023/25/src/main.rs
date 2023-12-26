use adventage::{part1demo, part2demo, day};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

day!(2023, 25);

part1demo!(
"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr", 54);

type TInput = HashMap<String, HashSet<String>>;

fn parse(input: &str) -> TInput {
    let mut initial: TInput = input
        .lines()
        .map(|line| {
            let (name, connections) = line.split_once(": ").unwrap();
            let name = name.to_string();
            let connections = connections.split(" ").map(String::from).collect::<HashSet<String>>();
            
            (name, connections)
        })
        .collect();

    for (name, connections) in initial.clone() {
        for connection in connections {
            let entry = initial.entry(connection).or_insert(HashSet::new());
            entry.insert(name.clone());
        }
    }

    initial
}

fn bfs(start: String, end: Option<String>, graph: &TInput) -> Vec<String> {
    let mut to_explore = VecDeque::from([start.clone()]);
    let mut prevs = HashMap::new();
    let mut last = start.clone();

    while let Some(node) = to_explore.pop_front() {
        if let Some(end) = &end {
            if *end == node {
                break;
            }
        } 

        for neighbor in graph.get(&node).unwrap() {
            if prevs.contains_key(neighbor) || *neighbor == start {
                continue;
            }

            prevs.insert(neighbor.clone(), node.clone());
            to_explore.push_back(neighbor.clone());
        }

        last = node.clone();
    }

    let mut current = end.unwrap_or(last);
    let mut path = vec![current.clone()];
    while let Some(prev) = prevs.get(&current) {
        path.push(prev.clone());
        current = prev.clone();
    }

    path.reverse();
    path
}

fn part1(connections: &TInput) -> usize {
    let mut graph: TInput = connections.clone();
    let start = graph.keys().cloned().next().unwrap();
    let mut end = None;

    // Snip out 3 whole paths
    for _ in 0..3 {
        let path = bfs(start.clone(), end.clone(), &graph);
        end = Some(path.last().unwrap().clone());
        for window in path.windows(2) {
            let from = &window[0];
            let to = &window[1];

            graph.get_mut(from).unwrap().remove(to);
            graph.get_mut(to).unwrap().remove(from);
        }
    }

    println!("Did my cuts");

    let mut assigned = HashSet::new();
    let mut sets = vec![];

    for node in graph.keys() {
        if assigned.contains(node) {
            continue;
        }

        let mut community = HashSet::new();
        let mut to_explore = vec![node.clone()];
        while let Some(node) = to_explore.pop() {
            if community.contains(&node) {
                continue;
            }

            community.insert(node.clone());
            assigned.insert(node.clone());
            for neighbor in graph.get(&node).unwrap() {
                to_explore.push(neighbor.clone());
            }
        }

        sets.push(community);
    }

    sets.iter().map(|s| s.len()).product()
}

fn part2(_map: &TInput) -> u32 {
    0
}

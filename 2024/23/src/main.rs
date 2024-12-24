use adventage::{day, part1demo, part2demo};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

day!(2024, 23);
part1demo!(
    "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
    7
);
part2demo!(
    "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
    "co,de,ka,ta"
);

type TInput = Vec<(String, String)>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|l| {
            let (first, second) = l.split_once("-").unwrap();
            (
                String::from(min(first, second)),
                String::from(max(first, second)),
            )
        })
        .collect()
}

fn part1(pairs: &TInput) -> usize {
    let mut adjacent = HashMap::new();
    let mut triplets = HashSet::new();
    for (first, second) in pairs {
        adjacent
            .entry(first.clone())
            .or_insert(HashSet::new())
            .insert(second.clone());
        adjacent
            .entry(second.clone())
            .or_insert(HashSet::new())
            .insert(first.clone());
    }

    for (node, neighbors) in &adjacent {
        if !node.starts_with("t") {
            continue;
        }

        for first in neighbors {
            for second in neighbors {
                if first == second {
                    continue;
                }

                if adjacent.get(first).unwrap().contains(second) {
                    let mut triplet = vec![first.clone(), second.clone(), node.clone()];
                    triplet.sort();

                    triplets.insert(triplet);
                }
            }
        }
    }

    triplets.len()
}

fn part2(pairs: &TInput) -> String {
    let mut adjacent = HashMap::new();
    for (first, second) in pairs {
        adjacent
            .entry(first.clone())
            .or_insert(HashSet::new())
            .insert(second.clone());
        adjacent
            .entry(second.clone())
            .or_insert(HashSet::new())
            .insert(first.clone());
    }

    let nodes: HashSet<String> = adjacent.keys().cloned().collect();
    let mut cliques = vec![];
    let mut to_explore = vec![(HashSet::new(), nodes, HashSet::new())];
    while let Some((must_contain, mut may_contain, mut cannot_contain)) = to_explore.pop() {
        if may_contain.len() == 0 && cannot_contain.len() == 0 {
            cliques.push(must_contain);
            continue;
        }

        for node in may_contain.clone() {
            let neighbors = adjacent.get(&node).unwrap();
            let next_may_contain = may_contain.intersection(&neighbors).cloned().collect();
            let next_cannot_contain = cannot_contain.intersection(&neighbors).cloned().collect();
            let mut next_must_contain = must_contain.clone();

            next_must_contain.insert(node.clone());
            to_explore.push((next_must_contain, next_may_contain, next_cannot_contain));

            may_contain.remove(&node);
            cannot_contain.insert(node.clone());
        }
    }

    let mut biggest: Vec<String> = cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .into_iter()
        .cloned()
        .collect();

    biggest.sort();

    biggest.join(&String::from(","))
}

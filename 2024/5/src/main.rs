use adventage::{day, part1demo, part2demo};
use std::collections::{HashMap, HashSet};

day!(2024, 5);
part1demo!(
    "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    143
);
part2demo!(
    "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    123
);

type TInput = (Vec<(u32, u32)>, Vec<Vec<u32>>);

fn parse(input: &str) -> TInput {
    let mut rules = vec![];
    let mut updates = vec![];

    for line in input.lines() {
        if line.contains("|") {
            let parts: Vec<u32> = line.split("|").map(|n| n.parse().unwrap()).collect();
            rules.push((parts[0], parts[1]));
        } else if line.contains(",") {
            let update = line.split(",").map(|n| n.parse().unwrap()).collect();
            updates.push(update);
        }
    }

    (rules, updates)
}

fn update_is_valid(rules: &[(u32, u32)], update: &[u32]) -> bool {
    let page_to_idx: HashMap<u32, usize> = update
        .iter()
        .enumerate()
        .map(|(idx, page)| (*page, idx))
        .collect();

    rules
        .iter()
        .filter(|r| update.contains(&r.0) && update.contains(&r.1))
        .all(|(first, second)| page_to_idx[first] < page_to_idx[second])
}

fn correct(rules: &[(u32, u32)], update: &[u32]) -> Vec<u32> {
    let mut corrected = vec![];

    while corrected.len() != update.len() {
        let applicable_rules = rules
            .iter()
            .filter(|r| update.contains(&r.0) && update.contains(&r.1) && !corrected.contains(&r.0))
            .cloned()
            .collect::<Vec<(u32, u32)>>();

        let mut eligible_to_place: HashSet<u32> = update
            .iter()
            .filter(|v| !corrected.contains(v))
            .cloned()
            .collect();
        for rule in applicable_rules {
            eligible_to_place.remove(&rule.1);
        }

        corrected.push(*eligible_to_place.iter().next().unwrap());
    }

    corrected
}

fn part1((rules, updates): &TInput) -> u32 {
    updates
        .iter()
        .filter(|u| update_is_valid(rules, *u))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn part2((rules, updates): &TInput) -> u32 {
    updates
        .iter()
        .filter(|u| !update_is_valid(rules, *u))
        .map(|u| correct(rules, u))
        .map(|u| u[u.len() / 2])
        .sum()
}

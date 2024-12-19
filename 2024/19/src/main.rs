use adventage::{day, part1demo, part2demo};
use std::collections::HashMap;

day!(2024, 19);
part1demo!(
    "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    6
);
part2demo!(
    "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    6
);

type TInput = (Vec<String>, Vec<String>);

fn parse(input: &str) -> TInput {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(String::from).collect();
    let patterns = patterns.lines().map(String::from).collect();

    (towels, patterns)
}

fn part1((towels, patterns): &TInput) -> usize {
    let mut cache = HashMap::new();
    patterns
        .iter()
        .filter(|pattern| ways(pattern, towels, &mut cache) > 0)
        .count()
}

fn ways(goal: &str, patterns: &Vec<String>, mut seen: &mut HashMap<String, usize>) -> usize {
    if let Some(prev) = seen.get(goal) {
        return *prev;
    }

    let from_here = patterns
        .iter()
        .filter(|pattern| goal.starts_with(*pattern))
        .map(|pattern| {
            let (_, rest) = goal.split_at(pattern.len());
            if rest.len() == 0 {
                1
            } else {
                ways(rest, patterns, &mut seen)
            }
        })
        .sum();

    seen.insert(goal.to_string(), from_here);

    from_here
}

fn part2((towels, patterns): &TInput) -> usize {
    let mut cache = HashMap::new();
    patterns
        .iter()
        .map(|pattern| ways(pattern, towels, &mut cache))
        .sum()
}

use adventage::{day, part1demo};
use std::collections::HashMap;

day!(2024, 11);
part1demo!("125 17", 55312);
type TInput = HashMap<u64, usize>;

fn parse(input: &str) -> TInput {
    input
        .split_whitespace()
        .map(|n| (n.parse().unwrap(), 1))
        .collect()
}

fn apply(stones: &TInput) -> TInput {
    let mut split = HashMap::new();
    for (stone, count) in stones {
        if *stone == 0 {
            *split.entry(1).or_insert(0) += count;
        } else if stone.ilog10() % 2 == 1 {
            let digits = stone.ilog10() + 1;
            let current = stone / (10_u64.pow(digits / 2));
            let next = stone % (10_u64.pow(digits / 2));

            *split.entry(current).or_insert(0) += count;
            *split.entry(next).or_insert(0) += count;
        } else {
            *split.entry(stone * 2024).or_insert(0) += count;
        }
    }
    split
}

fn part1(stones: &TInput) -> usize {
    let mut stones = stones.clone();
    for _ in 0..25 {
        stones = apply(&stones);
    }
    stones.iter().map(|(_, count)| count).sum()
}

fn part2(stones: &TInput) -> usize {
    let mut stones = stones.clone();
    for _ in 0..75 {
        stones = apply(&stones);
    }
    stones.iter().map(|(_, count)| count).sum()
}

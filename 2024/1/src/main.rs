use adventage::{part1demo, part2demo, day};
use std::iter::zip;

day!(2024, 1);
part1demo!("3   4
4   3
2   5
1   3
3   9
3   3", 11);

part2demo!("3   4
4   3
2   5
1   3
3   9
3   3", 31);

type TInput = Vec<Vec<u32>>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect())
        .collect()
}

fn part1(lists: &TInput) -> u32 {
    let mut left: Vec<u32> = lists.iter().map(|l| l[0]).collect();
    let mut right: Vec<u32> = lists.iter().map(|l| l[1]).collect();

    left.sort();
    right.sort();

    zip(left, right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part2(lists: &TInput) -> u32 {
    let left: Vec<u32> = lists.iter().map(|l| l[0]).collect();
    let right: Vec<u32> = lists.iter().map(|l| l[1]).collect();

    left.iter().map(|l| (right.iter().filter(|v| *v == l).count() as u32) * *l).sum()
}

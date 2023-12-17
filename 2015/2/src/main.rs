use adventage::day;
use std::cmp::max;
use std::cmp::min;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split("x")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .map(|pkg| (pkg[0] * pkg[1], pkg[0] * pkg[2], pkg[1] * pkg[2]))
        .map(|(a, b, c)| (2 * (a + b + c)) + min(a, min(b, c)))
        .sum()
}

fn part2(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .map(|pkg| (pkg[0], pkg[1], pkg[2]))
        .map(|(a, b, c)| (2 * ((a + b + c) - max(a, max(b, c)))) + (a * b * c))
        .sum()
}

day!(2015, 2);

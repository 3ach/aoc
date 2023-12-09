#![feature(iter_map_windows)]

use std::io;
use std::io::BufRead;

type Input = [Vec<i32>];

fn next(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|n| *n == 0) {
        return 0;
    }

    let difference = sequence.iter()
        .map_windows(|[prev, curr]| **curr - **prev)
        .collect();

    sequence.last().unwrap() + next(&difference)
}

fn prev(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|n| *n == 0) {
        return 0;
    }

    let difference = sequence.iter()
        .map_windows(|[prev, curr]| **curr - **prev)
        .collect();

    sequence.first().unwrap() - prev(&difference)
}

fn part1(input: &Input) -> i32 {
    input.iter()
        .map(next)
        .sum()
}

fn part2(input: &Input) -> i32 {
    input.iter()
        .map(prev)
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let nodes = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();

    let answer1 = part1(&nodes);
    let answer2 = part2(&nodes);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

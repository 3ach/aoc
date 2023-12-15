use adventage::day;
use std::io;
use std::io::BufRead;

fn part1(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut floor = 0;
    for (idx, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if floor < 0 {
            return idx as u32 + 1;
        }
    }
    panic!();
}

#[day]
fn parse() -> String {
    input.next().unwrap()
}

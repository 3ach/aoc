use adventage::{part1demo, part2demo, day};
use std::collections::HashSet;

part1demo!("+1, -2, +3, +1", 3);
part1demo!("+1, +1, +1", 3);
part1demo!("+1, +1, -2", 0);
part1demo!("-1, -2, -3", -6);

part2demo!("+1, -1", 0);
part2demo!("+3, +3, +4, -2, -4", 10);
part2demo!("-6, +3, +8, +5, -6", 5);
part2demo!("+7, +7, -2, -7, -4", 14);


day!(2018, 1);

fn parse(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|l| l.trim_matches(',').parse().unwrap()) 
        .collect()
}

fn part1(nums: &Vec<i32>) -> i32 {
    nums.iter()
        .fold(0, |acc, num| acc + num)
}

fn part2(nums: &Vec<i32>) -> i32 {
    let mut freq = 0;
    let mut seen = HashSet::new();

    for num in nums.iter().cycle() {
        if !seen.insert(freq) {
            return freq; 
        }

        freq += num;
    }

    panic!()
}

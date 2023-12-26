use adventage::{part1demo, part2demo, day};

day!(YEAR, DAY);

type TInput = Vec<String>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|l| String::from(l))
        .collect()
}

fn part1(_map: &TInput) -> u32 {
    0
}

fn part2(_map: &TInput) -> u32 {
    0
}

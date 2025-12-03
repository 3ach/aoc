use adventage::{day, part1demo, part2demo};
use std::cmp::{max, min};

day!(2025, 3);
part1demo!("987654321111111
811111111111119
234234234234278
818181911112111", 357);
part2demo!("987654321111111
811111111111119
234234234234278
818181911112111", 3121910778619);

type TInput = Vec<Vec<u64>>;

fn parse(input: &str) -> TInput {
    input.lines()
        .into_iter()
        .map(|line|
            line.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        ).collect()
}

fn best(bank: &Vec<u64>, number: usize) -> u64 {
    let mut best_at = bank.clone();

    for total in 0..number {
        let mut next_best_at = best_at.clone();
        let multiple = 10_u64.pow(total as u32);

        for idx in (0..next_best_at.len() - total).rev() {
            let current = if total > 0 {
                (bank[idx] * multiple) + best_at[idx + 1]
            } else {
                bank[idx]
            };

            next_best_at[idx] = max(next_best_at[min(idx + 1, next_best_at.len() - 1)], current);
        }
        
        best_at = next_best_at;
    }

    best_at.into_iter().max().unwrap()
}

fn part1(input: &TInput) -> u64 {
    input.iter()
        .map(|bank| best(bank, 2))
        .sum()
}

fn part2(input: &TInput) -> u64 {
    input.iter()
        .map(|bank| best(bank, 12))
        .sum()
}

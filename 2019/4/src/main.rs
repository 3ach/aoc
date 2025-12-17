use adventage::day;
use std::ops::RangeInclusive;

day!(2019, 4);

type TInput = RangeInclusive<u32>;

fn parse(input: &str) -> TInput {
    let parts: Vec<u32> = input.lines()
        .map(|line| line.split('-').map(|num| num.parse::<u32>().unwrap()).collect())
        .next()
        .unwrap();

    parts[0]..=parts[1]
}

fn part1(input: &TInput) -> usize {
    input
        .clone().map(|v| {
            let ones = v % 10;
            let tens = (v / 10) % 10;
            let hundreds = (v / 100) % 10;
            let thousands = (v / 1000) % 10;
            let ten_thousands = (v / 10000) % 10;
            let hundred_thousands = (v / 100000) % 10;

            if (
                ones == tens ||
                tens == hundreds ||
                hundreds == thousands ||
                thousands == ten_thousands ||
                ten_thousands == hundred_thousands
            ) && (
                ones >= tens &&
                tens >= hundreds &&
                hundreds >= thousands &&
                thousands >= ten_thousands &&
                ten_thousands >= hundred_thousands
            ) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &TInput) -> u32 {
    input
        .clone().map(|v| {
            let ones = v % 10;
            let tens = (v / 10) % 10;
            let hundreds = (v / 100) % 10;
            let thousands = (v / 1000) % 10;
            let ten_thousands = (v / 10000) % 10;
            let hundred_thousands = (v / 100000) % 10;

            if (
                (ones == tens && tens != hundreds) ||
                (tens == hundreds && tens != ones && hundreds != thousands) ||
                (hundreds == thousands && hundreds != tens && thousands != ten_thousands) ||
                (thousands == ten_thousands && thousands != hundreds && ten_thousands != hundred_thousands) ||
                (ten_thousands == hundred_thousands && ten_thousands != thousands)
            ) && (
                ones >= tens &&
                tens >= hundreds &&
                hundreds >= thousands &&
                thousands >= ten_thousands &&
                ten_thousands >= hundred_thousands
            ) {
                1
            } else {
                0
            }
        })
        .sum()
}

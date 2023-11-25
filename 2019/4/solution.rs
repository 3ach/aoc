use std::io::BufRead;
use std::io;
use std::ops::RangeInclusive;


type TInput = RangeInclusive<u32>;


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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let range: Vec<u32> = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.split('-').map(|num| num.parse::<u32>().unwrap()).collect())
        .next()
        .unwrap();

    let input = range[0]..=range[1];

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

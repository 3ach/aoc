use regex::Regex;
use std::io;
use std::io::BufRead;

fn part1(calibration: &[Vec<char>]) -> u32 {
    calibration
        .iter()
        .map(|line| {
            line.iter()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|digits| !digits.is_empty())
        .map(|digits| (digits[0] * 10) + digits[digits.len() - 1])
        .sum()
}

fn digit_to_num(digit: &str) -> u32 {
    match digit {
                    "one" | "eno" | "1" => 1,
                    "two" | "owt" | "2" => 2,
                    "three" | "eerht" | "3" => 3,
                    "four" | "ruof" | "4" => 4,
                    "five" | "evif" | "5" => 5,
                    "six" | "xis" | "6" => 6,
                    "seven" | "neves" | "7" => 7,
                    "eight" | "thgie" | "8" => 8,
                    "nine" | "enin" | "9" => 9,
                    _ => panic!(),
                }
}

fn part2(calibrations: &[Vec<char>]) -> u32 {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let re_rev = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();
    calibrations
        .iter()
        .map(|calibration| calibration.iter().cloned().collect::<String>())
        .inspect(|c| println!("{:?}", c))
        .map(|calibration| {
            let first = re.find(&calibration).unwrap();
            let first_end  = first.end();
            let first = digit_to_num(&first.as_str());

            let reversed = calibration.chars().rev().collect::<String>();
            let last = re_rev.find(&reversed).unwrap();
            let last_start = calibration.len() - last.end();
            let last = digit_to_num(&last.as_str());
            if last_start >= first_end {
                (first * 10) + last
            } else {
                (first * 10) + first
            }
        })
        .inspect(|c| println!("{:?}", c))
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let calibration = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let answer1 = part1(&calibration);
    let answer2 = part2(&calibration);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

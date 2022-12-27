use std::io::BufRead;
use std::io;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialEq, Eq)]
struct ParseNumberError {}

#[derive(Debug)]
struct SNAFUNum {
    num: i64,
}

impl FromStr for SNAFUNum {
    type Err = ParseNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num = 0;
        for digit in s.chars() {
            let digit = match digit {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!(),
            };

            num *= 5;
            num += digit;
        }

        if num < 0 {
            panic!("Got a negative result {} on {}", num, s);
        }
        Ok(SNAFUNum { num })
    }
}

impl ToString for SNAFUNum {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut remaining = self.num;
        while remaining > 0 {
            let place = remaining % 5;
            remaining = remaining / 5;
            match place {
                0 => result.insert(0, '0'),
                1 => result.insert(0, '1'),
                2 => result.insert(0, '2'),
                3 => {
                    remaining += 1;
                    result.insert(0, '=');
                },
                4 => {
                    remaining += 1;
                    result.insert(0, '-');
                },
                _ => panic!(),
            };
        }

        result
    }
}

type TInput = Vec<SNAFUNum>;

fn part1(input: &TInput) -> String {
    SNAFUNum { num: input.iter().map(|s| s.num).sum() }.to_string()
}

fn part2(input: &TInput) -> i64 {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.parse().unwrap())
        .collect();


	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

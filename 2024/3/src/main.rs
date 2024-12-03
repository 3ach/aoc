use adventage::{day, part1demo, part2demo};
use regex::Regex;

day!(2024, 3);
part1demo!(
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    161
);
part2demo!(
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    48
);

type TInput = String;

fn parse(input: &str) -> TInput {
    String::from(input)
}

fn part1(program: &TInput) -> i32 {
    let mul_regex = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
    mul_regex
        .captures_iter(program)
        .map(|capture| {
            capture
                .name("left")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap()
                * capture
                    .name("right")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap()
        })
        .sum()
}

fn part2(program: &TInput) -> i32 {
    let mut multiplying = true;
    let token_regex =
        Regex::new(r"((?<operation>(mul|do|don't))\((?<left>\d+)?,?(?<right>\d+)?\))").unwrap();
    token_regex
        .captures_iter(program)
        .map(|capture| {
            let left = capture.name("left");
            let right = capture.name("right");

            match capture.name("operation").unwrap().as_str() {
                "mul" if multiplying && left.is_some() && right.is_some() => {
                    right.unwrap().as_str().parse::<i32>().unwrap()
                        * left.unwrap().as_str().parse::<i32>().unwrap()
                }
                "do" => {
                    multiplying = true;
                    0
                }
                "don't" => {
                    multiplying = false;
                    0
                }
                _ => 0,
            }
        })
        .sum()
}

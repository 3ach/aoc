use adventage::{day, part1demo};

day!(2015, 10);

type TInput = String;

fn parse(input: &str) -> TInput {
    String::from(input.trim())
}

fn look_say(num: &str) -> String {
    let mut last: Option<(char, u32)> = None;
    let mut said = String::new();
    for c in num.chars() {
        if let Some((prev, cnt)) = last { 
            if c == prev {
                last = Some((c, cnt + 1));
            } else {
                said.push(char::from_digit(cnt, 10).unwrap());
                said.push(prev);
                last = Some((c, 1));
            } 
        } else {
            last = Some((c, 1));
        }
    }

    if let Some((prev, cnt)) = last { 
        said.push(char::from_digit(cnt, 10).unwrap());
        said.push(prev);
    }

    said
}

fn part1(num: &TInput) -> usize {
    let mut num = String::from(num);

    for _ in 0..40 {
        num = look_say(&num);
    }

    num.len()
}

fn part2(num: &TInput) -> usize {
    let mut num = String::from(num);

    for _ in 0..50 {
        num = look_say(&num);
    }

    num.len()
}

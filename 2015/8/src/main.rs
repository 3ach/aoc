#![feature(if_let_guard)]
use adventage::{day, part1demo, part2demo};

day!(2015, 8);
part1demo!("\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"", 12);
part2demo!("\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"", 19);

type TInput = Vec<String>;

#[derive(PartialEq, Debug)]
enum State {
    Init,
    Parsing, 
    Escaping,
    FirstHex,
    NextHex(u32),
    Done
}

fn parse(input: &str) -> TInput {
    input.lines().map(String::from).collect()
}

fn memory(s: &str) -> usize {
    let mut state = State::Init;
    let mut parsed = String::new();

    for c in s.chars() {
        state = match c {
            '"' if state == State::Init => State::Parsing,
            '"' if state == State::Parsing => State::Done,
            '"' if state == State::Escaping => {
                parsed.push('"');
                State::Parsing
            },

            '\\' if state == State::Parsing => State::Escaping,
            '\\' if state == State::Escaping => {
                parsed.push('\\');
                State::Parsing
            },

            'x' if state == State::Escaping => State::FirstHex,
            '0'..='9' | 'a'..='f' | 'A'..='F' if state == State::FirstHex => { State::NextHex(c.to_digit(16).unwrap()) },
            '0'..='9' | 'a'..='f' | 'A'..='F' if let State::NextHex(msb) = state => { 
                let hex = msb * 16 + c.to_digit(16).unwrap();
                parsed.push(char::from_u32(hex).unwrap());
                State::Parsing
            },

            _ if state == State::Escaping => {
                panic!("Unexpected escape seq {c}");
            }
            _ if state == State::FirstHex => {
                panic!("Unexpected hex char {c}");
            }
            _ if let State::NextHex(_) = state => {
                panic!("Unexpected hex char {c}");
            }
            _ if state == State::Done => panic!("Can't parse past end of the string!"),
            _ if state == State::Parsing => {
                parsed.push(c);
                State::Parsing
            },
            _ => panic!("Unexpected {c} in state {state:?}"),
        };
    }

    parsed.chars().count()
}

fn encode(s: &str) -> usize {
    s.chars()
        .map(|c|
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => 1,
                '"' => 2,
                '\\' => 2,
                _ => panic!("Unknown char {c}"),
            }
        ).sum::<usize>() + 2
}

fn part1(input: &TInput) -> usize {
    input.iter()
        .map(|s| s.len() - memory(s))
        .sum()
}

fn part2(input: &TInput) -> usize {
    input.iter()
        .map(|s| encode(s) - s.len())
        .sum()
}

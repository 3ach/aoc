use adventage::{day, part1demo, part2demo};

day!(2025, 6);
part1demo!("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +", 4277556);
part2demo!("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +", 3263827);

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Homework {
    terms: Vec<Vec<String>>,
    operators: Vec<Operator>,
}

type TInput = Homework;

fn parse(input: &str) -> TInput {
    let mut homework = Homework { 
        terms: vec![],
        operators: vec![],
    };
    let longest = input.lines()
        .map(|l| l.len())
        .max()
        .unwrap();

    let operator_line = input.lines().last().unwrap();
    let lengths: Vec<(usize, usize)> = operator_line.chars().enumerate()
        .filter(|(_, c)| *c != ' ')
        .skip(1)
        .chain(vec![(longest + 1, ' ')])
        .scan(0, |last, (current, _)| {
            let boundary = (*last, current - 1);
            *last = current;
            Some(boundary)
        })
        .collect();

    for line in input.lines() {
        let first = line.chars().nth(0).unwrap();
        if first == '*' || first == '+'{
            for token in line.split_whitespace() {
                match token {
                    "*" => homework.operators.push(Operator::Multiply),
                    "+" => homework.operators.push(Operator::Add),
                    _ => panic!(),
                }
            }
        } else {
            homework.terms.push(
                lengths.iter()
                .map(|(from, to)| String::from(&line[*from..*to]))
                .collect()
            );
        }
    }

    homework
}

fn part1(homework: &TInput) -> u64 {
    homework.operators.iter()
        .enumerate()
        .map(|(idx, operator)| {
            let terms = homework.terms.iter().map(|row| &row[idx]);
            match operator {
                Operator::Add => terms.fold(0, |sum, term| sum + term.trim().parse::<u64>().unwrap()),
                Operator::Multiply => terms.fold(1, |product, term| product * term.trim().parse::<u64>().unwrap()),
            }
        }).sum()
}

fn part2(homework: &TInput) -> u64 {
    homework.operators.iter()
        .enumerate()
        .map(|(idx, operator)| {
            let terms: Vec<&str> = homework.terms.iter().map(|row| row[idx].as_str()).collect();
            let terms = terms.iter().fold(vec![0u64; terms[0].len()], |acc, digits| 
                digits.chars()
                    .enumerate()
                    .map(|(idx, digit)| if let Some(digit) = digit.to_digit(10) {
                        (acc[idx] * 10) + (digit as u64)
                    } else {
                        acc[idx]
                    })
                    .collect()
            );
            match operator {
                Operator::Add => terms.iter().fold(0, |sum, term| sum + term),
                Operator::Multiply => terms.iter().fold(1, |product, term| product * term),
            }
        }).sum()
}

use std::io::BufRead;
use std::io;
use std::convert::TryInto;
use std::collections::VecDeque;

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
enum Token {
    Multiply,
    Add,
    Open,
    Close,
    Num(u64),
}

fn calculate(tokens: &[Token]) -> u64 {
    let mut postfix = VecDeque::new();
    let mut operators = vec![];

    for token in tokens {
        match token {
            Token::Num(_) => postfix.push_back(*token),
            Token::Open => operators.push(*token),
            Token::Add | Token::Multiply => {
                while let Some(op) = operators.pop() {
                    if op == Token::Open {
                        operators.push(Token::Open);
                        break;
                    }

                    postfix.push_back(op);
                }

                operators.push(*token);
            },
            Token::Close => {
                while let Some(op) = operators.pop() {
                    if op == Token::Open {
                        break;
                    }
                    
                    postfix.push_back(op);
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        postfix.push_back(op);
    }

    let mut out = vec![];

    while let Some(tok) = postfix.pop_front() {
        match tok {
            Token::Add => {
                let left = out.pop().unwrap();
                let right = out.pop().unwrap();

                out.push(left + right);
            },
            Token::Multiply => {
                let left = out.pop().unwrap();
                let right = out.pop().unwrap();

                out.push(left * right);
            },
            Token::Num(num) => out.push(num),
            _ => panic!(),
        }
    }

    out.pop().unwrap()
}

fn calculate_precedence(tokens: &[Token]) -> u64 {
    let mut postfix = VecDeque::new();
    let mut operators = vec![];

    for token in tokens {
        match token {
            Token::Num(_) => postfix.push_back(*token),
            Token::Open => operators.push(*token),
            Token::Add | Token::Multiply => {
                while let Some(op) = operators.pop() {
                    if op == Token::Open || op == Token::Multiply {
                        operators.push(op);
                        break;
                    }

                    postfix.push_back(op);
                }

                operators.push(*token);
            },
            Token::Close => {
                while let Some(op) = operators.pop() {
                    if op == Token::Open {
                        break;
                    }
                    
                    postfix.push_back(op);
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        postfix.push_back(op);
    }

    let mut out = vec![];

    while let Some(tok) = postfix.pop_front() {
        match tok {
            Token::Add => {
                let left = out.pop().unwrap();
                let right = out.pop().unwrap();

                out.push(left + right);
            },
            Token::Multiply => {
                let left = out.pop().unwrap();
                let right = out.pop().unwrap();

                out.push(left * right);
            },
            Token::Num(num) => out.push(num),
            _ => panic!(),
        }
    }

    out.pop().unwrap()
}

fn part1(problems: &[Vec<Token>]) -> u64 {
    problems.iter()
        .map(|problem| calculate(problem))
        .sum()
}

fn part2(problems: &[Vec<Token>]) -> u64 {
    problems.iter()
        .map(|problem| calculate_precedence(problem))
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let problems: Vec<Vec<Token>> = reader.lines()
        .map(|line| line.expect("Can't read stdin")
            .chars()
            .filter(|c| *c != ' ')
            .map(|tok| {
                match tok {
                    '*' => Token::Multiply,
                    '+' => Token::Add,
                    '(' => Token::Open,
                    ')' => Token::Close,
                    _ => Token::Num(tok.to_string().parse().unwrap()),
                }
            }).collect()
        ).collect();

	let answer1 = part1(&problems);
	let answer2 = part2(&problems);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

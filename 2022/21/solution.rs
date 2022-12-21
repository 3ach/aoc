use std::io::BufRead;
use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Op {
    Add,
    Subtract,
    Divide, 
    Multiply
}


#[derive(Debug, Clone)]
enum Job {
    Math(String, Op, String),
    Number(i64),
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job
}

type TInput = Vec<Monkey>;

fn dependency_tree

fn part1(input: &TInput) -> i64 {
    let mut resolved: HashMap<_, i64> = input.iter()
        .filter_map(|monkey| match &monkey.job {
            Job::Number(num) => Some((monkey.name.clone(), *num)),
            _ => None,
        }).collect();

    while !resolved.contains_key("root") {
        for monkey in input {
            if resolved.contains_key(&monkey.name) {
                continue;
            }

            if let Job::Math(lhs, op, rhs) = &monkey.job {
                if let (Some(lhs), Some(rhs)) = (resolved.get(lhs), resolved.get(rhs)) {
                    match op {
                        Op::Add => resolved.insert(monkey.name.clone(), lhs + rhs),
                        Op::Subtract => resolved.insert(monkey.name.clone(), lhs - rhs),
                        Op::Multiply => resolved.insert(monkey.name.clone(), lhs * rhs),
                        Op::Divide => resolved.insert(monkey.name.clone(), lhs / rhs),
                    };
                }
            }
        }
    }

    *resolved.get("root").unwrap()
}

fn part2(input: &TInput) -> i64 {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let (name, job) = line.split_once(": ").unwrap();

            if job.contains(' ') {
                let parts = job.split(" ").collect::<Vec<_>>();
                let op = match parts[1] {
                    "+" => Op::Add,
                    "-" => Op::Subtract, 
                    "*" => Op::Multiply,
                    "/" => Op::Divide,
                    _ => panic!("Unknown op {}", parts[1]),
                };

                Monkey { name: name.to_string(), job: Job::Math(parts[0].to_string(), op, parts[2].to_string()) }
            } else {
                Monkey { name: name.to_string(), job: Job::Number(job.parse().unwrap()) }
            }
        })
        .collect();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

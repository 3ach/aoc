use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone)]
enum Op {
    Eq,
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug, Clone)]
enum Job {
    Math(String, Op, String),
    Number(i64),
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    job: Job,
}

type TInput = Vec<Monkey>;

fn resolve(input: &TInput, which: String) -> Option<i64> {
    let mut resolved: HashMap<_, i64> = input
        .iter()
        .filter_map(|monkey| match &monkey.job {
            Job::Number(num) => Some((monkey.name.clone(), *num)),
            _ => None,
        })
        .collect();

    while !resolved.contains_key(&which) {
        let resolved_count = resolved.len();
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
                        _ => panic!(),
                    };
                }
            }
        }

        if resolved_count == resolved.len() {
            return None;
        }
    }

    Some(resolved[&which])
}

fn part1(input: &TInput) -> i64 {
    resolve(input, "root".to_string()).unwrap()
}

fn part2(input: &TInput) -> i64 {
    let input: Vec<Monkey> = input
        .iter()
        .cloned()
        .filter(|m| m.name != "humn")
        .map(|mut m| {
            if m.name == "root" {
		if let Job::Math(lhs, ref mut op, rhs) = &mut m.job {
			*op = Op::Eq;
		}
            }
            m
        })
        .collect();

    let mut current = "root".to_string();
    let mut carry: i64 = 0;

    loop {
	if current == "humn" { return carry; }
        let monkey = input.iter().filter(|m| m.name == current).next().unwrap();
        if let Job::Math(lhs, op, rhs) = &monkey.job {
            let left = resolve(&input, lhs.to_string());
            let right = resolve(&input, rhs.to_string());

	    let (unresolved, resolved) = match (left, right) {
		(None, Some(value)) => (lhs, value),
		(Some(value), None) => (rhs, value),
		_ => panic!(),
		};

	    //print!("At node {:?}, carry: {} -> ", monkey, carry);

	    carry = match (op, left) {
		(Op::Eq, _) => resolved,
		(Op::Add, _) => carry - resolved,
		(Op::Subtract, Some(_)) => resolved - carry,
		(Op::Subtract, None)  => carry + resolved,
		(Op::Multiply, _) => carry / resolved,
		(Op::Divide, None) => carry * resolved,
		(Op::Divide, Some(_)) => resolved / carry,
	    };

            current = unresolved.to_string();
        } else {
            panic!();
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader
        .lines()
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

                Monkey {
                    name: name.to_string(),
                    job: Job::Math(parts[0].to_string(), op, parts[2].to_string()),
                }
            } else {
                Monkey {
                    name: name.to_string(),
                    job: Job::Number(job.parse().unwrap()),
                }
            }
        })
        .collect();

    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

use std::io::BufRead;
use std::io;

type TInput = Vec<Monkey>;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Add
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Num(u128),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation_operator: Operator,
    operation_operand: Operand,
    divisor: u128,
    iftrue: usize,
    iffalse: usize 
}

fn create_monkey() -> Monkey {
    Monkey { 
        items: vec![], 
        operation_operator: Operator::Multiply, 
        operation_operand: Operand::Old,
        divisor: 0,
        iftrue: 0, 
        iffalse: 0 }
}

fn play_round(which: usize, monkeys: &mut TInput, anxious: bool, common: u128) {
    let monkey = &mut monkeys[which];
    let mut dests = vec![];
    
    while let Some(item) = monkey.items.pop() {
        let mut newitem = match (monkey.operation_operator, monkey.operation_operand) {
            (Operator::Multiply, Operand::Old) => item * item,
            (Operator::Add, Operand::Old) => item + item,
            (Operator::Multiply, Operand::Num(num)) => item * num,
            (Operator::Add, Operand::Num(num)) => item + num,
        };

        if !anxious { 
            newitem /= 3;
        }

        newitem %= common;

        if newitem % monkey.divisor == 0 {
            dests.push((monkey.iftrue, newitem));
        } else {
            dests.push((monkey.iffalse, newitem));
        }
    }

    for (dest, item) in dests {
        monkeys[dest].items.push(item);
    }
}

fn part1(input: &TInput) -> usize {
    let mut input = input.clone();
    let mut inspections: Vec<usize> = input.iter().map(|_| 0).collect();
    let common = input.iter().map(|m| m.divisor).product();

    for _ in 0..20 {
        for monkey in 0..input.len() {
            inspections[monkey] += input[monkey].items.len();
            play_round(monkey, &mut input, false, common);
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn part2(input: &TInput) -> usize {
    let mut input = input.clone();
    let mut inspections: Vec<usize> = input.iter().map(|_| 0).collect();
    let common = input.iter().map(|m| m.divisor).product();

    for _ in 0..10000 {
        for monkey in 0..input.len() {
            inspections[monkey] += input[monkey].items.len();
            play_round(monkey, &mut input, true, common);
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut input = vec![];
    let mut monkey = create_monkey();
    
    for line in reader.lines() {
        let line = line.expect("Couldn't read stdin");
        if line == "" {
            input.push(monkey);
            monkey = create_monkey();
            continue;
        }
        
        if let Some((instruction, parameter)) = line.trim().split_once(": ") {
            match instruction {
                "Starting items" => {
                    monkey.items.extend(parameter.split(", ").map(|item| item.parse::<u128>().unwrap()))
                },
                "Operation" => {
                    if let Some((_, rest)) = parameter.split_once(" = old ") {
                        if let Some((operation, operand)) = rest.split_once(" ") {
                            match operation {
                                "*" => monkey.operation_operator = Operator::Multiply,
                                "+" => monkey.operation_operator = Operator::Add,
                                _ => panic!("Unknown operator {}", operation)
                            }

                            match operand {
                                "old" => monkey.operation_operand = Operand::Old,
                                 _ => monkey.operation_operand = Operand::Num(operand.parse().unwrap())
                            }
                        } else {
                            panic!("Couldn't parse {}", rest);
                        }
                    } else {
                        panic!("Couldn't parse {}", parameter);
                    }
                },
                "Test" => {
                    if let Some((_, divisor)) = parameter.rsplit_once(" ") {
                        monkey.divisor = divisor.parse().unwrap();
                    } else {
                        panic!("could not parse {}", parameter)
                    }
                },
                "If true" => {
                    if let Some((_, other)) = parameter.rsplit_once(" ") {
                        monkey.iftrue = other.parse().unwrap();
                    } else {
                        panic!("could not parse {}", parameter)
                    }
                }
                "If false" => {
                    if let Some((_, other)) = parameter.rsplit_once(" ") {
                        monkey.iffalse = other.parse().unwrap();
                    } else {
                        panic!("could not parse {}", parameter)
                    }
                },
                _ => panic!("Couldn't handle {}", instruction)
            }
        } else {
            continue;
        }
    }

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

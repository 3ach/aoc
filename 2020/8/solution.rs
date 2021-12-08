use std::io::BufRead;
use std::io;
use std::collections::HashSet;

#[derive(Debug,Clone,Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn part1(program: &Vec<Instruction>) -> i32 {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut instruction_ptr = 0;
    let mut accumulator = 0;

    while !visited.contains(&instruction_ptr) {
        visited.insert(instruction_ptr);

        match program[instruction_ptr as usize] {
            Instruction::Nop(_) => instruction_ptr += 1,
            Instruction::Acc(arg) => { 
                instruction_ptr += 1;
                accumulator += arg;
            },
            Instruction::Jmp(arg) => instruction_ptr += arg,
        } 
    }

    return accumulator;
}

fn run_program_flip_nth(program: &Vec<Instruction>, n: usize) -> Option<i32> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut instruction_ptr = 0;
    let mut accumulator = 0;

    while instruction_ptr >= 0 && (instruction_ptr as usize) < program.len() {
        if visited.contains(&instruction_ptr) {
            // looping.
            return None;
        }

        visited.insert(instruction_ptr);

        let mut instruction = program[instruction_ptr as usize];
        if instruction_ptr as usize == n {
            instruction = match instruction {
                Instruction::Nop(arg) => Instruction::Jmp(arg),
                Instruction::Jmp(arg) => Instruction::Nop(arg),
                _ => return None,
            }
        }

        match instruction {
            Instruction::Nop(_) => instruction_ptr += 1,
            Instruction::Acc(arg) => { 
                instruction_ptr += 1;
                accumulator += arg;
            },
            Instruction::Jmp(arg) => instruction_ptr += arg,
        } 
    }

    if instruction_ptr as usize == program.len() {
        return Some(accumulator);
    }
    
    return None;
}

fn part2(program: &Vec<Instruction>) -> i32 {
    for flip in 0..program.len() {
        match run_program_flip_nth(program, flip) {
            Some(accumulator) => return accumulator,
            None => continue
        }
    }

    panic!("No answer!");
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let program: Vec<Instruction> = reader.lines().map(|line| {
        let line = line.expect("couldn't read stdin");
		let mut tok = line.split_whitespace();

        let op = tok.next().unwrap();
        let arg: i32 = tok.next().unwrap().parse().unwrap();

        match op {
           "nop" => Instruction::Nop(arg),
           "jmp" => Instruction::Jmp(arg),
           "acc" => Instruction::Acc(arg),
           _ => panic!("Unknown command!")
        }
	}).collect();

	let answer1 = part1(&program);
	let answer2 = part2(&program);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

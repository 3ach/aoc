use std::io;
use std::io::BufRead;

type Program = Vec<usize>;

fn add(ip: usize, program: &mut Program) -> usize {
    assert!(ip < program.len() - 3);

    let left_address = program[ip + 1];
    let right_address = program[ip + 2];
    let destination_address = program[ip + 3];

    assert!(destination_address < program.len());

    program[destination_address] = program[left_address] + program[right_address];

    ip + 4
}

fn multiply(ip: usize, program: &mut Program) -> usize {
    assert!(ip < program.len() - 4);

    let left_address = program[ip + 1];
    let right_address = program[ip + 2];
    let destination_address = program[ip + 3];

    assert!(destination_address < program.len());

    program[destination_address] = program[left_address] * program[right_address];

    ip + 4
}

fn run(mut program: &mut Program) {
    let mut ip = 0;
    loop {
        assert!(ip < program.len());
        let instruction = program[ip];

        ip = match instruction {
            1 => add(ip, &mut program),
            2 => multiply(ip, &mut program),
            99 => break,
            _ => panic!("Unsupported instruction"),
        };
    }
}

fn part1(program: &Program) -> usize {
    let mut program = program.clone();
    program[1] = 12;
    program[2] = 2;
    run(&mut program);

    program[0]
}

fn part2(program: &Program) -> usize {

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = program.clone();

            program[1] = noun;
            program[2] = verb;
            run(&mut program);

            if program[0] == 19690720 {
                return (100 * noun) + verb;
            }
        }
    }

    panic!();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let program: Program = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .next()
        .unwrap();

    let answer1 = part1(&program);
    println!("Part 1: {}", answer1);;

    let answer2 = part2(&program);
    println!("Part 2: {}", answer2);
    
    Ok(())
}

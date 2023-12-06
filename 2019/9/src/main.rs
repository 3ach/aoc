extern crate intcode;
use intcode::{Program, run};
use std::io;
use std::io::BufRead;

fn part1(program: &Program) -> i64 {
    let input = vec![1];
    let program = program.clone();
    let out = run(&program, &input);

    *out.iter().last().unwrap()
}

fn part2(program: &Program) -> i64 {
    let input = vec![2];
    let program = program.clone();
    let out = run(&program, &input);

    *out.iter().last().unwrap()
}

fn main() -> io::Result<()> {
    let program: Program = {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        buf.trim()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    };

    let answer1 = part1(&program);
    println!("Part 1: {}", answer1);

    let answer2 = part2(&program);
    println!("Part 2: {}", answer2);

    Ok(())
}

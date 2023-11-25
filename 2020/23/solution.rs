use std::io;
use std::ops::RangeInclusive;
use std::collections::HashMap;

fn part1(input: &mut [usize; 9]) -> usize {
    let mut pos = 8;
    
    for mov in 1..2 {
        let current = input[pos];
        let cups = [input[current], input[input[current]], input[input[input[current]]]];
        println!("{:?}", input);
        println!("{:?}", current);
        println!("{:?}", cups);
    }

    0
}

fn part2() -> usize {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    stdin.read_line(&mut line);
    let input = line.trim().chars().map(|c| (c.to_digit(10).unwrap() - 1) as usize).collect::<Vec<_>>();
    let mut positions = [0usize; 9];

    for idx in 1..9 {
       positions[input[idx - 1]] = input[idx];
    }

    positions[input[8]] = input[0];

	let answer1 = part1(&mut positions.clone());
	let answer2 = part2();

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

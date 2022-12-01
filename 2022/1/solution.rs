use std::io::BufRead;
use std::io;

fn part1(calories: &[Vec<u32>]) -> u32 {
    calories.iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap()
}

fn part2(calories: &[Vec<u32>]) -> u32 {
    let mut elves: Vec<u32> = calories.iter()
        .map(|elf| elf.iter().sum())
        .collect();

    elves.sort();

    elves.iter()
        .rev()
        .take(3)
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut calories = vec![];
    let mut current = vec![];

    for line in reader.lines() {
        let line = line.expect("Couldn't read stdin");
        if line.len() == 0 {
            calories.push(current);
            current = vec![];
            continue;
        }

        let item = line.parse().unwrap();
        current.push(item);
    }

    calories.push(current);

	let answer1 = part1(&calories);
	let answer2 = part2(&calories);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

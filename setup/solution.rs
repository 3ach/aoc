use std::io::BufRead;
use std::io;

type TInput = Vec<String>;

fn part1(input: &TInput) -> u32 {
    0
}

fn part2(input: &TInput) -> u32 {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .collect();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

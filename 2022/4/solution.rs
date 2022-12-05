use std::io::BufRead;
use std::io;
use std::ops::RangeInclusive;

type TInput = Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>;

fn part1(input: &TInput) -> usize {
    input.iter()
        .map(|(first, second)| if first.start() <= second.start() { (first, second) } else { (second, first) })
        .filter(|(first, second)| first.end() >= second.end() || (first.start() == second.start() && second.end() >= first.end()))
        .count()
}

fn part2(input: &TInput) -> usize {
    input.iter()
        .map(|(first, second)| if first.start() <= second.start() { (first, second) } else { (second, first) })
        .filter(|(first, second)| first.end() >= second.start())
        .count()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let (first, second) = line.split_once(",").unwrap();
            let ((fs, fe), (ss, se)) = (first.split_once("-").unwrap(), second.split_once("-").unwrap());
        ((fs.parse().unwrap(), fe.parse().unwrap()), (ss.parse().unwrap(), se.parse().unwrap()))
        })
        .map(|((fs, fe), (ss, se))| ((fs..=fe), (ss..=se)))
        .collect();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

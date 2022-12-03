use std::io::BufRead;
use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

type TInput = Vec<(HashSet<u8>, HashSet<u8>)>;

fn part1(input: &TInput) -> u32 {
    input.iter()
        .map(|sack| sack.0.intersection(&sack.1).next().unwrap())
        .map(|overlap| if overlap.is_ascii_uppercase() { 27 + overlap - b'A' } else { 1 + overlap - b'a' })
        .map(|score| u32::from(score))
        .sum()
}

fn part2(input: &TInput) -> u32 {
    let elves: Vec<HashSet<&u8>> = input.iter()
        .map(|sack| sack.0.union(&sack.1).collect())
        .collect();

    elves.chunks(3)
        .map(|triplet| ( triplet[0].intersection(&triplet[1]).cloned().collect(), triplet[2].clone()))
        .map(|(sofar, third): (HashSet<&u8>, HashSet<&u8>)| sofar.intersection(&third).next().unwrap().clone())
        .map(|overlap| if overlap.is_ascii_uppercase() { 27 + overlap - b'A' } else { 1 + overlap - b'a' })
        .map(|score| u32::from(score))
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|mut first| {
            let second = first.split_off(first.len() / 2);

            (HashSet::from_iter(first.bytes()), HashSet::from_iter(second.bytes()))
        })
        .collect();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

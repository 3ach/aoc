use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn part1(cards: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    cards.iter()
        .map(|(scratch, winning)| scratch.intersection(&winning).count())
        .filter_map(|overlap| if overlap > 1 { Some((2_u32).pow(overlap as u32 - 1)) } else { None })
        .sum()
}

fn part2(cards: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    let mut counts = vec![0; cards.len()];
    for (num, card) in cards.iter()
        .map(|(scratch, winning)| scratch.intersection(&winning).count())
        .enumerate() {
            counts[num] += 1;
            for next in num+1..=num+card {
                counts[next] += counts[num];
            }
        }

    counts.iter().sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let cards = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| String::from(line.split_once(": ").unwrap().1))
        .map(|line| line.split(" | ")
             .map(|card| card.split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect::<HashSet<u32>>())
             .collect::<Vec<HashSet<u32>>>())
        .map(|cards| (cards[0].clone(), cards[1].clone()))
        .collect::<Vec<(HashSet<u32>, HashSet<u32>)>>();


    let answer1 = part1(&cards);
    let answer2 = part2(&cards);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

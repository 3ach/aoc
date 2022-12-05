use std::io;
use std::collections::HashMap;
use std::convert::TryInto;

fn part1(game: &[u32], round: usize) -> u32{
    let mut said: HashMap<u32, usize> = game.iter()
        .enumerate()
        .map(|(idx, val)| (*val, idx.try_into().unwrap()))
        .collect();
   
    let mut last = game[game.len() - 1];
    let mut next = last;

    for turn in game.len()..round {
        if said.contains_key(&last) {
            next = (turn - 1 - said[&last]).try_into().unwrap();
        } else {
            next = 0;
        }

        said.insert(last, turn - 1);
        last = next;
    }

    last
}

fn part2() -> u32 {
    0
}

fn main() -> io::Result<()> {
    println!("{:?}", part1(&[0, 3, 6], 30000000));
    println!("{:?}", part1(&[1, 3, 2], 30000000));
    println!("{:?}", part1(&[2, 1, 3], 30000000));
    println!("{:?}", part1(&[1, 2, 3], 30000000));
    println!("{:?}", part1(&[2, 3, 1], 30000000));
    println!("{:?}", part1(&[3, 2, 1], 30000000));
    println!("{:?}", part1(&[3, 1, 2], 30000000));

    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input)?;
    let game: Vec<u32> = input.trim()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();

	let answer1 = part1(&game, 2020);
	let answer2 = part1(&game, 30000000);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

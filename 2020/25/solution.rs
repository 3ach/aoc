use std::io::BufRead;
use std::io;


fn loop_size(observed: u64) -> u64 {
    let mut size = 0;
    let mut key = 1;

    while key != observed {
        key = key * 7;
        key = key % 20201227;
        size += 1;
    }

    size
}

fn transform(loop_size: u64, subject: u64) -> u64 {
    println!("Transforming {} with a loop of {}", subject, loop_size);
    let mut val = 1;

    for _ in 0..loop_size {
        val = val * subject;
        val = val % 20201227;
    }

    val
}

fn part1(card: u64, door: u64) -> u64 {
    println!("card: {}, door: {}", card, door);

    let card_loop = loop_size(card);
    let door_loop = loop_size(door);

    println!("card loop: {}, door loop: {}", card_loop, door_loop);

    let door_encryption_key = transform(door_loop, card);
    let card_encryption_key = transform(card_loop, door);

    if door_encryption_key != card_encryption_key {
        panic!("what? door: {}, card: {}", door_encryption_key, card_encryption_key);
    } 

    door_encryption_key
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: Vec<u64> = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.parse().unwrap())
        .collect();

	let answer1 = part1(input[0], input[1]);
	println!("Answer 1: {}", answer1);

    Ok(())
}

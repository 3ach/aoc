use std::io::BufRead;
use std::io;
use std::cmp;

fn part1(positions: &Vec<i64>) -> i64 {
    let mut positions = positions.clone();
    positions.sort();

    let median = positions.len() / 2;
    let position = positions[median];

    let mut fuel = 0;
    for crab in positions {
        fuel += (crab - position).abs();
    }

    return fuel;
}

fn part2(positions: &Vec<i64>) -> i64 {
    let average = positions.iter().map(|p| *p as f64).sum::<f64>() / positions.len() as f64;
    let floor_average = average.floor() as i64;
    let round_average = average.round() as i64;

    let mut floor_fuel: i64 = 0;
    let mut round_fuel: i64 = 0;
    for crab in positions {
        let floor_distance = (crab - floor_average).abs();
        floor_fuel += (0..=floor_distance).sum::<i64>();

        let round_distance = (crab - round_average).abs();
        round_fuel += (0..=round_distance).sum::<i64>();
    }

    return cmp::min(floor_fuel, round_fuel);
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut reader = stdin.lock();

    reader.read_line(&mut buffer).expect("Couldn't read from stdin");

    let crabs:  Vec<i64> = buffer.trim().split(",").map(|crab| crab.parse().unwrap()).collect();

	let answer1 = part1(&crabs);
	let answer2 = part2(&crabs);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

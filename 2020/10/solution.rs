use std::io::BufRead;
use std::io;
use std::cmp;

fn part1(adapters: &Vec<u32>) -> u16 {
    let mut ones = 1;
    let mut threes = 1;

    for pos in 1..adapters.len() {
        let jump = adapters[pos] - adapters[pos - 1];
        if jump == 1 {
            ones += 1;
        } else if jump == 3 {
            threes += 1;
        }
    }

    return ones * threes;
}

fn part2(adapters: &Vec<u32>) -> u64 {
    let mut paths: Vec<u64> = vec![1];

    for adapter in 1..adapters.len() {
        let lookback = cmp::min(3, adapter);
        let mut paths_to_here = (adapter-lookback..adapter)
            .filter(|prev| adapters[adapter] - adapters[*prev] <= 3)
            .map(|prev| paths[prev])
            .sum();

        if adapters[adapter] <= 3 {
            paths_to_here += 1
        }

        paths.push(paths_to_here);
    }

    return *paths.last().unwrap();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let mut adapters: Vec<u32> = reader.lines().map(|line| {
        let line = line.expect("couldn't read stdin");
        line.parse().unwrap()
	}).collect();

    adapters.sort();

	let answer1 = part1(&adapters);
	let answer2 = part2(&adapters);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

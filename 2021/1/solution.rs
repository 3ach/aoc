use std::io::BufRead;
use std::io;

fn part1(depths: &Vec<i32>) -> i32 {
	let mut increases = 0;

	for range in 1..depths.len() {
		if depths[range] > depths[range - 1] {
			increases += 1;
		}
	}
	
	return increases;	
}

fn part2(depths: &Vec<i32>) -> i32 {
	if depths.len() < 4 {
		return 0;
	}

	let mut increases = 0;
	let mut window = depths[0] + depths[1] + depths[2];
	
	for range in 3..depths.len() {
		let next_window = window + depths[range] - depths[range - 3];

		if next_window > window {
			increases += 1;
		}

		window = nextWindow;
	}

	return increases;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let depths: Vec<i32> = reader.lines().map(|line| {
		line.expect("couldn't read stdin")
			.parse::<i32>()
			.unwrap()
	}).collect();

	let answer1 = part1(&depths);
	let answer2 = part2(&depths);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

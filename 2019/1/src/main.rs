use std::io::BufRead;
use std::io;

fn part1(masses: &[i32]) -> i32 {
    masses.iter()
        .map(|mass| (mass / 3) - 2)
        .sum()
}

fn part2(masses: &[i32]) -> i32 {
    let mut fuel = 0;

    let mut masses: Vec<i32> = masses.iter().cloned().collect();

    loop {
        masses = masses.iter()
            .map(|mass| (mass / 3) - 2)
            .map(|mass| if mass < 0 { 0 } else { mass })
            .collect();

        let round_fuel: i32 = masses.iter().sum();
        if round_fuel == 0 {
            break;
        }

        fuel += round_fuel;
    }

    fuel
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let masses: Vec<i32> = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.parse().unwrap())
        .collect();

	let answer1 = part1(&masses);
	let answer2 = part2(&masses);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

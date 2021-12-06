use std::io::BufRead;
use std::io;

fn time(state: &mut [i64; 9]) {
    let breeding = state[0];

    for day in 1..9 {
       state[day - 1] = state[day];
    }

    state[8] = breeding;
    state[6] += breeding;
}

fn part1(state: &[i64; 9]) -> i64 {
    let mut part_state = state.clone();
    for _ in 0..80 {
        time(&mut part_state);
    }

    return part_state.iter().sum();
}

fn part2(state: &[i64; 9]) -> i64 {
    let mut part_state = state.clone();
    for _ in 0..256 {
        time(&mut part_state);
    }

    return part_state.iter().sum();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut reader = stdin.lock();
    let mut state = [0; 9];

    reader.read_line(&mut buffer).expect("Couldn't read from stdin");

    let initial: Vec<usize> = buffer.trim().split(",").map(|fish| fish.parse().unwrap()).collect();

    for fish in initial {
        state[fish] += 1
    }

	let answer1 = part1(&state);
	let answer2 = part2(&state);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

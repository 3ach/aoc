use std::io::BufRead;
use std::io;
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    input
        .as_bytes()
        .windows(4)
        .enumerate()
        .filter(|(_, window)| {
            let mut v = vec![];
            for pos in 0..4 {
                if v.contains(&window[pos]) {
                    return false;
                }

                v.push(window[pos]);
            }

            return true;
        })
        .map(|(idx, _)| idx + 4)
        .next()
        .unwrap()
}

fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .enumerate()
        .filter(|(_, window)| {
            let mut v = vec![];
            for pos in 0..14 {
                if v.contains(&window[pos]) {
                    return false;
                }

                v.push(window[pos]);
            }

            return true;
        })
        .map(|(idx, _)| idx + 14)
        .next()
        .unwrap()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: String = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .take(1)
        .next()
        .unwrap();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

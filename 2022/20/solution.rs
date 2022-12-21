use std::io::BufRead;
use std::io;

type TInput = Vec<i64>;

fn mix(input: &TInput, times: usize, decryption_key: i64) -> TInput {
    let mut result = input.iter()
        .map(|v| v * decryption_key)
        .enumerate()
        .collect::<Vec<_>>();

    for _ in 0..times {
        for (orig_idx, _) in input.iter().enumerate() {
            let (current_idx, item) = result.iter().enumerate().filter(|(_, pair)| pair.0 == orig_idx).map(|(idx, pair)| (idx, pair.1)).next().unwrap();
            let mut new_idx = current_idx as i64 + item;
            if new_idx >= input.len() as i64 || new_idx < 0 {
                new_idx %= (input.len() as i64 - 1) as i64;
                if new_idx < 0 {
                    new_idx += input.len() as i64 - 1;
                }
            }

            let new_idx = new_idx as usize;

            result.remove(current_idx);
            result.insert(new_idx, (orig_idx, item));
        }
    }

    result.iter().map(|(_, i)| *i).collect()
}

fn part1(input: &TInput) -> i64 {
    let mixed = mix(input, 1, 1);
    
    mixed.iter()
        .cycle()
        .skip_while(|x| **x != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

fn part2(input: &TInput) -> i64 {
    let mixed = mix(input, 10, 811589153);
    
    mixed.iter()
        .cycle()
        .skip_while(|x| **x != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.parse().unwrap())
        .collect();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

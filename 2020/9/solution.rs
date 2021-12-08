use std::io::BufRead;
use std::io;

fn preamble(data: &Vec<u64>) -> usize {
    if data.len() > 25 {
        return 25;
    }

    return 5;
}

fn part1(data: &Vec<u64>) -> u64 {
    let preamble_length = preamble(data);
    
    'validate: for pos in preamble_length..data.len() {
        let item = data[pos];
        let candidates = &data[pos-preamble_length..pos];

        for a in 0..preamble_length {
            for b in 0..preamble_length {
                if a != b {
                    if candidates[a] + candidates[b] == item {
                        continue 'validate
                    }
                }
            }
        }

        return item;
    }

    panic!("No answer!")
}

fn part2(data: &Vec<u64>, target: u64) -> u64 {
    let mut start = 0;
    let mut end = 2;


    while start < end {
        let sum: u64 = data[start..end].iter().sum();
        if sum > target {
            start += 1;
        } else if sum < target {
            end += 1
        } else if sum == target {
            let smallest = data[start..end].iter().min().unwrap();
            let largest = data[start..end].iter().max().unwrap();
            return smallest + largest;
        }
    }

    panic!("No answer");
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let data: Vec<u64> = reader.lines().map(|line| {
        let line = line.expect("couldn't read stdin");
        line.parse().unwrap()
	}).collect();

	let answer1 = part1(&data);
	let answer2 = part2(&data, answer1);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

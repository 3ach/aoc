use std::io::BufRead;
use std::io;
use std::cmp;
use std::collections::HashSet;
use std::convert::TryInto;

fn step(octopodes: &mut Vec<Vec<u8>>) -> usize {
    let mut flashed: HashSet<(isize, isize)> = HashSet::new();
    let mut energized: Vec<(isize, isize)> = vec![];

    for row in 0..octopodes.len() {
        for col in 0..octopodes.len() {
            energized.push((row as isize, col as isize));
        }
    }

    while energized.len() > 0 {
        let (row, col) = energized.remove(0);
        if row < 0 || row >= octopodes.len() as isize || col < 0 || col >= octopodes[0].len() as isize {
            continue;
        }
        
        if flashed.contains(&(row, col)) {
            continue;
        }

        octopodes[row as usize][col as usize] += 1;

        if octopodes[row as usize][col as usize] > 9 {
            flashed.insert((row, col));

            for rdiff in -1..=1 {
                for cdiff in -1..=1 {
                    if rdiff == 0 && cdiff == 0 {
                        continue
                    }

                    if flashed.contains(&(row + rdiff, col + cdiff)) {
                        continue
                    }

                    energized.push((row + rdiff, col + cdiff));
                }
            }
        }
    }

    for (row, col) in &flashed {
        octopodes[*row as usize][*col as usize] = 0;
    }

    return flashed.len();
}

fn part1(octopodes: &Vec<Vec<u8>>) -> usize {
    let mut flashes = 0;
    let mut octopodes = octopodes.clone();

    for _ in 0..100 {
        flashes += step(&mut octopodes);
    }

    return flashes;
}

fn part2(octopodes: &Vec<Vec<u8>>) -> u32 {
    let mut octopodes = octopodes.clone();
    let target = octopodes.len() * octopodes[0].len();
    let mut steps = 1;

    while step(&mut octopodes) != target {
        steps += 1;
    }

    return steps;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let octopodes: Vec<Vec<u8>> = reader.lines()
        .map(|line| line.expect("Can't read stdin")
             .chars()
             .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
             .collect())
        .collect();

	let answer1 = part1(&octopodes);
	let answer2 = part2(&octopodes);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

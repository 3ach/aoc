use std::io::BufRead;
use std::io;
use std::cmp;
use std::collections::HashSet;
use std::convert::TryInto;

struct Octopus<'a> {
    energy: u8,
    neighbors: [Option<&'a Octopus<'a>>; 8],
    last_flash: usize,
}

fn part1(octopodes: &Vec<Vec<u8>>) -> u32 {
    let mut flashes = 0;
    let mut octopodes = octopodes.clone();

    for step in 0..100 {
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

        flashes += flashed.len() as u32;

        for (row, col) in flashed {
            octopodes[row as usize][col as usize] = 0;
        }
    }

    return flashes;
}

fn print_energies(octopodes: &Vec<Vec<u8>>) {
    for r in 0..octopodes.len() {
        for c in 0..octopodes[0].len() {
            if octopodes[r][c] > 0 {
                print!("{:x}", octopodes[r][c]);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn part2(octopodes: &Vec<Vec<u8>>) -> u32 {
    let target = octopodes.len() * octopodes[0].len();
    let mut octopodes = octopodes.clone();
    let mut step = 0;

    loop {
        step += 1;
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


        if flashed.len() == target {
            return step
        }

        for (row, col) in flashed {
            octopodes[row as usize][col as usize] = 0;
        }
    }
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

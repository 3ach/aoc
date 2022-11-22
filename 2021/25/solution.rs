use std::io::BufRead;
use std::io;
use std::fmt::{Debug,Formatter,Result};


#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    East,
    South
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::East => write!(f, ">"),
            Cell::South => write!(f, "v")
        }
    }
}

fn print_map(cucumbers: &[Vec<Cell>]) {
    for row in cucumbers {
        for cell in row {
            print!("{:?}", cell);
        }
        println!();
    }
}

fn part1(cucumbers: &[Vec<Cell>]) -> usize {
    let mut runs = 1;
    let mut current = cucumbers.to_vec();

    loop {
        let mut moved = false;
        let mut next = vec![];

        for row in &current {
            let mut next_row = vec![];
            let mut extra = false;
            for idx in 0..row.len() {
                let next_idx = (idx + 1) % row.len();
                if row[idx] == Cell::East && row[next_idx] == Cell::Empty {
                    moved = true;
                    next_row.push(Cell::Empty);
                    if next_idx == 0 {
                        next_row[0] = Cell::East;
                    } else {
                        extra = true;
                        next_row.push(Cell::East);
                    }
                } else {
                    if !extra {
                        next_row.push(row[idx]);
                    } else {
                        extra = false;
                    }
                }
            }

            next.push(next_row);
        }

        let mut to_south = vec![];
        for col in 0..next[0].len() {
            let mut skip = false;
            for row in 0..next.len() {
                let next_row = (row + 1) % next.len();
                if next[row][col] == Cell::South && next[next_row][col] == Cell::Empty {
                    to_south.push((row, col));
                    moved = true;
                }
            }
        }

        for (row, col) in to_south {
            let next_row = (row + 1) % next.len();
            next[next_row][col] = Cell::South;   
            next[row][col] = Cell::Empty;   
        }

        if !moved {
            break;
        }

        current = next;
        runs += 1;
    }

    runs
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let cucumbers: Vec<Vec<Cell>> = reader.lines().map(|line| {
		line.expect("Couldn't read stdin")
            .chars()
            .map(|c| {
                match c {
                    '.' => Cell::Empty,
                    'v' => Cell::South,
                    '>' => Cell::East,
                    _ => panic!()
                }
        })
        .collect()
	}).collect();

	let answer1 = part1(&cucumbers[..]);

	println!("Answer 1: {}", answer1);

    Ok(())
}

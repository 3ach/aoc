use std::io::BufRead;
use std::io;
use std::collections::{HashMap, HashSet, VecDeque};

type TInput = HashMap<(usize, usize), u8>;

fn part1(input: &TInput, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut todo = VecDeque::from([(start, 0)]);
    let mut hit = HashSet::new();

    while let Some((point, path)) = todo.pop_front() {
        if point == end {
            return path;
        }

        if hit.contains(&point) {
            continue;
        }

        hit.insert(point);

        let elevation = *input.get(&point).unwrap();
        let (row, col) = point;
        if let Some(next_elevation) = input.get(&(row - 1, col)) {
            if *next_elevation <= elevation || *next_elevation - 1 == elevation {
                todo.push_back(((row - 1, col), path + 1));
            }
        }

        if let Some(next_elevation) = input.get(&(row + 1, col)) {
            if *next_elevation <= elevation || *next_elevation - 1 == elevation {
                todo.push_back(((row + 1, col), path + 1));
            }
        }

        if let Some(next_elevation) = input.get(&(row, col - 1)) {
            if *next_elevation <= elevation || *next_elevation - 1 == elevation {
                todo.push_back(((row, col - 1), path + 1));
            }
        }
        if let Some(next_elevation) = input.get(&(row, col + 1)) {
            if *next_elevation <= elevation || *next_elevation - 1 == elevation {
                todo.push_back(((row, col + 1), path + 1));
            }
        }
    }

    panic!("Couldn't get to the end!");
}

fn part2(input: &TInput, end: (usize, usize)) -> usize {
    let mut todo = VecDeque::from([(end, 0)]);
    let mut hit = HashSet::new();

    while let Some((point, path)) = todo.pop_front() {
        let elevation = *input.get(&point).unwrap();
        if elevation == 0 {
            return path;
        }

        if hit.contains(&point) {
            continue;
        }

        hit.insert(point);

        let (row, col) = point;
        if let Some(next_elevation) = input.get(&(row - 1, col)) {
            if *next_elevation >= elevation || *next_elevation + 1 == elevation {
                todo.push_back(((row - 1, col), path + 1));
            }
        }

        if let Some(next_elevation) = input.get(&(row + 1, col)) {
            if *next_elevation >= elevation || *next_elevation + 1 == elevation {
                todo.push_back(((row + 1, col), path + 1));
            }
        }

        if let Some(next_elevation) = input.get(&(row, col - 1)) {
            if *next_elevation >= elevation || *next_elevation + 1 == elevation {
                todo.push_back(((row, col - 1), path + 1));
            }
        }
        if let Some(next_elevation) = input.get(&(row, col + 1)) {
            if *next_elevation >= elevation || *next_elevation + 1 == elevation {
                todo.push_back(((row, col + 1), path + 1));
            }
        }
    }

    panic!("Couldn't get to the end!");
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .enumerate()
        .map(|(r, row)| {
            row.chars()
             .enumerate()
             .map(|(c, mut space)| {
                 if space == 'S' {
                    space = 'a';
                    start = (r, c);
                 } else if space == 'E' {
                     space = 'z';
                     end = (r, c);
                }
                
                 ((r, c), space as u8 - 'a' as u8)
             }).collect::<Vec<((usize, usize), u8)>>()
        }).flatten()
        .collect();

	let answer1 = part1(&input, start, end);
	let answer2 = part2(&input, end);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

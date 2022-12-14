use std::io::BufRead;
use std::io;
use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Rock,
    Sand
}

type Point = (usize, usize);

type TInput = HashMap<Point, Tile>;

fn drop(cave: &TInput, lowest: usize, floor: bool) -> Option<Point> {
    let mut sand = (500, 0);
   
    loop {
        if floor {
            if sand.1 == lowest {
                return Some(sand);
            }
        } else {
            if sand.1 > lowest {
                return None;
            }
        }

        if !cave.contains_key(&(sand.0, sand.1 + 1)) {
            sand = (sand.0, sand.1 + 1);
        } else if !cave.contains_key(&(sand.0 - 1, sand.1 + 1)) {
            sand = (sand.0 - 1, sand.1 + 1);
        } else if !cave.contains_key(&(sand.0 + 1, sand.1 + 1)) {
            sand = (sand.0 + 1, sand.1 + 1);
        } else {
            break;
        }
    }

    if floor && sand == (500, 0) {
        None
    } else {
        Some(sand)
    }
}

fn part1(input: &TInput) -> usize {
    let mut input = input.clone();
    let lowest = *input.keys()
        .map(|(_, y)| y)
        .max()
        .unwrap();

    let mut sands = 0;

    while let Some(sand) = drop(&input, lowest, false) {
        input.insert(sand, Tile::Sand);
        sands += 1;
    }

    sands
}

fn part2(input: &TInput) -> u32 {
    let mut input = input.clone();
    let lowest = *input.keys()
        .map(|(_, y)| y)
        .max()
        .unwrap();

    let mut sands = 0;

    while let Some(sand) = drop(&input, lowest + 1, true) {
        input.insert(sand, Tile::Sand);
        sands += 1;
    }

    sands + 1
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.split(" -> ")
             .map(|pair| pair.split_once(",").unwrap())
             .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
             .fold(vec![], |mut so_far: Vec<Point>, point| {
                 let mut line = vec![];
                 let mut reverse = false;

                 if let Some(last) = so_far.last() {
                     if last.0 == point.0 {
                         for y in min(last.1, point.1)..=max(last.1, point.1) {
                            line.push((last.0, y));
                         }

                        reverse = last.1 > point.1
                     } else if last.1 == point.1 {
                         for x in min(last.0, point.0)..=max(last.0, point.0) {
                            line.push((x, last.1));
                         }

                        reverse = last.0 > point.0
                     } else {
                         panic!("DIAGONAL! {:?} -> {:?}", last, point);
                     }
                 } else {
                     so_far.push(point);
                 }

                 if reverse {
                     line.reverse();
                 }

                 so_far.append(&mut line);
                 so_far
             })
             .iter()
             .map(|point| (*point, Tile::Rock))
             .collect::<Vec<(Point, Tile)>>()
        )
        .flatten()
        .collect();

	let answer1 = part1(&input);
	println!("Answer 1: {}", answer1);

	let answer2 = part2(&input);
	println!("Answer 2: {}", answer2);

    Ok(())
}

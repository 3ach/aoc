use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

type Elf = (isize, isize);
type TInput = HashSet<Elf>;

fn run(map: &TInput, round: usize) -> TInput {
    let mut proposed: HashMap<Elf, Vec<Elf>> = HashMap::new();

    for elf in map {
        let north = HashSet::from([(elf.0 - 1, elf.1 - 1), (elf.0, elf.1 - 1), (elf.0 + 1, elf.1 - 1)]);
        let south = HashSet::from([(elf.0 - 1, elf.1 + 1), (elf.0, elf.1 + 1), (elf.0 + 1, elf.1 + 1)]);
        let east = HashSet::from([(elf.0 + 1, elf.1 - 1), (elf.0 + 1, elf.1), (elf.0 + 1, elf.1 + 1)]);
        let west = HashSet::from([(elf.0 - 1, elf.1 - 1), (elf.0 - 1, elf.1), (elf.0 - 1, elf.1 + 1)]);

        let north_neighbors = north.intersection(&map).count();
        let south_neighbors = south.intersection(&map).count();
        let east_neighbors = east.intersection(&map).count();
        let west_neighbors = west.intersection(&map).count();

        let directions = [north, south, west, east];
        let neighbors = [north_neighbors, south_neighbors, west_neighbors, east_neighbors];

        if north_neighbors + south_neighbors + east_neighbors + west_neighbors == 0 {
            proposed.entry(elf.clone()).or_insert(vec![]).push(elf.clone());
            continue;
        }

        let mut moved = false;
        for attempt in 0..4 {
            let idx = (round + attempt) % 4;

            if neighbors[idx] == 0 {
                moved = true;
                let next = directions[idx].iter().filter(|(x, y)| *x == elf.0 || *y == elf.1).next().unwrap();
                let entry = proposed.entry(next.clone()).or_insert(vec![]).push(elf.clone());
                break;
            }
        }
        
        if !moved {
            proposed.entry(elf.clone()).or_insert(vec![]).push(elf.clone());
        }
    }

    proposed.iter()
        .map(|(proposed, elves)| if elves.len() == 1 { vec![*proposed] } else { elves.clone() })
        .flatten()
        .collect()
}

fn part1(input: &TInput) -> usize {
    let mut map = input.clone();

    for round in 0..10 {
        map = run(&map, round);
    }

    let xmin = *map.iter().map(|(x, _)| x).min().unwrap();
    let xmax = *map.iter().map(|(x, _)| x).max().unwrap() + 1;
    let ymin = *map.iter().map(|(_, y)| y).min().unwrap();
    let ymax = *map.iter().map(|(_, y)| y).max().unwrap() + 1;

    (xmin.abs_diff(xmax) * ymin.abs_diff(ymax)) - map.len()
}

fn part2(input: &TInput) -> usize {
    let mut map = input.clone();
    let mut round = 0;

    loop {
        let next = run(&map, round);
        if next.intersection(&map).count() == map.len() {
            return round + 1;
        }

        map = next;
        round += 1;
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .enumerate()
        .map(|(ridx, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(cidx, c)| if c == '#' { Some((cidx as isize, ridx as isize)) } else { None })
                .collect::<HashSet<Elf>>()
        })
        .flatten()
        .collect();

    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

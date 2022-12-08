use std::io::BufRead;
use std::io;
use std::collections::{HashMap, HashSet};

type TInput = Vec<Vec<u32>>;

fn can_see(tree: u32, tallest: Option<u32>) -> bool {
    match tallest {
        None => true,
        Some(prev) if tree > prev => true,
        _ => false,
    }
}

fn tallest(input: &TInput) -> HashSet<(usize, usize)> {
    let mut visible = HashSet::new();

    for ridx in 0..input.len() {
        let mut tallest = None;

        for cidx in 0..input[0].len() {
            let tree = input[ridx][cidx];
            if can_see(tree, tallest) {
                tallest = Some(tree);
                visible.insert((ridx, cidx));
            }
        }

        tallest = None;
        for cidx in (0..input[0].len()).rev() {
            let tree = input[ridx][cidx];
            if can_see(tree, tallest) {
                tallest = Some(tree);
                visible.insert((ridx, cidx));
            }
        }
    }

    for cidx in 0..input[0].len() {
        let mut tallest = None;
        for ridx in 0..input.len() {
            let tree = input[ridx][cidx];
            if can_see(tree, tallest) {
                tallest = Some(tree);
                visible.insert((ridx, cidx));
            }
        }

        tallest = None;
        for ridx in (0..input.len()).rev() {
            let tree = input[ridx][cidx];
            if can_see(tree, tallest) {
                tallest = Some(tree);
                visible.insert((ridx, cidx));
            }
        }
    }

    visible
}

fn part1(input: &TInput) -> usize {
    tallest(input).len()
}

fn part2(input: &TInput) -> usize {
    let mut max = 0;
    let tallest = tallest(input);
    for (ridx, cidx) in tallest {
        let height = input[ridx][cidx];
        let up = ridx - (0..ridx).rev()
            .filter(|r| input[*r][cidx] >= height)
            .take(1)
            .next()
            .unwrap_or(0) as usize;

        let down = (ridx+1..input.len())
            .filter(|r| input[*r][cidx] >= height)
            .take(1)
            .next()
            .unwrap_or(input.len() - 1) - ridx;

        let left = ((cidx as isize) - (0..cidx).rev()
            .filter(|c| input[ridx][*c] >= height)
            .map(|r| r as isize)
            .take(1)
            .next()
            .unwrap_or(0)) as usize;

        let right = (cidx+1..input[0].len())
            .filter(|c| input[ridx][*c] >= height)
            .take(1)
            .next()
            .unwrap_or(input.len() - 1) - cidx;

        let score = up * down * left * right;
        if score > max {
            max = score
        }
    }

    max
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.chars().map(|c| c as u32 - '0' as u32).collect())
        .collect();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

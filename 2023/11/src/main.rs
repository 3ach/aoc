use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

struct Starmap {
    rows: usize,
    cols: usize,
    galaxies: HashSet<(usize, usize)>,
}

fn expand(input: &Starmap, amount: usize) -> usize {
    let empty_rows: HashSet<usize> = (0..input.rows)
        .filter(|r| !input.galaxies.iter().any(|(row, _)| row == r))
        .collect();
    let empty_cols: HashSet<usize> = (0..input.cols)
        .filter(|c| !input.galaxies.iter().any(|(_, col)| col == c))
        .collect();

    input
        .galaxies
        .iter()
        .map(|start| {
            input
                .galaxies
                .iter()
                .map(|end| {
                    if end == start {
                        0
                    } else {
                        start.0.abs_diff(end.0)
                            + start.1.abs_diff(end.1)
                            + empty_cols
                                .iter()
                                .filter(|c| **c > min(start.1, end.1) && **c < max(start.1, end.1))
                                .count() * (amount - 1)
                            + empty_rows
                                .iter()
                                .filter(|r| **r > min(start.0, end.0) && **r < max(start.0, end.0))
                                .count() * (amount - 1)
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>() / 2
}

fn part1(input: &Starmap) -> usize {
    expand(input, 2)
}

fn part2(input: &Starmap) -> usize {
    expand(input, 1000000)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut rows = 0;
    let mut cols = 0;

    let galaxies = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .enumerate()
        .map(|(r, line)| {
            if r >= rows {
                rows = r + 1;
            }

            if line.len() > cols {
                cols = line.len();
            }

            line.chars()
                .enumerate()
                .filter(|(_, pt)| *pt == '#')
                .map(move |(c, _)| (r, c))
                .collect::<HashSet<(usize, usize)>>()
        })
        .flatten()
        .collect::<HashSet<(usize, usize)>>();

    let mut starmap = Starmap {
        rows,
        cols,
        galaxies,
    };

    let answer1 = part1(&starmap);
    let answer2 = part2(&starmap);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

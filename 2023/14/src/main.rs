use std::collections::BTreeSet;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

type Point = (usize, usize);
type Rockset = BTreeSet<Point>;

fn north(rounds: &Rockset, cubes: &Rockset) -> Rockset {
    let mut end = BTreeSet::new();

    for (mut row, col) in rounds {
        while row > 0 && !end.contains(&(row - 1, *col)) && !cubes.contains(&(row - 1, *col)) {
            row -= 1;
        }

        end.insert((row, *col));
    }

    end
}

fn west(rounds: &Rockset, cubes: &Rockset) -> Rockset {
    let mut end = BTreeSet::new();

    for (row, mut col) in rounds {
        while col > 0 && !end.contains(&(*row, col - 1)) && !cubes.contains(&(*row, col - 1)) {
            col -= 1;
        }

        end.insert((*row, col));
    }

    end
}

fn east(rounds: &Rockset, cubes: &Rockset, col_max: usize) -> Rockset {
    let mut end = BTreeSet::new();

    for (row, mut col) in rounds.iter().rev() {
        while col < col_max && !end.contains(&(*row, col + 1)) && !cubes.contains(&(*row, col + 1)) {
            col += 1;
        }

        end.insert((*row, col));
    }

    end
}

fn south(rounds: &Rockset, cubes: &Rockset, row_max: usize) -> Rockset {
    let mut end = BTreeSet::new();

    for (mut row, col) in rounds.iter().rev() {
        while row < row_max && !end.contains(&(row + 1, *col)) && !cubes.contains(&(row + 1, *col)) {
            row += 1;
        }

        end.insert((row, *col));
    }

    end
}

fn part1(rounds: &Rockset, cubes: &Rockset) -> usize {
    let end = north(rounds, cubes);
    let row_max = rounds.iter().map(|(row, _)| row).max().unwrap();

    end.iter().map(|(row, _)| row_max - row + 1).sum()
}

fn part2(rounds: &Rockset, cubes: &Rockset) -> usize {
    let row_max = *rounds.iter().map(|(row, _)| row).max().unwrap();
    let col_max = *rounds.iter().map(|(_, col)| col).max().unwrap();

    let mut encountered = HashMap::new();
    let mut current = rounds.clone();
    let mut round = 0;
    let target = 1000000000;

    while round < target {
        let repr = format!("{:?}", current.iter().collect::<Vec<&Point>>());
        if let Some(cycle) = encountered.get(&repr) {
            let period = round - cycle;
            round += ((target - round) / period) * period;
        } else { 
            encountered.insert(repr, round);
        }

        current = north(&current, cubes);
        current = west(&current, cubes);
        current = south(&current, cubes, row_max);
        current = east(&current, cubes, col_max);
        round += 1;
    }

    current.iter().map(|(row, _)| row_max - row + 1).sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let (cubes, rounds): (BTreeSet<(Point, char)>, BTreeSet<(Point, char)>) = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, cell)| ((row, col), cell))
                .collect::<BTreeSet<(Point, char)>>()
        })
        .flatten()
        .filter(|(_, cell)| *cell != '.')
        .partition(|(_, cell)| *cell == '#');

    let cubes = cubes.iter().map(|(pt, _)| pt).cloned().collect();
    let rounds = rounds.iter().map(|(pt, _)| pt).cloned().collect();

    let answer1 = part1(&rounds, &cubes);
    println!("Answer 1: {}", answer1);
    let answer2 = part2(&rounds, &cubes);
    println!("Answer 2: {}", answer2);

    Ok(())
}

use adventage::{day, part1demo, part2demo};
use std::collections::HashSet;

day!(2025, 4);
part1demo!("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.", 13);
part2demo!("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.", 43);

type Point = (isize, isize);
type TInput = HashSet<Point>;

fn parse(input: &str) -> TInput {
    input.lines()
        .into_iter()
        .enumerate()
        .map(|(row, line)|
            line.chars()
                .into_iter()
                .enumerate()
                .filter_map(move |(col, c)| 
                    match c {
                        '@' => Some((col as isize, row as isize)),
                        _ => None,
                    }
                )
        ).flatten()
        .collect()
}

fn neighbors(point: &Point) -> [Point; 8] {
    [
        (point.0 - 1, point.1 - 1),
        (point.0, point.1 - 1),
        (point.0 + 1, point.1 - 1),
        (point.0 - 1, point.1),
        (point.0 + 1, point.1),
        (point.0 - 1, point.1 + 1),
        (point.0, point.1 + 1),
        (point.0 + 1, point.1 + 1),
    ]
}

fn accessible(input: &TInput) -> TInput {
    input.iter()
        .filter(|roll| 
            neighbors(roll).iter()
                .filter(|neighbor| input.contains(&neighbor))
                .count() < 4
        ).cloned()
        .collect()
}

fn part1(input: &TInput) -> usize {
    accessible(input).len()
}

fn part2(input: &TInput) -> usize {
    let mut removed = HashSet::new();
    let mut map = input.clone();

    loop {
        let removable = accessible(&map);
        if removable.is_empty() {
            break;
        }

        for point in removable {
            map.remove(&point);
            removed.insert(point);
        }
    }

    removed.len()
}

use adventage::{day, part1demo, part2demo};
use std::collections::{HashMap, HashSet};

day!(2024, 8);
part1demo!(
    "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    14
);
part1demo!(
    "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........",
    2
);
part2demo!(
    "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    34
);

enum Space {
    Node(char),
    Open,
}
type Point = (isize, isize);
type TInput = HashMap<Point, Space>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate().map(move |(col, c)| match c {
                '.' => ((col as isize, row as isize), Space::Open),
                _ => ((col as isize, row as isize), Space::Node(c)),
            })
        })
        .flatten()
        .collect()
}

fn antinodes(map: &TInput, count: isize) -> HashSet<Point> {
    let x_max = *map.keys().map(|(col, _)| col).max().unwrap();
    let y_max = *map.keys().map(|(_, row)| row).max().unwrap();

    let range = if count == 1 { 1..=1 } else { 0..=count };

    map.iter()
        .filter_map(|(pos, tile)| match tile {
            Space::Open => None,
            Space::Node(c) => Some((pos, c)),
        })
        .map(|(pos, c)| {
            map.iter()
                .filter_map(move |(other, tile)| match tile {
                    Space::Node(o) if c == o && other != pos => {
                        Some((other.0 - pos.0, other.1 - pos.1))
                    }
                    _ => None,
                })
                .map(|delta| {
                    range
                        .clone()
                        .map(move |d| (pos.0 - d * delta.0, pos.1 - d * delta.1))
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|(x, y)| *x >= 0 && *x <= x_max && *y >= 0 && *y <= y_max)
        .collect()
}

fn part1(map: &TInput) -> usize {
    antinodes(map, 1).len()
}

fn part2(map: &TInput) -> usize {
    antinodes(map, 100).len()
}

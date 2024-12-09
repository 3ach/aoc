use adventage::{day, part1demo, part2demo};
use std::collections::{HashMap, HashSet};

day!(2024, 6);
part1demo!(
    "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
    41
);
part2demo!(
    "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
    6
);

type Point = (isize, isize);

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
enum Tile {
    Open,
    Taken,
}

fn step(pos: Point, dir: &Direction) -> Point {
    match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn turn(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

type TInput = (HashMap<Point, Tile>, Point);

fn parse(input: &str) -> TInput {
    let mut start = None;

    let map = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(col, c)| match c {
                    '^' => {
                        start = Some((col as isize, row as isize));
                        Some(((col as isize, row as isize), Tile::Open))
                    }
                    '#' => Some(((col as isize, row as isize), Tile::Taken)),
                    '.' => Some(((col as isize, row as isize), Tile::Open)),
                    _ => None,
                })
                .collect::<Vec<(Point, Tile)>>()
        })
        .flatten()
        .collect();

    (map, start.unwrap())
}

fn explore(map: &HashMap<Point, Tile>, start: Point) -> Option<HashSet<Point>> {
    let mut visited = HashSet::new();
    let mut current = start;
    let mut direction = Direction::Up;

    while map.contains_key(&current) {
        let next = step(current, &direction);
        if !visited.insert((direction, current)) {
            return None;
        }

        match map.get(&next) {
            Some(Tile::Open) | None => current = next,
            _ => direction = turn(&direction),
        }
    }

    Some(visited.iter().map(|(_, p)| *p).collect())
}

fn part1((map, start): &TInput) -> usize {
    explore(map, *start).unwrap().len()
}

fn part2((map, start): &TInput) -> usize {
    let mut map = map.clone();
    explore(&map, *start)
        .unwrap()
        .iter()
        .filter(|point| {
            map.insert(**point, Tile::Taken);
            let looped = explore(&map, *start).is_none();
            map.insert(**point, Tile::Open);
            looped
        }).count()
}

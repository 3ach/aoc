use adventage::{part1demo, part2demo, day};
use std::collections::HashSet;

part1demo!("R2, L3", 5);
part1demo!("R2, R2, R2", 2);
part1demo!("R5, L5, R5, R3", 12);
part2demo!("R8, R4, R4, R8", 4);

day!(2016, 1);

#[derive(Clone, Copy)]
enum Turn {
    Right,
    Left,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

type Point = (i32, i32);

fn turn(d: Direction, t: Turn) -> Direction {
    match (d, t) {
        (Direction::North, Turn::Right) => Direction::East,
        (Direction::East, Turn::Right) => Direction::South,
        (Direction::South, Turn::Right) => Direction::West,
        (Direction::West, Turn::Right) => Direction::North,
        (Direction::North, Turn::Left) => Direction::West,
        (Direction::West, Turn::Left) => Direction::South,
        (Direction::South, Turn::Left) => Direction::East,
        (Direction::East, Turn::Left) => Direction::North,
    }
}

fn advance(p: Point, d: Direction, s: i32) -> Point {
    match d {
        Direction::North => (p.0, p.1 + s),
        Direction::East => (p.0 + s, p.1),
        Direction::South => (p.0, p.1 - s),
        Direction::West => (p.0 - s, p.1),
    }
}


fn parse(input: &str) -> Vec<(Turn, i32)> {
    input
        .lines()
        .map(|l| 
            l.split(", ").map(|s| (match s.chars().next().unwrap() {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => panic!(),
        }, s.chars().skip(1).collect::<String>().parse::<i32>().unwrap())))
        .flatten()
        .collect()
}

fn part1(steps: &Vec<(Turn, i32)>) -> u32 {
    let mut direction = Direction::North;
    let mut pos = (0, 0);

    for step in steps {
        direction = turn(direction, step.0);
        pos = advance(pos, direction, step.1);
    }

    (pos.1.abs() + pos.0.abs()) as u32
}

fn part2(steps: &Vec<(Turn, i32)>) -> u32 {
    let mut direction = Direction::North;
    let mut pos = (0, 0);
    let mut seen = HashSet::new();

    for step in steps {
        direction = turn(direction, step.0);

        for _ in 0..step.1 {
            if !seen.insert(pos) {
                break;
            }
            pos = advance(pos, direction, 1);
        }
    }

    (pos.1.abs() + pos.0.abs()) as u32
}

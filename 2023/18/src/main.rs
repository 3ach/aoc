#![feature(iter_map_windows)]

use adventage::{part1demo, part2demo, day};

part1demo!("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)", 62);

part2demo!("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)", 952408144115);

day!(2023, 18);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct DigStep {
    dir: Direction,
    len: isize, 
    real_len: isize, 
    real_dir: Direction
}

fn parse(input: &str) -> Vec<DigStep> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let direction = match parts[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!(),
            };
            
            let distance = parts[1].parse::<isize>().unwrap();
            let color = &parts[2][2..8];
            let real_len = color[0..5].chars().fold(0, |acc, pl| (acc * 16) + pl.to_digit(16).unwrap()) as isize;

            let real_dir = match color.chars().last().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!(),
            };

            DigStep { dir: direction, len: distance, real_len: real_len, real_dir: real_dir }
        })
        .collect()
}

fn part1(plan: &Vec<DigStep>) -> isize {
    let mut current = (0, 0);
    let mut path = vec![];
    let mut len = 0;

    for step in plan {
        path.push(current);
        len += step.len;

        current = match step.dir {
            Direction::Up => (current.0, current.1 - step.len),
            Direction::Down => (current.0, current.1 + step.len),
            Direction::Left => (current.0 - step.len, current.1),
            Direction::Right => (current.0 + step.len, current.1),
        };
    }

    let interior = path.iter()
        .map_windows(|[p, n]| (p.0 + n.0) * (p.1 - n.1))
        .sum::<isize>().abs() / 2;

    interior + 1 - (len / 2) + len
}

fn part2(plan: &Vec<DigStep>) -> isize {
    let mut current = (0, 0);
    let mut path = vec![];
    let mut len = 0;

    for step in plan {
        path.push(current);
        len += step.real_len;

        current = match step.real_dir {
            Direction::Up => (current.0, current.1 - step.real_len),
            Direction::Down => (current.0, current.1 + step.real_len),
            Direction::Left => (current.0 - step.real_len, current.1),
            Direction::Right => (current.0 + step.real_len, current.1),
        };
    }

    let interior = path.iter()
        .map_windows(|[p, n]| (p.0 + n.0) * (p.1 - n.1))
        .sum::<isize>().abs() / 2;

    interior + 1 - (len / 2) + len
}

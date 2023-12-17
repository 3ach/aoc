use adventage::{part1demo, part2demo, day};
use std::collections::HashSet;

part1demo!(">", 2);
part1demo!("^>v<", 4);
part1demo!("^v^v^v^v^v", 2);

part2demo!("^v", 3);
part2demo!("^>v<", 3);
part2demo!("^v^v^v^v^v", 11);

day!(2015, 3);

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!(),
        })
        .collect()
}

fn part1(map: &Vec<Direction>) -> u32 {
    let mut current = (0isize, 0isize);
    let mut seen = HashSet::new();

    seen.insert(current);

    for step in map {
        current = match step {
            Direction::Up => (current.0, current.1 + 1),
            Direction::Down => (current.0, current.1 - 1),
            Direction::Left => (current.0 - 1, current.1),
            Direction::Right => (current.0 + 1, current.1),
        };

        seen.insert(current);
    }


    seen.len() as u32
}

fn part2(map: &Vec<Direction>) -> u32 {
    let mut real_current = (0isize, 0isize);
    let mut robo_current = (0isize, 0isize);
    let mut seen = HashSet::new();

    seen.insert(real_current);

    for (idx, step) in map.iter().enumerate() {
        let current = if idx % 2 == 0 {
            &mut real_current
        } else {
            &mut robo_current
        };

        *current = match step {
            Direction::Up => (current.0, current.1 + 1),
            Direction::Down => (current.0, current.1 - 1),
            Direction::Left => (current.0 - 1, current.1),
            Direction::Right => (current.0 + 1, current.1),
        };

        seen.insert(*current);
    }


    seen.len() as u32
}

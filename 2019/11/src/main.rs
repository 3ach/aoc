extern crate intcode;
use intcode::{enter, init, Program};
use std::io;
use std::io::BufRead;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(program: &Program) -> usize {
    let mut painted = HashSet::new();
    let mut ever_painted = HashSet::new();
    let mut current: (i64, i64) = (0, 0);
    let mut direction = Direction::Up;

    let mut program = program.clone();
    let mut done = false;
    let mut output = vec![];
    let mut execution = &mut init(&mut program, &[]);

    while !done {
        let color = match painted.contains(&current) {
            true => 1,
            false => 0,
        };

        (done, output, execution) = enter(execution, &[color]);
        
        if output[0] == 1 {
            painted.insert(current);
        } else {
            painted.remove(&current);
        }

        ever_painted.insert(current);

        (current, direction) = match (direction, output[1]) {
            (Direction::Up, 1) => ((current.0 + 1, current.1), Direction::Right),
            (Direction::Right, 1) => ((current.0, current.1 - 1), Direction::Down),
            (Direction::Down, 1) => ((current.0 - 1, current.1), Direction::Left),
            (Direction::Left, 1) => ((current.0, current.1 + 1), Direction::Up),

            (Direction::Up, 0) => ((current.0 - 1, current.1), Direction::Left),
            (Direction::Left, 0) => ((current.0, current.1 - 1), Direction::Down),
            (Direction::Down, 0) => ((current.0 + 1, current.1), Direction::Right),
            (Direction::Right, 0) => ((current.0, current.1 + 1), Direction::Up),
           
            _ => panic!(),
        };
    }

    ever_painted.len()
}

fn part2(program: &Program) {
    let mut painted = HashSet::new();
    let mut current: (i64, i64) = (0, 0);
    let mut direction = Direction::Up;

    let mut program = program.clone();
    let mut done = false;
    let mut output = vec![];
    let mut execution = &mut init(&mut program, &[]);

    painted.insert(current);

    while !done {
        let color = match painted.contains(&current) {
            true => 1,
            false => 0,
        };

        (done, output, execution) = enter(execution, &[color]);
        
        if output[0] == 1 {
            painted.insert(current);
        } else {
            painted.remove(&current);
        }

        (current, direction) = match (direction, output[1]) {
            (Direction::Up, 1) => ((current.0 + 1, current.1), Direction::Right),
            (Direction::Right, 1) => ((current.0, current.1 - 1), Direction::Down),
            (Direction::Down, 1) => ((current.0 - 1, current.1), Direction::Left),
            (Direction::Left, 1) => ((current.0, current.1 + 1), Direction::Up),

            (Direction::Up, 0) => ((current.0 - 1, current.1), Direction::Left),
            (Direction::Left, 0) => ((current.0, current.1 - 1), Direction::Down),
            (Direction::Down, 0) => ((current.0 + 1, current.1), Direction::Right),
            (Direction::Right, 0) => ((current.0, current.1 + 1), Direction::Up),
           
            _ => panic!(),
        };
    }

    let xmin = *painted.iter().map(|(x, _)| x).min().unwrap();
    let xmax = *painted.iter().map(|(x, _)| x).max().unwrap();
    let ymin = *painted.iter().map(|(_, y)| y).min().unwrap();
    let ymax = *painted.iter().map(|(_, y)| y).max().unwrap();

    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            if painted.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let program: Program = {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        buf.trim()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    };

    let answer1 = part1(&program);
    println!("Part 1: {}", answer1);

    part2(&program);

    Ok(())
}

extern crate intcode;
use intcode::{run, init, enter, Program};
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

fn part1(program: &Program) -> usize {
    let output = run(program, &[]);
    let mut map = HashMap::new();
   
    for description in output.chunks(3) {
        let x = description[0]; 
        let y = description[1]; 
        let tile = match description[2] {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!(),
        };

        map.insert((x, y), tile);
    }

    map.iter().filter(|(_, tile)| **tile == Tile::Block).count()
}

fn part2(program: &Program) -> i64 {
    let mut program = program.clone();
    program[0] = 2;

    let mut exec = &mut init(&program, &[]);
    let mut done = false;
    let mut output = vec![];
    let mut map = HashMap::new();
    let mut score = 0;

    (done, output, exec) = enter(exec, &[]);

    while !done || !output.is_empty() {
        for description in output.chunks(3) {
            let x = description[0]; 
            let y = description[1]; 
            let tile = match description[2] {
                0 => Some(Tile::Empty),
                1 => Some(Tile::Wall),
                2 => Some(Tile::Block),
                3 => Some(Tile::Paddle),
                4 => Some(Tile::Ball),
                _ if x == -1 && y == 0 => { score = description[2]; None },
                _ => panic!(),
            };

            if let Some(tile) = tile {
                map.insert((x, y), tile);
            }
        }

        let ymin = *map.iter().map(|((_, y), _)| y).min().unwrap();
        let ymax = *map.iter().map(|((_, y), _)| y).max().unwrap();
        let xmin = *map.iter().map(|((x, _), _)| x).min().unwrap();
        let xmax = *map.iter().map(|((x, _), _)| x).max().unwrap();

        println!("SCORE: {}", score);
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if let Some(tile) = map.get(&(x, y)) {
                    match tile {
                        Tile::Empty => print!(" "),
                        Tile::Wall => print!("X"),
                        Tile::Block => print!("#"),
                        Tile::Paddle => print!("-"),
                        Tile::Ball => print!("o"),
                    }
                } else {
                    print!("?");
                }
            }
            println!();
        }

        let ball = map.iter().filter(|(_, t)| **t == Tile::Ball).map(|(pos, _)| pos).next().unwrap();
        let paddle = map.iter().filter(|(_, t)| **t == Tile::Paddle).map(|(pos, _)| pos).next().unwrap();

        let direction = if ball.0 > paddle.0 && ball.1 < paddle.1 {
            println!("MOVING RIGHT");
            1
        } else if ball.0 < paddle.0 && ball.1 < paddle.1 {
            println!("MOVING LEFT");
            -1
        } else {
            println!("STAYING");
            0
        };

        if !done {
            (done, output, exec) = enter(exec, &[direction]);
        } else {
            break;
        }
    }

    score
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

    let answer2 = part2(&program);
    println!("Part 2: {}", answer2);

    Ok(())
}

use std::collections::{HashMap, VecDeque, HashSet};
use std::io;
use std::io::BufRead;
use std::cmp;

#[derive(Debug, Clone)]
enum Blizzard {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone)]
struct Tile {
    blizzards: Vec<Blizzard>,
    wall: bool,
}

type Point = (isize, isize);

type TInput = HashMap<Point, Tile>;

fn step(blizzards: &TInput) -> TInput {
    let mut next: HashMap<Point, Tile> = blizzards
        .iter()
        .map(|(pos, tile)| {
            (
                *pos,
                Tile {
                    blizzards: vec![],
                    wall: tile.wall,
                },
            )
        })
        .collect();

    let mut xmax = 0;
    let mut ymax = 0;

    for ((x, y), tile) in blizzards {
        let x = *x;
        let y = *y;
        xmax = cmp::max(xmax, x);
        ymax = cmp::max(ymax, y);

        for blizzard in &tile.blizzards {
            match blizzard {
                Blizzard::Left => next.get_mut(&(x - 1, y)).unwrap().blizzards.push(blizzard.clone()),
                Blizzard::Right => next.get_mut(&(x + 1, y)).unwrap().blizzards.push(blizzard.clone()),
                Blizzard::Up => next.get_mut(&(x, y - 1)).unwrap().blizzards.push(blizzard.clone()),
                Blizzard::Down => next.get_mut(&(x, y + 1)).unwrap().blizzards.push(blizzard.clone()),
            }
        }
    }

    for y in 0..=ymax {
        for x in 0..=xmax {
            let tile = next.get_mut(&(x, y)).unwrap();
            if !tile.wall || tile.blizzards.len() == 0 {
                continue;
            }
            let mut blizzards = tile.blizzards.clone();
            tile.blizzards.clear();

            if x == 0 {
                next.get_mut(&(xmax - 1, y)).unwrap().blizzards.append(&mut blizzards);
            } else if x == xmax {
                next.get_mut(&(1, y)).unwrap().blizzards.append(&mut blizzards);
            } else if y == 0 {
                next.get_mut(&(x, ymax - 1)).unwrap().blizzards.append(&mut blizzards);
            } else if y == ymax {
                next.get_mut(&(x, 1)).unwrap().blizzards.append(&mut blizzards);
            }
        }
    }

    next
}

fn run(map: &TInput, targets: &mut Vec<Point>) -> isize {
    let mut possibilities = VecDeque::from([((1, 0), 0)]);
    let cardinal = vec![(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut visited = HashSet::new();
    let mut maps = HashMap::from([(0, map.clone())]);

    while let Some(possibility) = possibilities.pop_front() {
        let (position, distance) = possibility;
        if position == targets[0] {
            if targets.len() == 1 {
                return distance - 1;
            }

            targets.remove(0);
            possibilities = VecDeque::new();
        }

        if !visited.insert((position, distance)) {
            continue;
        }

        if !maps.contains_key(&distance) {
            let prev = &maps[&(distance - 1)];
            maps.insert(distance, step(prev));
        }

        let map = &maps[&distance];
        let mut moved = false;
        for direction in &cardinal {
            let next_pos = (position.0 + direction.0, position.1 + direction.1);
            if let Some(tile) = map.get(&next_pos) {
                if !tile.wall && tile.blizzards.len() == 0 {
                    moved = true;
                    possibilities.push_back((next_pos, distance + 1));
                }
            }
        }
    }

    panic!();
}

fn part1(input: &TInput) -> isize {
    let xmax = *input.iter().map(|((x, _), _)| x).max().unwrap();
    let ymax = *input.iter().map(|((_, y), _)| y).max().unwrap();
    let mut targets = vec![(xmax - 1, ymax)];

    run(input, &mut targets)
}

fn part2(input: &TInput) -> isize {
    let xmax = *input.iter().map(|((x, _), _)| x).max().unwrap();
    let ymax = *input.iter().map(|((_, y), _)| y).max().unwrap();
    let mut targets = vec![(xmax - 1, ymax), (1, 0), (xmax - 1, ymax)];

    run(input, &mut targets)
}

fn print_map(map: &TInput, person: Point) {
    let xmax = *map.iter().map(|((x, _), _)| x).max().unwrap();
    let ymax = *map.iter().map(|((_, y), _)| y).max().unwrap();

    for y in 0..=ymax {
        for x in 0..=xmax {
            let tile = map.get(&(x, y)).unwrap();
            if (x, y) == person {
                print!("E");
            } else if tile.wall {
                print!("#");
            } else if tile.blizzards.len() == 0 {
                print!(".");
            } else if tile.blizzards.len() == 1 {
                match tile.blizzards[0] {
                    Blizzard::Up => print!("^"),
                    Blizzard::Down => print!("v"),
                    Blizzard::Left => print!("<"),
                    Blizzard::Right => print!(">"),
                };
            } else {
                print!("{}", tile.blizzards.len());
            }
        }
        println!("");
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .enumerate()
        .map(|(ridx, row)| row.chars().enumerate().map(|(cidx, c)| ((cidx as isize, ridx as isize), match c {
            '#' => Tile { wall: true, blizzards: vec![] },
            '.' => Tile { wall: false, blizzards: vec![] },
            '^' => Tile { wall: false, blizzards: vec![Blizzard::Up] },
            'v' => Tile { wall: false, blizzards: vec![Blizzard::Down] },
            '>' => Tile { wall: false, blizzards: vec![Blizzard::Right] },
            '<' => Tile { wall: false, blizzards: vec![Blizzard::Left] },
            _ => panic!(),
        })).collect::<Vec<(Point, Tile)>>())
        .flatten()
        .collect();

    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

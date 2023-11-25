use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
    
}

fn parse(description: &str) -> Vec<Direction> {
    let mut directions = vec![];
    let mut last = None;

    for tok in description.chars() {
        let out = match tok {
            'e' if last.is_none() => Some(Direction::East),
            'w' if last.is_none()  => Some(Direction::West),

            'n' if last.is_none()  => { last = Some(Direction::North); None},
            's' if last.is_none()  => { last = Some(Direction::South); None},

            'e' if last.is_some_and(|d| d == Direction::North) => {last = None; Some(Direction::Northeast) },
            'w' if last.is_some_and(|d| d == Direction::North) => {last = None; Some(Direction::Northwest) },
            'e' if last.is_some_and(|d| d == Direction::South) => {last = None; Some(Direction::Southeast) },
            'w' if last.is_some_and(|d| d == Direction::South) => {last = None; Some(Direction::Southwest) },

            _ => panic!("Unexpected, {} ({:?})!", tok, last),
        };

        if let Some(dir) = out {
            directions.push(dir);
        }
    }

    directions
}

fn init(directions: &[Vec<Direction>]) -> HashSet<(isize, isize)> {
    let mut flipped = HashSet::new();

    for direction in directions {
        let tile = follow((0, 0), &direction);
        
        if flipped.contains(&tile) {
            flipped.remove(&tile);
        } else {
            flipped.insert(tile);
        }
    }

    flipped
}

fn follow(origin: (isize, isize), steps: &[Direction]) -> (isize, isize) {
    let mut pos = origin;

    for step in steps {
        match step {
            Direction::East => pos = (pos.0 + 2, pos.1),
            Direction::West => pos = (pos.0 - 2, pos.1),
            Direction::Northeast => pos = (pos.0 + 1, pos.1 + 2),
            Direction::Southeast => pos = (pos.0 + 1, pos.1 - 2),
            Direction::Northwest => pos = (pos.0 - 1, pos.1 + 2),
            Direction::Southwest => pos = (pos.0 - 1, pos.1 - 2),
            _ => panic!("Unsupported direction"),
        }
    }

    pos
}

fn neighbors(tile: (isize, isize)) -> [(isize, isize); 6] {
    [(tile.0 + 2, tile.1),
     (tile.0 - 2, tile.1),
     (tile.0 + 1, tile.1 + 2),
     (tile.0 + 1, tile.1 - 2),
     (tile.0 - 1, tile.1 + 2),
     (tile.0 - 1, tile.1 - 2)]
}

fn part1(directions: &[Vec<Direction>]) -> usize {
    let flipped = init(directions);
    flipped.len()
}

fn part2(directions: &[Vec<Direction>]) -> usize {
    let mut map = init(directions);

    for round in 0..100 {
        let mut considered = HashMap::new();
        for tile in &map {
            for neighbor in neighbors(*tile) {
                considered.entry(neighbor).and_modify(|t| *t += 1).or_insert(1);
            }
        }
       
        let mut next = HashSet::new();
        for (tile, adjacent) in considered {
            if map.contains(&tile) {
                if adjacent <= 2 {
                    next.insert(tile);
                }
            } else {
                if adjacent == 2 {
                    next.insert(tile);
                }
            }
        }

        map = next;
    }

    map.len()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: Vec<Vec<Direction>> = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| parse(&line))
        .collect();

	let answer1 = part1(&input);
	println!("Answer 1: {}", answer1);

	let answer2 = part2(&input);
	println!("Answer 2: {}", answer2);

    Ok(())
}

use adventage::day;
use intcode::{run, Program};
use std::collections::HashSet;

day!(2019, 17);

type TInput = Program;
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Turn(Turn),
    Walk(usize),
}

fn turn(dir: Direction, turn: Turn) -> Direction {
    match (turn, dir) {
        (Turn::Left, Direction::Up) => Direction::Left,
        (Turn::Left, Direction::Left) => Direction::Down,
        (Turn::Left, Direction::Down) => Direction::Right,
        (Turn::Left, Direction::Right) => Direction::Up,
        (Turn::Right, Direction::Up) => Direction::Right,
        (Turn::Right, Direction::Right) => Direction::Down,
        (Turn::Right, Direction::Down) => Direction::Left,
        (Turn::Right, Direction::Left) => Direction::Up,
    }
}

fn step(direction: Direction, current: (isize, isize)) -> (isize, isize) {
    match direction {
        Direction::Up => (current.0 - 1, current.1),
        Direction::Down => (current.0 + 1, current.1),
        Direction::Left => (current.0, current.1 - 1),
        Direction::Right=> (current.0, current.1 + 1),
    }
}

fn part1(program: &TInput) -> isize {
    let exec = run(program, &[]);
    let map: String = exec.iter().map(|c| *c as u8 as char).collect();
    let scaffold: HashSet<(isize, isize)> = map
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| {
                    if c == '#' {
                        Some((row as isize, col as isize))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(isize, isize)>>()
        })
        .flatten()
        .collect();

    scaffold
        .iter()
        .filter(|(r, c)| {
            scaffold
                .iter()
                .filter(|(or, oc)| {
                    (or == r && oc.abs_diff(*c) == 1) || (or.abs_diff(*r) == 1 && oc == c)
                })
                .count()
                > 2
        })
        .map(|(r, c)| r * c)
        .sum()
}

fn part2(program: &TInput) -> isize {
    let exec = run(program, &[]);
    let map: String = exec.iter().map(|c| *c as u8 as char).collect();
    let mut start = None;

    let scaffold: HashSet<(isize, isize)> = map
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, c)| {
                    if c == '^' {
                        start = Some((row as isize, col as isize));
                    }
                    if c == '#' {
                        Some((row as isize, col as isize))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(isize, isize)>>()
        })
        .flatten()
        .collect();

    let mut current = start.unwrap().clone();
    let mut direction = Direction::Up;
    let mut path = vec![];
    let mut steps = 0;


    loop {
        let ahead = step(direction, current);
        if let Some(_) = scaffold.get(&ahead) {
            current = ahead;
            steps += 1;
            continue;
        } else {
            if steps > 0 {
                path.push(Step::Walk(steps));
            }
            steps = 0;
        }

        let left_dir = turn(direction, Turn::Left);
        let right_dir = turn(direction, Turn::Right);
        let left = step(left_dir, current);
        let right = step(right_dir, current);

        if let Some(_) = scaffold.get(&left) {
            path.push(Step::Turn(Turn::Left));
            direction = left_dir;
        } else if let Some(_) = scaffold.get(&right) {
            path.push(Step::Turn(Turn::Right));
            direction = right_dir;
        } else {
            break;
        }
    }

    println!("{}", path.len());

    let mut possible_as = vec![];

    for a_len in 1..12 {
        let a_len = a_len * 2;
        let a = &path[0..a_len];
        let mut a_idxs = vec![0];
        'a: for start in a_len..(path.len() - a_len) {
            for idx in 0..a_len {
                if path[idx] != path[start + idx] {
                    continue 'a;
                }
            }

            a_idxs.push(start);
        }

        let encoded = a.iter().map(|p| match p {
            Step::Turn(Turn::Left) => "L".to_string(),
            Step::Turn(Turn::Right) => "R".to_string(), 
            Step::Walk(amt) => amt.to_string(),
        }).collect::<Vec<String>>()
        .join(",");
        
        if encoded.len() <= 20 {
            possible_as.push((a, a_idxs));
        }
    }

    let mut possible_abs = vec![];

    for (a, a_idxs) in &possible_as {
        for b_len in 1..12 {
            let b_len = b_len * 2;
            let b = &path[a.len()..a.len() + b_len];
            let mut b_idxs = vec![a.len()];
            'a: for start in (a.len() + b_len)..(path.len() - b_len) {
                for idx in 0..b_len {
                    if path[a.len() + idx] != path[start + idx] || a_idxs.contains(&(start + idx)){
                        continue 'a;
                    }
                }

                b_idxs.push(start);
            }

            let encoded = b.iter().map(|p| match p {
                Step::Turn(Turn::Left) => "L".to_string(),
                Step::Turn(Turn::Right) => "R".to_string(), 
                Step::Walk(amt) => amt.to_string(),
            }).collect::<Vec<String>>()
            .join(",");
            
            if encoded.len() <= 20 {
                possible_abs.push(((a, a_idxs), (b, b_idxs)));
            }
        }
    }

    println!("{:?}", path);

    for ((a, a_idx), (b, b_idx)) in &possible_abs {
        let mut current = 0;
        let mut abbreviated = vec![];
        let mut c = None;
        let mut all_idx = a_idx.to_vec();
        all_idx.append(&mut b_idx.clone());
        println!("----");
        println!("A is {:?} at {:?}", a, a_idx);
        println!("B is {:?} at {:?}", b, b_idx);

        while current < path.len() {
            if a_idx.contains(&current) {
                abbreviated.push("A");
                current += a.len();
            } else if b_idx.contains(&current) {
                abbreviated.push("B");
                current += b.len();
            } else {
                let next_idx = *all_idx.iter().filter(|x| **x > current).min().unwrap_or(&path.len());

                let c_trial = &path[current..next_idx];
                if let Some(c) = c {
                    if c_trial != c {
                        println!("{:?} can't be C, {:?} left to go.", c_trial, &path[next_idx..]);
                        break;
                    }
                } else {
                    println!("C is {:?}, so far I have {:?}", c_trial, abbreviated);
                    c = Some(c_trial);
                }
                abbreviated.push("C");
                current += c_trial.len();
            }
        }

        if current >= path.len() {
            println!("I made it to the end! {:?}", abbreviated);
            panic!();
        } else {
            println!("{:?} {:?} {:?} {:?}", abbreviated, a, b, c);
        }
    }


    println!("{:#?}", possible_abs.len());

    0
}

fn parse(input: &str) -> TInput {
    input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

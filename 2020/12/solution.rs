use std::io::BufRead;
use std::io;
use std::cmp;
use std::convert::TryInto;

#[derive(Debug,Clone,Copy)]
enum Action{
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn rotate(heading: (i32, i32), times: i32) -> (i32, i32) {
    if times == 0 {
        return heading;
    }

    return rotate((-heading.1, heading.0), times - 1);
}

fn part1(directions: &Vec<Action>) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    let mut heading:  (i32, i32) = (-1, 0);

    for direction in directions {
        match direction {
            Action::North(distance) => position.1 += *distance,
            Action::South(distance) => position.1 -= *distance,
            Action::East(distance) => position.0 -= *distance,
            Action::West(distance) => position.0 += *distance,
            Action::Left(distance) => {
                let quarters = 4 - (distance / 90) % 4;
                heading = rotate(heading, quarters);
            },
            Action::Right(distance) => {
                let quarters = ((distance / 90) % 4);
                heading = rotate(heading, quarters);
            },
            Action::Forward(distance) => {
                let (hx, hy) = heading;
                let (dx, dy) = (hx * distance, hy * distance);

                position.0 += dx;
                position.1 += dy;
            }
        }
    }

    return (position.0.abs() + position.1.abs()) as i32;
}

fn part2(directions: &Vec<Action>) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (-10, 1);

    for direction in directions {
        match direction {
            Action::North(distance) => waypoint.1 += *distance,
            Action::South(distance) => waypoint.1 -= *distance,
            Action::East(distance) => waypoint.0 -= *distance,
            Action::West(distance) => waypoint.0 += *distance,
            Action::Left(distance) => {
                let quarters = 4 - (distance / 90) % 4;
                waypoint = rotate(waypoint, quarters);
            },
            Action::Right(distance) => {
                let quarters = ((distance / 90) % 4);
                waypoint = rotate(waypoint, quarters);
            },
            Action::Forward(distance) => {
                let (hx, hy) = waypoint;
                let (dx, dy) = (hx * distance, hy * distance);

                position.0 += dx;
                position.1 += dy;
            }
        }
    }

    return (position.0.abs() + position.1.abs()) as i32;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let actions: Vec<Action> = reader.lines().map(|line| {
        let line = line.unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        match line.chars().nth(0).unwrap() {
            'N' => Action::North(value),
            'S' => Action::South(value),
            'E' => Action::East(value),
            'W' => Action::West(value),
            'L' => Action::Left(value),
            'R' => Action::Right(value),
            'F' => Action::Forward(value),
            _ => panic!()
        }
    }).collect();

	let answer1 = part1(&actions);
	let answer2 = part2(&actions);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}

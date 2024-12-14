use adventage::{day, part1demo, part2demo};
use std::{thread, time};
use std::collections::HashSet;

day!(2024, 14);
part1demo!(
    "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    12
);

#[derive(Copy, Clone, Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

type TInput = Vec<Robot>;

fn room_size(robots: &TInput) -> (isize, isize) {
    if robots.len() > 20 {
        (101, 103)
    } else {
        (11, 7)
    }
}

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|l| {
            let (position, velocity) = l.split_once(" ").unwrap();
            let position: Vec<isize> = position[2..]
                .split(",")
                .map(|p| p.parse().unwrap())
                .collect();
            let velocity: Vec<isize> = velocity[2..]
                .split(",")
                .map(|p| p.parse().unwrap())
                .collect();

            Robot {
                position: (position[0], position[1]),
                velocity: (velocity[0], velocity[1]),
            }
        })
        .collect()
}

fn print_room(robots: &TInput) {
    let size = room_size(robots);
    let (row, col) = (size.0 as usize, size.1 as usize);
    let mut room = vec![vec![0; col]; row];

    for robot in robots {
        room[robot.position.0 as usize][robot.position.1 as usize] += 1;
    }

    for col in 0..col {
        for row in 0..row {
            if room[row][col] > 0 {
                print!(".");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

fn mirrored_pct(robots: &TInput) -> usize {
    let room = room_size(robots);
    let robots: HashSet<_> = robots.iter().map(|r| r.position).collect();
    let mut mirrored = 0;

    for robot in &robots {
        if robot.0 >= room.0 / 2 {
            continue
        }

        if robots.contains(&(room.0 - robot.0 - 1, robot.1)) {
            mirrored += 1;
        }
    }

    (mirrored * 100) / robots.len()
}

fn part1(robots: &TInput) -> u32 {
    let room = room_size(robots);
    let mut robots = robots.clone();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position.0 += robot.velocity.0;
            if robot.position.0 >= room.0 {
                robot.position.0 -= room.0;
            } else if robot.position.0 < 0 {
                robot.position.0 += room.0;
            }

            robot.position.1 += robot.velocity.1;
            if robot.position.1 >= room.1 {
                robot.position.1 -= room.1;
            } else if robot.position.1 < 0 {
                robot.position.1 += room.1;
            }
        }
    }

    let halfway = (room.0 / 2, room.1 / 2);

    robots
        .iter()
        .filter(|robot| robot.position.0 != halfway.0 && robot.position.1 != halfway.1)
        .fold(vec![0, 0, 0, 0], |mut acc, &robot| {
            let idx = match (robot.position.0 > halfway.0, robot.position.1 > halfway.1) {
                (false, false) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (true, true) => 3,
            };

            acc[idx] += 1;
            acc
        })
        .iter()
        .product()
}

fn part2(robots: &TInput) -> u32 {
    let room = room_size(robots);
    let mut robots = robots.clone();
    for step in 0..10403 {
        for robot in robots.iter_mut() {
            robot.position.0 += robot.velocity.0;
            if robot.position.0 >= room.0 {
                robot.position.0 -= room.0;
            } else if robot.position.0 < 0 {
                robot.position.0 += room.0;
            }

            robot.position.1 += robot.velocity.1;
            if robot.position.1 >= room.1 {
                robot.position.1 -= room.1;
            } else if robot.position.1 < 0 {
                robot.position.1 += room.1;
            }
        }

        if mirrored_pct(&robots) > 10 {
            print_room(&robots);
            return step + 1;
        }

    }

    panic!()
}

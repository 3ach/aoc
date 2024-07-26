use intcode::{init, enter, Program};
use adventage::day;
use std::collections::HashSet;

day!(2019, 15);

type TInput = Program;

fn explore(program: &TInput) -> (HashSet<(isize, isize)>, (isize, isize)) {
    let exec = init(program, &[]);
    let mut states = vec![((0isize, 0isize), exec)];
    let mut map: HashSet<(isize, isize)> = HashSet::new();
    let mut tank = None;

    while let Some((loc, state)) = states.pop() {
        if !map.insert(loc) {
            continue;
        }

        let mut up = state.clone();
        let (_, up_result) = enter(&mut up, &[1]);
        match up_result[0] {
            1 => states.push(((loc.0, loc.1 + 1), up)),
            2 => { 
                states.push(((loc.0, loc.1 + 1), up));
                tank = Some((loc.0, loc.1 + 1));
            },
            _ => {}
        };

        let mut down = state.clone();
        let (_, down_result) = enter(&mut down, &[2]);
        match down_result[0] {
            1 => states.push(((loc.0, loc.1 - 1), down)),
            2 => { 
                states.push(((loc.0, loc.1 - 1), down));
                tank = Some((loc.0, loc.1 - 1));
            },
            _ => {}
        };

        let mut left = state.clone();
        let (_, left_result) = enter(&mut left, &[4]);
        match left_result[0] {
            1 => states.push(((loc.0 - 1, loc.1), left)),
            2 => { 
                states.push(((loc.0 - 1, loc.1), left));
                tank = Some((loc.0 - 1, loc.1));
            },
            _ => {}
        };

        let mut right = state.clone();
        let (_, right_result) = enter(&mut right, &[3]);
        match right_result[0] {
            1 => states.push(((loc.0 + 1, loc.1), right)),
            2 => { 
                states.push(((loc.0 + 1, loc.1), right));
                tank = Some((loc.0 + 1, loc.1));
            },
            _ => {}
        };
    }

    (map, tank.unwrap())
}

fn part1(program: &TInput) -> usize {
    let (map, tank) = explore(program);
    let mut to_explore = vec![((0, 0), 0)];
    let mut explored = HashSet::new();
    while let Some((point, distance)) = to_explore.pop() {
        if point == tank {
            return distance;
        }

        if !explored.insert(point) {
            continue;
        }

        if map.contains(&(point.0, point.1 + 1)) {
            to_explore.push(((point.0, point.1 + 1), distance + 1));
        }

        if map.contains(&(point.0, point.1 - 1)) {
            to_explore.push(((point.0, point.1 - 1), distance + 1));
        }

        if map.contains(&(point.0 + 1, point.1)) {
            to_explore.push(((point.0 + 1, point.1), distance + 1));
        }

        if map.contains(&(point.0 - 1, point.1)) {
            to_explore.push(((point.0 - 1, point.1), distance + 1));
        }

    }

    panic!();
}

fn part2(program: &TInput) -> usize {
    let (map, tank) = explore(program);
    let mut to_explore = vec![(tank, 0)];
    let mut explored = HashSet::new();
    let mut max_time = 0;

    while let Some((point, time)) = to_explore.pop() {
        if !explored.insert(point) {
            continue;
        }

        if time > max_time {
            max_time = time;
        }

        if map.contains(&(point.0, point.1 + 1)) {
            to_explore.insert(0, ((point.0, point.1 + 1), time + 1));
        }

        if map.contains(&(point.0, point.1 - 1)) {
            to_explore.insert(0, ((point.0, point.1 - 1), time + 1));
        }

        if map.contains(&(point.0 + 1, point.1)) {
            to_explore.insert(0, ((point.0 + 1, point.1), time + 1));
        }

        if map.contains(&(point.0 - 1, point.1)) {
            to_explore.insert(0, ((point.0 - 1, point.1), time + 1));
        }

    }

    max_time
}


fn parse(input: &str) -> TInput {
        input.trim()
            .split(",")
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
}

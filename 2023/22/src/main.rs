use adventage::{day, part1demo, part2demo};
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

part1demo!(
    "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
    5
);

part2demo!(
    "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
    7
);

part1demo!(
    "0,0,3~0,0,5
3,0,3~3,0,5
3,3,3~3,3,5
0,3,3~0,3,5
0,0,1~0,3,1
3,0,7~3,3,7
0,0,9~3,0,9",
    6
);

part1demo!(
    "0,0,1~0,0,5
0,0,7~0,3,7
0,3,6~0,3,6",
    2
);

day!(2023, 22);

type Point = (usize, usize, usize);
type TInput = HashMap<Point, usize>;

fn comma_to_point(coord: &str) -> Point {
    let point = coord
        .split(',')
        .map(|p| p.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (point[0], point[1], point[2])
}

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(brick, line)| {
            let (start, end) = line.split_once("~").unwrap();
            let start = comma_to_point(start);
            let end = comma_to_point(end);

            let mut cubes = vec![];

            for x in min(start.0, end.0)..=max(start.0, end.0) {
                for y in min(start.1, end.1)..=max(start.1, end.1) {
                    for z in min(start.2, end.2)..=max(start.2, end.2) {
                        cubes.push(((x, y, z), brick));
                    }
                }
            }

            cubes
        })
        .flatten()
        .collect()
}

fn settle(bricks: &TInput) -> (TInput, Vec<Vec<usize>>) {
    let mut settled = HashMap::new();
    let mut supports = vec![vec![]; *bricks.values().max().unwrap() + 1];
    let max_z = *bricks.keys().map(|(_, _, z)| z).max().unwrap();

    for z in 1..=max_z {
        for brick in bricks
            .iter()
            .filter_map(|((_, _, bz), b)| if z == *bz { Some(b) } else { None })
        {
            let mut points: Vec<Point> = bricks
                .iter()
                .filter_map(|(pt, b)| {
                    if b == brick && !settled.values().any(|v| v == b) {
                        Some(pt.clone())
                    } else {
                        None
                    }
                })
                .collect();
            for _ in 1..z {
                let mut below = vec![];
                for point in &points {
                    if let Some(brick_below) = settled.get(&(point.0, point.1, point.2 - 1)) {
                        if !below.contains(brick_below) && brick_below != brick {
                            below.push(*brick_below);
                        }
                    }
                }

                if !below.is_empty() {
                    supports[*brick] = below;
                    break;
                }

                points = points.iter().map(|p| (p.0, p.1, p.2 - 1)).collect();
            }

            for point in points {
                settled.insert(point.clone(), *brick);
            }
        }
    }

    (settled, supports)
}

fn part1(midair: &TInput) -> usize {
    let (_, supports) = settle(midair);

    supports
        .iter()
        .enumerate()
        .filter(|(brick, _)| {
            !supports
                .iter()
                .any(|supporters| supporters.len() == 1 && supporters.contains(brick))
        })
        .count()
}

fn part2(midair: &TInput) -> usize {
    let (_, supports) = settle(midair);
    let mut directs = vec![vec![]; supports.len()];

    for brick in 0..supports.len() {
        for below in &supports[brick] {
            directs[*below].push(brick);
        }
    }

    println!("{:?}", directs);

    let mut chains: Vec<usize> = vec![];
    for idx in 0..supports.len() {
        let mut falling = HashSet::new();
        let mut to_fall = VecDeque::from(directs[idx].clone());
        while let Some(brick) = to_fall.pop_front() {
            if supports[brick].iter().all(|supporter| falling.contains(supporter) || *supporter == idx) {
                    falling.insert(brick);
                    for child in &directs[brick] {
                        to_fall.push_back(*child);
                    }
            }
        }

        chains.push(falling.len());
    }

    println!("{:?}", chains);

    chains.iter().sum()
}

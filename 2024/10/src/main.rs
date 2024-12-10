use adventage::{day, part1demo, part2demo};
use std::collections::{HashMap, HashSet, VecDeque};

day!(2024, 10);
part1demo!(
    "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    36
);
part2demo!(
    "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    81
);

type Point = (isize, isize);
type TInput = HashMap<Point, u32>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(move |(col, c)| ((col as isize, row as isize), (c as u32 - '0' as u32)))
        })
        .flatten()
        .collect()
}

fn neighbors(point: &Point) -> [Point; 4] {
    [
        (point.0 - 1, point.1),
        (point.0 + 1, point.1),
        (point.0, point.1 - 1),
        (point.0, point.1 + 1),
    ]
}

fn part1(map: &TInput) -> usize {
    let mut to_explore: VecDeque<(Point, Point)> = map
        .iter()
        .filter_map(|(pos, height)| {
            if *height == 9 {
                Some((pos.clone(), pos.clone()))
            } else {
                None
            }
        })
        .collect();

    let mut reachable: HashMap<Point, HashSet<Point>> = map
        .iter()
        .map(|(pos, _)| (pos.clone(), HashSet::new()))
        .collect();

    while let Some((pos, peak)) = to_explore.pop_front() {
        let height = *map.get(&pos).unwrap();

        let mut adjacent = neighbors(&pos)
            .into_iter()
            .filter_map(|neighbor| match map.get(&neighbor) {
                Some(neighbor_height) if height == neighbor_height + 1 => {
                    Some((neighbor, peak.clone()))
                }
                _ => None,
            })
            .collect();

        reachable.get_mut(&pos).unwrap().insert(peak);

        to_explore.append(&mut adjacent);
    }

    map.iter()
        .map(|(pos, height)| {
            if *height == 0 {
                reachable.get(pos).unwrap().len()
            } else {
                0
            }
        })
        .sum()
}

fn part2(map: &TInput) -> u32 {
    let mut to_explore: VecDeque<Point> = map
        .iter()
        .filter_map(|(pos, height)| {
            if *height == 9 {
                Some(pos.clone())
            } else {
                None
            }
        })
        .collect();

    let mut reachable: HashMap<Point, u32> = map.iter().map(|(pos, _)| (pos.clone(), 0)).collect();

    while let Some(pos) = to_explore.pop_front() {
        let height = *map.get(&pos).unwrap();

        let mut adjacent = neighbors(&pos)
            .into_iter()
            .filter(|neighbor| match map.get(&neighbor) {
                Some(neighbor_height) if height == neighbor_height + 1 => true,
                _ => false,
            })
            .collect();

        *reachable.get_mut(&pos).unwrap() += 1;

        to_explore.append(&mut adjacent);
    }

    map.iter()
        .map(|(pos, height)| {
            if *height == 0 {
                *reachable.get(pos).unwrap()
            } else {
                0
            }
        })
        .sum()
}

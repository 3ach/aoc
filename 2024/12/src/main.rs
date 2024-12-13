#![feature(iter_map_windows)]
use adventage::{day, part1demo, part2demo};
use std::collections::{HashMap, HashSet};

day!(2024, 12);
part1demo!(
    "AAAA
BBCD
BBCC
EEEC",
    140
);
part1demo!(
    "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
    772
);
part1demo!(
    "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    1930
);
part2demo!(
    "AAAA
BBCD
BBCC
EEEC",
    80
);
part2demo!(
    "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
    436
);
part2demo!(
    "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
    236
);
part2demo!(
    "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
    368
);
part2demo!(
    "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    1206
);

type Point = (isize, isize);
type TInput = HashMap<Point, char>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(move |(col, c)| ((col as isize, row as isize), c))
        })
        .flatten()
        .collect()
}

fn neighbors4(point: Point) -> Vec<Point> {
    vec![
        (point.0 + 1, point.1),
        (point.0 - 1, point.1),
        (point.0, point.1 + 1),
        (point.0, point.1 - 1),
    ]
}

fn neighbors8(point: Point) -> Vec<Point> {
    vec![
        (point.0 + 1, point.1),
        (point.0 + 1, point.1 - 1),
        (point.0, point.1 - 1),
        (point.0 - 1, point.1 - 1),
        (point.0 - 1, point.1),
        (point.0 - 1, point.1 + 1),
        (point.0, point.1 + 1),
        (point.0 + 1, point.1 + 1),
    ]
}

fn area(region: &HashSet<Point>) -> usize {
    region.len()
}

fn perimeter(region: &HashSet<Point>) -> usize {
    region
        .iter()
        .map(|pt| {
            4 - neighbors4(*pt)
                .iter()
                .filter(|pt| region.contains(*pt))
                .count()
        })
        .sum()
}

fn edges(region: &HashSet<Point>) -> usize {
    region
        .iter()
        .map(|pt| {
            neighbors8(*pt)
                .iter()
                .map(|neighbor| region.contains(neighbor))
                .cycle()
                .map_windows(|[edge1, corner, edge2]| {
                    if (!*corner && ((*edge1 && *edge2) || !(*edge1 || *edge2)))
                        || (*corner && !*edge1 && !edge2)
                    {
                        1
                    } else {
                        0
                    }
                })
                .step_by(2)
                .take(4)
                .sum::<usize>()
        })
        .sum()
}

fn regions(garden: &TInput) -> Vec<HashSet<Point>> {
    let mut regions = vec![];
    let mut assigned = HashSet::new();
    let mut deferred = vec![(0, 0)];

    while let Some(anchor) = deferred.pop() {
        if assigned.contains(&anchor) {
            continue;
        }

        let name = *garden.get(&anchor).unwrap();
        let mut to_explore = vec![anchor];
        let mut region = HashSet::new();

        while let Some(point) = to_explore.pop() {
            if !region.insert(point) {
                continue;
            }

            assigned.insert(point);

            let (mut same, mut unrelated): (Vec<Point>, Vec<Point>) = neighbors4(point)
                .iter()
                .filter(|pt| garden.contains_key(*pt))
                .partition(|pt| *garden.get(*pt).unwrap() == name);

            to_explore.append(&mut same);
            deferred.append(&mut unrelated);
        }

        regions.push(region);
    }

    regions
}

fn part1(garden: &TInput) -> usize {
    regions(garden)
        .iter()
        .map(|region| area(region) * perimeter(region))
        .sum()
}

fn part2(garden: &TInput) -> usize {
    regions(garden)
        .iter()
        .map(|region| area(region) * edges(region))
        .sum()
}

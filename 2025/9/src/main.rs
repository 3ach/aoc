use adventage::{day, part1demo, part2demo};
use std::cmp::{max, min};

day!(2025, 9);
part1demo!(
    "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    50
);
part2demo!(
    "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
    24
);

type Point = (u64, u64);
type TInput = Vec<Point>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn part1(tiles: &TInput) -> u64 {
    let mut best = 0;
    for first_idx in 0..tiles.len() {
        let first = tiles[first_idx];
        for second_idx in (first_idx + 1)..tiles.len() {
            let second = tiles[second_idx];
            let area = (first.0.abs_diff(second.0) + 1) * (first.1.abs_diff(second.1) + 1);
            best = max(best, area);
        }
    }

    best
}

fn edges(corners: &TInput) -> Vec<(Point, Point)> {
    (0..corners.len())
        .map(|start| (corners[start], corners[(start + 1) % corners.len()]))
        .collect()
}

fn within(rect: (Point, Point), pt: Point) -> bool {
    let top = max(rect.0.1, rect.1.1);
    let bottom = min(rect.0.1, rect.1.1);
    let right = max(rect.0.0, rect.1.0);
    let left = min(rect.0.0, rect.1.0);

    pt.0 > left && pt.0 < right && pt.1 > bottom && pt.1 < top
}

fn part2(tiles: &TInput) -> u64 {
    let mut best = 0;
    let edges = edges(tiles);

    for first_idx in 0..tiles.len() {
        let first = tiles[first_idx];
        'rect: for second_idx in (first_idx + 1)..tiles.len() {
            let second = tiles[second_idx];
            let area = (first.0.abs_diff(second.0) + 1) * (first.1.abs_diff(second.1) + 1);

            if area > best {
                for edge in &edges {
                    if edge.0.0 == edge.1.0 {
                        let ymin = min(edge.0.1, edge.1.1);
                        let ymax = max(edge.0.1, edge.1.1);
                        for y in ymin..=ymax {
                            if within((first, second), (edge.0.0, y)) {
                                continue 'rect;
                            }
                        }
                    } else {
                        let xmin = min(edge.0.0, edge.1.0);
                        let xmax = max(edge.0.0, edge.1.0);
                        for x in xmin..=xmax {
                            if within((first, second), (x, edge.0.1)) {
                                continue 'rect;
                            }
                        }
                    }
                }
                best = max(best, area);
            }
        }
    }

    best
}

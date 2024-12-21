use adventage::{day, part1demo, part2demo};
use std::collections::{HashSet, VecDeque};

day!(2024, 20);
part1demo!(
    "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
    44
);
part2demo!(
    "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
    285
);

type Point = (isize, isize);
#[derive(Debug, Clone)]
struct Maze {
    spaces: HashSet<Point>,
    start: Point,
    end: Point,
    max: Point,
}
type TInput = Maze;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(row, r)| {
            r.chars()
                .enumerate()
                .map(move |(col, c)| ((col as isize, row as isize), c))
        })
        .flatten()
        .fold(
            Maze {
                spaces: HashSet::new(),
                start: (0, 0),
                end: (0, 0),
                max: (0, 0),
            },
            |mut maze, ((col, row), c)| {
                if col > maze.max.0 {
                    maze.max.0 = col;
                }

                if row > maze.max.1 {
                    maze.max.1 = col;
                }

                match c {
                    '.' => {
                        maze.spaces.insert((col, row));
                    }
                    'S' => {
                        maze.start = (col, row);
                        maze.spaces.insert((col, row));
                    }
                    'E' => {
                        maze.end = (col, row);
                        maze.spaces.insert((col, row));
                    }
                    _ => {}
                };

                maze
            },
        )
}

fn neighbors(point: &Point) -> Vec<Point> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .map(|(dx, dy)| (point.0 + *dx, point.1 + *dy))
        .collect()
}

fn route(maze: &TInput) -> Vec<Point> {
    let mut seen = HashSet::new();
    let mut to_explore = VecDeque::from([(maze.start, vec![])]);

    while let Some((current, mut path)) = to_explore.pop_front() {
        if !seen.insert(current) {
            continue;
        }

        path.push(current);

        if current == maze.end {
            return path;
        }

        for neighbor in neighbors(&current) {
            if maze.spaces.contains(&neighbor) {
                to_explore.push_back((neighbor, path.clone()));
            }
        }
    }

    panic!()
}

fn distance(a: &Point, b: &Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn cheats(path: &Vec<Point>, max: usize) -> Vec<usize> {
    path.iter()
        .enumerate()
        .map(|(along, start)| {
            path.iter()
                .enumerate()
                .skip(along + 1)
                .filter(|(_, end)| distance(start, *end) <= max)
                .filter_map(move |(further_along, end)| {
                    let savings = further_along - along;
                    let cost = distance(start, end);
                    if savings > cost {
                        Some(savings - cost)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn part1(maze: &TInput) -> usize {
    let mut maze = maze.clone();
    let threshold = if maze.max.0 > 15 { 100 } else { 1 };

    let path = route(&mut maze);
    cheats(&path, 2)
        .into_iter()
        .filter(|cheat| *cheat >= threshold)
        .count()
}

fn part2(maze: &TInput) -> usize {
    let mut maze = maze.clone();
    let threshold = if maze.max.0 > 15 { 100 } else { 50 };

    let path = route(&mut maze);
    cheats(&path, 20)
        .into_iter()
        .filter(|cheat| *cheat >= threshold)
        .count()
}

use adventage::{day, part1demo, part2demo};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

day!(2024, 16);
part1demo!(
    "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
    7036
);
part1demo!(
    "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    11048
);
part2demo!(
    "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
    45
);
part2demo!(
    "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    64
);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Wall,
    Open,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}
type Point = (isize, isize);

type TInput = HashMap<Point, Tile>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(row, r)| {
            r.chars().enumerate().map(move |(col, c)| {
                ((col as isize, row as isize), match c {
                    'E' => Tile::End,
                    'S' => Tile::Start,
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    _ => panic!(),
                })
            })
        })
        .flatten()
        .collect()
}

fn neighbors(point: &Point) -> Vec<(Point, Direction)> {
    let directions = [
        ((0, -1), Direction::North),
        ((1, 0), Direction::East),
        ((0, 1), Direction::South),
        ((-1, 0), Direction::West),
    ];

    directions
        .iter()
        .map(|(d, direction)| ((point.0 + d.0, point.1 + d.1), *direction))
        .collect()
}

fn part1(maze: &TInput) -> u32 {
    let start = *maze
        .iter()
        .filter_map(|(pos, tile)| {
            if *tile == Tile::Start {
                Some(pos)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    let mut to_explore = PriorityQueue::new();
    let mut cheapests = HashMap::new();
    to_explore.push((start, Direction::East), Reverse(0));

    while let Some(((current, direction), cost)) = to_explore.pop() {
        let cost = cost.0;
        if *maze.get(&current).unwrap() == Tile::End {
            return cost;
        }

        if let Some(cheapest) = cheapests.get(&(current, direction)) {
            if *cheapest <= cost {
                continue;
            }
        }

        cheapests.insert((current, direction), cost);
        for (neighbor, neighbor_direction) in neighbors(&current) {
            let cost = cost
                + if neighbor_direction == direction {
                    1
                } else {
                    1001
                };

            if let Some(tile) = maze.get(&neighbor) {
                if *tile == Tile::Open || *tile == Tile::End {
                    to_explore.push((neighbor, neighbor_direction), Reverse(cost));
                }
            }
        }
    }

    panic!()
}

fn part2(maze: &TInput) -> usize {
    let start = *maze
        .iter()
        .filter_map(|(pos, tile)| {
            if *tile == Tile::Start {
                Some(pos)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    let mut to_explore = PriorityQueue::new();
    let mut cheapests = HashMap::new();

    to_explore.push(
        (start, (start, Direction::East), Direction::East),
        Reverse(0),
    );

    while let Some(((current, predecessor, direction), cost)) = to_explore.pop() {
        let cost = cost.0;

        let entry = cheapests
            .entry((current, direction))
            .or_insert((cost, vec![]));
        if cost > entry.0 {
            continue;
        }

        if cost < entry.0 {
            *entry = (cost, vec![]);
        }

        entry.1.push(predecessor);

        if *maze.get(&current).unwrap() == Tile::End {
            break;
        }

        let opposite = match direction {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        };

        for (neighbor, neighbor_direction) in neighbors(&current) {
            if let Some(tile) = maze.get(&neighbor) {
                if *tile == Tile::Wall {
                    continue;
                }

                if neighbor_direction == direction {
                    to_explore.push(
                        (neighbor, (current, direction), neighbor_direction),
                        Reverse(cost + 1),
                    );
                } else if neighbor_direction != opposite {
                    to_explore.push(
                        (current, (current, direction), neighbor_direction),
                        Reverse(cost + 1000),
                    );
                }
            }
        }
    }
    let end = *maze
        .iter()
        .filter_map(
            |(pos, tile)| {
                if *tile == Tile::End { Some(pos) } else { None }
            },
        )
        .next()
        .unwrap();

    let mut ends = cheapests
        .iter()
        .filter(|((cell, d), _)| *cell == end)
        .map(|(space, _)| space.clone())
        .collect::<Vec<_>>();
    let mut along = HashSet::new();
    while let Some(current) = ends.pop() {
        if !along.insert(current) {
            continue;
        }

        ends.append(
            &mut cheapests
                .get(&current)
                .iter()
                .map(|(c, p)| p.clone())
                .flatten()
                .collect(),
        );
    }
    along.iter().map(|(c, d)| c).collect::<HashSet<_>>().len()
}

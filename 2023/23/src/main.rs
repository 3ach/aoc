use adventage::{part1demo, part2demo, day};
use std::collections::HashMap;
use std::collections::HashSet;

part1demo!("#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#", 94);


part2demo!("#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#", 154);

day!(2023, 23);

type Point = (isize, isize);
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Wall,
    Open,
    Down,
    Up,
    Left,
    Right
}

type TInput = HashMap<Point, Tile>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(row, r)| r.chars()
             .enumerate()
             .map(move |(col, t)| ((row as isize, col as isize), match t {
                 '#' => Tile::Wall,
                 '.' => Tile::Open,
                 'v' => Tile::Down, 
                 '^' => Tile::Up,
                 '>' => Tile::Right,
                 '<' => Tile::Left,
                 _ => panic!()
             }))
             .collect::<TInput>()
        ).flatten()
        .collect()
}

fn neighbors(point: Point) -> [Point; 4] {
    [(point.0 + 1, point.1),
     (point.0 - 1, point.1),
     (point.0, point.1 + 1),
     (point.0, point.1 - 1)]
}

fn allowed(tile: Tile, from: Point, to: Point) -> bool {
    match tile {
        Tile::Open => true, 
        Tile::Up if from.0 > to.0 => true,
        Tile::Down if from.0 < to.0 => true,
        Tile::Left if from.1 > to.1 => true,
        Tile::Right if from.1 < to.1 => true,
        _ => false,
    }
}

fn coalesce(map: &TInput, start: Point, end: Point, slip: bool) -> HashMap<Point, HashMap<Point, usize>> {
    let mut nodes = HashMap::new();
    let mut unvisited = vec![(start, (start.0 + 1, start.1))];

    while let Some((node, first)) = unvisited.pop() {
        let mut path = vec![node, first];
        loop {
            let current: Point = *path.last().unwrap();
            let neighbors: Vec<Point> = neighbors(current).iter()
                .filter(|neighbor| map.contains_key(neighbor))
                .filter(|neighbor| *map.get(neighbor).unwrap() != Tile::Wall)
                .filter(|neighbor| !slip || allowed(*map.get(&current).unwrap(), current, **neighbor))
                .filter(|neighbor| !path.contains(neighbor))
                .cloned()
                .collect();

            if neighbors.len() > 1 {
                if !nodes.contains_key(&current) {
                    for neighbor in neighbors {
                        unvisited.push((current, neighbor));
                    }
                }

                nodes.entry(node).or_insert(HashMap::new()).insert(current, path.len() - 1);
                if !slip {
                    nodes.entry(current).or_insert(HashMap::new()).insert(node, path.len() - 1);
                }
                break;
            } else if neighbors.len() == 1 {
                let neighbor = *neighbors.first().unwrap();
                path.push(neighbor);
            } else if current == end {
                nodes.entry(node).or_insert(HashMap::new()).insert(current, path.len() - 1);
                break;
            } else {
                break;
            }
        }
    }

    nodes
}

fn longest(map: HashMap<Point, HashMap<Point, usize>>, start: Point, end: Point) -> usize {
    let mut completed = vec![];
    let mut in_progress = vec![(start, 0usize, HashSet::new())];

    while let Some((point, length, visited)) = in_progress.pop() {
        let from_point = map.get(&point).unwrap();

        for (next, distance) in from_point {
            if *next == end {
                completed.push(length + distance);
            } else if !visited.contains(next) {
                let mut visited = visited.clone();
                visited.insert(next);
                in_progress.push((*next, length + distance, visited));
            }
        }
    }

    *completed.iter().max().unwrap()
}


fn part1(map: &TInput) -> usize {
    let start = *map.iter()
        .filter_map(|(pt, tile)| if *tile == Tile::Open { Some(pt) } else { None })
        .min_by_key(|pt| pt.0)
        .unwrap();

    let end = *map.iter()
        .filter_map(|(pt, tile)| if *tile == Tile::Open { Some(pt) } else { None })
        .max_by_key(|pt| pt.0)
        .unwrap();

    let coalesced = coalesce(map, start, end, true);
    longest(coalesced, start, end)
}

fn part2(map: &TInput) -> usize {
    let start = *map.iter()
        .filter_map(|(pt, tile)| if *tile == Tile::Open { Some(pt) } else { None })
        .min_by_key(|pt| pt.0)
        .unwrap();

    let end = *map.iter()
        .filter_map(|(pt, tile)| if *tile == Tile::Open { Some(pt) } else { None })
        .max_by_key(|pt| pt.0)
        .unwrap();

    let coalesced = coalesce(map, start, end, false);

    longest(coalesced, start, end)
}
